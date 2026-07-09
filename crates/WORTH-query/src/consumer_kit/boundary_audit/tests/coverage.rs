use super::super::{
    hard_prohibition_boundary_audit_coverage, WorthQueryBoundaryAuditCoverageMechanism,
};
use crate::hard_prohibition_registry;

#[test]
fn boundary_audit_coverage_matches_registry_without_drift() {
    super::super::registry_coverage::assert_boundary_audit_coverage_matches_registry();

    let registry = hard_prohibition_registry();
    let coverage = hard_prohibition_boundary_audit_coverage();
    for row in registry.rows() {
        let coverage_row = coverage
            .row(row.seam())
            .expect("registry seam must have audit coverage");
        assert_eq!(
            coverage_row.mechanism(),
            WorthQueryBoundaryAuditCoverageMechanism::AstMethodNameResolved
        );
        assert!(coverage_row.audit_required());
    }
}
