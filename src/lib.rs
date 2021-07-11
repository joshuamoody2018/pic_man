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

    #[structopt(short = "f", long = "find")]
    pub find: Option<String>,
}

pub fn run(opts: Commands) -> Result<()> {
    let (files, num_dirs) = get_paths(&opts.path, opts.recursive)?;
    let files_by_size = get_files_by_size(&files)?;

    if opts.del_dups {
        del_exact_dups(files_by_size)?;
    }

    if let Some(query) = opts.find {
        match find_file(&opts.path, &query, opts.recursive)? {
            Some(p) => println!("File path for {} is: {:?}", &query, p),
            None => println!("File {} not found.", &query),
        };
    }

    if opts.statistics {
        statistics::print_statistics(vec![
            NumPathsFound((files.len(), num_dirs)),
            PathsFound(&files),
        ]);
    }

    Ok(())
}

pub fn get_paths(path: &PathBuf, recursive: bool) -> Result<(Vec<PathBuf>,u32)> {
    let mut files = vec![];
    let mut dirs = vec![];
    let mut num_dirs: u32 = 0;

    for file_entry in fs::read_dir(&path)? {

        let file_entry = file_entry?.path();

        if recursive {
            if file_entry.is_dir() {
                dirs.push(file_entry);
                num_dirs += 1;
            }else if file_entry.is_file() {
                files.push(file_entry);
            }
        }else {
            if file_entry.is_file() {
                files.push(file_entry)
            }
        }
    }

    if dirs.len() > 0 {
        for dir in dirs.iter() {
            let (more_files, dirs_in_iter) = &mut get_paths(&dir, recursive)?;
            files.append(more_files);
            num_dirs += *dirs_in_iter;
        }
    }

    Ok((files, num_dirs))
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

pub fn find_file<'a>(path: &'a PathBuf, query: &'a str, recursive: bool) -> Result<Option<PathBuf>> {
    let (files, _): (Vec<PathBuf>, _) = get_paths(path, recursive)?;

    Ok(
        files.into_iter()
            .find(move |f|
                *f == PathBuf::from(query)
            )
    )
}