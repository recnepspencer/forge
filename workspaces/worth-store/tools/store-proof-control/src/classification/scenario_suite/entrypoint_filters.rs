use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

pub(super) fn scenario_filters_for_suite(
    suite_source: &str,
) -> Result<BTreeMap<String, String>, String> {
    let source_path = Path::new(suite_source);
    let source = fs::read_to_string(source_path)
        .map_err(|error| format!("could not read suite entrypoint {suite_source}: {error}"))?;
    let parent = source_path.parent().unwrap_or_else(|| Path::new("."));
    let mut filters = BTreeMap::new();
    let mut declared_path = None;
    for line in source.lines() {
        let trimmed = line.trim();
        if let Some(path) = path_attribute(trimmed) {
            declared_path = Some(parent.join(path));
            continue;
        }
        let Some(module) = module_declaration(trimmed) else {
            continue;
        };
        let Some(path) = declared_path.take() else {
            continue;
        };
        let Some(responsibility) = scenario_responsibility(source_path, &path, &module) else {
            continue;
        };
        if filters.insert(responsibility.clone(), module).is_some() {
            return Err(format!(
                "suite entrypoint declares scenario responsibility twice: {responsibility}"
            ));
        }
    }
    Ok(filters)
}

fn path_attribute(line: &str) -> Option<&str> {
    line.strip_prefix("#[path = \"")?.strip_suffix("\"]")
}

fn module_declaration(line: &str) -> Option<String> {
    let line = line.strip_prefix("pub ").unwrap_or(line);
    let name = line.strip_prefix("mod ")?.strip_suffix(';')?.trim();
    (!name.is_empty()).then(|| name.to_owned())
}

fn scenario_responsibility(suite: &Path, path: &Path, module: &str) -> Option<String> {
    let normalized = canonical(path);
    if let Some((_, relative)) = normalized.split_once("/tests/scenarios/") {
        let mut components = relative.split('/');
        let domain = components.next()?;
        let scenario = components.next()?;
        return Some(format!("{domain}/{scenario}"));
    }
    (suite.file_stem().and_then(|stem| stem.to_str()) == Some("io_scheduling")
        && module == "producer_declarations")
        .then(|| "scheduling/producer_declarations".to_owned())
}

fn canonical(path: &Path) -> String {
    path.canonicalize()
        .unwrap_or_else(|_| PathBuf::from(path))
        .to_string_lossy()
        .replace("\\\\?\\", "")
        .replace('\\', "/")
}
