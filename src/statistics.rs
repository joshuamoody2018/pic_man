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