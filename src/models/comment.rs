use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::ids::CommentId;

use super::{text::RichText, users::User, Parent};

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct CommentCommon {
    /// Unique identifier for the database
    pub id: CommentId,

    /// Body of comment
    pub rich_text: Vec<RichText>,

    pub created_time: DateTime<Utc>,

    pub last_edited_time: DateTime<Utc>,

    pub parent: Parent,

    pub created_by: User,

    pub discussion_id: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
#[serde(tag = "object", rename_all = "snake_case")]
pub enum Comment {
    Comment {
        #[serde(flatten)]
        common: CommentCommon,
    },
}
