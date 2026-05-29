use std::fs;
use std::path::PathBuf;

fn domain_view_sources() -> Vec<(String, String)> {
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("src")
        .join("projection")
        .join("read_views")
        .join("domain")
        .join("views");
    let mut sources = fs::read_dir(dir)
        .expect("domain views directory should be readable")
        .filter_map(Result::ok)
        .map(|entry| entry.path())
        .filter(|path| path.extension().is_some_and(|ext| ext == "rs"))
        .filter(|path| {
            path.file_name()
                .and_then(|name| name.to_str())
                .is_some_and(|name| name != "boundary_tests.rs" && name != "mod.rs")
        })
        .map(|path| {
            let name = path
                .file_name()
                .and_then(|name| name.to_str())
                .expect("view file name should be utf8")
                .to_string();
            let source =
                fs::read_to_string(&path).expect("domain view file should be readable as text");
            (name, source)
        })
        .collect::<Vec<_>>();
    sources.sort_by(|left, right| left.0.cmp(&right.0));
    sources
}

#[test]
fn domain_view_files_do_not_import_or_name_raw_query_rows() {
    for (name, source) in domain_view_sources() {
        assert!(
            !source.contains("ForgeQueryEntity"),
            "{name} should consume typed retained topology rows instead of raw query rows"
        );
        assert!(
            !source.contains("RetainedTopologyRows"),
            "{name} should not depend on the retained-row cursor directly"
        );
        assert!(
            !source.contains("serde_json::Value"),
            "{name} should not decode retained payload values directly"
        );
    }
}

#[test]
fn domain_view_files_do_not_walk_retained_payload_maps_directly() {
    for (name, source) in domain_view_sources() {
        assert!(
            !source.contains(".payload"),
            "{name} should not inspect retained payload maps directly"
        );
        assert!(
            !source.contains("get(\"relations\")"),
            "{name} should not decode retained relation targets directly"
        );
        assert!(
            !source.contains("get(\"relation_identities\")"),
            "{name} should not decode retained relation record ids directly"
        );
    }
}




