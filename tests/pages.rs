mod common;
use common::test_client;
use notion::{
    ids::BlockId,
    models::{
        search::{FilterProperty, FilterValue, NotionSearch},
        Object,
    },
};

#[tokio::test]
async fn get_block_children() -> Result<(), Box<dyn std::error::Error>> {
    let api = test_client();

    let search_response = api
        .search(NotionSearch::Filter {
            value: FilterValue::Page,
            property: FilterProperty::Object,
        })
        .await?;

    println!("{:?}", search_response.results.len());

    for object in search_response.results {
        match object {
            Object::Page { page } => api
                .get_block_children(BlockId::from(page.id))
                .await
                .unwrap(),
            _ => panic!("Should not have received anything but pages!"),
        };
    }

    Ok(())
}
