mod artifact_editor;
mod clean_artifact_manifest;
mod corruption_operator;
mod counters;
mod editor_result_audit;
mod frame_checksum;
mod parent_oracle;
mod producer_fixture;
mod recovery_process;
mod scenario;
mod verifier_process;
mod wire;

#[cfg(test)]
mod oracle_expectation_assertions;
#[cfg(test)]
mod tests;
#[cfg(test)]
mod test_world;
#[cfg(test)]
mod wire_tests;

pub(crate) use artifact_editor::apply_declared_corruption;
pub(crate) use clean_artifact_manifest::{
    CleanRootArtifactManifest, CleanRootArtifactRecord, RootArtifactIdentity,
    RootArtifactManifestDenial, RootArtifactRole,
};
pub(crate) use corruption_operator::{DeclaredRootCorruption, RootCorruptionCode};
pub(crate) use counters::RootLocalizationCounters;
pub(crate) use editor_result_audit::{EditorAuditDenial, EditorResultAudit};
pub(crate) use parent_oracle::{
    derive_parent_expectation, ExpectedMinimumBlastRadius, ExpectedRootCause,
    ExpectedRootLocalization, ExpectedRootPosture,
};
pub(crate) use recovery_process::RuntimeRootObservationConnectorRequest;
pub(crate) use scenario::{
    ExternalReportPathDenial, ExternalReportPaths, FreshRootArtifactRow,
    FreshRootArtifactRowDenial, RootSliceScenario,
};
pub(crate) use verifier_process::OfflineRootObservationConnectorRequest;
pub(crate) use wire::{
    RootWireDenial, RootWireIdentity, RootWireRole,
};
