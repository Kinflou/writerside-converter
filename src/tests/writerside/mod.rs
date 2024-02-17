// Standard Uses

use std::path::Path;

// Crate Uses
use crate::writerside::{config::{Ihp, Instance, Topics}, instance_profile::{InstanceProfile, TocElement}};



#[test]
fn parse_config() {
    let config_path = super::PROJECT_PATH.join("Writerside/writerside.cfg");
    let config = Ihp::from_path(&config_path).unwrap();
    
    assert_eq!(
        Ihp { 
            version: "2.0".to_string(),
            topics: Topics {
                dir: Path::new("topics").to_path_buf(),
                web_path: Path::new("topics").to_path_buf()
            },
            instance: Instance { src: Path::new("project.tree").to_path_buf() }
        },
        config
    );
}


#[test]
fn parse_instance_profile() {
    let profile_path = super::PROJECT_PATH.join("Writerside/project.tree");
    let profile = InstanceProfile::from_path(&profile_path).unwrap();
    
    assert_eq!(
        InstanceProfile {
            id: "project".to_string(), name: "A Project".to_string(),
            start_page: "index.md".to_string(),
            toc_elements: vec![
                TocElement { topic: "index.md".to_string(), elements: vec![] },
            ]
        },
        profile
    );
}
