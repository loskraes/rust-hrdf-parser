// 4 file(s).
// File(s) read by the parser:
// INFOTEXT_DE, INFOTEXT_EN, INFOTEXT_FR, INFOTEXT_IT
use rustc_hash::FxHashMap;

use crate::{
    error::{Error, OptionExt},
    models::{InformationText, Language, Model},
    parsing::{ColumnDefinition, ExpectedType, FileParser, ParsedValue, RowDefinition, RowParser},
    storage::ResourceStorage,
};

pub fn parse(path: &str) -> Result<ResourceStorage<InformationText>, Error> {
    log::info!("Parsing INFOTEXT_DE...");
    log::info!("Parsing INFOTEXT_EN...");
    log::info!("Parsing INFOTEXT_FR...");
    log::info!("Parsing INFOTEXT_IT...");

    #[rustfmt::skip]
    let row_parser = RowParser::new(vec![
        // This row is used to create a InformationText instance.
        RowDefinition::from(vec![
            ColumnDefinition::new(1, 9, ExpectedType::Integer32),
        ]),
    ]);
    let parser = FileParser::new(&format!("{path}/INFOTEXT_DE"), row_parser)?;

    let data = parser
        .parse()
        .map(|x| x.map(|(_, _, values)| create_instance(values)))
        .collect::<Result<Vec<_>, _>>()?;
    let mut data = InformationText::vec_to_map(data);

    load_content(path, &mut data, Language::German)?;
    load_content(path, &mut data, Language::English)?;
    load_content(path, &mut data, Language::French)?;
    load_content(path, &mut data, Language::Italian)?;

    Ok(ResourceStorage::new(data))
}

fn load_content(
    path: &str,
    data: &mut FxHashMap<i32, InformationText>,
    language: Language,
) -> Result<(), Error> {
    #[rustfmt::skip]
    let row_parser = RowParser::new(vec![
        // This row contains the content in a specific language.
        RowDefinition::from(vec![
            ColumnDefinition::new(1, 9, ExpectedType::Integer32),
            ColumnDefinition::new(11, -1, ExpectedType::String),
        ]),
    ]);
    let filename = match language {
        Language::German => "INFOTEXT_DE",
        Language::English => "INFOTEXT_EN",
        Language::French => "INFOTEXT_FR",
        Language::Italian => "INFOTEXT_IT",
    };
    let parser = FileParser::new(&format!("{path}/{filename}"), row_parser)?;

    parser.parse().try_for_each(|x| {
        let (_, _, values) = x?;
        set_content(values, data, language)?;
        Ok(())
    })
}

// ------------------------------------------------------------------------------------------------
// --- Data Processing Functions
// ------------------------------------------------------------------------------------------------

fn create_instance(mut values: Vec<ParsedValue>) -> InformationText {
    let id: i32 = values.remove(0).into();

    InformationText::new(id)
}

fn set_content(
    mut values: Vec<ParsedValue>,
    data: &mut FxHashMap<i32, InformationText>,
    language: Language,
) -> Result<(), Error> {
    let id: i32 = values.remove(0).into();
    let description: String = values.remove(0).into();

    data.get_mut(&id)
        .ok_or_eyre("Unknown ID")?
        .set_content(language, &description);

    Ok(())
}
