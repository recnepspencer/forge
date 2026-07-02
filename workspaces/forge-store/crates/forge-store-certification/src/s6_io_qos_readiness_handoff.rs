use forge_store_physical_isolation::{
    publish_s6_io_qos_isolation_readiness_from_s5_closeout, ExecutedS5IsolationCloseout,
    S6IoQosIsolationReadiness, S6IoQosIsolationReadinessDenial,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum S6IoQosReadinessHandoffMaterializationDenial {
    S6(S6IoQosIsolationReadinessDenial),
}

pub fn materialize_s6_io_qos_isolation_readiness(
    closeout: ExecutedS5IsolationCloseout,
) -> Result<S6IoQosIsolationReadiness, S6IoQosReadinessHandoffMaterializationDenial> {
    publish_s6_io_qos_isolation_readiness_from_s5_closeout(closeout)
        .map_err(S6IoQosReadinessHandoffMaterializationDenial::S6)
}
