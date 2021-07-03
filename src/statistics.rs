use std::path::PathBuf;

pub struct Statistics<'a> {
    pub paths_found: &'a Vec<PathBuf>,
}

impl Statistics<'_> {
    pub fn new(paths_found: &Vec<PathBuf>) -> Statistics {
        Statistics { paths_found }
    }

    pub fn print_statistics(&self) {
        print!("----STATISTICS----\n\r");
        self.print_paths_found();
    }

    fn print_paths_found(&self) {
        print!("\tPATHS FOUND:\n\r");
        for file in self.paths_found.iter() {
            print!("\t\t{:?}\n\r", file);
        }
        print!("\tEND PATHS FOUND\n\r");
    }
}

