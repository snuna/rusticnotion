use test_log::test;
mod common;
use common::test_client;
use rusticnotion::models::{
    block::FileOrEmojiObject,
    search::{
        DatabaseQuery, FilterCondition, FilterProperty, FilterValue, NotionSearch,
        PropertyCondition, TextCondition,
    },
    Object,
};

#[test(tokio::test)]
async fn get_database() -> Result<(), Box<dyn std::error::Error>> {
    let api = test_client();

    let response = api
        .search(NotionSearch::Filter {
            value: FilterValue::Database,
            property: FilterProperty::Object,
        })
        .await?;

    let db = response
        .results()
        .iter()
        .filter_map(|o| match o {
            Object::Database { database } => Some(database),
            _ => None,
        })
        .next()
        .expect("Test expected to find at least one database in notion")
        .clone();

    // todo: fix this clone issue
    let db_result = api.get_database(db.clone()).await?;

    assert_eq!(db, db_result);

    Ok(())
}

#[test(tokio::test)]
async fn query_database() -> Result<(), Box<dyn std::error::Error>> {
    let api = test_client();

    let response = api
        .search(NotionSearch::Filter {
            value: FilterValue::Database,
            property: FilterProperty::Object,
        })
        .await?;

    let db = response
        .results()
        .iter()
        .filter_map(|o| match o {
            Object::Database { database } => Some(database),
            _ => None,
        })
        .next()
        .expect("Test expected to find at least one database in notion")
        .clone();

    assert_eq!(
        db.icon,
        Some(FileOrEmojiObject::Emoji {
            emoji: "🪑".to_string(),
        })
    );

    let pages = api
        .query_database(
            db.clone(),
            DatabaseQuery {
                filter: Some(FilterCondition::Property {
                    property: "Name".to_string(),
                    condition: PropertyCondition::RichText(TextCondition::Contains(
                        "First".to_string(),
                    )),
                }),
                ..Default::default()
            },
        )
        .await?;

    assert_eq!(pages.results().len(), 1);
    assert_eq!(
        pages.results()[0].icon,
        Some(FileOrEmojiObject::Emoji {
            emoji: "🌋".to_string(),
        })
    );

    let pages = api.query_database(db, DatabaseQuery::default()).await?;

    for page in pages.results() {
        assert!(page.icon.is_some());
    }

    Ok(())
}
