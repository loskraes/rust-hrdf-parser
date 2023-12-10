// FEIERTAG
use std::{collections::HashMap, error::Error, rc::Rc};

use chrono::NaiveDate;

use crate::{
    models::Holiday,
    parsing::{ColumnDefinition, ExpectedType, FileParser, RowDefinition, RowParser},
};

use super::ParsedValue;

pub fn load_holidays() -> Result<Vec<Rc<Holiday>>, Box<dyn Error>> {
    println!("Parsing FEIERTAG...");
    #[rustfmt::skip]
    let row_parser = RowParser::new(vec![
        // This row is used to create a Holiday instance.
        RowDefinition::from(vec![
            ColumnDefinition::new(1, 10, ExpectedType::String),  // Complies with the SBB standard.
            ColumnDefinition::new(12, -1, ExpectedType::String), // Complies with the SBB standard.
        ]),
    ]);
    let file_parser = FileParser::new("data/FEIERTAG", row_parser)?;

    let mut holidays = Vec::new();

    for (_, _, values) in file_parser.parse() {
        holidays.push(create_holiday(values)?);
    }

    Ok(holidays)
}

// ------------------------------------------------------------------------------------------------
// --- Helper Functions
// ------------------------------------------------------------------------------------------------

fn parse_name(raw_name: String) -> HashMap<String, String> {
    raw_name
        .split('>')
        .filter(|&s| !s.is_empty())
        .map(|s| {
            let mut parts = s.split('<');

            let v = parts.next().unwrap().to_string();
            let k = parts.next().unwrap().to_string();

            (k, v)
        })
        .fold(HashMap::new(), |mut acc, (k, v)| {
            acc.insert(k, v);
            acc
        })
}

fn create_holiday(mut values: Vec<ParsedValue>) -> Result<Rc<Holiday>, Box<dyn Error>> {
    let date: String = values.remove(0).into();
    let name: String = values.remove(0).into();

    let date = NaiveDate::parse_from_str(&date, "%d.%m.%Y")?;
    let name = parse_name(name);

    Ok(Rc::new(Holiday::new(date, name)))
}
