use std::fs;
use std::path::Path;

const FAMILY_HEADINGS: [&str; 8] = [
    "##### Durability and recovery",
    "##### Recovery-source precedence",
    "##### Compaction visibility",
    "##### Lease and reclaim",
    "##### Quarantine and readmission",
    "##### Import publication",
    "##### Replication admission",
    "##### Shared-frontier composition",
];

#[test]
fn s9_owns_explicit_state_transition_and_denial_evidence_for_every_protocol_family() {
    let spec_path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../../../_docs/worth-store/storage-foundation-s9.md");
    let spec = fs::read_to_string(&spec_path).expect("S9 specification is readable");
    let section_start = spec
        .find("#### Checked protocol state, transition, and denial tables")
        .expect("S9 owns the checked protocol evidence section");
    let phase_end = spec[section_start..]
        .find("\n### Phase 13:")
        .map_or(spec.len(), |offset| section_start + offset);
    let evidence = &spec[section_start..phase_end];

    for (index, heading) in FAMILY_HEADINGS.iter().enumerate() {
        let family_start = evidence
            .find(heading)
            .unwrap_or_else(|| panic!("missing S9 protocol evidence heading {heading}"));
        let family_end = FAMILY_HEADINGS[index + 1..]
            .iter()
            .filter_map(|next| evidence[family_start + heading.len()..].find(next))
            .min()
            .map_or(evidence.len(), |offset| {
                family_start + heading.len() + offset
            });
        let family = &evidence[family_start..family_end];

        for required_row in [
            "| States/frontiers |",
            "| Legal transitions/actions |",
            "| Typed denials/blocked edges |",
        ] {
            assert!(
                family.contains(required_row),
                "{heading} lacks required evidence row {required_row}"
            );
        }
    }
}
