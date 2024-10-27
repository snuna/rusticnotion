use std::fmt::{Display, Formatter};

use crate::models::text::RichText;
use crate::models::users::User;

use crate::ids::{DatabaseId, PageId, PropertyId};
use crate::models::{DateTime, Number, Utc};
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[cfg(test)]
mod tests;

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
pub enum PropertyConfiguration {
    /// Represents the special Title property required on every database.
    /// See <https://developers.notion.com/reference/database#title-configuration>
    Title {
        id: PropertyId,
    },
    /// Represents a Text property
    /// <https://developers.notion.com/reference/database#text-configuration>
    #[serde(rename = "rich_text")]
    Text {
        id: PropertyId,
    },
    /// Represents a Number Property
    /// See <https://developers.notion.com/reference/database#number-configuration>
    Number {
        id: PropertyId,
        /// How the number is displayed in Notion.
        number: NumberDetails,
    },
    /// Represents a Select Property
    /// See <https://developers.notion.com/reference/database#select-configuration>
    Select {
        id: PropertyId,
        select: Select,
    },
    /// Represents a Status property
    Status {
        id: PropertyId,
        status: Status,
    },
    /// Represents a Multi-select Property
    /// See <https://developers.notion.com/reference/database#multi-select-configuration>
    MultiSelect {
        id: PropertyId,
        multi_select: Select,
    },
    /// Represents a Date Property
    /// See <https://developers.notion.com/reference/database#date-configuration>
    Date {
        id: PropertyId,
    },
    /// Represents a People Property
    /// See <https://developers.notion.com/reference/database#people-configuration>
    People {
        id: PropertyId,
    },
    /// Represents a File Property
    /// See <https://developers.notion.com/reference/database#file-configuration>
    // Todo: File a bug with notion
    //       Documentation issue: docs claim type name is `file` but it is in fact `files`
    Files {
        id: PropertyId,
    },
    /// Represents a Checkbox Property
    /// See <https://developers.notion.com/reference/database#checkbox-configuration>
    Checkbox {
        id: PropertyId,
    },
    /// Represents a URL Property
    /// See <https://developers.notion.com/reference/database#url-configuration>
    Url {
        id: PropertyId,
    },
    /// Represents a Email Property
    /// See <https://developers.notion.com/reference/database#email-configuration>
    Email {
        id: PropertyId,
    },
    /// Represents a Phone number Property
    /// See <https://developers.notion.com/reference/database#phone-number-configuration>
    PhoneNumber {
        id: PropertyId,
    },
    /// See <https://developers.notion.com/reference/database#formula-configuration>
    Formula {
        id: PropertyId,
        formula: Formula,
    },
    /// See <https://developers.notion.com/reference/database#relation-configuration>
    Relation {
        id: PropertyId,
        relation: Relation,
    },
    /// See <https://developers.notion.com/reference/database#rollup-configuration>
    Rollup {
        id: PropertyId,
        rollup: Rollup,
    },
    /// See <https://developers.notion.com/reference/database#created-time-configuration>
    CreatedTime {
        id: PropertyId,
    },
    /// See <https://developers.notion.com/reference/database#created-by-configuration>
    CreatedBy {
        id: PropertyId,
    },
    /// See <https://developers.notion.com/reference/database#last-edited-time-configuration>
    LastEditedTime {
        id: PropertyId,
    },
    /// See <https://developers.notion.com/reference/database#last-edited-by-configuration>
    LastEditBy {
        id: PropertyId,
    },

    Button {
        id: PropertyId,
    },
}
impl PropertyConfiguration {
    pub fn id(&self) -> &PropertyId {
        match self {
            PropertyConfiguration::Title { id } => id,
            PropertyConfiguration::Text { id } => id,
            PropertyConfiguration::Number { id, .. } => id,
            PropertyConfiguration::Select { id, .. } => id,
            PropertyConfiguration::Status { id, .. } => id,
            PropertyConfiguration::MultiSelect { id, .. } => id,
            PropertyConfiguration::Date { id } => id,
            PropertyConfiguration::People { id } => id,
            PropertyConfiguration::Files { id } => id,
            PropertyConfiguration::Checkbox { id } => id,
            PropertyConfiguration::Url { id } => id,
            PropertyConfiguration::Email { id } => id,
            PropertyConfiguration::PhoneNumber { id } => id,
            PropertyConfiguration::Formula { id, .. } => id,
            PropertyConfiguration::Relation { id, .. } => id,
            PropertyConfiguration::Rollup { id, .. } => id,
            PropertyConfiguration::CreatedTime { id } => id,
            PropertyConfiguration::CreatedBy { id } => id,
            PropertyConfiguration::LastEditedTime { id } => id,
            PropertyConfiguration::LastEditBy { id } => id,
            PropertyConfiguration::Button { id } => id,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct SelectedValue {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<SelectOptionId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    pub color: Color,
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

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]

pub enum PropertyValue {
    // <https://developers.notion.com/reference/property-object#title-configuration>
    Title {
        id: PropertyId,
        title: Vec<RichText>,
    },
    /// <https://developers.notion.com/reference/property-object#text-configuration>
    #[serde(rename = "rich_text")]
    Text {
        id: PropertyId,
        rich_text: Vec<RichText>,
    },
    /// <https://developers.notion.com/reference/property-object#number-configuration>
    Number {
        id: PropertyId,
        number: Option<Number>,
    },
    /// <https://developers.notion.com/reference/property-object#select-configuration>
    Select {
        id: PropertyId,
        select: Option<SelectedValue>,
    },
    /// <https://developers.notion.com/reference/property-object#status-configuration>
    Status {
        id: PropertyId,
        status: Option<SelectedValue>,
    },
    /// <https://developers.notion.com/reference/property-object#multi-select-configuration>
    MultiSelect {
        id: PropertyId,
        multi_select: Option<Vec<SelectedValue>>,
    },
    /// <https://developers.notion.com/reference/property-object#date-configuration>
    Date {
        id: PropertyId,
        date: Option<DateValue>,
    },
    /// <https://developers.notion.com/reference/property-object#formula-configuration>
    Formula {
        id: PropertyId,
        formula: FormulaResultValue,
    },
    /// <https://developers.notion.com/reference/property-object#relation-configuration>
    /// It is actually an array of relations
    Relation {
        id: PropertyId,
        relation: Option<Vec<RelationValue>>,
    },
    /// <https://developers.notion.com/reference/property-object#rollup-configuration>
    Rollup {
        id: PropertyId,
        rollup: Option<RollupValue>,
    },
    /// <https://developers.notion.com/reference/property-object#people-configuration>
    People {
        id: PropertyId,
        people: Vec<User>,
    },
    /// <https://developers.notion.com/reference/property-object#files-configuration>
    Files {
        id: PropertyId,
        files: Option<Vec<FileReference>>,
    },
    /// <https://developers.notion.com/reference/property-object#checkbox-configuration>
    Checkbox {
        id: PropertyId,
        checkbox: bool,
    },
    /// <https://developers.notion.com/reference/property-object#url-configuration>
    Url {
        id: PropertyId,
        url: Option<String>,
    },
    /// <https://developers.notion.com/reference/property-object#email-configuration>
    Email {
        id: PropertyId,
        email: Option<String>,
    },
    /// <https://developers.notion.com/reference/property-object#phone-number-configuration>
    PhoneNumber {
        id: PropertyId,
        phone_number: Option<String>,
    },
    /// <https://developers.notion.com/reference/property-object#created-time-configuration>
    CreatedTime {
        id: PropertyId,
        created_time: DateTime<Utc>,
    },
    /// <https://developers.notion.com/reference/property-object#created-by-configuration>
    CreatedBy {
        id: PropertyId,
        created_by: User,
    },
    /// <https://developers.notion.com/reference/property-object#last-edited-time-configuration>
    LastEditedTime {
        id: PropertyId,
        last_edited_time: DateTime<Utc>,
    },
    /// <https://developers.notion.com/reference/property-object#last-edited-by-configuration>
    LastEditedBy {
        id: PropertyId,
        last_edited_by: User,
    },
    Button {
        id: PropertyId,
    },
}
impl PropertyValue {
    pub fn id(&self) -> &PropertyId {
        match self {
            PropertyValue::Title { id, .. } => id,
            PropertyValue::Text { id, .. } => id,
            PropertyValue::Number { id, .. } => id,
            PropertyValue::Select { id, .. } => id,
            PropertyValue::Status { id, .. } => id,
            PropertyValue::MultiSelect { id, .. } => id,
            PropertyValue::Date { id, .. } => id,
            PropertyValue::People { id, .. } => id,
            PropertyValue::Files { id, .. } => id,
            PropertyValue::Checkbox { id, .. } => id,
            PropertyValue::Url { id, .. } => id,
            PropertyValue::Email { id, .. } => id,
            PropertyValue::PhoneNumber { id, .. } => id,
            PropertyValue::Formula { id, .. } => id,
            PropertyValue::Relation { id, .. } => id,
            PropertyValue::Rollup { id, .. } => id,
            PropertyValue::CreatedTime { id, .. } => id,
            PropertyValue::CreatedBy { id, .. } => id,
            PropertyValue::LastEditedTime { id, .. } => id,
            PropertyValue::LastEditedBy { id, .. } => id,
            PropertyValue::Button { id } => id,
        }
    }

    /// Allows for easy access to the property value.
    ///
    /// This is useful if you know the type of the property you want to access and don't need match statements
    ///
    /// ```ignore
    /// let title = property.expect_value::<Vec<RichText>>().unwrap();
    /// ```
    /// This will fail if the actual property type is not compatible with the expected value type.
    ///
    /// See the following implementations of `FromPropertyValue` for supported types:
    /// - [`Vec<RichText>`](PropertyValue::from_property_value::<Vec<RichText>>) for Title, Text
    /// - [`Option<Number>`](PropertyValue::from_property_value::<Option<Number>>) for Number
    /// - [`Option<SelectedValue>`](PropertyValue::from_property_value::<Option<SelectedValue>>) for Select, Status, MultiSelect
    /// - [`Option<Vec<SelectedValue>>`](PropertyValue::from_property_value::<Option<Vec<SelectedValue>>>) for MultiSelect
    /// - [`Option<DateValue>`](PropertyValue::from_property_value::<Option<DateValue>>) for Date
    /// - [`Option<Vec<RelationValue>>`](PropertyValue::from_property_value::<Option<Vec<RelationValue>>>) for Relation
    /// - [`Option<Vec<FileReference>>`](PropertyValue::from_property_value::<Option<Vec<FileReference>>>) for Files
    /// - [`bool`](PropertyValue::from_property_value::<bool>) for Checkbox
    /// - [`Option<String>`](PropertyValue::from_property_value::<Option<String>>) for Url, Email, PhoneNumber
    /// - [`DateTime<Utc>`](PropertyValue::from_property_value::<DateTime<Utc>>) for CreatedTime, LastEditedTime
    /// - [`User`](PropertyValue::from_property_value::<User>) for CreatedBy, LastEditedBy
    ///
    /// You can also create your own implementation of `FromPropertyValue` for custom types. This is useful if you
    /// need to convert the property value to a specific type in your project.
    ///
    /// For example if you want to access text property values always as a `String` without formatting you can do the following:
    ///
    /// ```ignore
    /// use rusticnotion::models::properties::{FromPropertyValue, PropertyValue, WrongPropertyTypeError}
    ///
    /// // We can use the new type pattern to create a custom type that implements FromPropertyValue
    /// // This allows us easy access to property value in our desired type
    /// struct TextValue(String);
    /// impl FromPropertyValue for TextValue {
    ///     fn from_property_value(value: PropertyValue) -> Result<Self, WrongPropertyTypeError> {
    ///         match value {
    ///             PropertyValue::Text { rich_text, .. } => {
    ///                 let combined_text = rich_text
    ///                     .iter()
    ///                     .map(|rt| rt.plain_text().to_string())
    ///                     .collect::<Vec<String>>()
    ///                     .join("");
    ///                 Ok(TextValue(combined_text))
    ///             }
    ///             _ => Err(WrongPropertyTypeError {
    ///                 expected: vec!["Text".to_string()],
    ///                 actual: value.type_name(),
    ///             }),
    ///         }
    ///     }
    /// }
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
    pub fn expect_value<T: FromPropertyValue>(&self) -> Result<T, WrongPropertyTypeError> {
        T::from_property_value(self.clone())
    }

    /// Allows for easy access to the title property value.
    /// This is a shortcut for `expect_value::<Vec<RichText>>()` which is more explicit about the expected property type.
    /// This will also return an error if the property is not a title, even if `Vec<RichText>` is implemented for the property.
    pub fn expect_title(&self) -> Result<Vec<RichText>, WrongPropertyTypeError> {
        match self {
            PropertyValue::Title { .. } => self.expect_value::<Vec<RichText>>(),
            _ => Err(WrongPropertyTypeError {
                expected: vec!["Title".to_string()],
                actual: self.type_name(),
            }),
        }
    }

    /// Allows for easy access to the text property value.
    /// This is a shortcut for `expect_value::<Vec<RichText>>()` which is more explicit about the expected property type.
    /// This will also return an error if the property is not a text, even if `Vec<RichText>` is implemented for the property.
    pub fn expect_text(&self) -> Result<Vec<RichText>, WrongPropertyTypeError> {
        match self {
            PropertyValue::Text { .. } => self.expect_value::<Vec<RichText>>(),
            _ => Err(WrongPropertyTypeError {
                expected: vec!["Text".to_string()],
                actual: self.type_name(),
            }),
        }
    }

    /// Allows for easy access to the number property value.
    /// This is a shortcut for `expect_value::<Number>()` which is more explicit about the expected property type.
    /// This will also return an error if the property is not a number, even if `Option<Number>` is implemented for the property.
    pub fn expect_number(&self) -> Result<Option<Number>, WrongPropertyTypeError> {
        match self {
            PropertyValue::Number { .. } => self.expect_value::<Option<Number>>(),
            _ => Err(WrongPropertyTypeError {
                expected: vec!["Number".to_string()],
                actual: self.type_name(),
            }),
        }
    }

    /// Allows for easy access to the select property value.
    /// This is a shortcut for `expect_value::<Option<SelectedValue>>()` which is more explicit about the expected property type.
    /// This will also return an error if the property is not a select, even if `Option<SelectedValue>` is implemented for the property.
    pub fn expect_select(&self) -> Result<Option<SelectedValue>, WrongPropertyTypeError> {
        match self {
            PropertyValue::Select { .. } => self.expect_value::<Option<SelectedValue>>(),
            _ => Err(WrongPropertyTypeError {
                expected: vec!["Select".to_string()],
                actual: self.type_name(),
            }),
        }
    }

    /// Allows for easy access to the status property value.
    /// This is a shortcut for `expect_value::<Option<SelectedValue>>()` which is more explicit about the expected property type.
    /// This will also return an error if the property is not a status, even if `Option<SelectedValue>` is implemented for the property.
    pub fn expect_status(&self) -> Result<Option<SelectedValue>, WrongPropertyTypeError> {
        match self {
            PropertyValue::Status { .. } => self.expect_value::<Option<SelectedValue>>(),
            _ => Err(WrongPropertyTypeError {
                expected: vec!["Status".to_string()],
                actual: self.type_name(),
            }),
        }
    }

    /// Allows for easy access to the multi-select property value.
    /// This is a shortcut for `expect_value::<Vec<SelectedValue>>()` which is more explicit about the expected property type.
    /// This will also return an error if the property is not a multi-select, even if `Option<Vec<SelectedValue>>` is implemented for the property.
    pub fn expect_multi_select(
        &self
    ) -> Result<Option<Vec<SelectedValue>>, WrongPropertyTypeError> {
        match self {
            PropertyValue::MultiSelect { .. } => self.expect_value::<Option<Vec<SelectedValue>>>(),
            _ => Err(WrongPropertyTypeError {
                expected: vec!["MultiSelect".to_string()],
                actual: self.type_name(),
            }),
        }
    }

    /// Allows for easy access to the date property value.
    /// This is a shortcut for `expect_value::<Option<DateValue>>()` which is more explicit about the expected property type.
    /// This will also return an error if the property is not a date, even if `Option<DateValue>` is implemented for the property.
    pub fn expect_date(&self) -> Result<Option<DateValue>, WrongPropertyTypeError> {
        match self {
            PropertyValue::Date { .. } => self.expect_value::<Option<DateValue>>(),
            _ => Err(WrongPropertyTypeError {
                expected: vec!["Date".to_string()],
                actual: self.type_name(),
            }),
        }
    }

    /// Allows for easy access to the people property value.
    /// This is a shortcut for `expect_value::<Vec<User>>()` which is more explicit about the expected property type.
    /// This will also return an error if the property is not a people, even if `Option<Vec<User>>` is implemented for the property.
    pub fn expect_people(&self) -> Result<Option<Vec<User>>, WrongPropertyTypeError> {
        match self {
            PropertyValue::People { .. } => self.expect_value::<Option<Vec<User>>>(),
            _ => Err(WrongPropertyTypeError {
                expected: vec!["People".to_string()],
                actual: self.type_name(),
            }),
        }
    }

    /// Allows for easy access to the files property value.
    /// This is a shortcut for `expect_value::<Option<Vec<FileReference>>>()` which is more explicit about the expected property type.
    /// This will also return an error if the property is not a files, even if `Option<Vec<FileReference>>` is implemented for the property.
    pub fn expect_files(&self) -> Result<Option<Vec<FileReference>>, WrongPropertyTypeError> {
        match self {
            PropertyValue::Files { .. } => self.expect_value::<Option<Vec<FileReference>>>(),
            _ => Err(WrongPropertyTypeError {
                expected: vec!["Files".to_string()],
                actual: self.type_name(),
            }),
        }
    }

    /// Allows for easy access to the checkbox property value.
    /// This is a shortcut for `expect_value::<bool>()` which is more explicit about the expected property type.
    /// This will also return an error if the property is not a checkbox, even if `bool` is implemented for the property.
    pub fn expect_checkbox(&self) -> Result<bool, WrongPropertyTypeError> {
        match self {
            PropertyValue::Checkbox { .. } => self.expect_value::<bool>(),
            _ => Err(WrongPropertyTypeError {
                expected: vec!["Checkbox".to_string()],
                actual: self.type_name(),
            }),
        }
    }

    /// Allows for easy access to the url property value.
    /// This is a shortcut for `expect_value::<Option<String>>()` which is more explicit about the expected property type.
    /// This will also return an error if the property is not a url, even if `Option<String>` is implemented for the property.
    pub fn expect_url(&self) -> Result<Option<String>, WrongPropertyTypeError> {
        match self {
            PropertyValue::Url { .. } => self.expect_value::<Option<String>>(),
            _ => Err(WrongPropertyTypeError {
                expected: vec!["Url".to_string()],
                actual: self.type_name(),
            }),
        }
    }

    /// Allows for easy access to the email property value.
    /// This is a shortcut for `expect_value::<Option<String>>()` which is more explicit about the expected property type.
    /// This will also return an error if the property is not an email, even if `Option<String>` is implemented for the property.
    pub fn expect_email(&self) -> Result<Option<String>, WrongPropertyTypeError> {
        match self {
            PropertyValue::Email { .. } => self.expect_value::<Option<String>>(),
            _ => Err(WrongPropertyTypeError {
                expected: vec!["Email".to_string()],
                actual: self.type_name(),
            }),
        }
    }

    /// Allows for easy access to the phone number property value.
    /// This is a shortcut for `expect_value::<Option<String>>()` which is more explicit about the expected property type.
    /// This will also return an error if the property is not a phone number, even if `Option<String>` is implemented for the property.
    pub fn expect_phone_number(&self) -> Result<Option<String>, WrongPropertyTypeError> {
        match self {
            PropertyValue::PhoneNumber { .. } => self.expect_value::<Option<String>>(),
            _ => Err(WrongPropertyTypeError {
                expected: vec!["PhoneNumber".to_string()],
                actual: self.type_name(),
            }),
        }
    }

    /// Allows for easy access to the formula property value.
    /// This is a shortcut for `expect_value::<FormulaResultValue>()` which is more explicit about the expected property type.
    /// This will also return an error if the property is not a formula, even if `FormulaResultValue` is implemented for the property.
    pub fn expect_formula(&self) -> Result<FormulaResultValue, WrongPropertyTypeError> {
        match self {
            PropertyValue::Formula { .. } => self.expect_value::<FormulaResultValue>(),
            _ => Err(WrongPropertyTypeError {
                expected: vec!["Formula".to_string()],
                actual: self.type_name(),
            }),
        }
    }

    /// Allows for easy access to the relation property value.
    /// This is a shortcut for `expect_value::<Option<Vec<RelationValue>>>()` which is more explicit about the expected property type.
    /// This will also return an error if the property is not a relation, even if `Option<Vec<RelationValue>>` is implemented for the property.
    pub fn expect_relation(&self) -> Result<Option<Vec<RelationValue>>, WrongPropertyTypeError> {
        match self {
            PropertyValue::Relation { .. } => self.expect_value::<Option<Vec<RelationValue>>>(),
            _ => Err(WrongPropertyTypeError {
                expected: vec!["Relation".to_string()],
                actual: self.type_name(),
            }),
        }
    }

    /// Allows for easy access to the rollup property value.
    /// This is a shortcut for `expect_value::<Option<RollupValue>>()` which is more explicit about the expected property type.
    /// This will also return an error if the property is not a rollup, even if `Option<RollupValue>` is implemented for the property.
    pub fn expect_rollup(&self) -> Result<Option<RollupValue>, WrongPropertyTypeError> {
        match self {
            PropertyValue::Rollup { .. } => self.expect_value::<Option<RollupValue>>(),
            _ => Err(WrongPropertyTypeError {
                expected: vec!["Rollup".to_string()],
                actual: self.type_name(),
            }),
        }
    }

    /// Allows for easy access to the created time property value.
    /// This is a shortcut for `expect_value::<DateTime<Utc>>()` which is more explicit about the expected property type.
    /// This will also return an error if the property is not a created time, even if `DateTime<Utc>` is implemented for the property.
    pub fn expect_created_time(&self) -> Result<DateTime<Utc>, WrongPropertyTypeError> {
        match self {
            PropertyValue::CreatedTime { .. } => self.expect_value::<DateTime<Utc>>(),
            _ => Err(WrongPropertyTypeError {
                expected: vec!["CreatedTime".to_string()],
                actual: self.type_name(),
            }),
        }
    }

    /// Allows for easy access to the created by property value.
    /// This is a shortcut for `expect_value::<User>()` which is more explicit about the expected property type.
    /// This will also return an error if the property is not a created by, even if `User` is implemented for the property.
    pub fn expect_created_by(&self) -> Result<User, WrongPropertyTypeError> {
        match self {
            PropertyValue::CreatedBy { .. } => self.expect_value::<User>(),
            _ => Err(WrongPropertyTypeError {
                expected: vec!["CreatedBy".to_string()],
                actual: self.type_name(),
            }),
        }
    }

    /// Allows for easy access to the last edited time property value.
    /// This is a shortcut for `expect_value::<DateTime<Utc>>()` which is more explicit about the expected property type.
    /// This will also return an error if the property is not a last edited time, even if `DateTime<Utc>` is implemented for the property.
    pub fn expect_last_edited_time(&self) -> Result<DateTime<Utc>, WrongPropertyTypeError> {
        match self {
            PropertyValue::LastEditedTime { .. } => self.expect_value::<DateTime<Utc>>(),
            _ => Err(WrongPropertyTypeError {
                expected: vec!["LastEditedTime".to_string()],
                actual: self.type_name(),
            }),
        }
    }

    /// Allows for easy access to the last edited by property value.
    /// This is a shortcut for `expect_value::<User>()` which is more explicit about the expected property type.
    /// This will also return an error if the property is not a last edited by, even if `User` is implemented for the property.
    pub fn expect_last_edited_by(&self) -> Result<User, WrongPropertyTypeError> {
        match self {
            PropertyValue::LastEditedBy { .. } => self.expect_value::<User>(),
            _ => Err(WrongPropertyTypeError {
                expected: vec!["LastEditedBy".to_string()],
                actual: self.type_name(),
            }),
        }
    }

    /// Allows for easy access to the button property value.
    /// This is a shortcut for `expect_value::<()>()` which is more explicit about the expected property type.
    /// This will also return an error if the property is not a button, even if `()` is implemented for the property.
    pub fn expect_button(&self) -> Result<(), WrongPropertyTypeError> {
        match self {
            PropertyValue::Button { .. } => Ok(()),
            _ => Err(WrongPropertyTypeError {
                expected: vec!["Button".to_string()],
                actual: self.type_name(),
            }),
        }
    }

    pub fn type_name(&self) -> String {
        match self {
            PropertyValue::Title { .. } => "Title".to_string(),
            PropertyValue::Text { .. } => "Text".to_string(),
            PropertyValue::Number { .. } => "Number".to_string(),
            PropertyValue::Select { .. } => "Select".to_string(),
            PropertyValue::Status { .. } => "Status".to_string(),
            PropertyValue::MultiSelect { .. } => "MultiSelect".to_string(),
            PropertyValue::Date { .. } => "Date".to_string(),
            PropertyValue::People { .. } => "People".to_string(),
            PropertyValue::Files { .. } => "Files".to_string(),
            PropertyValue::Checkbox { .. } => "Checkbox".to_string(),
            PropertyValue::Url { .. } => "Url".to_string(),
            PropertyValue::Email { .. } => "Email".to_string(),
            PropertyValue::PhoneNumber { .. } => "PhoneNumber".to_string(),
            PropertyValue::Formula { .. } => "Formula".to_string(),
            PropertyValue::Relation { .. } => "Relation".to_string(),
            PropertyValue::Rollup { .. } => "Rollup".to_string(),
            PropertyValue::CreatedTime { .. } => "CreatedTime".to_string(),
            PropertyValue::CreatedBy { .. } => "CreatedBy".to_string(),
            PropertyValue::LastEditedTime { .. } => "LastEditedTime".to_string(),
            PropertyValue::LastEditedBy { .. } => "LastEditedBy".to_string(),
            PropertyValue::Button { .. } => "Button".to_string(),
        }
    }
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
    fn from_property_value(value: PropertyValue) -> Result<Self, WrongPropertyTypeError>;
}

impl FromPropertyValue for Vec<RichText> {
    fn from_property_value(value: PropertyValue) -> Result<Self, WrongPropertyTypeError> {
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

impl FromPropertyValue for Option<Number> {
    fn from_property_value(value: PropertyValue) -> Result<Self, WrongPropertyTypeError> {
        match value {
            PropertyValue::Number { number, .. } => Ok(number),
            _ => Err(WrongPropertyTypeError {
                expected: vec!["Number".to_string()],
                actual: value.type_name(),
            }),
        }
    }
}

impl FromPropertyValue for Option<SelectedValue> {
    fn from_property_value(value: PropertyValue) -> Result<Self, WrongPropertyTypeError> {
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

impl FromPropertyValue for Option<Vec<SelectedValue>> {
    fn from_property_value(value: PropertyValue) -> Result<Self, WrongPropertyTypeError> {
        match value {
            PropertyValue::MultiSelect { multi_select, .. } => Ok(multi_select),
            _ => Err(WrongPropertyTypeError {
                expected: vec!["MultiSelect".to_string()],
                actual: value.type_name(),
            }),
        }
    }
}

impl FromPropertyValue for Option<DateValue> {
    fn from_property_value(value: PropertyValue) -> Result<Self, WrongPropertyTypeError> {
        match value {
            PropertyValue::Date { date, .. } => Ok(date),
            _ => Err(WrongPropertyTypeError {
                expected: vec!["Date".to_string()],
                actual: value.type_name(),
            }),
        }
    }
}

impl FromPropertyValue for FormulaResultValue {
    fn from_property_value(value: PropertyValue) -> Result<Self, WrongPropertyTypeError> {
        match value {
            PropertyValue::Formula { formula, .. } => Ok(formula),
            _ => Err(WrongPropertyTypeError {
                expected: vec!["Formula".to_string()],
                actual: value.type_name(),
            }),
        }
    }
}

impl FromPropertyValue for Option<Vec<RelationValue>> {
    fn from_property_value(value: PropertyValue) -> Result<Self, WrongPropertyTypeError> {
        match value {
            PropertyValue::Relation { relation, .. } => Ok(relation),
            _ => Err(WrongPropertyTypeError {
                expected: vec!["Relation".to_string()],
                actual: value.type_name(),
            }),
        }
    }
}

impl FromPropertyValue for Option<Vec<FileReference>> {
    fn from_property_value(value: PropertyValue) -> Result<Self, WrongPropertyTypeError> {
        match value {
            PropertyValue::Files { files, .. } => Ok(files),
            _ => Err(WrongPropertyTypeError {
                expected: vec!["Files".to_string()],
                actual: value.type_name(),
            }),
        }
    }
}

impl FromPropertyValue for bool {
    fn from_property_value(value: PropertyValue) -> Result<Self, WrongPropertyTypeError> {
        match value {
            PropertyValue::Checkbox { checkbox, .. } => Ok(checkbox),
            _ => Err(WrongPropertyTypeError {
                expected: vec!["Checkbox".to_string()],
                actual: value.type_name(),
            }),
        }
    }
}

impl FromPropertyValue for Option<String> {
    fn from_property_value(value: PropertyValue) -> Result<Self, WrongPropertyTypeError> {
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

impl FromPropertyValue for DateTime<Utc> {
    fn from_property_value(value: PropertyValue) -> Result<Self, WrongPropertyTypeError> {
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

impl FromPropertyValue for User {
    fn from_property_value(value: PropertyValue) -> Result<Self, WrongPropertyTypeError> {
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

impl FromPropertyValue for Option<Vec<User>> {
    fn from_property_value(value: PropertyValue) -> Result<Self, WrongPropertyTypeError> {
        match value {
            PropertyValue::People { people, .. } => Ok(Some(people)),
            _ => Err(WrongPropertyTypeError {
                expected: vec!["People".to_string()],
                actual: value.type_name(),
            }),
        }
    }
}

impl FromPropertyValue for Option<RollupValue> {
    fn from_property_value(value: PropertyValue) -> Result<Self, WrongPropertyTypeError> {
        match value {
            PropertyValue::Rollup { rollup, .. } => Ok(rollup),
            _ => Err(WrongPropertyTypeError {
                expected: vec!["Rollup".to_string()],
                actual: value.type_name(),
            }),
        }
    }
}
