use forge_query::facade::runtime::ForgeQueryRuntimeFacadeFamily;

use crate::workload_platform::evidence_lookup_inventory::EvidenceLookupQuerySurface;

use super::super::current_evidence_lookup_query_consumer_kit;

#[test]
fn support_pinning_uses_live_query_rows() {
    let closeout = current_evidence_lookup_query_consumer_kit().expect("consumer kit closeout");

    assert_eq!(
        closeout.support_requirement_rows().len(),
        closeout.query_surface_row_count(EvidenceLookupQuerySurface::SupportPinning)
    );
    assert_eq!(
        closeout.support_rows().len(),
        closeout.support_requirement_rows().len()
    );

    for family in [ForgeQueryRuntimeFacadeFamily::Read] {
        assert!(closeout
            .support_requirement_rows()
            .iter()
            .any(|row| row.runtime_family() == family));
    }

    for requirement in closeout.support_requirement_rows() {
        let row = closeout
            .support_rows()
            .iter()
            .find(|row| {
                row.runtime_family() == requirement.runtime_family()
                    && row.source_touchpoint() == requirement.touchpoint()
            })
            .expect("derived requirement row pinned against a live support row");
        assert_eq!(row.required_query_surface(), requirement.query_surface());
        assert!(!row.query_support_surface().is_empty());
        assert!(!row.snapshot_row_digest().is_empty());
        assert_eq!(
            row.support_pin_report_digest(),
            closeout.support_pin_report_digest()
        );
    }

    assert!(closeout
        .binding_rows_for_query_surface(EvidenceLookupQuerySurface::SupportPinning)
        .iter()
        .all(|row| row.support_pin_report_digest() == Some(closeout.support_pin_report_digest())));
}
