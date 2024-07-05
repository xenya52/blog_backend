use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct BlogContent {
    pub headline: String,
    pub description: String,
    pub last_updated: String,
}
