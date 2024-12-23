use std::path::Path;

// 3 file(s).
// File(s) read by the parser:
// GLEIS, GLEIS_LV95, GLEIS_WGS
// ---
// Note: this parser collects both the Platform and JourneyPlatform resources.
use rustc_hash::FxHashMap;

use crate::{
    error::{Error, OptionExt},
    models::{CoordinateSystem, Coordinates, JourneyPlatform, Model, Platform},
    parsing::{
        ColumnDefinition, ExpectedType, FastRowMatcher, FileParser, ParsedValue, RowDefinition,
        RowParser,
    },
    storage::ResourceStorage,
    utils::{create_time_from_value, AutoIncrement},
};

pub fn parse(
    path: &Path,
    journeys_pk_type_converter: &FxHashMap<(i32, String), i32>,
) -> Result<(ResourceStorage<JourneyPlatform>, ResourceStorage<Platform>), Error> {
    log::info!("Parsing GLEIS...");
    const ROW_A: i32 = 1;
    const ROW_B: i32 = 2;

    #[rustfmt::skip]
    let row_parser = RowParser::new(vec![
        // This row is used to create a JourneyPlatform instance.
        RowDefinition::new(ROW_A, Box::new(FastRowMatcher::new(9, 1, "#", false)), vec![
            ColumnDefinition::new(1, 7, ExpectedType::Integer32),
            ColumnDefinition::new(9, 14, ExpectedType::Integer32),
            ColumnDefinition::new(16, 21, ExpectedType::String),
            ColumnDefinition::new(24, 30, ExpectedType::Integer32), // Should be 23-30, but here the # character is ignored.
            ColumnDefinition::new(32, 35, ExpectedType::OptionInteger32),
            ColumnDefinition::new(37, 42, ExpectedType::OptionInteger32),
        ]),
        // This row is used to create a Platform instance.
        RowDefinition::new(ROW_B, Box::new(FastRowMatcher::new(9, 1, "#", true)), vec![
            ColumnDefinition::new(1, 7, ExpectedType::Integer32),
            ColumnDefinition::new(10, 16, ExpectedType::Integer32), // Should be 9-16, but here the # character is ignored.
            ColumnDefinition::new(18, -1, ExpectedType::String),
        ]),
    ]);
    let parser = FileParser::new(&path.join("GLEIS"), row_parser)?;

    let auto_increment = AutoIncrement::new();
    let mut platforms = Vec::new();
    let mut platforms_pk_type_converter = FxHashMap::default();

    let mut bytes_offset = 0;
    let mut journey_platform = Vec::new();

    for x in parser.parse() {
        let (id, bytes_read, values) = x?;
        match id {
            ROW_A => {
                bytes_offset += bytes_read;
                journey_platform.push(values);
            }
            ROW_B => {
                platforms.push(create_platform(
                    values,
                    &auto_increment,
                    &mut platforms_pk_type_converter,
                )?);
            }
            _ => unreachable!(),
        }
    }

    let mut platforms = Platform::vec_to_map(platforms);

    let journey_platform = journey_platform
        .into_iter()
        .map(|values| {
            create_journey_platform(
                values,
                journeys_pk_type_converter,
                &platforms_pk_type_converter,
            )
        })
        .collect::<Result<Vec<_>, _>>()?;
    let journey_platform = JourneyPlatform::vec_to_map(journey_platform);

    log::info!("Parsing GLEIS_LV95...");
    #[rustfmt::skip]
    load_coordinates_for_platforms(path, CoordinateSystem::LV95, bytes_offset, &platforms_pk_type_converter, &mut platforms)?;
    log::info!("Parsing GLEIS_WGS84...");
    #[rustfmt::skip]
    load_coordinates_for_platforms(path, CoordinateSystem::WGS84, bytes_offset, &platforms_pk_type_converter, &mut platforms)?;

    Ok((
        ResourceStorage::new(journey_platform),
        ResourceStorage::new(platforms),
    ))
}

fn load_coordinates_for_platforms(
    path: &Path,
    coordinate_system: CoordinateSystem,
    bytes_offset: u64,
    pk_type_converter: &FxHashMap<(i32, i32), i32>,
    data: &mut FxHashMap<i32, Platform>,
) -> Result<(), Error> {
    const ROW_A: i32 = 1;
    const ROW_B: i32 = 2;
    const ROW_C: i32 = 3;

    #[rustfmt::skip]
    let row_parser = RowParser::new(vec![
        // This row is ignored, as the data has already been retrieved from the GLEIS file.
        RowDefinition::new(ROW_A, Box::new(FastRowMatcher::new(18, 1, "G", true)), Vec::new()),
        // This row contains the SLOID.
        RowDefinition::new(ROW_B, Box::new(FastRowMatcher::new(18, 3, "I A", true)), vec![
            ColumnDefinition::new(1, 7, ExpectedType::Integer32),
            ColumnDefinition::new(10, 16, ExpectedType::Integer32), // Should be 9-16, but here the # character is ignored.
            ColumnDefinition::new(22, -1, ExpectedType::String),    // This column has not been explicitly defined in the SBB specification.
        ]),
        // This row contains the LV95/WGS84 coordinates.
        RowDefinition::new(ROW_C, Box::new(FastRowMatcher::new(18, 1, "K", true)), vec![
            ColumnDefinition::new(1, 7, ExpectedType::Integer32),
            ColumnDefinition::new(10, 16, ExpectedType::Integer32), // Should be 9-16, but here the # character is ignored.
            ColumnDefinition::new(20, 26, ExpectedType::Float),     // This column has not been explicitly defined in the SBB specification.
            ColumnDefinition::new(28, 34, ExpectedType::Float),     // This column has not been explicitly defined in the SBB specification.
        ]),
    ]);
    let filename = match coordinate_system {
        CoordinateSystem::LV95 => "GLEIS_LV95",
        CoordinateSystem::WGS84 => "GLEIS_WGS",
    };
    let parser = FileParser::new_with_bytes_offset(&path.join(filename), row_parser, bytes_offset)?;

    parser.parse().try_for_each(|x| {
        let (id, _, values) = x?;
        match id {
            ROW_A => {}
            ROW_B => platform_set_sloid(values, coordinate_system, pk_type_converter, data)?,
            ROW_C => platform_set_coordinates(values, coordinate_system, pk_type_converter, data)?,
            _ => unreachable!(),
        }
        Ok(())
    })
}

// ------------------------------------------------------------------------------------------------
// --- Helper Functions
// ------------------------------------------------------------------------------------------------

fn create_journey_platform(
    mut values: Vec<ParsedValue>,
    journeys_pk_type_converter: &FxHashMap<(i32, String), i32>,
    platforms_pk_type_converter: &FxHashMap<(i32, i32), i32>,
) -> Result<JourneyPlatform, Error> {
    let stop_id: i32 = values.remove(0).into();
    let journey_id: i32 = values.remove(0).into();
    let administration: String = values.remove(0).into();
    let index: i32 = values.remove(0).into();
    let time: Option<i32> = values.remove(0).into();
    let bit_field_id: Option<i32> = values.remove(0).into();

    let journey_id = *journeys_pk_type_converter
        .get(&(journey_id, administration))
        .ok_or_eyre("Unknown legacy ID")?;

    let platform_id = *platforms_pk_type_converter
        .get(&(stop_id, index))
        .ok_or_eyre("Unknown legacy ID")?;

    let time = time.map(|x| create_time_from_value(x as u32));

    Ok(JourneyPlatform::new(
        journey_id,
        platform_id,
        time,
        bit_field_id,
    ))
}

fn create_platform(
    mut values: Vec<ParsedValue>,
    auto_increment: &AutoIncrement,
    platforms_pk_type_converter: &mut FxHashMap<(i32, i32), i32>,
) -> Result<Platform, Error> {
    let stop_id: i32 = values.remove(0).into();
    let index: i32 = values.remove(0).into();
    let platform_data: String = values.remove(0).into();

    let id = auto_increment.next();
    let (code, sectors) = parse_platform_data(platform_data)?;

    platforms_pk_type_converter.insert((stop_id, index), id);
    Ok(Platform::new(id, code, sectors, stop_id))
}

fn platform_set_sloid(
    mut values: Vec<ParsedValue>,
    coordinate_system: CoordinateSystem,
    pk_type_converter: &FxHashMap<(i32, i32), i32>,
    data: &mut FxHashMap<i32, Platform>,
) -> Result<(), Error> {
    // The SLOID is processed only when loading LV95 coordinates.
    if coordinate_system == CoordinateSystem::LV95 {
        let stop_id: i32 = values.remove(0).into();
        let index: i32 = values.remove(0).into();
        let sloid: String = values.remove(0).into();

        let id = pk_type_converter
            .get(&(stop_id, index))
            .ok_or_eyre("Unknown legacy ID")?;

        data.get_mut(id).ok_or_eyre("Unknown ID")?.set_sloid(sloid);
    }

    Ok(())
}

fn platform_set_coordinates(
    mut values: Vec<ParsedValue>,
    coordinate_system: CoordinateSystem,
    pk_type_converter: &FxHashMap<(i32, i32), i32>,
    data: &mut FxHashMap<i32, Platform>,
) -> Result<(), Error> {
    let stop_id: i32 = values.remove(0).into();
    let index: i32 = values.remove(0).into();
    let mut xy1: f64 = values.remove(0).into();
    let mut xy2: f64 = values.remove(0).into();

    if coordinate_system == CoordinateSystem::WGS84 {
        // WGS84 coordinates are stored in reverse order for some unknown reason.
        (xy1, xy2) = (xy2, xy1);
    }

    let coordinate = Coordinates::new(coordinate_system, xy1, xy2);

    let id = &pk_type_converter
        .get(&(stop_id, index))
        .ok_or_eyre("Unknown legacy ID")?;
    let platform = data.get_mut(id).ok_or_eyre("Unknown ID")?;

    match coordinate_system {
        CoordinateSystem::LV95 => platform.set_lv95_coordinates(coordinate),
        CoordinateSystem::WGS84 => platform.set_wgs84_coordinates(coordinate),
    }

    Ok(())
}

// ------------------------------------------------------------------------------------------------
// --- Helper Functions
// ------------------------------------------------------------------------------------------------

fn parse_platform_data(mut platform_data: String) -> Result<(String, Option<String>), Error> {
    platform_data = format!("{} ", platform_data);
    let data = platform_data.split("' ").filter(|&s| !s.is_empty()).fold(
        FxHashMap::default(),
        |mut acc, item| {
            let parts: Vec<&str> = item.split(" '").collect();
            acc.insert(parts[0], parts[1]);
            acc
        },
    );

    // There should always be a G entry.
    let code = data
        .get("G")
        .ok_or_eyre("Entry of type \"G\" missing.")?
        .to_string();
    let sectors = data.get("A").map(|s| s.to_string());

    Ok((code, sectors))
}
