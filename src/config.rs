use std::{
    collections::{hash_map::Iter, HashMap},
    path::PathBuf,
};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct TkProject {
    pub name: String,
    pub path: PathBuf,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default)]
pub struct TkConfig {
    #[serde(flatten)]
    projects: HashMap<String, TkProject>,
}

impl TkConfig {
    pub fn is_empty(&self) -> bool {
        self.projects.is_empty()
    }

    pub fn iter(&self) -> Iter<String, TkProject> {
        self.projects.iter()
    }

    pub fn contains(&self, id: &str) -> bool {
        self.projects.contains_key(id)
    }

    pub fn add_or_replace(&mut self, id: &str, project: TkProject) -> anyhow::Result<()> {
        self.projects.insert(id.to_owned(), project);
        self.store()
    }

    pub fn load() -> anyhow::Result<Self> {
        let config: TkConfig = confy::load("tksync", "default-config")?;
        Ok(config)
    }

    pub fn store(&self) -> anyhow::Result<()> {
        confy::store("tksync", "default-config", &self)?;
        Ok(())
    }
}
