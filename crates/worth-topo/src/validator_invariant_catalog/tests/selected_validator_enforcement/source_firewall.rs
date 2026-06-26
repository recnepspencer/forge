use crate::validator_invariant_catalog::WorthTopologySelectedValidatorEnforcementSourceFirewallReport;

#[test]
fn selected_validator_enforcement_lane_rejects_old_whole_view_authority() {
    let report =
        WorthTopologySelectedValidatorEnforcementSourceFirewallReport::for_selected_validator_enforcement_lane()
            .expect("selected validator enforcement source firewall should scan");

    assert!(report.scanned_file_count() > 5);
    assert!(report.forbidden_token_count() > 0);
    assert!(
        report.violations().is_empty(),
        "selected validator enforcement lane contains old authority residue: {:?}",
        report.violations()
    );
}
