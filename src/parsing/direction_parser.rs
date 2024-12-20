use std::path::Path;

// 1 file(s).
// File(s) read by the parser:
// RICHTUNG
use rustc_hash::FxHashMap;

use crate::{
    error::Error,
    models::{Direction, Model},
    parsing::{ColumnDefinition, ExpectedType, FileParser, ParsedValue, RowDefinition, RowParser},
    storage::ResourceStorage,
};

type DirectionAndTypeConverter = (ResourceStorage<Direction>, FxHashMap<String, i32>);

pub fn parse(path: &Path) -> Result<DirectionAndTypeConverter, Error> {
    log::info!("Parsing RICHTUNG...");
    #[rustfmt::skip]
    let row_parser = RowParser::new(vec![
        // This row is used to create a Direction instance.
        RowDefinition::from(vec![
            ColumnDefinition::new(1, 7, ExpectedType::String),
            ColumnDefinition::new(9, -1, ExpectedType::String),
        ]),
    ]);
    let parser = FileParser::new(&path.join("RICHTUNG"), row_parser)?;

    let mut pk_type_converter = FxHashMap::default();

    let data = parser
        .parse()
        .map(|x| x.and_then(|(_, _, values)| create_instance(values, &mut pk_type_converter)))
        .collect::<Result<Vec<_>, _>>()?;
    let data = Direction::vec_to_map(data);

    Ok((ResourceStorage::new(data), pk_type_converter))
}

// ------------------------------------------------------------------------------------------------
// --- Data Processing Functions
// ------------------------------------------------------------------------------------------------

fn create_instance(
    mut values: Vec<ParsedValue>,
    pk_type_converter: &mut FxHashMap<String, i32>,
) -> Result<Direction, Error> {
    let legacy_id: String = values.remove(0).into();
    let name: String = values.remove(0).into();

    let id = remove_first_char(&legacy_id).parse::<i32>()?;

    pk_type_converter.insert(legacy_id, id);
    Ok(Direction::new(id, name))
}

// ------------------------------------------------------------------------------------------------
// --- Helper Functions
// ------------------------------------------------------------------------------------------------

fn remove_first_char(value: &str) -> &str {
    let mut chars = value.chars();
    chars.next();
    chars.as_str()
}
