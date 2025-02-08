use crate::ids::UserId;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct UserCommon {
    pub id: UserId,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar_url: Option<String>,
}

#[derive(Serialize, Deserialize, Default, Clone, Debug, Eq, PartialEq)]
pub struct Person {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
}

#[derive(Serialize, Deserialize, Default, Clone, Debug, Eq, PartialEq)]
pub struct Bot {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
#[serde(tag = "object", rename_all = "snake_case")]
pub enum User {
    Person {
        #[serde(flatten)]
        common: UserCommon,
        person: Person,
    },
    Bot {
        #[serde(flatten)]
        common: UserCommon,
        bot: Bot,
    },
    User {
        #[serde(flatten)]
        common: UserCommon,
        #[serde(skip_serializing_if = "Option::is_none")]
        person: Option<Person>,
    },
}
