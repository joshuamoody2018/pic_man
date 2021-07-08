use std::path::PathBuf;

pub enum Statistics<'a> {
    PathsFound(&'a Vec<PathBuf>),
    NumPathsFound((usize, u32)),
}

pub fn print_statistics(stats: Vec<Statistics>) {
    println!("\n\r------------STATISTICS------------\n\r");
    for stat in stats {
        match stat {
            self::Statistics::PathsFound(paths) => print_paths_found(paths),
            self::Statistics::NumPathsFound((num_files, num_dirs)) => print_num_paths_found(num_files, num_dirs),
        }
    }
    println!("\n\r----------END STATISTICS----------");
}

fn print_paths_found(paths: &Vec<PathBuf>) {
    println!("\tPATHS FOUND:");
    for file in paths.iter() {
        println!("\t\t{:?}", file);
    }
}

fn print_num_paths_found(num_files: usize, num_dirs: u32) {

    println!("\tNUMBER OF FILES FOUND: {}",num_files);
    println!("\tNUMBER OF DIRECTORIES FOUND: {}",num_dirs);
}

