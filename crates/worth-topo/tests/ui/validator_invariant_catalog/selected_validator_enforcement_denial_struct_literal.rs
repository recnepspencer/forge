use topology::facade::{
    WorthTopologySelectedValidatorEnforcementDenial,
    WorthTopologySelectedValidatorEnforcementDenialKind,
};

fn main() {
    let _ = WorthTopologySelectedValidatorEnforcementDenial {
        kind: WorthTopologySelectedValidatorEnforcementDenialKind::MissingSelectedFamily,
        family: "loop_wiring",
        selected_obligation_digest: None,
        witness_selected_obligation_digest: None,
        expected_obligation_kind: None,
        actual_obligation_kind: None,
        support_status: None,
        denial_digest: String::new(),
    };
}
