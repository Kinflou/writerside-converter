// Relative Modules
pub mod config;
pub mod instance_profile;

// Standard Uses
use std::path::{Path, PathBuf};

// External Uses
use eyre::{Context, Result};


// Long naming conventions are on purpose to be descriptive because this crate is small.


pub fn fetch_writerside_config_from_project(project_path: &Path) -> Result<config::Ihp> {
    let writerside_path = project_path.join("Writerside");
    let config_path = writerside_path.join("writerside.cfg");

    let config_file = std::fs::read_to_string(&config_path)
        .with_context(|| format!("Couldn't find writerside configuration file at: '{}'", config_path.display()))?;
    let config: config::Ihp = quick_xml::de::from_str(&config_file)?;

    Ok(config)
}

pub fn fetch_profile_from_config_and_writerside_project_path(config: &config::Ihp, project_path: &Path)
    -> Result<(instance_profile::InstanceProfile, PathBuf)> 
{
    let writerside_path = project_path.join("Writerside");
    
    let instance_profile_path = writerside_path.join(&config.instance.src);
    let instance_profile_file = std::fs::read_to_string(&instance_profile_path)?;

    let instance_profile = quick_xml::de::from_str(&instance_profile_file)?;

    Ok((instance_profile, instance_profile_path))
}


