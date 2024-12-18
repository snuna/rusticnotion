use crate::ids::UserId;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct UserCommon {
    pub id: UserId,
    pub name: Option<String>,
    pub avatar_url: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct Person {
    pub email: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct Bot {
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
        person: Option<Person>,
    },
}

impl User {
    pub fn to_common(&self) -> &UserCommon {
        match self {
            User::Person { common, person: _ } => common,
            User::Bot { common, bot: _ } => common,
            User::User { common, person: _ } => common,
        }
    }
}
