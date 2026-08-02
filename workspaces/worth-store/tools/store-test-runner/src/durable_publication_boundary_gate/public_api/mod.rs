mod facade_reachability;
mod locked_surfaces;

use std::collections::BTreeSet;

use super::read_repository_document;
use facade_reachability::assert_facade_reachability;
use locked_surfaces::{
    INHERITED_SURFACES, PHASE_EIGHT_SURFACES, PHASE_FIVE_SURFACES, PHASE_FOUR_SURFACES,
    PHASE_NINE_SURFACES, PHASE_SEVEN_SURFACES, PHASE_SIX_SURFACES, PHASE_THREE_SURFACES,
    PHASE_TWO_SURFACES,
};

const API_DOCUMENT: &str = "_docs/worth-store/physical-reconstruction-c7-public-api.csv";
const HEADER: &str =
    "surface,path,source_anchor,current_semantics,disposition,destination_owner,phase";

#[test]
fn every_locked_public_surface_resolves_and_has_one_final_disposition() {
    let document = read_repository_document(API_DOCUMENT).expect("read C.7 public API inventory");
    let rows = parse_api(&document).expect("parse C.7 public API inventory");
    let surfaces = validate_inventory_rows(rows);
    let expected = locked_public_surfaces();
    assert_eq!(
        expected.len(),
        locked_public_surface_count(),
        "C.7 public API boundary sets contain a duplicate"
    );
    let actual = surfaces.iter().map(String::as_str).collect::<BTreeSet<_>>();
    assert_eq!(
        actual, expected,
        "C.7 public API inventory must equal the locked boundary set"
    );
    assert_facade_reachability();
}

fn validate_inventory_rows(rows: Vec<ApiRow>) -> BTreeSet<String> {
    let mut surfaces = BTreeSet::new();
    for row in rows {
        assert!(
            surfaces.insert(row.surface.clone()),
            "duplicate C.7 API disposition for {}",
            row.surface
        );
        let source = read_repository_document(&format!("workspaces/worth-store/{}", row.path))
            .unwrap_or_else(|denial| panic!("{denial}"));
        assert!(
            source.contains(&row.anchor),
            "C.7 API `{}` lost source anchor `{}`",
            row.surface,
            row.anchor
        );
        assert!(!row.current_semantics.is_empty());
        assert!(!row.destination_owner.is_empty());
        assert!(matches!(
            row.disposition.as_str(),
            "preserve" | "narrow" | "move" | "replace" | "delete"
        ));
        assert!(matches!(
            row.phase.as_str(),
            "phase-2"
                | "phase-3"
                | "phase-4"
                | "phase-5"
                | "phase-6"
                | "phase-7"
                | "phase-8"
                | "phase-9"
        ));
    }
    surfaces
}

fn locked_public_surfaces() -> BTreeSet<&'static str> {
    INHERITED_SURFACES
        .into_iter()
        .chain(PHASE_TWO_SURFACES)
        .chain(PHASE_THREE_SURFACES)
        .chain(PHASE_FOUR_SURFACES)
        .chain(PHASE_FIVE_SURFACES)
        .chain(PHASE_SIX_SURFACES)
        .chain(PHASE_SEVEN_SURFACES)
        .chain(PHASE_EIGHT_SURFACES)
        .chain(PHASE_NINE_SURFACES)
        .collect()
}

fn locked_public_surface_count() -> usize {
    INHERITED_SURFACES.len()
        + PHASE_TWO_SURFACES.len()
        + PHASE_THREE_SURFACES.len()
        + PHASE_FOUR_SURFACES.len()
        + PHASE_FIVE_SURFACES.len()
        + PHASE_SIX_SURFACES.len()
        + PHASE_SEVEN_SURFACES.len()
        + PHASE_EIGHT_SURFACES.len()
        + PHASE_NINE_SURFACES.len()
}

fn parse_api(document: &str) -> Result<Vec<ApiRow>, String> {
    let mut lines = document.lines();
    if lines.next() != Some(HEADER) {
        return Err("C.7 public API inventory has an invalid schema".to_owned());
    }
    lines
        .filter(|line| !line.trim().is_empty())
        .enumerate()
        .map(|(index, line)| {
            let columns = line.split(',').map(str::trim).collect::<Vec<_>>();
            if columns.len() != 7 || columns.iter().any(|column| column.is_empty()) {
                return Err(format!("invalid C.7 public API row {}", index + 2));
            }
            Ok(ApiRow {
                surface: columns[0].to_owned(),
                path: columns[1].to_owned(),
                anchor: columns[2].to_owned(),
                current_semantics: columns[3].to_owned(),
                disposition: columns[4].to_owned(),
                destination_owner: columns[5].to_owned(),
                phase: columns[6].to_owned(),
            })
        })
        .collect()
}

struct ApiRow {
    surface: String,
    path: String,
    anchor: String,
    current_semantics: String,
    disposition: String,
    destination_owner: String,
    phase: String,
}
