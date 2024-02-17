// Standard Uses
use std::path::Path;

// External Uses
use eyre::{eyre, Result};
use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize)]
#[serde(rename="instance-profile")]
#[derive(Debug, PartialEq)]
pub struct InstanceProfile {
    #[serde(rename = "@id")]
    pub id: String,

    #[serde(rename = "@name")]
    pub name: String,

    #[serde(rename = "@start-page")]
    pub start_page: String,

    #[serde(default, rename ="toc-element")]
    pub toc_elements: Vec<TocElement>
}

impl InstanceProfile {
    pub fn from_path(path: &Path) -> Result<Self> {
        let file = std::fs::read_to_string(path)?;
        let config = quick_xml::de::from_str(&*file)?;
        
        Ok(config)
    }

    pub fn to_string(&self) -> Result<String> {
        quick_xml::se::to_string(self).map_err(|e| eyre!("{}", e))
    }
}

#[derive(Serialize, Deserialize)]
#[derive(Debug, PartialEq)]
pub struct TocElement {
    #[serde(rename = "@topic")]
    pub topic: String,

    #[serde(default, rename ="toc-element")]
    pub elements: Vec<TocElement>
}

/*

pub fn find_project_tree_path(writerside_path: &Path) -> Result<PathBuf> {
    let config_path = writerside_path.join(writerside_path);
    
    Ok()
}
*/

