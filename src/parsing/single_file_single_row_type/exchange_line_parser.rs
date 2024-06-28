// 1 file(s).
// File(s) read by the parser:
// UMSTEIGL
use std::{error::Error, str::FromStr};

use rustc_hash::FxHashMap;

use crate::{
    models::{DirectionType, Model, ExchangeTimeLine},
    parsing::{ColumnDefinition, ExpectedType, FileParser, ParsedValue, RowDefinition, RowParser},
    storage::SimpleResourceStorage,
    utils::AutoIncrement,
};

pub fn parse(
    transport_types_pk_type_converter: &FxHashMap<String, i32>,
) -> Result<SimpleResourceStorage<ExchangeTimeLine>, Box<dyn Error>> {
    println!("Parsing UMSTEIGL...");
    #[rustfmt::skip]
    let row_parser = RowParser::new(vec![
        // This row is used to create a LineExchangeTime instance.
        RowDefinition::from(vec![
            ColumnDefinition::new(1, 7, ExpectedType::OptionInteger32),
            ColumnDefinition::new(9, 14, ExpectedType::String),
            ColumnDefinition::new(16, 18, ExpectedType::String),
            ColumnDefinition::new(20, 27, ExpectedType::String),
            ColumnDefinition::new(29, 29, ExpectedType::String),
            ColumnDefinition::new(31, 36, ExpectedType::String),
            ColumnDefinition::new(38, 40, ExpectedType::String),
            ColumnDefinition::new(42, 49, ExpectedType::String),
            ColumnDefinition::new(51, 51, ExpectedType::String),
            ColumnDefinition::new(53, 55, ExpectedType::Integer16),
            ColumnDefinition::new(56, 56, ExpectedType::String),
        ]),
    ]);
    let parser = FileParser::new("data/UMSTEIGL", row_parser)?;

    let auto_increment = AutoIncrement::new();

    let data = parser
        .parse()
        .map(|(_, _, values)| {
            create_instance(values, &auto_increment, transport_types_pk_type_converter)
        })
        .collect();
    let data = ExchangeTimeLine::vec_to_map(data);

    Ok(SimpleResourceStorage::new(data))
}

// ------------------------------------------------------------------------------------------------
// --- Data Processing Functions
// ------------------------------------------------------------------------------------------------

fn create_instance(
    mut values: Vec<ParsedValue>,
    auto_increment: &AutoIncrement,
    transport_types_pk_type_converter: &FxHashMap<String, i32>,
) -> ExchangeTimeLine {
    let stop_id: Option<i32> = values.remove(0).into();
    let administration_1: String = values.remove(0).into();
    let transport_type_id_1: String = values.remove(0).into();
    let line_id_1: String = values.remove(0).into();
    let direction_1: String = values.remove(0).into();
    let administration_2: String = values.remove(0).into();
    let transport_type_id_2: String = values.remove(0).into();
    let line_id_2: String = values.remove(0).into();
    let direction_2: String = values.remove(0).into();
    let duration: i16 = values.remove(0).into();
    let is_guaranteed: String = values.remove(0).into();

    let transport_type_id_1 = *transport_types_pk_type_converter
        .get(&transport_type_id_1)
        .unwrap();

    let line_id_1 = if line_id_1 == "*" {
        None
    } else {
        Some(line_id_1)
    };

    let direction_1 = if direction_1 == "*" {
        None
    } else {
        Some(DirectionType::from_str(&direction_1).unwrap())
    };

    let transport_type_id_2 = *transport_types_pk_type_converter
        .get(&transport_type_id_2)
        .unwrap();

    let line_id_2 = if line_id_2 == "*" {
        None
    } else {
        Some(line_id_2)
    };

    let direction_2 = if direction_2 == "*" {
        None
    } else {
        Some(DirectionType::from_str(&direction_2).unwrap())
    };

    let is_guaranteed = is_guaranteed == "!";

    ExchangeTimeLine::new(
        auto_increment.next(),
        stop_id,
        administration_1,
        transport_type_id_1,
        line_id_1,
        direction_1,
        administration_2,
        transport_type_id_2,
        line_id_2,
        direction_2,
        duration,
        is_guaranteed,
    )
}
