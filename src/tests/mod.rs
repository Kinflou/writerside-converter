// Relative Modules
mod writerside;

// Standard Uses
use std::path::{Path, PathBuf};

// External Uses
use once_cell::sync::Lazy;


const PROJECT_PATH: Lazy<PathBuf> = Lazy::new(|| Path::new("src/tests/_data_/project/").to_path_buf());
