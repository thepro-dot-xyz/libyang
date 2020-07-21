#![allow(dead_code)]

pub struct Yang {
    paths: Vec<&'static str>,
}

impl Yang {
    // Constructor.
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
}
