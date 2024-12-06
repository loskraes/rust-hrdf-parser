// 1 file(s).
// File(s) read by the parser:
// UMSTEIGZ
use rustc_hash::FxHashMap;

use crate::{
    error::{Error, OptionExt},
    models::{ExchangeTimeJourney, Model},
    parsing::{ColumnDefinition, ExpectedType, FileParser, ParsedValue, RowDefinition, RowParser},
    storage::ResourceStorage,
    utils::AutoIncrement,
};

pub fn parse(
    path: &str,
    journeys_pk_type_converter: &FxHashMap<(i32, String), i32>,
) -> Result<ResourceStorage<ExchangeTimeJourney>, Error> {
    log::info!("Parsing UMSTEIGZ...");
    #[rustfmt::skip]
    let row_parser = RowParser::new(vec![
        // This row is used to create a JourneyExchangeTime instance.
        RowDefinition::from(vec![
            ColumnDefinition::new(1, 7, ExpectedType::Integer32),
            ColumnDefinition::new(9, 14, ExpectedType::Integer32),
            ColumnDefinition::new(16, 21, ExpectedType::String),
            ColumnDefinition::new(23, 28, ExpectedType::Integer32),
            ColumnDefinition::new(30, 35, ExpectedType::String),
            ColumnDefinition::new(37, 39, ExpectedType::Integer16),
            ColumnDefinition::new(40, 40, ExpectedType::String),
            ColumnDefinition::new(42, 47, ExpectedType::OptionInteger32),
        ]),
    ]);
    let parser = FileParser::new(&format!("{path}/UMSTEIGZ"), row_parser)?;

    let auto_increment = AutoIncrement::new();

    let data = parser
        .parse()
        .map(|x| {
            x.and_then(|(_, _, values)| {
                create_instance(values, &auto_increment, journeys_pk_type_converter)
            })
        })
        .collect::<Result<Vec<_>, _>>()?;
    let data = ExchangeTimeJourney::vec_to_map(data);

    Ok(ResourceStorage::new(data))
}

// ------------------------------------------------------------------------------------------------
// --- Data Processing Functions
// ------------------------------------------------------------------------------------------------

fn create_instance(
    mut values: Vec<ParsedValue>,
    auto_increment: &AutoIncrement,
    journeys_pk_type_converter: &FxHashMap<(i32, String), i32>,
) -> Result<ExchangeTimeJourney, Error> {
    let stop_id: i32 = values.remove(0).into();
    let journey_id_1: i32 = values.remove(0).into();
    let administration_1: String = values.remove(0).into();
    let journey_id_2: i32 = values.remove(0).into();
    let administration_2: String = values.remove(0).into();
    let duration: i16 = values.remove(0).into();
    let is_guaranteed: String = values.remove(0).into();
    let bit_field_id: Option<i32> = values.remove(0).into();

    let journey_id_1 = *journeys_pk_type_converter
        .get(&(journey_id_1, administration_1))
        .ok_or_eyre("Unknown legacy ID")?;

    let journey_id_2 = *journeys_pk_type_converter
        .get(&(journey_id_2, administration_2))
        .ok_or_eyre("Unknown legacy ID")?;

    let is_guaranteed = is_guaranteed == "!";

    Ok(ExchangeTimeJourney::new(
        auto_increment.next(),
        stop_id,
        journey_id_1,
        journey_id_2,
        duration,
        is_guaranteed,
        bit_field_id,
    ))
}
