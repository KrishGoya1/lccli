use std::fs;
use std::path::PathBuf;
use anyhow::{Result, Context};
use directories::ProjectDirs;
use crate::fetch::leetcode::ProblemData;

pub struct Cache {
    cache_dir: PathBuf,
}

impl Cache {
    pub fn new() -> Result<Self> {
        let proj_dirs = ProjectDirs::from("com", "lccli", "lccli")
            .ok_or_else(|| anyhow::anyhow!("Could not determine config directory"))?;
        let cache_dir = proj_dirs.cache_dir();
        
        if !cache_dir.exists() {
            fs::create_dir_all(cache_dir).context("Failed to create cache directory")?;
        }

        Ok(Self {
            cache_dir: cache_dir.to_path_buf(),
        })
    }

    fn get_path(&self, problem_id: &str) -> PathBuf {
        self.cache_dir.join(format!("{}.json", problem_id))
    }

    pub fn get(&self, problem_id: &str) -> Result<Option<ProblemData>> {
        let path = self.get_path(problem_id);
        if !path.exists() {
            return Ok(None);
        }

        let content = fs::read_to_string(&path).context("Failed to read cache file")?;
        let data: ProblemData = serde_json::from_str(&content).context("Failed to deserialize cached data")?;
        Ok(Some(data))
    }

    pub fn save(&self, problem_id: &str, data: &ProblemData) -> Result<()> {
        let path = self.get_path(problem_id);
        let content = serde_json::to_string_pretty(data).context("Failed to serialize data")?;
        fs::write(&path, content).context("Failed to write to cache file")?;
        Ok(())
    }
}
