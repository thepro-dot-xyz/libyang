#![allow(dead_code)]

use std::fs::{self, File};
use std::io::{Error, ErrorKind};
use std::path::PathBuf;

pub struct Yang {
    paths: Vec<&'static str>,
}

pub struct Modules {}

impl Yang {
    pub fn new() -> Self {
        Yang { paths: vec![] }
    }

    // Add colon ':' separated path to YANG file load paths.
    pub fn add_path(&mut self, paths: &'static str) {
        for path in paths.split(":") {
            self.paths.push(path);
        }
    }

    pub fn paths(&self) -> &Vec<&'static str> {
        &self.paths
    }

    pub fn scan_dir(&self, dir: &str, name: &str, _recur: bool) -> Result<PathBuf, Error> {
        let mut candidate = vec![];
        let mut basename = String::from(name.trim_end_matches(".yang"));
        basename.push_str("@");

        let dirent = fs::read_dir(dir)?;
        for entry in dirent {
            if let Ok(entry) = entry {
                if let Ok(file_type) = entry.file_type() {
                    if file_type.is_file() {
                        if let Some(os_str) = entry.path().file_name() {
                            if let Some(file_str) = os_str.to_str() {
                                if file_str == name {
                                    return Ok(entry.path());
                                }
                                if let None = name.find('@') {
                                    // Try revision match such as 'ietf-dhcp@2016-08-25.yang'.
                                    if file_str.starts_with(&basename)
                                        && file_str.ends_with(".yang")
                                    {
                                        candidate.push(entry.path());
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        if candidate.len() == 0 {
            return Err(Error::new(ErrorKind::Other, "can't find file"));
        }

        // When the specified file is not found by exact match, directories are
        // scanned for "name@revision-date.yang" files, the latest (sorted by
        // YYYY-MM-DD revision-date) of candidates will be selected.
        candidate.sort();

        Ok(candidate.pop().unwrap())
    }

    pub fn find_file(&self, name: &str) -> Result<File, Error> {
        let mut file_path = PathBuf::from(name);

        // Find slash in name.
        if let None = name.find('/') {
            let mut file_name = String::from(name);
            if !file_name.ends_with(".yang") {
                file_name = String::from(name) + ".yang";
            }
            if let Ok(v) = self.scan_dir(".", &file_name, false) {
                file_path = v
            }
        }
        println!("result file_path {:?}", file_path);
        println!("result file_parent {:?}", file_path.parent());

        // If file has path, add the path to paths.
        match File::open(&file_path) {
            Ok(file) => {
                // let parent = file_path.parent();
                // match parent {
                //     Some(p) => {
                //         println!("path {:?}", p.to_str().unwrap());
                //         self.add_path(p.to_str().unwrap());
                //     }
                //     None => {}
                // }
                return Ok(file);
            }
            Err(_) => {
                if let Some(_) = name.find('/') {
                    return Err(Error::new(ErrorKind::Other, "can't find file"));
                }
            }
        }
        Err(Error::new(ErrorKind::Other, "can't find file"))
    }

    pub fn read(&self, _ms: &Modules, name: &str) -> Result<(), Error> {
        // Find file.
        let _file = self.find_file(name)?;

        // Read file contents.
        // let data = read_file(file)?;

        // // Parse file.
        // let ast = parse_data(data)?;

        Ok(())
    }
}

impl Modules {
    pub fn new() -> Self {
        Modules {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_path() {
        let mut yang = Yang::new();
        yang.add_path("/etc/openconfigd/yang:/opt/zebra/yang");
        yang.add_path("/var/yang");

        let paths = ["/etc/openconfigd/yang", "/opt/zebra/yang", "/var/yang"];

        assert_eq!(yang.paths, paths);
    }

    #[test]
    fn module_read() {
        let mut yang = Yang::new();
        yang.add_path("/etc/openconfigd/yang:/opt/zebra/yang");

        let ms = Modules::new();
        yang.read(&ms, "coreswitch").unwrap();
    }
}
