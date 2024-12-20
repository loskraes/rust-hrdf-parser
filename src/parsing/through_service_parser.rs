use std::path::Path;

// 1 file(s).
// File(s) read by the parser:
// DURCHBI
use rustc_hash::FxHashMap;

use crate::{
    error::{Error, OptionExt},
    models::{Model, ThroughService},
    parsing::{ColumnDefinition, ExpectedType, FileParser, ParsedValue, RowDefinition, RowParser},
    storage::ResourceStorage,
    utils::AutoIncrement,
};

pub fn parse(
    path: &Path,
    journeys_pk_type_converter: &FxHashMap<(i32, String), i32>,
) -> Result<ResourceStorage<ThroughService>, Error> {
    log::info!("Parsing DURCHBI...");
    #[rustfmt::skip]
    let row_parser = RowParser::new(vec![
        // This row is used to create a ThroughService instance.
        RowDefinition::from(vec![
            ColumnDefinition::new(1, 6, ExpectedType::Integer32),
            ColumnDefinition::new(8, 13, ExpectedType::String),
            ColumnDefinition::new(15, 21, ExpectedType::Integer32),
            ColumnDefinition::new(23, 28, ExpectedType::Integer32),
            ColumnDefinition::new(30, 35, ExpectedType::String),
            ColumnDefinition::new(37, 42, ExpectedType::Integer32), // Should be INT16 according to the standard. The standard contains an error. The correct type is INT32.
            ColumnDefinition::new(44, 50, ExpectedType::OptionInteger32),
        ]),
    ]);
    let parser = FileParser::new(&path.join("DURCHBI"), row_parser)?;

    let auto_increment = AutoIncrement::new();

    let data = parser
        .parse()
        .map(|x| {
            x.and_then(|(_, _, values)| {
                create_instance(values, &auto_increment, journeys_pk_type_converter)
            })
        })
        .collect::<Result<Vec<_>, _>>()?;
    let data = ThroughService::vec_to_map(data);

    Ok(ResourceStorage::new(data))
}

// ------------------------------------------------------------------------------------------------
// --- Data Processing Functions
// ------------------------------------------------------------------------------------------------

fn create_instance(
    mut values: Vec<ParsedValue>,
    auto_increment: &AutoIncrement,
    journeys_pk_type_converter: &FxHashMap<(i32, String), i32>,
) -> Result<ThroughService, Error> {
    let journey_1_id: i32 = values.remove(0).into();
    let journey_1_administration: String = values.remove(0).into();
    let journey_1_stop_id: i32 = values.remove(0).into();
    let journey_2_id: i32 = values.remove(0).into();
    let journey_2_administration: String = values.remove(0).into();
    let bit_field_id: i32 = values.remove(0).into();
    let journey_2_stop_id: Option<i32> = values.remove(0).into();

    let journey_1_id = *journeys_pk_type_converter
        .get(&(journey_1_id, journey_1_administration))
        .ok_or_eyre("Unknown legacy ID")?;

    let journey_2_id = *journeys_pk_type_converter
        .get(&(journey_2_id, journey_2_administration))
        .ok_or_eyre("Unknown legacy ID")?;

    Ok(ThroughService::new(
        auto_increment.next(),
        journey_1_id,
        journey_1_stop_id,
        journey_2_id,
        journey_2_stop_id,
        bit_field_id,
    ))
}
