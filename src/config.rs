use std::{
    collections::{hash_map::Iter, HashMap},
    fs::{self, File},
    path::{Path, PathBuf},
};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct TkProject {
    pub name: String,
    pub path: PathBuf,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct TkConfig {
    #[serde(flatten)]
    projects: HashMap<String, TkProject>,
}

impl TkConfig {
    const CONFIG_PATH: &str = "config.yml";

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
        let config_path = Path::new(Self::CONFIG_PATH);
        if !config_path.exists() {
            return Ok(Self {
                projects: Default::default(),
            });
        }

        let config_file = File::open(config_path)?;
        Ok(serde_yaml::from_reader(config_file)?)
    }

    pub fn store(&self) -> anyhow::Result<()> {
        let yaml_string = serde_yaml::to_string(self)?;
        fs::write(Self::CONFIG_PATH, yaml_string)?;
        Ok(())
    }
}
