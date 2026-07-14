use worth_store_physical_isolation::{
    SchedulerIsolationCapabilityRequest, S6StoreIsolationHandoffEvidence,
};

fn main() {
    let evidence: S6StoreIsolationHandoffEvidence = unimplemented!();
    let _ = SchedulerIsolationCapabilityRequest::from_store_handoff_evidence(evidence);
}
