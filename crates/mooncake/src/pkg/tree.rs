use colored::{ColoredString, Colorize};
use std::collections::HashSet;
use std::path::Path;
use walkdir::WalkDir;

use moonutil::common::{
    read_module_desc_file_in_dir, read_module_from_json, DEP_PATH, MOON_MOD_JSON,
};

pub fn bold(top: &HashSet<String>, item: &str) -> ColoredString {
    if top.contains(item) {
        item.bold()
    } else {
        item.into()
    }
}

pub fn tree(source_dir: &Path, target_dir: &Path) -> anyhow::Result<i32> {
    let _ = target_dir;
    let root_m = read_module_desc_file_in_dir(source_dir)?;
    let mut top = HashSet::new();
    for (name, dep) in root_m.deps {
        top.insert(format!("{}@{}", name, dep.version));
    }

    let mooncakes_dir = source_dir.join(DEP_PATH);
    if !mooncakes_dir.exists() {
        return Ok(0);
    }
    let walker = WalkDir::new(mooncakes_dir).into_iter();
    let mut t: Vec<(String, Vec<String>)> = Vec::new();
    for entry in walker {
        let entry = entry?;
        if entry.file_name() == MOON_MOD_JSON {
            log::debug!("{:?}", entry);
            let m = read_module_from_json(entry.path())?;
            log::debug!("{:#?}", m);
            let mut deps = vec![];
            for (name, dep) in m.deps.into_iter() {
                deps.push(format!("{}@{}", name, dep.version));
            }

            let cur = match m.version {
                Some(v) => format!("{}@{}", m.name, v),
                None => m.name,
            };
            t.push((cur, deps));
        }
    }
    for item in t.iter() {
        println!("{}:", bold(&top, &item.0));
        for dep in item.1.iter() {
            println!("  {}", bold(&top, dep));
        }
    }
    Ok(0)
}