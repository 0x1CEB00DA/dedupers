use walkdir::{WalkDir,WalkDirIterator,DirEntry,Error};

pub fn scan(dirs: &[&str]) -> Result<Vec<String>, Error> {
    scan_and_filter(dirs, |_| { true })
}

pub fn scan_and_filter<P>(dirs: &[&str], filter: P) -> Result<Vec<String>, Error>
    where P: Fn(&DirEntry) -> bool {
    let mut files = Vec::<String>::new();
    for dir in dirs {
        // This should be passed in as an argument to allow the function to be tested
        let walker = WalkDir::new(dir).into_iter().filter_entry(&filter);
        for entry in walker {
            let real_entry = entry?;
            let file_name = real_entry.path().display().to_string();
            files.push(file_name);
        }
    }
    Ok(files)
}
