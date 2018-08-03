//)use serde_json;
//use std::{fs::File, io, path::Path};

#[derive(Clone, Deserialize, Debug)]
pub struct DataStore {
    facts: Vec<String>, // TODO: Change this to a set
}

impl DataStore {
    pub fn new() -> Self {
        Self { facts: Vec::new() }
    }

    pub fn len(&self) -> usize {
        self.facts.len()
    }

    pub fn add(&mut self, fact: String) {
        self.facts.push(fact);
    }

    pub fn is_empty(&self) -> bool {
        self.facts.is_empty()
    }

    pub fn get(&self, index: usize) -> &str {
        &self.facts[index]
    }
}
