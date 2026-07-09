use worth_store_physical_isolation::{
    publish_s6_io_qos_isolation_readiness_from_s5_closeout, ExecutedS5IsolationCloseout,
    S6IoQosIsolationReadinessDenial,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum S6IoQosReadinessHandoffMaterializationDenial {
    S6(S6IoQosIsolationReadinessDenial),
}

/// Verify an executed S5 closeout can hand off to production isolation law without minting readiness.
pub(crate) fn verify_executed_closeout_handoff_admissible(
    closeout: ExecutedS5IsolationCloseout,
) -> Result<(), S6IoQosReadinessHandoffMaterializationDenial> {
    let readiness = publish_s6_io_qos_isolation_readiness_from_s5_closeout(closeout)
        .map_err(S6IoQosReadinessHandoffMaterializationDenial::S6)?;
    if readiness.unsupported_qos_claims().iter().any(|claim| {
        worth_store_physical_isolation::reject_qos_claim_as_s5_readiness(*claim).is_ok()
    }) {
        return Err(S6IoQosReadinessHandoffMaterializationDenial::S6(
            S6IoQosIsolationReadinessDenial::UnsupportedQoSClaimRequested(
                readiness.unsupported_qos_claims()[0],
            ),
        ));
    }
    Ok(())
}