use std::fmt;
use serde::{Deserialize, Serialize}; //restapi test
#[derive(Debug, Deserialize, Serialize, Clone)] //restapi test
pub struct Todo {
    pub id: i32,
    pub name: String,
    pub description: String,
}
impl fmt::Display for Todo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ID: {}, Name: {}, Description: {}", self.id, self.name, self.description)
    }
}