mod admission;
mod admitted_artifact;
mod namespace_join;
mod planned_selector;
mod rejection;
mod root_protocol;
mod untrusted_source;

pub(crate) use admission::{
    admit_current_root_selector, admit_previous_root_selector, admit_root_manifest,
};
pub(crate) use namespace_join::RecoveryArtifactNamespaceJoin;
pub(crate) use planned_selector::admit_staged_current_selector;
pub(crate) use rejection::RecoveryIntegrityIngressRejection;
pub(crate) use root_protocol::{
    admit_addressed_root, admit_current_selector, admit_previous_selector,
};
pub(crate) use untrusted_source::UntrustedRecoverySource;

#[cfg(test)]
mod owner_valid_compile_contracts {
    use worth_store_physical_integrity::{
        IntegrityValidatedCurrentRootSelector, IntegrityValidatedPreviousRootSelector,
        IntegrityValidatedRootManifest,
    };

    use super::*;

    fn current<'media>(
        source: UntrustedRecoverySource<'media>,
        validated: IntegrityValidatedCurrentRootSelector<'media>,
    ) {
        let _ = admit_current_root_selector(source, validated);
    }

    fn previous<'media>(
        source: UntrustedRecoverySource<'media>,
        validated: IntegrityValidatedPreviousRootSelector<'media>,
    ) {
        let _ = admit_previous_root_selector(source, validated);
    }

    fn manifest<'media>(
        source: UntrustedRecoverySource<'media>,
        validated: IntegrityValidatedRootManifest<'media>,
    ) {
        let _ = admit_root_manifest(source, validated);
    }

    #[test]
    fn phase_two_ingress_shapes_consume_sealed_validation() {
        let _ = current;
        let _ = previous;
        let _ = manifest;
    }
}
