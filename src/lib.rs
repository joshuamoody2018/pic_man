use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io::{Read, Result};
use std::path::PathBuf;

pub fn get_files_by_size() -> Result<HashMap<u64, Vec<PathBuf>>> {
    let mut size_names: HashMap<u64, Vec<PathBuf>> = HashMap::new();

    for file_entry in fs::read_dir(".")? {
        let file_entry = file_entry?.path();

        if file_entry.is_file() {
            size_names.entry(fs::metadata(&file_entry)?.len())
                .or_insert(vec![])
                .push(file_entry);
        }
    }

    size_names.retain(|_, v| v.len() > 1);

    Ok(size_names)
}

pub fn del_exact_dups(files_by_size: HashMap<u64, Vec<PathBuf>>) -> Result<()> {
    for (_, entries) in files_by_size {
        let mut entries = entries.iter();

        match entries.next() {
            Some(entry) => {
                let mut buf_one = vec![];
                File::open(entry)?.read_to_end(&mut buf_one)?;

                while let Some(entry) = entries.next() {
                    let mut buf_two = vec![];
                    File::open(entry)?.read_to_end(&mut buf_two)?;

                    if buf_one == buf_two {
                        fs::remove_file(entry)?;
                    }
                }
            }
            None => continue,
        }
    }

    Ok(())
}