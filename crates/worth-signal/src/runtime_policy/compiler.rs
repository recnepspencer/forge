use super::admission::{AdmittedSignalRuntimePolicy, SignalRuntimePolicyAdmissionDenial};
use super::request::SignalRuntimePolicyRequest;
use super::resolved::InstalledSignalRuntimePolicy;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SignalRuntimePolicyCompilationDenial {
    Admission(SignalRuntimePolicyAdmissionDenial),
    ObservationSessionActive,
}

pub fn compile_signal_runtime_policy(
    request: SignalRuntimePolicyRequest,
) -> Result<InstalledSignalRuntimePolicy, SignalRuntimePolicyCompilationDenial> {
    let admitted = AdmittedSignalRuntimePolicy::admit(request)
        .map_err(SignalRuntimePolicyCompilationDenial::Admission)?;
    let requested_policy = admitted.request().policy();
    Ok(InstalledSignalRuntimePolicy::new(
        super::lowering::resolve_signal_runtime_policy(admitted),
        requested_policy,
    ))
}
