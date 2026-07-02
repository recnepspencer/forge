use forge_store_physical_isolation::{
    S6IoQosIsolationReadinessRequest, S6StoreIsolationHandoffEvidence,
};

fn main() {
    let evidence: S6StoreIsolationHandoffEvidence = unimplemented!();
    let _ = S6IoQosIsolationReadinessRequest::from_store_handoff_evidence(evidence);
}
