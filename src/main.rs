use std::process;
use no_duplicates::*;

fn main() {
    let files_by_size = get_files_by_size()
        .unwrap_or_else(|e| {
            eprintln!("Error: {}", e);
            process::exit(1);
        });

    del_exact_dups(files_by_size)
        .unwrap_or_else(|e| {
            eprintln!("Error: {}", e);
            process::exit(1);
        });
}




#[cfg(test)]
mod tests {
    #[test]
    fn test_files_by_size() {}
}