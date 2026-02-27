use std::{fs, path::Path};

pub fn read_project_conf(key: &str) -> Option<String> {
    let candidates = [Path::new("project.conf"), Path::new("../project.conf")];

    for path in candidates {
        let Ok(contents) = fs::read_to_string(path) else {
            continue;
        };

        for line in contents.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }

            let Some((k, value)) = line.split_once('=') else {
                continue;
            };

            if k.trim() != key {
                continue;
            }

            let value = value.trim().trim_matches('"').trim_matches('\'');
            if !value.is_empty() {
                return Some(value.to_string());
            }
        }
    }

    None
}
