use worth_store_physical_integrity::{
    IntegrityValidatedCurrentRootSelector, IntegrityValidatedPreviousRootSelector,
    IntegrityValidatedRootManifest,
};

use super::admitted_artifact::{
    IntegrityAdmittedCurrentRootSelector, IntegrityAdmittedPreviousRootSelector,
    IntegrityAdmittedRootManifest,
};
use super::{RecoveryIntegrityIngressRejection, UntrustedRecoverySource};

pub(crate) fn admit_current_root_selector<'media>(
    source: UntrustedRecoverySource<'media>,
    validated: IntegrityValidatedCurrentRootSelector<'media>,
) -> Result<IntegrityAdmittedCurrentRootSelector<'media>, RecoveryIntegrityIngressRejection> {
    IntegrityAdmittedCurrentRootSelector::bind(source, validated)
}

pub(crate) fn admit_previous_root_selector<'media>(
    source: UntrustedRecoverySource<'media>,
    validated: IntegrityValidatedPreviousRootSelector<'media>,
) -> Result<IntegrityAdmittedPreviousRootSelector<'media>, RecoveryIntegrityIngressRejection> {
    IntegrityAdmittedPreviousRootSelector::bind(source, validated)
}

pub(crate) fn admit_root_manifest<'media>(
    source: UntrustedRecoverySource<'media>,
    validated: IntegrityValidatedRootManifest<'media>,
) -> Result<IntegrityAdmittedRootManifest<'media>, RecoveryIntegrityIngressRejection> {
    IntegrityAdmittedRootManifest::bind(source, validated)
}

#[cfg(test)]
mod tests {
    use worth_proof::TransitionOutcome;
    use worth_store::physical_runtime::{
        FilesystemAccessPosture, FilesystemMediaAdmission, PhysicalRuntimeAdmission, PhysicalStore,
        QualifiedRecoveryFilesystemMedia,
    };
    use worth_store_physical_format::{
        DurableRootSelector, PhysicalRecordFormatDeclaration, RootSelectorIdentity,
        RootSelectorRole, ROOT_SELECTOR_BYTES,
    };
    use worth_store_physical_integrity::{
        validate_current_root_selector, CurrentRootSelectorIntegrityValidation,
        PhysicalArtifactScope, PhysicalByteRange,
    };

    use super::*;

    #[test]
    fn validated_source_incarnation_cannot_admit_an_identical_second_read() {
        let parent = tempfile::tempdir().expect("test parent");
        let root = parent.path().join("source-incarnation");
        let runtime = PhysicalStore::admit(
            PhysicalRuntimeAdmission::new(root.clone()).expect("declared root"),
        )
        .expect("ordinary runtime admission");
        let admission = FilesystemMediaAdmission::production(
            FilesystemAccessPosture::CoordinatedServiceAccount,
        );
        let media = match runtime.try_admit_filesystem_media(admission).into_raw() {
            TransitionOutcome::Success(media) => media,
            _ => panic!("ordinary media initialization failed"),
        };
        let store = media.store_identity();
        let _ = media.close();
        let format = PhysicalRecordFormatDeclaration::builder().admit().unwrap();
        let selector = DurableRootSelector::new(
            store,
            format,
            RootSelectorIdentity::new(1).unwrap(),
            RootSelectorRole::Current,
            1,
            None,
            None,
        )
        .unwrap();
        let records = root.join("families").join("records");
        std::fs::create_dir_all(&records).unwrap();
        std::fs::write(records.join("root-current.selector"), selector.encode()).unwrap();

        let media = QualifiedRecoveryFilesystemMedia::qualify_existing(&root)
            .unwrap()
            .admit_persisted_store()
            .unwrap();
        let mut discovery = media.bounded_discovery(2, 4096).unwrap();
        let source_a = discovery
            .read_current_selector(ROOT_SELECTOR_BYTES as u64)
            .unwrap();
        let source_b = discovery
            .read_current_selector(ROOT_SELECTOR_BYTES as u64)
            .unwrap();
        let scope = PhysicalArtifactScope::current_root_selector(
            store,
            format,
            PhysicalByteRange::new(0, ROOT_SELECTOR_BYTES as u64).unwrap(),
        );

        let validated_a = validate(&source_a, scope);
        assert!(admit_current_root_selector(
            UntrustedRecoverySource::new(&source_a, scope),
            validated_a,
        )
        .is_ok());
        let validated_a = validate(&source_a, scope);
        assert!(matches!(
            admit_current_root_selector(
                UntrustedRecoverySource::new(&source_b, scope),
                validated_a,
            ),
            Err(RecoveryIntegrityIngressRejection::SourceIncarnationMismatch)
        ));
        drop(discovery.finish());
    }

    fn validate<'media>(
        source: &'media worth_store::physical_runtime::ObservedRecoveryArtifact,
        scope: PhysicalArtifactScope,
    ) -> IntegrityValidatedCurrentRootSelector<'media> {
        let input = UntrustedRecoverySource::new(source, scope)
            .input()
            .expect("present selector");
        let (validation, _) = validate_current_root_selector(input, scope);
        let CurrentRootSelectorIntegrityValidation::Intact(validated) = validation else {
            panic!("selector must validate")
        };
        validated
    }
}
