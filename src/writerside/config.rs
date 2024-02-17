// Standard Uses
use std::path::{Path, PathBuf};

// External Uses
use eyre::Result;
use serde::{Serialize, Deserialize};



#[derive(Serialize, Deserialize)]
#[derive(Debug, PartialEq)]
pub struct Config {
    pub ihp: Ihp
}

impl Config {
    pub fn from_path(path: &Path) -> Result<Self> {
        let file = std::fs::read_to_string(path)?;
        let config = quick_xml::de::from_str(&*file)?;
        
        Ok(config)
    }
}

#[derive(Serialize, Deserialize)]
#[derive(Debug, PartialEq)]
pub struct Ihp {
    #[serde(rename = "@version")]
    pub version: String,
    
    pub topics: Topics,
    pub instance: Instance
}

impl Ihp {
    pub fn from_path(path: &Path) -> Result<Self> {
        let file = std::fs::read_to_string(path)?;
        let config = quick_xml::de::from_str(&*file)?;
        
        Ok(config)
    }
}


#[derive(Serialize, Deserialize)]
#[derive(Debug, PartialEq)]
pub struct Topics {
    #[serde(rename = "@dir")]
    pub dir: PathBuf,

    #[serde(rename = "@web-path")]
    pub web_path: PathBuf
}

#[derive(Serialize, Deserialize)]
#[derive(Debug, PartialEq)]
pub struct Instance {
    #[serde(rename = "@src")]
    pub src: PathBuf
}
