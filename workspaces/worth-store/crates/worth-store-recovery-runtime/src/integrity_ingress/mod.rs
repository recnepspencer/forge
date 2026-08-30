mod admission;
mod admitted_artifact;
mod counters;
mod families;
mod namespace_join;
mod observation;
mod planned_selector;
mod rejection;
mod root_protocol;
mod routing;
mod source;

#[cfg(test)]
mod tests;

#[allow(unused_imports)]
pub(crate) use admitted_artifact::IntegrityAdmittedRecoveryArtifact;
pub(crate) use counters::RecoveryIntegrityIngressCounters;
#[allow(unused_imports)]
pub(crate) use families::checkpoint::{
    IntegrityAdmittedCheckpointProjection, IntegrityAdmittedCheckpointStream,
};
pub(crate) use families::extent::{
    admit_extent_chunk_projection, admit_extent_manifest_projection,
};
pub(crate) use families::page::admit_page_projection;
use families::root::{
    admit_current_root_selector, admit_previous_root_selector, admit_root_manifest,
};
pub(crate) use namespace_join::RecoveryArtifactNamespaceJoin;
pub(crate) use observation::{
    RecoveryIntegrityIngressObservation, RecoveryIntegrityIngressObservationOutcome,
};
pub(crate) use planned_selector::admit_staged_current_selector;
pub(crate) use rejection::RecoveryIntegrityIngressRejection;
pub(crate) use root_protocol::{
    admit_addressed_root, admit_current_selector, admit_observed_bootstrap_catalog,
    admit_previous_selector,
};
#[allow(unused_imports)]
pub(crate) use routing::{observe_absent_recovery_artifact, RecoveryIntegrityIngressAttempt};
use source::{ObservedRecoverySource, ObservedWalFrameSource};

#[cfg(test)]
mod owner_valid_compile_contracts {
    use worth_store_physical_integrity::*;

    use super::*;

    fn current<'media>(
        source: ObservedRecoverySource<'media>,
        validated: IntegrityValidatedCurrentRootSelector<'media>,
    ) {
        let _ = admit_current_root_selector(source, validated);
    }

    fn previous<'media>(
        source: ObservedRecoverySource<'media>,
        validated: IntegrityValidatedPreviousRootSelector<'media>,
    ) {
        let _ = admit_previous_root_selector(source, validated);
    }

    fn manifest<'media>(
        source: ObservedRecoverySource<'media>,
        validated: IntegrityValidatedRootManifest<'media>,
    ) {
        let _ = admit_root_manifest(source, validated);
    }

    #[test]
    fn phase_five_ingress_shapes_consume_every_sealed_family_validation() {
        let _ = current;
        let _ = previous;
        let _ = manifest;
        families::owner_valid_compile_contracts();
    }
}
