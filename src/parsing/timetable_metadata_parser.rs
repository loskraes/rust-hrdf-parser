// 1 file(s).
// File(s) read by the parser:
// ECKDATEN

use chrono::{NaiveDate, NaiveDateTime};

use crate::{
    Result, TimetableMetadataKey, TimetableMetadataValue,
    models::{Model, TimetableMetadataEntry},
    parsing::{
        AdvancedRowMatcher, ColumnDefinition, ExpectedType, FastRowMatcher, FileParser,
        ParsedValue, RowDefinition, RowParser,
    },
    storage::ResourceStorage,
};

pub fn parse(path: &str) -> Result<ResourceStorage<TimetableMetadataEntry>> {
    log::info!("Parsing ECKDATEN...");
    const ROW_A: i32 = 1;
    const ROW_B: i32 = 2;

    #[rustfmt::skip]
    let row_parser = RowParser::new(vec![
        // This row contains the period start/end date in which timetables are effective.
        RowDefinition::new(ROW_A, Box::new(AdvancedRowMatcher::new(r"^[0-9]{2}.[0-9]{2}.[0-9]{4}$")?), vec![
            ColumnDefinition::new(1, 10, ExpectedType::String),
        ]),
        // This row contains the name, the creation date, the version and the provider of the timetable.
        RowDefinition::new(ROW_B, Box::new(FastRowMatcher::new(1, 0, "", true)), vec![
            ColumnDefinition::new(1, -1, ExpectedType::String),
        ]),
    ]);
    let parser = FileParser::new(&format!("{path}/ECKDATEN"), row_parser)?;

    let mut data: Vec<ParsedValue> = parser
        .parse()
        .map(|x| x.map(|(_, _, mut values)| values.remove(0)))
        .collect::<Result<Vec<_>>>()?;

    let start_date: String = data.remove(0).into();
    let end_date: String = data.remove(0).into();
    let other_data: String = data.remove(0).into();

    let start_date = NaiveDate::parse_from_str(&start_date, "%d.%m.%Y")?;
    let end_date = NaiveDate::parse_from_str(&end_date, "%d.%m.%Y")?;
    let mut other_data: Vec<String> = other_data.split('$').map(String::from).collect();

    let name = other_data.remove(0);
    let created_at = other_data.remove(0);
    let version = other_data.remove(0);
    let provider = other_data.remove(0);

    let created_at = NaiveDateTime::parse_from_str(&created_at, "%d.%m.%Y %H:%M:%S")?;

    // TODO: Ensure other_data is empty ? else keep it in an entry

    let rows = vec![
        (
            TimetableMetadataKey::StartDate,
            TimetableMetadataValue::NaiveDate(start_date),
        ),
        (
            TimetableMetadataKey::EndDate,
            TimetableMetadataValue::NaiveDate(end_date),
        ),
        (
            TimetableMetadataKey::Name,
            TimetableMetadataValue::String(name),
        ),
        (
            TimetableMetadataKey::CreatedAt,
            TimetableMetadataValue::NaiveDateTime(created_at),
        ),
        (
            TimetableMetadataKey::Version,
            TimetableMetadataValue::String(version),
        ),
        (
            TimetableMetadataKey::Provider,
            TimetableMetadataValue::String(provider),
        ),
    ];

    let data: Vec<TimetableMetadataEntry> = rows
        .into_iter()
        .map(|(key, value)| TimetableMetadataEntry::new(key, value))
        .collect();
    let data = TimetableMetadataEntry::vec_to_map(data);

    Ok(ResourceStorage::new(data))
}
