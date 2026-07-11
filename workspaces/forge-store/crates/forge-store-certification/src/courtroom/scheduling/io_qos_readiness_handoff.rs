use forge_store_physical_isolation::{
    publish_scheduler_isolation_capability_from_executed_evidence, ExecutedIsolationEvidence,
    IsolationReadinessDenial,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum S6IoQosReadinessHandoffMaterializationDenial {
    S6(IsolationReadinessDenial),
}

/// Verify an executed S5 closeout can hand off to production isolation law without minting readiness.
pub(crate) fn verify_executed_closeout_handoff_admissible(
    closeout: ExecutedIsolationEvidence,
) -> Result<(), S6IoQosReadinessHandoffMaterializationDenial> {
    let readiness = publish_scheduler_isolation_capability_from_executed_evidence(closeout)
        .map_err(S6IoQosReadinessHandoffMaterializationDenial::S6)?;
    if readiness.unsupported_qos_claims().iter().any(|claim| {
        forge_store_physical_isolation::reject_unsupported_qos_claim_as_isolation_readiness(*claim)
            .is_ok()
    }) {
        return Err(S6IoQosReadinessHandoffMaterializationDenial::S6(
            IsolationReadinessDenial::UnsupportedQoSClaimRequested(
                readiness.unsupported_qos_claims()[0],
            ),
        ));
    }
    Ok(())
}
