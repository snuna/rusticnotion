use crate::models::properties::{
    DateOrDateTime, PropertyValueData, RollupPropertyValue, RollupValue,
};
use crate::models::properties::{FormulaResultValue, PropertyValue};
use chrono::NaiveDate;

#[test]
fn verify_date_parsing() {
    let date = NaiveDate::from_ymd_opt(2021, 1, 2).unwrap();
    let result = serde_json::to_string(&DateOrDateTime::Date(date)).unwrap();
    let parsed: DateOrDateTime = serde_json::from_str(&result).unwrap();
    println!("{:?}", parsed);
}

#[test]
fn parse_date_property() {
    let _property: PropertyValue =
        serde_json::from_str(include_str!("tests/date_property.json")).unwrap();
}

#[test]
fn parse_null_select_property() {
    let _property: PropertyValue =
        serde_json::from_str(include_str!("tests/null_select_property.json")).unwrap();
}

#[test]
fn parse_select_property() {
    let _property: PropertyValue =
        serde_json::from_str(include_str!("tests/select_property.json")).unwrap();
}

#[test]
fn parse_text_property_with_link() {
    let _property: PropertyValue =
        serde_json::from_str(include_str!("tests/text_with_link.json")).unwrap();
}

#[test]
fn parse_rollup_property() {
    let property: PropertyValue =
        serde_json::from_str(include_str!("tests/rollup_property.json")).unwrap();

    assert!(matches!(
        property.data,
        PropertyValueData::Rollup {
            rollup: Some(RollupValue::Array { .. }),
            ..
        }
    ));

    if let PropertyValueData::Rollup {
        rollup: Some(RollupValue::Array { array }),
        ..
    } = property.data
    {
        assert!(matches!(array[0], RollupPropertyValue::Text { .. }))
    }
}

#[test]
fn parse_number_formula_prop() {
    let _property: PropertyValue =
        serde_json::from_str(include_str!("tests/formula_number_value.json")).unwrap();
}

#[test]
fn parse_date_formula_prop() {
    let _property: PropertyValue =
        serde_json::from_str(include_str!("tests/formula_date_value.json")).unwrap();
}

#[test]
fn parse_number_formula() {
    let _value: FormulaResultValue = serde_json::from_str(
        r#"{
    "type": "number",
    "number": 0
  }"#,
    )
    .unwrap();
}

#[test]
fn parse_phone_number_values() {
    let _property: Vec<PropertyValue> =
        serde_json::from_str(include_str!("tests/phone_number_property_value.json")).unwrap();
}
