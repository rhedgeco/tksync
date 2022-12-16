use std::{
    collections::{hash_map::Iter, HashMap},
    path::{Path, PathBuf},
};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct TkProject {
    pub name: String,
    pub path: PathBuf,
}

impl TkProject {
    pub fn get_full_path_using_id(&self, id: &str) -> PathBuf {
        let sub_folder_name = format!("{}-{}-tksync", self.name, id);
        let sub_folder = Path::new(&sub_folder_name);
        self.path.join(sub_folder)
    }
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

    pub fn get(&self, id: &str) -> Option<&TkProject> {
        self.projects.get(id)
    }

    pub fn contains(&self, id: &str) -> bool {
        self.projects.contains_key(id)
    }

    pub fn remove(&mut self, id: &str) {
        self.projects.remove(id);
    }

    pub fn add_or_replace(&mut self, id: &str, project: TkProject) {
        self.projects.insert(id.to_owned(), project);
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
