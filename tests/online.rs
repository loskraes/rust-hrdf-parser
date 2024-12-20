use hrdf_parser::{Hrdf, Version};

#[cfg_attr(not(feature = "test-online"), ignore)]
#[tokio::test]
async fn last_2024() {
    Hrdf::new(
        Version::V_5_40_41_2_0_5,
        "https://opentransportdata.swiss/en/dataset/timetable-54-2024-hrdf/permalink",
        true,
    )
    .await
    .unwrap();
}

#[cfg_attr(not(feature = "test-online"), ignore)]
#[tokio::test]
async fn last_2025() {
    Hrdf::new(
        Version::V_5_40_41_2_0_5,
        "https://opentransportdata.swiss/en/dataset/timetable-54-2025-hrdf/permalink",
        true,
    )
    .await
    .unwrap();
}

#[cfg_attr(not(feature = "test-online"), ignore)]
#[tokio::test]
async fn last_draft() {
    Hrdf::new(
        Version::V_5_40_41_2_0_5,
        "https://opentransportdata.swiss/fr/dataset/timetable-54-draft-hrdf/permalink",
        true,
    )
    .await
    .unwrap();
}
