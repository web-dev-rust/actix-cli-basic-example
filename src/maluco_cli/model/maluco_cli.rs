use serde::{Deserialize, Serialize}; 

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Object {pub age: usize, pub name: String, pub school: String, } 

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct ObjectUpdate {pub age: Option<usize>, pub name: Option<String>, pub school: Option<String>, } 

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct ObjectResponse
{pub object: Object, pub id: String,} 
