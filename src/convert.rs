// Standard Uses
use std::path::Path;

// Crate Uses
use crate::writerside::{self, instance_profile::TocElement};

// External Uses
use eyre::{Result, Context, bail};



pub fn convert_docs(docs_path: &Path, project_path: &Path,) -> Result<()> {
    println!("Converting doc files:");
    convert_docs_recursive(docs_path, project_path).context("Couldn't convert recursively")?;

    Ok(())
}

fn convert_docs_recursive(docs_path: &Path, project_path: &Path) -> Result<()> {
    let config = writerside::fetch_writerside_config_from_project(project_path)?;
    let writerside_path = project_path.join("Writerside/");

    let (mut profile, profile_path) = writerside::fetch_profile_from_config_and_writerside_project_path(
        &config, &project_path
    ).context("Couldn't parse target profile")?;
    
    let topics_path = writerside_path.join(config.topics.dir);
    profile.toc_elements = vec![recurse_directory_and_move(&docs_path, &topics_path, docs_path, &topics_path, 0)?];

    // panic!("{:#?}", profile);

    std::fs::write(&profile_path, profile.to_string()?)
        .context("Couldn't write target's new instance profile")?;

    println!("\nUpdating instance profile at {}", profile_path.display());

    Ok(())
}

fn recurse_directory_and_move(
    docs_path: &Path, topics_path: &Path, input_path: &Path, output_path: &Path, depth: usize
) -> Result<TocElement> {
    let index_path = input_path.join("index.md");
    
    if !index_path.exists() {
        bail!("Docs directory doesn't have an 'index.md', please add one at '{}'", index_path.display());
    }

    let relative_index_path = index_path.strip_prefix(&docs_path)?;
    let index_filename = into_hierarchichal_filename(&relative_index_path);

    let entries = glob::glob(&*format!("{}/*.md", input_path.display()))?;
    
    let mut sub_elements = vec![];
    for entry in entries {
        let doc_path = entry?;
        let relative_doc_path = doc_path.strip_prefix(&docs_path)?;
        
        if doc_path.file_name().unwrap() == "index.md" { continue; }
        
        let filename = into_hierarchichal_filename(relative_doc_path);
        let output_path = topics_path.join(relative_doc_path.parent().unwrap())
            .join(&filename);

        sub_elements.push(
            generate_input_file_to_output(&doc_path, &filename, &output_path, depth)?
        );
    }

    for dir in std::fs::read_dir(input_path)? {
        let entry = dir?.path();
        
        if !entry.is_dir() { continue; }
        
        sub_elements.push(
            recurse_directory_and_move(
                docs_path, topics_path, &entry, output_path, depth + 1
            )?
        );
    }

    Ok(TocElement { topic: index_filename, elements: sub_elements })
}

fn generate_input_file_to_output(
    doc_path: &Path, relative_topic_doc_path: &str, topic_doc_path: &Path, depth: usize
) -> Result<TocElement> {
    std::fs::create_dir_all(topic_doc_path.parent().unwrap())?;

    println!("  {}- {} -> {}", "  ".repeat(depth), doc_path.display(), topic_doc_path.display());
    std::fs::copy(doc_path, topic_doc_path)?;

    println!("{}", relative_topic_doc_path);

    Ok(TocElement { topic: relative_topic_doc_path.to_string(), elements: vec![]})
}

fn into_hierarchichal_filename(path: &Path) -> String {
    let mut output_filename = String::new();

    let components = path.components().collect::<Vec<_>>();
    for (idx, part) in components.iter().enumerate() {
        output_filename += &*part.as_os_str().to_str().unwrap().replace(char::is_whitespace, "_");

        if idx == components.len() - 1 { break; }
        
        output_filename += "__";
    }

    output_filename
}
