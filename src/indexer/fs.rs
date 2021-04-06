use std::fs;
use walkdir::WalkDir;

pub fn walk(path: String) -> Result<Vec<String>, std::io::Error> {
    let mut res = vec![];
    for entry in WalkDir::new(&path)
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let f_name = entry.path().to_string_lossy();

        if f_name.ends_with(".html") {
            res.push(f_name.to_string());
        }
    }
    Ok(res)
}

pub fn file(path: &String) -> Result<String, std::io::Error> {
    Ok(fs::read_to_string(path)?)
}
