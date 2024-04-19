
use std::io::Error;

use serde::{Deserialize, Serialize};

use crate::api::models::SendTodoParamResult;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Todo {
    pub id: String,
    pub content: String,
    pub checked: bool,
}

impl Todo {
    pub fn get(self, str: &str) -> Result<SendTodoParamResult, Error> {
        match str {
            "id" => Ok(SendTodoParamResult::Str(self.id)),
            "context" => Ok(SendTodoParamResult::Str(self.content)),
            "checked" => Ok(SendTodoParamResult::Bool(self.checked)),
            _ => panic!("Invalid Input"),
        }
    }
}
