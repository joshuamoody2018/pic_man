use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io::{Read, Result};
use std::path::PathBuf;
use structopt::StructOpt;
use crate::statistics::Statistics::{NumPathsFound, PathsFound};

mod statistics;

#[derive(Debug, StructOpt)]
#[structopt(name = "Options")]
pub struct Commands {
    #[structopt(short = "p", long = "path", default_value = ".", parse(from_os_str))]
    pub path: std::path::PathBuf,

    #[structopt(short = "d", long = "delete_dups")]
    pub del_dups: bool,

    #[structopt(short = "r", long = "recursive")]
    pub recursive: bool,

    #[structopt(short = "s", long = "statistics")]
    pub statistics: bool,
}

pub fn run(opts: Commands) -> Result<()> {
    let (files, dirs) = get_paths(&opts.path, opts.recursive)?;
    let files_by_size = get_files_by_size(&files)?;

    if opts.del_dups {
        del_exact_dups(files_by_size)?;
    }

    if opts.statistics {
        statistics::print_statistics(vec![
            NumPathsFound((files.len(), dirs.len())),
            PathsFound(&files),
        ]);
    }

    Ok(())
}

pub fn get_paths(path: &PathBuf, recursive: bool) -> Result<(Vec<PathBuf>,Vec<PathBuf>)> {
    let mut files = vec![];
    let mut dirs = vec![];

    for file_entry in fs::read_dir(&path)? {

        let file_entry = file_entry?.path();

        if recursive {
            if file_entry.is_dir() {
                dirs.push(file_entry);
            }else if file_entry.is_file() {
                files.push(file_entry);
            }
        }else {
            if file_entry.is_file() {
                files.push(file_entry)
            }
        }
    }
    let mut dirs_copy = dirs.clone();

    if dirs.len() > 0 {
        for dir in dirs.iter() {
            let (more_files, more_dirs) = &mut get_paths(&dir, recursive)?;
            files.append(more_files);
            dirs_copy.append(more_dirs);
        }
    }

    Ok((files, dirs_copy))
}

pub fn get_files_by_size(files: &Vec<PathBuf>) -> Result<HashMap<u64, Vec<&PathBuf>>> {
    let mut size_names: HashMap<u64, Vec<&PathBuf>> = HashMap::new();

    for file in files {
        size_names.entry(fs::metadata(file)?.len())
            .or_insert(vec![])
            .push(file);
    }

    size_names.retain(|_, v| v.len() > 1);

    Ok(size_names)
}

pub fn del_exact_dups(files: HashMap<u64, Vec<&PathBuf>>) -> Result<()> {
    for (_, entries) in files {
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