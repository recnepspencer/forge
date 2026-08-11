use std::collections::BTreeMap;

use super::disposition_contract::expected_current_disposition;
use super::parse_inventory;
use crate::fresh_process_recovery_boundary_gate::documents::{
    read_repository_document, API_INVENTORY,
};

#[test]
fn dispositions_name_one_real_destination_owner() {
    let document = read_repository_document(API_INVENTORY).expect("read C.8 API inventory");
    let rows = parse_inventory(&document).expect("parse C.8 API inventory");
    let mut surfaces = BTreeMap::new();
    for row in rows {
        assert!(
            surfaces
                .insert((row.scope.clone(), row.surface.clone()), ())
                .is_none(),
            "duplicate C.8 API row for {} {}",
            row.scope,
            row.surface
        );
        match row.scope.as_str() {
            "current" | "current-certification" => assert_eq!(
                (
                    row.disposition.as_str(),
                    row.destination_owner.as_str(),
                    row.phase.as_str(),
                ),
                expected_current_disposition(&row.source_owner, &row.surface),
                "wrong C.8 disposition for {}",
                row.surface
            ),
            "destination" => {
                assert_eq!(row.disposition, "create");
                let expected_owner = if row.source_owner.starts_with("worth-") {
                    row.source_owner.clone()
                } else {
                    format!("worth-store-recovery-runtime/{}", row.source_owner)
                };
                assert_eq!(row.destination_owner, expected_owner);
            }
            other => panic!("unknown C.8 API scope `{other}`"),
        }
        assert_valid_owner(&row.destination_owner, row.disposition == "delete");
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
}

fn assert_valid_owner(owner: &str, deletion: bool) {
    if deletion {
        assert_eq!(owner, "none");
        return;
    }
    assert_ne!(owner, "none");
    let leaf = owner.rsplit('/').next().unwrap_or(owner);
    assert!(
        !matches!(
            leaf,
            "recovery" | "physics" | "support" | "evidence" | "utility"
        ),
        "generic C.8 API destination owner `{owner}`"
    );
}
