use std::fmt;
use serde::{Deserialize, Serialize};

use crate::api::models::SendTodoParamResult;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Todo {
    pub id: String,
    pub name: String,
    pub description: String,
    pub checked: bool
}
impl fmt::Display for Todo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ID: {}, Name: {}, Description: {}", self.id, self.name, self.description)
    }
}
impl Todo {
    pub fn get(self, str: &str) -> Result<SendTodoParamResult, rocket::Error> {
        match str {
            "id" => Ok(SendTodoParamResult::Str(self.id)),
            "name" => Ok(SendTodoParamResult::Str(self.name)),
            "description" => Ok(SendTodoParamResult::Str(self.description)),
            "checked" => Ok(SendTodoParamResult::Bool(self.checked)),
            _ => panic!("Invalid Input"),
        }
    }
}