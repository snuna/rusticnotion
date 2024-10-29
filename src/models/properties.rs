use std::convert::{TryFrom, TryInto};
use std::fmt::{Display, Formatter};

use crate::models::text::RichText;
use crate::models::users::User;

use crate::ids::{DatabaseId, PageId, PropertyId};
use crate::models::{DateTime, Number, Utc};
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[cfg(test)]
mod tests;

/// A property can exist together with an id or standalone as `PropertyValue`.
/// This trait allows us to treat both cases the same way, when we don't care about the id.
/// The `type_name` method is used to get the type of the property value as string, which is useful for error handling.
pub trait Property {
    fn value(&self) -> &PropertyValue;
    fn type_name(&self) -> String;
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct PropertyWithId<T> {
    pub id: PropertyId,
    #[serde(flatten)]
    pub value: T,
}

impl Property for PropertyWithId<PropertyValue> {
    fn value(&self) -> &PropertyValue {
        &self.value
    }

    fn type_name(&self) -> String {
        self.value.type_name()
    }
}

impl Property for PropertyValue {
    fn value(&self) -> &PropertyValue {
        self
    }

    fn type_name(&self) -> String {
        match self {
            PropertyValue::Title { .. } => "Title",
            PropertyValue::Text { .. } => "Text",
            PropertyValue::Number { .. } => "Number",
            PropertyValue::Select { .. } => "Select",
            PropertyValue::Status { .. } => "Status",
            PropertyValue::MultiSelect { .. } => "MultiSelect",
            PropertyValue::Date { .. } => "Date",
            PropertyValue::People { .. } => "People",
            PropertyValue::Files { .. } => "Files",
            PropertyValue::Checkbox { .. } => "Checkbox",
            PropertyValue::Url { .. } => "Url",
            PropertyValue::Email { .. } => "Email",
            PropertyValue::PhoneNumber { .. } => "PhoneNumber",
            PropertyValue::Formula { .. } => "Formula",
            PropertyValue::Relation { .. } => "Relation",
            PropertyValue::Rollup { .. } => "Rollup",
            PropertyValue::CreatedTime { .. } => "CreatedTime",
            PropertyValue::CreatedBy { .. } => "CreatedBy",
            PropertyValue::LastEditedTime { .. } => "LastEditedTime",
            PropertyValue::LastEditedBy { .. } => "LastEditedBy",
            PropertyValue::Button { .. } => "Button",
        }
        .to_string()
    }
}

impl PropertyExpect for PropertyValue {}
impl PropertyExpect for PropertyWithId<PropertyValue> {}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum PropertyConfigurationData {
    Title,
    #[serde(rename = "rich_text")]
    Text,
    Number {
        number: NumberDetails,
    },
    Select {
        select: Select,
    },
    Status {
        status: Status,
    },
    MultiSelect {
        multi_select: Select,
    },
    Date,
    People,
    Files,
    Checkbox,
    Url,
    Email,
    PhoneNumber,
    Formula {
        formula: Formula,
    },
    Relation {
        relation: Relation,
    },
    Rollup {
        rollup: Rollup,
    },
    CreatedTime,
    CreatedBy,
    LastEditedTime,
    LastEditBy,
    Button,
}

/// How the number is displayed in Notion.
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Copy, Clone, Hash)]
#[serde(rename_all = "snake_case")]
pub enum NumberFormat {
    Number,
    NumberWithCommas,
    Percent,
    Dollar,
    Euro,
    Pound,
    Yen,
    Ruble,
    Rupee,
    Won,
    Yuan,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Hash, Clone)]
pub struct NumberDetails {
    pub format: NumberFormat,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone)]
#[serde(transparent)]
pub struct SelectOptionId(String);

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Copy, Clone)]
#[serde(rename_all = "lowercase")]
pub enum Color {
    Default,
    Gray,
    Brown,
    Orange,
    Yellow,
    Green,
    Blue,
    Purple,
    Pink,
    Red,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct SelectOption {
    pub name: String,
    pub id: SelectOptionId,
    pub color: Color,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct Select {
    /// Sorted list of options available for this property.
    pub options: Vec<SelectOption>,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct StatusGroupOption {
    pub name: String,
    pub id: SelectOptionId,
    pub color: Color,
    pub option_ids: Vec<SelectOptionId>,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct Status {
    /// Sorted list of options available for this property.
    pub options: Vec<SelectOption>,
    /// Sorted list of groups available for this property.
    pub groups: Vec<StatusGroupOption>,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct Formula {
    /// Formula to evaluate for this property
    pub expression: String,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct Relation {
    /// The database this relation refers to.
    /// New linked pages must belong to this database in order to be valid.
    pub database_id: DatabaseId,
    /// By default, relations are formed as two synced properties across databases:
    ///     if you make a change to one property, it updates the synced property at the same time.
    /// `synced_property_name` refers to the name of the property in the related database.
    pub synced_property_name: Option<String>,
    /// By default, relations are formed as two synced properties across databases:
    ///     if you make a change to one property, it updates the synced property at the same time.
    /// `synced_property_id` refers to the id of the property in the related database.
    /// This is usually a short string of random letters and symbols.
    pub synced_property_id: Option<PropertyId>,
}

/// The function used to roll up the values of the relation property.
/// <https://developers.notion.com/reference/page-property-values#rollup>
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Copy, Clone)]
#[serde(rename_all = "snake_case")]
pub enum RollupFunction {
    Average,
    Checked,
    Count,
    CountPerGroup,
    CountValues,
    DateRange,
    EarliestDate,
    Empty,
    LatestDate,
    Max,
    Median,
    Min,
    NotEmpty,
    PercentChecked,
    PercentEmpty,
    PercentNotEmpty,
    PercentPerGroup,
    PercentUnchecked,
    Range,
    ShowOriginal,
    ShowUnique,
    Sum,
    Unchecked,
    Unique,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct Rollup {
    /// The name of the relation property this property is responsible for rolling up.
    pub relation_property_name: String,
    /// The id of the relation property this property is responsible for rolling up.
    pub relation_property_id: PropertyId,
    /// The name of the property of the pages in the related database
    /// that is used as an input to `function`.
    pub rollup_property_name: String,
    /// The id of the property of the pages in the related database
    /// that is used as an input to `function`.
    pub rollup_property_id: String,
    /// The function that is evaluated for every page in the relation of the rollup.
    pub function: RollupFunction,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum PropertyValue {
    Title {
        title: Vec<RichText>,
    },
    #[serde(rename = "rich_text")]
    Text {
        rich_text: Vec<RichText>,
    },
    Number {
        number: Option<Number>,
    },
    Select {
        select: Option<SelectedValue>,
    },
    Status {
        status: Option<SelectedValue>,
    },
    MultiSelect {
        multi_select: Option<Vec<SelectedValue>>,
    },
    Date {
        date: Option<DateValue>,
    },
    Formula {
        formula: FormulaResultValue,
    },
    Relation {
        relation: Option<Vec<RelationValue>>,
    },
    Rollup {
        rollup: Option<RollupValue>,
    },
    People {
        people: Vec<User>,
    },
    Files {
        files: Option<Vec<FileReference>>,
    },
    Checkbox {
        checkbox: bool,
    },
    Url {
        url: Option<String>,
    },
    Email {
        email: Option<String>,
    },
    PhoneNumber {
        phone_number: Option<String>,
    },
    CreatedTime {
        created_time: DateTime<Utc>,
    },
    CreatedBy {
        created_by: User,
    },
    LastEditedTime {
        last_edited_time: DateTime<Utc>,
    },
    LastEditedBy {
        last_edited_by: User,
    },
    Button,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct SelectedValue {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<SelectOptionId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<Color>,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
#[serde(untagged)]
pub enum DateOrDateTime {
    Date(NaiveDate),
    DateTime(DateTime<Utc>),
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct DateValue {
    pub start: DateOrDateTime,
    pub end: Option<DateOrDateTime>,
    pub time_zone: Option<String>,
}

/// Formula property value objects represent the result of evaluating a formula
/// described in the database's properties.
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum FormulaResultValue {
    String { string: Option<String> },
    Number { number: Option<Number> },
    Boolean { boolean: Option<bool> },
    Date { date: Option<DateValue> },
}

/// Relation property value objects contain an array of page references within the relation property.
/// A page reference is an object with an id property,
/// with a string value (UUIDv4) corresponding to a page ID in another database.
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct RelationValue {
    pub id: PageId,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum RollupValue {
    Number { number: Option<Number> },
    Date { date: Option<DateTime<Utc>> },
    Array { array: Vec<RollupPropertyValue> },
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum FileReference {
    External { name: String, external: External },
    File { name: String, file: File },
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct External {
    pub url: String,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct File {
    pub url: String,
    pub expiry_time: String,
}

/// <https://developers.notion.com/reference/page#rollup-property-value-element>
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum RollupPropertyValue {
    /// <https://developers.notion.com/reference/page#rich-text-property-values>
    #[serde(rename = "rich_text")]
    Text {
        rich_text: Vec<RichText>,
    },
    /// <https://developers.notion.com/reference/page#number-property-values>
    Number {
        number: Option<Number>,
    },
    /// <https://developers.notion.com/reference/page#select-property-values>
    Select {
        select: Option<SelectedValue>,
    },
    Status {
        status: Option<SelectedValue>,
    },
    MultiSelect {
        multi_select: Option<Vec<SelectedValue>>,
    },
    Date {
        date: Option<DateValue>,
    },
    /// <https://developers.notion.com/reference/page#formula-property-values>
    Formula {
        formula: FormulaResultValue,
    },
    /// <https://developers.notion.com/reference/page#relation-property-values>
    /// It is actually an array of relations
    Relation {
        relation: Option<Vec<RelationValue>>,
    },
    /// <https://developers.notion.com/reference/page#rollup-property-values>
    Rollup {
        rollup: Option<RollupValue>,
    },
    People {
        people: Vec<User>,
    },
    Files {
        files: Option<Vec<FileReference>>,
    },
    Checkbox {
        checkbox: bool,
    },
    Url {
        url: Option<String>,
    },
    Email {
        email: Option<String>,
    },
    PhoneNumber {
        phone_number: String,
    },
    CreatedTime {
        created_time: DateTime<Utc>,
    },
    CreatedBy {
        created_by: User,
    },
    LastEditedTime {
        last_edited_time: DateTime<Utc>,
    },
    LastEditedBy {
        last_edited_by: User,
    },
}

pub trait PropertyExpect: Property {
    /// Allows for easy access to the property value.
    ///
    /// This is useful if you know the type of the property you want to access and don't need match statements
    ///
    /// ```ignore
    /// let title = property.expect_value::<Vec<RichText>>().unwrap();
    /// ```
    /// This will fail if the actual property type is not compatible with the expected value type.
    ///
    /// See the following implementations of `TryFrom<PropertyValue>` for supported types:
    /// - `Vec<RichText>` for Title, Text
    /// - `Option<Number>` for Number
    /// - `Option<SelectedValue>` for Select, Status
    /// - `Option<Vec<SelectedValue>>` for MultiSelect
    /// - `Option<DateValue>` for Date
    /// - `Option<Vec<RelationValue>>` for Relation
    /// - `Option<Vec<FileReference>>` for Files
    /// - `bool` for Checkbox
    /// - `Option<String>` for Url, Email, PhoneNumber
    /// - `DateTime<Utc>` for CreatedTime, LastEditedTime
    /// - `User` for CreatedBy, LastEditedBy
    ///
    /// You can also create your own implementation of `TryFrom<PropertyValue>` for custom types. This is useful if you
    /// need to convert the property value to a specific type in your project.
    ///
    /// For example if you want to access text property values always as a `String` without formatting you can do the following:
    ///
    /// ```ignore
    /// use rusticnotion::models::properties::{Property, PropertyExpect, PropertyValue, WrongPropertyTypeError}
    ///
    /// struct TextValue(String);
    /// impl TryFrom<PropertyValue> for TextValue {
    ///     type Error = WrongPropertyTypeError;
    ///
    ///     fn try_from(value: PropertyValue) -> Result<Self, Self::Error> {
    ///         match value {
    ///             PropertyValue::Text { rich_text, .. } => {
    ///                   let combined_text = rich_text
    ///                       .iter()
    ///                       .map(|rt| rt.plain_text().to_string())
    ///                       .collect::<Vec<String>>()
    ///                       .join("");
    ///                   Ok(TextValue(combined_text))
    ///               }
    ///               _ => Err(WrongPropertyTypeError {
    ///                   expected: vec!["Text".to_string()],
    ///                   actual: value.type_name(),
    ///               }),
    ///           }
    ///     }
    ///}
    /// assert_eq!(
    ///     page.properties
    ///         .get_by_name("Text")
    ///         .unwrap()
    ///         .expect_value::<TextValue>()
    ///         .unwrap()
    ///         .0,
    ///     "hello world".to_string()
    /// );
    /// ```
    fn expect_value<T>(&self) -> Result<T, WrongPropertyTypeError>
    where
        T: TryFrom<PropertyValue, Error = WrongPropertyTypeError>,
    {
        self.value().to_owned().try_into()
    }

    /// Allows for easy access to the title property value.
    /// This is a shortcut for `expect_value::<Vec<RichText>>()` which is more explicit about the expected property type.
    /// This will also return an error if the property is not a title, even if `Vec<RichText>` is implemented for the property.
    fn expect_title(&self) -> Result<Vec<RichText>, WrongPropertyTypeError> {
        match self.value() {
            PropertyValue::Title { .. } => self.value().to_owned().try_into(),
            _ => Err(WrongPropertyTypeError {
                expected: vec!["Title".to_string()],
                actual: self.type_name(),
            }),
        }
    }

    /// Allows for easy access to the text property value.
    /// This is a shortcut for `expect_value::<Vec<RichText>>()` which is more explicit about the expected property type.
    /// This will also return an error if the property is not a text, even if `Vec<RichText>` is implemented for the property.
    fn expect_text(&self) -> Result<Vec<RichText>, WrongPropertyTypeError> {
        match self.value() {
            PropertyValue::Text { .. } => self.value().to_owned().try_into(),
            _ => Err(WrongPropertyTypeError {
                expected: vec!["Text".to_string()],
                actual: self.type_name(),
            }),
        }
    }

    /// Allows for easy access to the number property value.
    /// This is a shortcut for `expect_value::<Number>()` which is more explicit about the expected property type.
    /// This will also return an error if the property is not a number, even if `Option<Number>` is implemented for the property.
    fn expect_number(&self) -> Result<Option<Number>, WrongPropertyTypeError> {
        match self.value() {
            PropertyValue::Number { .. } => self.value().to_owned().try_into(),
            _ => Err(WrongPropertyTypeError {
                expected: vec!["Number".to_string()],
                actual: self.type_name(),
            }),
        }
    }

    /// Allows for easy access to the select property value.
    /// This is a shortcut for `expect_value::<Option<SelectedValue>>()` which is more explicit about the expected property type.
    /// This will also return an error if the property is not a select, even if `Option<SelectedValue>` is implemented for the property.
    fn expect_select(&self) -> Result<Option<SelectedValue>, WrongPropertyTypeError> {
        match self.value() {
            PropertyValue::Select { .. } => self.value().to_owned().try_into(),
            _ => Err(WrongPropertyTypeError {
                expected: vec!["Select".to_string()],
                actual: self.type_name(),
            }),
        }
    }

    /// Allows for easy access to the status property value.
    /// This is a shortcut for `expect_value::<Option<SelectedValue>>()` which is more explicit about the expected property type.
    /// This will also return an error if the property is not a status, even if `Option<SelectedValue>` is implemented for the property.
    fn expect_status(&self) -> Result<Option<SelectedValue>, WrongPropertyTypeError> {
        match self.value() {
            PropertyValue::Status { .. } => self.value().to_owned().try_into(),
            _ => Err(WrongPropertyTypeError {
                expected: vec!["Status".to_string()],
                actual: self.type_name(),
            }),
        }
    }

    /// Allows for easy access to the multi-select property value.
    /// This is a shortcut for `expect_value::<Vec<SelectedValue>>()` which is more explicit about the expected property type.
    /// This will also return an error if the property is not a multi-select, even if `Option<Vec<SelectedValue>>` is implemented for the property.
    fn expect_multi_select(&self) -> Result<Option<Vec<SelectedValue>>, WrongPropertyTypeError> {
        match self.value() {
            PropertyValue::MultiSelect { .. } => self.value().to_owned().try_into(),
            _ => Err(WrongPropertyTypeError {
                expected: vec!["MultiSelect".to_string()],
                actual: self.type_name(),
            }),
        }
    }

    /// Allows for easy access to the date property value.
    /// This is a shortcut for `expect_value::<Option<DateValue>>()` which is more explicit about the expected property type.
    /// This will also return an error if the property is not a date, even if `Option<DateValue>` is implemented for the property.
    fn expect_date(&self) -> Result<Option<DateValue>, WrongPropertyTypeError> {
        match self.value() {
            PropertyValue::Date { .. } => self.value().to_owned().try_into(),
            _ => Err(WrongPropertyTypeError {
                expected: vec!["Date".to_string()],
                actual: self.type_name(),
            }),
        }
    }

    /// Allows for easy access to the people property value.
    /// This is a shortcut for `expect_value::<Vec<User>>()` which is more explicit about the expected property type.
    /// This will also return an error if the property is not a people, even if `Option<Vec<User>>` is implemented for the property.
    fn expect_people(&self) -> Result<Option<Vec<User>>, WrongPropertyTypeError> {
        match self.value() {
            PropertyValue::People { .. } => self.value().to_owned().try_into(),
            _ => Err(WrongPropertyTypeError {
                expected: vec!["People".to_string()],
                actual: self.type_name(),
            }),
        }
    }

    /// Allows for easy access to the files property value.
    /// This is a shortcut for `expect_value::<Option<Vec<FileReference>>>()` which is more explicit about the expected property type.
    /// This will also return an error if the property is not a files, even if `Option<Vec<FileReference>>` is implemented for the property.
    fn expect_files(&self) -> Result<Option<Vec<FileReference>>, WrongPropertyTypeError> {
        match self.value() {
            PropertyValue::Files { .. } => self.value().to_owned().try_into(),
            _ => Err(WrongPropertyTypeError {
                expected: vec!["Files".to_string()],
                actual: self.type_name(),
            }),
        }
    }

    /// Allows for easy access to the checkbox property value.
    /// This is a shortcut for `expect_value::<bool>()` which is more explicit about the expected property type.
    /// This will also return an error if the property is not a checkbox, even if `bool` is implemented for the property.
    fn expect_checkbox(&self) -> Result<bool, WrongPropertyTypeError> {
        match self.value() {
            PropertyValue::Checkbox { .. } => self.value().to_owned().try_into(),
            _ => Err(WrongPropertyTypeError {
                expected: vec!["Checkbox".to_string()],
                actual: self.type_name(),
            }),
        }
    }

    /// Allows for easy access to the url property value.
    /// This is a shortcut for `expect_value::<Option<String>>()` which is more explicit about the expected property type.
    /// This will also return an error if the property is not a url, even if `Option<String>` is implemented for the property.
    fn expect_url(&self) -> Result<Option<String>, WrongPropertyTypeError> {
        match self.value() {
            PropertyValue::Url { .. } => self.value().to_owned().try_into(),
            _ => Err(WrongPropertyTypeError {
                expected: vec!["Url".to_string()],
                actual: self.type_name(),
            }),
        }
    }

    /// Allows for easy access to the email property value.
    /// This is a shortcut for `expect_value::<Option<String>>()` which is more explicit about the expected property type.
    /// This will also return an error if the property is not an email, even if `Option<String>` is implemented for the property.
    fn expect_email(&self) -> Result<Option<String>, WrongPropertyTypeError> {
        match self.value() {
            PropertyValue::Email { .. } => self.value().to_owned().try_into(),
            _ => Err(WrongPropertyTypeError {
                expected: vec!["Email".to_string()],
                actual: self.type_name(),
            }),
        }
    }

    /// Allows for easy access to the phone number property value.
    /// This is a shortcut for `expect_value::<Option<String>>()` which is more explicit about the expected property type.
    /// This will also return an error if the property is not a phone number, even if `Option<String>` is implemented for the property.
    fn expect_phone_number(&self) -> Result<Option<String>, WrongPropertyTypeError> {
        match self.value() {
            PropertyValue::PhoneNumber { .. } => self.value().to_owned().try_into(),
            _ => Err(WrongPropertyTypeError {
                expected: vec!["PhoneNumber".to_string()],
                actual: self.type_name(),
            }),
        }
    }

    /// Allows for easy access to the formula property value.
    /// This is a shortcut for `expect_value::<FormulaResultValue>()` which is more explicit about the expected property type.
    /// This will also return an error if the property is not a formula, even if `FormulaResultValue` is implemented for the property.
    fn expect_formula(&self) -> Result<FormulaResultValue, WrongPropertyTypeError> {
        match self.value() {
            PropertyValue::Formula { .. } => self.value().to_owned().try_into(),
            _ => Err(WrongPropertyTypeError {
                expected: vec!["Formula".to_string()],
                actual: self.type_name(),
            }),
        }
    }

    /// Allows for easy access to the relation property value.
    /// This is a shortcut for `expect_value::<Option<Vec<RelationValue>>>()` which is more explicit about the expected property type.
    /// This will also return an error if the property is not a relation, even if `Option<Vec<RelationValue>>` is implemented for the property.
    fn expect_relation(&self) -> Result<Option<Vec<RelationValue>>, WrongPropertyTypeError> {
        match self.value() {
            PropertyValue::Relation { .. } => self.value().to_owned().try_into(),
            _ => Err(WrongPropertyTypeError {
                expected: vec!["Relation".to_string()],
                actual: self.type_name(),
            }),
        }
    }

    /// Allows for easy access to the rollup property value.
    /// This is a shortcut for `expect_value::<Option<RollupValue>>()` which is more explicit about the expected property type.
    /// This will also return an error if the property is not a rollup, even if `Option<RollupValue>` is implemented for the property.
    fn expect_rollup(&self) -> Result<Option<RollupValue>, WrongPropertyTypeError> {
        match self.value() {
            PropertyValue::Rollup { .. } => self.value().to_owned().try_into(),
            _ => Err(WrongPropertyTypeError {
                expected: vec!["Rollup".to_string()],
                actual: self.type_name(),
            }),
        }
    }

    /// Allows for easy access to the created time property value.
    /// This is a shortcut for `expect_value::<DateTime<Utc>>()` which is more explicit about the expected property type.
    /// This will also return an error if the property is not a created time, even if `DateTime<Utc>` is implemented for the property.
    fn expect_created_time(&self) -> Result<DateTime<Utc>, WrongPropertyTypeError> {
        match self.value() {
            PropertyValue::CreatedTime { .. } => self.value().to_owned().try_into(),
            _ => Err(WrongPropertyTypeError {
                expected: vec!["CreatedTime".to_string()],
                actual: self.type_name(),
            }),
        }
    }

    /// Allows for easy access to the created by property value.
    /// This is a shortcut for `expect_value::<User>()` which is more explicit about the expected property type.
    /// This will also return an error if the property is not a created by, even if `User` is implemented for the property.
    fn expect_created_by(&self) -> Result<User, WrongPropertyTypeError> {
        match self.value() {
            PropertyValue::CreatedBy { .. } => self.value().to_owned().try_into(),
            _ => Err(WrongPropertyTypeError {
                expected: vec!["CreatedBy".to_string()],
                actual: self.type_name(),
            }),
        }
    }

    /// Allows for easy access to the last edited time property value.
    /// This is a shortcut for `expect_value::<DateTime<Utc>>()` which is more explicit about the expected property type.
    /// This will also return an error if the property is not a last edited time, even if `DateTime<Utc>` is implemented for the property.
    fn expect_last_edited_time(&self) -> Result<DateTime<Utc>, WrongPropertyTypeError> {
        match self.value() {
            PropertyValue::LastEditedTime { .. } => self.value().to_owned().try_into(),
            _ => Err(WrongPropertyTypeError {
                expected: vec!["LastEditedTime".to_string()],
                actual: self.type_name(),
            }),
        }
    }

    /// Allows for easy access to the last edited by property value.
    /// This is a shortcut for `expect_value::<User>()` which is more explicit about the expected property type.
    /// This will also return an error if the property is not a last edited by, even if `User` is implemented for the property.
    fn expect_last_edited_by(&self) -> Result<User, WrongPropertyTypeError> {
        match self.value() {
            PropertyValue::LastEditedBy { .. } => self.value().to_owned().try_into(),
            _ => Err(WrongPropertyTypeError {
                expected: vec!["LastEditedBy".to_string()],
                actual: self.type_name(),
            }),
        }
    }

    /// Allows for easy access to the button property value.
    /// This is a shortcut for `expect_value::<()>()` which is more explicit about the expected property type.
    /// This will also return an error if the property is not a button, even if `()` is implemented for the property.
    fn expect_button(&self) -> Result<(), WrongPropertyTypeError> {
        match self.value() {
            PropertyValue::Button { .. } => Ok(()),
            _ => Err(WrongPropertyTypeError {
                expected: vec!["Button".to_string()],
                actual: self.type_name().to_string(),
            }),
        }
    }
}

#[derive(Serialize, Deserialize, Eq, PartialEq, Debug, Clone)]
pub struct WrongPropertyTypeError {
    pub expected: Vec<String>,
    pub actual: String,
}

impl Display for WrongPropertyTypeError {
    fn fmt(
        &self,
        f: &mut Formatter<'_>,
    ) -> std::fmt::Result {
        write!(
            f,
            "Wrong property type: expected one of {:?}, got {}",
            self.expected, self.actual
        )
    }
}

pub trait FromPropertyValue: Sized {
    fn from_property_value(property: impl Property) -> Result<Self, WrongPropertyTypeError>;
}

impl TryFrom<PropertyValue> for Vec<RichText> {
    type Error = WrongPropertyTypeError;

    fn try_from(value: PropertyValue) -> Result<Self, Self::Error> {
        match value {
            PropertyValue::Title { title, .. } => Ok(title),
            PropertyValue::Text { rich_text, .. } => Ok(rich_text),
            _ => Err(WrongPropertyTypeError {
                expected: vec!["Title".to_string(), "Text".to_string()],
                actual: value.type_name(),
            }),
        }
    }
}

impl TryFrom<PropertyValue> for Option<Number> {
    type Error = WrongPropertyTypeError;

    fn try_from(value: PropertyValue) -> Result<Self, Self::Error> {
        match value {
            PropertyValue::Number { number, .. } => Ok(number),
            _ => Err(WrongPropertyTypeError {
                expected: vec!["Number".to_string()],
                actual: value.type_name(),
            }),
        }
    }
}

impl TryFrom<PropertyValue> for Option<SelectedValue> {
    type Error = WrongPropertyTypeError;

    fn try_from(value: PropertyValue) -> Result<Self, Self::Error> {
        match value {
            PropertyValue::Select { select, .. } => Ok(select),
            PropertyValue::Status { status, .. } => Ok(status),
            _ => Err(WrongPropertyTypeError {
                expected: vec!["Select".to_string(), "Status".to_string()],
                actual: value.type_name(),
            }),
        }
    }
}

impl TryFrom<PropertyValue> for Option<Vec<SelectedValue>> {
    type Error = WrongPropertyTypeError;

    fn try_from(value: PropertyValue) -> Result<Self, Self::Error> {
        match value {
            PropertyValue::MultiSelect { multi_select, .. } => Ok(multi_select),
            _ => Err(WrongPropertyTypeError {
                expected: vec!["MultiSelect".to_string()],
                actual: value.type_name(),
            }),
        }
    }
}

impl TryFrom<PropertyValue> for Option<DateValue> {
    type Error = WrongPropertyTypeError;

    fn try_from(value: PropertyValue) -> Result<Self, Self::Error> {
        match value {
            PropertyValue::Date { date, .. } => Ok(date),
            _ => Err(WrongPropertyTypeError {
                expected: vec!["Date".to_string()],
                actual: value.type_name(),
            }),
        }
    }
}

impl TryFrom<PropertyValue> for FormulaResultValue {
    type Error = WrongPropertyTypeError;

    fn try_from(value: PropertyValue) -> Result<Self, Self::Error> {
        match value {
            PropertyValue::Formula { formula, .. } => Ok(formula),
            _ => Err(WrongPropertyTypeError {
                expected: vec!["Formula".to_string()],
                actual: value.type_name(),
            }),
        }
    }
}

impl TryFrom<PropertyValue> for Option<Vec<RelationValue>> {
    type Error = WrongPropertyTypeError;

    fn try_from(value: PropertyValue) -> Result<Self, Self::Error> {
        match value {
            PropertyValue::Relation { relation, .. } => Ok(relation),
            _ => Err(WrongPropertyTypeError {
                expected: vec!["Relation".to_string()],
                actual: value.type_name(),
            }),
        }
    }
}

impl TryFrom<PropertyValue> for Option<Vec<FileReference>> {
    type Error = WrongPropertyTypeError;

    fn try_from(value: PropertyValue) -> Result<Self, Self::Error> {
        match value {
            PropertyValue::Files { files, .. } => Ok(files),
            _ => Err(WrongPropertyTypeError {
                expected: vec!["Files".to_string()],
                actual: value.type_name(),
            }),
        }
    }
}

impl TryFrom<PropertyValue> for bool {
    type Error = WrongPropertyTypeError;

    fn try_from(value: PropertyValue) -> Result<Self, Self::Error> {
        match value {
            PropertyValue::Checkbox { checkbox, .. } => Ok(checkbox),
            _ => Err(WrongPropertyTypeError {
                expected: vec!["Checkbox".to_string()],
                actual: value.type_name(),
            }),
        }
    }
}

impl TryFrom<PropertyValue> for Option<String> {
    type Error = WrongPropertyTypeError;

    fn try_from(value: PropertyValue) -> Result<Self, Self::Error> {
        match value {
            PropertyValue::Url { url, .. } => Ok(url),
            PropertyValue::Email { email, .. } => Ok(email),
            PropertyValue::PhoneNumber { phone_number, .. } => Ok(phone_number),
            _ => Err(WrongPropertyTypeError {
                expected: vec![
                    "Url".to_string(),
                    "Email".to_string(),
                    "PhoneNumber".to_string(),
                ],
                actual: value.type_name(),
            }),
        }
    }
}

impl TryFrom<PropertyValue> for DateTime<Utc> {
    type Error = WrongPropertyTypeError;

    fn try_from(value: PropertyValue) -> Result<Self, Self::Error> {
        match value {
            PropertyValue::CreatedTime { created_time, .. } => Ok(created_time),
            PropertyValue::LastEditedTime {
                last_edited_time, ..
            } => Ok(last_edited_time),
            _ => Err(WrongPropertyTypeError {
                expected: vec!["CreatedTime".to_string(), "LastEditedTime".to_string()],
                actual: value.type_name(),
            }),
        }
    }
}

impl TryFrom<PropertyValue> for User {
    type Error = WrongPropertyTypeError;

    fn try_from(value: PropertyValue) -> Result<Self, Self::Error> {
        match value {
            PropertyValue::CreatedBy { created_by, .. } => Ok(created_by),
            PropertyValue::LastEditedBy { last_edited_by, .. } => Ok(last_edited_by),
            _ => Err(WrongPropertyTypeError {
                expected: vec!["CreatedBy".to_string(), "LastEditedBy".to_string()],
                actual: value.type_name(),
            }),
        }
    }
}

impl TryFrom<PropertyValue> for Option<Vec<User>> {
    type Error = WrongPropertyTypeError;

    fn try_from(value: PropertyValue) -> Result<Self, Self::Error> {
        match value {
            PropertyValue::People { people, .. } => Ok(Some(people)),
            _ => Err(WrongPropertyTypeError {
                expected: vec!["People".to_string()],
                actual: value.type_name(),
            }),
        }
    }
}

impl TryFrom<PropertyValue> for Option<RollupValue> {
    type Error = WrongPropertyTypeError;

    fn try_from(value: PropertyValue) -> Result<Self, Self::Error> {
        match value {
            PropertyValue::Rollup { rollup, .. } => Ok(rollup),
            _ => Err(WrongPropertyTypeError {
                expected: vec!["Rollup".to_string()],
                actual: value.type_name(),
            }),
        }
    }
}
