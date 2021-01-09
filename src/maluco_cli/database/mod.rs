use crate::maluco_cli::model::maluco_cli::{Object, ObjectUpdate};
use std::collections::HashMap;
pub struct Context(HashMap<String, Object>);

impl Context {
    pub fn new() -> Self {
        let map: HashMap<String, Object> = HashMap::new();
        Context{0: map}
    }

    pub fn create(&mut self, key: String, value: Object) -> bool {
        match self.0.insert(key, value) {
            None => true,
            Some(_) => false,
        }
    }

    pub fn get(&mut self, key: String) -> Option<Object>{
        match self.0.get(&key) {
            None => None,
            Some(s) => Some(s.to_owned())
        }
    }

    pub fn delete(&mut self, key: String) -> bool {
        match self.0.remove(&key) {
            None => false,
            Some(_) => true
        }
    }

    pub fn all(&mut self) -> Vec<Object> {
        self.0.iter()
            .map(|(k,v)| *v)
            .collect::<Vec<Object>>()
    }

    pub fn update(&mut self, key: String, value: ObjectUpdate) -> bool {
        if let Some(x) = self.0.get_mut(&key) {
            // EXAMPLE CODE, update to your needs
            if value.age.is_none() && value.name.is_none() && value.school.is_none() {
                return false;
            }

            if value.age.is_some() {
                x.age = value.age.unwrap();
            }

            if value.name.is_some() {
                x.name = value.name.unwrap();
            }

            if value.school.is_some() {
                x.school = value.school.unwrap();
            }

            true
        } else {
            false
        }
    }
}

