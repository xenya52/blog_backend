use std::io::Error;

use serde::{Deserialize, Serialize};

use super::todo::Todo;
use crate::api::models::SendUserParamResult;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct User {
    pub id: String,
    pub name: String,
    pub birthday: String,
    pub email: String,
    pub todos: Vec<Todo>,
}

impl User {
    pub fn get(self, str: &str) -> Result<SendUserParamResult, Error> {
        match str {
            "id" => Ok(SendUserParamResult::Str(self.id)),
            "name" => Ok(SendUserParamResult::Str(self.name)),
            "birthday" => Ok(SendUserParamResult::Str(self.birthday)),
            "email" => Ok(SendUserParamResult::Str(self.email)),
            "todos" => Ok(SendUserParamResult::Vec(self.todos)),
            _ => panic!("Coult not find parameter {}", str.to_string()),
        }
    }
}
