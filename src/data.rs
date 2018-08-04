use std::path::{Path, PathBuf};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct DataStore {
    facts: Vec<DataEntry>,
    path: PathBuf,
}

impl DataStore {
    pub fn new<P>(path: P) -> Self
    where
        P: Into<PathBuf>,
    {
        Self {
            facts: Vec::new(),
            path: path.into(),
        }
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

    pub fn path(&self) -> &Path {
        &self.path
    }
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum DataEntry {
    Text(String),
    Image(String),
}
