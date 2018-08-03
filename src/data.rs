//use serde_json;
//use std::{fs::File, io, path::Path};

#[derive(Clone, Deserialize, Debug)]
pub struct DataStore {
    facts: Vec<DataEntry>, // TODO: Change this to a set
}

impl DataStore {
    pub fn new() -> Self {
        Self { facts: Vec::new() }
    }

    pub fn len(&self) -> usize {
        self.facts.len()
    }

    pub fn push(&mut self, fact: DataEntry) {
        self.facts.push(fact);
    }

    pub fn pop(&mut self) -> Option<DataEntry> {
        self.facts.pop()
    }

    pub fn is_empty(&self) -> bool {
        self.facts.is_empty()
    }

    pub fn get(&self, index: usize) -> &DataEntry {
        &self.facts[index]
    }
}

#[derive(Clone, Deserialize, Debug)]
pub enum DataEntry {
    Text(String),
    Image(String),
}
