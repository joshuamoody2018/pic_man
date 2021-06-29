use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io::{Read, Result};
use std::path::PathBuf;

use structopt::StructOpt;

#[derive(StructOpt)]
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
    let files = get_files(&opts.path, opts.recursive)?;

    if opts.del_dups {
        del_exact_dups(get_files_by_size(&files)?)?;
    }

    if opts.statistics {
        statistics::print_statistics(files);
    }

    Ok(())
}

pub fn get_files(path: &PathBuf, recursive: bool) -> Result<Vec<PathBuf>> {
    let mut files = vec![];

    for file_entry in fs::read_dir(&path)? {
        let file_entry = file_entry?.path();
        if recursive {
            files.extend(get_files(&path, recursive)?);
        } else {
            if file_entry.is_file() {
                files.push(file_entry)
            }
        }
    }

    Ok(files)
}

pub fn get_files_by_size(paths: &Vec<PathBuf>) -> Result<HashMap<u64, Vec<&PathBuf>>> {
    let mut size_names: HashMap<u64, Vec<&PathBuf>> = HashMap::new();

    for file_entry in paths {
        size_names.entry(fs::metadata(file_entry)?.len())
            .or_insert(vec![])
            .push(file_entry);
    }

    size_names.retain(|_, v| v.len() > 1);

    Ok(size_names)
}

pub fn del_exact_dups(files_by_size: HashMap<u64, Vec<&PathBuf>>) -> Result<()> {
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

pub mod statistics {
    use std::path::PathBuf;

    pub fn print_statistics(files: Vec<PathBuf>) {
        print_paths_found(&files);
    }

    pub fn print_paths_found(files: &Vec<PathBuf>) {
        print!("----STATISTICS----\n\r");
        print!("\tPATHS FOUND:\n\r");

        for file in files.iter() {
            print!("\t\t{:?}\n\r", file);
        }

        print!("\tEND PATHS FOUND\n\r");
    }
}