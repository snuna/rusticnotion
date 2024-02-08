use test_log::test;
mod common;
use common::test_client;
use rusticnotion::models::search::{FilterProperty, FilterValue, NotionSearch};

#[test(tokio::test)]
async fn list_databases() -> Result<(), Box<dyn std::error::Error>> {
    let api = test_client();

    dbg!(api.list_databases().await?);

    Ok(())
}

#[test(tokio::test)]
async fn search_databases() -> Result<(), Box<dyn std::error::Error>> {
    let api = test_client();

    let response = api
        .search(NotionSearch::Filter {
            property: FilterProperty::Object,
            value: FilterValue::Database,
        })
        .await?;

    assert!(!response.results.is_empty());

    Ok(())
}

#[test(tokio::test)]
async fn search_pages() -> Result<(), Box<dyn std::error::Error>> {
    let api = test_client();

    let response = api
        .search(NotionSearch::Filter {
            property: FilterProperty::Object,
            value: FilterValue::Page,
        })
        .await?;

    assert!(!response.results.is_empty());

    Ok(())
}
