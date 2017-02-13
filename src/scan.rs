use walkdir::{WalkDir,WalkDirIterator,DirEntry,Error};

pub fn scan(dirs: &[&str]) -> (Vec<String>, Vec<Error>) {
    scan_and_filter(dirs, |_| { true })
}

pub fn scan_and_filter<P>(dirs: &[&str], filter: P) -> (Vec<String>, Vec<Error>)
    where P: Fn(&DirEntry) -> bool {
    let mut files = Vec::<String>::new();
    let mut errors = Vec::<Error>::new();
    for dir in dirs {
        // This should be passed in as an argument to allow the function to be tested
        let walker = WalkDir::new(dir).into_iter().filter_entry(&filter);
        for entry in walker {
            match entry {
                Ok(file) => {
                    let file_name = file.path().display().to_string();
                    files.push(file_name)
                },
                Err(error) => errors.push(error)
            }
        }
    }
    (files, errors)
}
