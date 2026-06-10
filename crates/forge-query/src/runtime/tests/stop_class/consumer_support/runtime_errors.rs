use super::super::super::support::*;

pub(in super::super) fn temporal_public_family_admission_error(
    workspace_name: &str,
    reason: &str,
) -> ForgeQueryRuntimeError {
    bridge_runtime_with_support(
        ForgeQueryRuntimeSupportProfile::scaffold_backend_profile().with_family_support(
            ForgeQueryRuntimeFamilySupport::supported_with_teaching_posture_and_reason(
                ForgeQueryRuntimeFacadeFamily::Temporal,
                ForgeQueryRuntimeFamilyTeachingPosture::SupportGateOnly,
                [ForgeQueryAuthorityLane::TemporalExecutionState],
                [],
                ["runtime-backed-temporal-basis-state-inspection"],
                reason,
            ),
        ),
    )
    .workspace(workspace_name)
    .expect("workspace should open")
    .admit_public_api_family(ForgeQueryRuntimeFacadeFamily::Temporal)
    .expect_err("support-gated temporal family should fail closed")
}
