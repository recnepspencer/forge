use worth_spatial::facade::evidence_lookup_source_firewall::{
    EvidenceLookupForbiddenAuthorityKind, EvidenceLookupSourceFirewallRow,
    EvidenceLookupSourceFirewallRowPosture,
};

fn main() {
    let _ = EvidenceLookupSourceFirewallRow {
        source_path: String::new(),
        matched_surface: String::new(),
        forbidden_authority_kind: EvidenceLookupForbiddenAuthorityKind::BroadReceiptScan,
        posture: EvidenceLookupSourceFirewallRowPosture::ForbiddenProductionAuthority,
        exception_kind: None,
        reason: String::new(),
    };
}
