use super::fixtures::rewire_operator_enforcement_closeout;
use crate::validator_invariant_catalog::{
    WorthTopologyLegalityCatalogError, WorthTopologyOperatorCertificationCutoverCloseout,
    WorthTopologyOperatorCertificationCutoverDenialKind,
    WorthTopologyOperatorCertificationCutoverSourceFirewallReport,
    WorthTopologyOperatorCertificationOldExpectationResidueReport,
    WorthTopologyOperatorCertificationOldExpectationResidueRow,
};

#[test]
fn uncapped_old_expectation_authority_is_rejected() {
    let enforcement = rewire_operator_enforcement_closeout();
    let residue = WorthTopologyOperatorCertificationOldExpectationResidueReport::from_rows([
        WorthTopologyOperatorCertificationOldExpectationResidueRow::uncapped_authority(
            "certification/topology_operator_closeout/expectations.rs",
            "validator-expectation-array",
            "worth-topo",
            "uncapped old rows cannot remain proof",
            "Phase 8 deletes old authority",
        ),
    ]);

    let error =
        WorthTopologyOperatorCertificationCutoverCloseout::from_selected_graph_obligation_enforcement_with_reports(
            &enforcement,
            WorthTopologyOperatorCertificationCutoverSourceFirewallReport::clean_for_cutover(),
            residue,
        )
        .unwrap_err();

    let WorthTopologyLegalityCatalogError::OperatorCertificationCutover(denial) = error else {
        panic!("expected operator certification cutover denial");
    };
    assert_eq!(
        denial.kind(),
        WorthTopologyOperatorCertificationCutoverDenialKind::UncappedOldExpectationAuthority
    );
}
