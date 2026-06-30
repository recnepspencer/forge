use worth_spatial::facade::evidence_lookup_source_firewall::{
    EvidenceLookupSourceFirewallCounters, EvidenceLookupSourceFirewallOutcome,
    EvidenceLookupSourceFirewallReport,
};

fn main() {
    let _ = EvidenceLookupSourceFirewallReport {
        rows: Vec::new(),
        counters: EvidenceLookupSourceFirewallCounters::default(),
        outcome: EvidenceLookupSourceFirewallOutcome::Clean,
        firewall_digest: String::new(),
    };
}
