use worth_store::physical_runtime::ObservedRecoveryArtifact;
use worth_store_physical_format::{
    DurablePhysicalRootManifest, DurableRootSelector, RootSelectorRole,
};
use worth_store_physical_integrity::{
    IntegrityValidatedCurrentRootSelector, IntegrityValidatedPreviousRootSelector,
    IntegrityValidatedRootManifest, UntrustedPhysicalArtifact,
};

use super::{RecoveryIntegrityIngressRejection, UntrustedRecoverySource};

pub(crate) struct IntegrityAdmittedCurrentRootSelector<'media> {
    _source: &'media ObservedRecoveryArtifact,
    validated: IntegrityValidatedCurrentRootSelector<'media>,
}

pub(crate) struct IntegrityAdmittedPreviousRootSelector<'media> {
    _source: &'media ObservedRecoveryArtifact,
    validated: IntegrityValidatedPreviousRootSelector<'media>,
}

pub(crate) struct IntegrityAdmittedRootManifest<'media> {
    _source: &'media ObservedRecoveryArtifact,
    validated: IntegrityValidatedRootManifest<'media>,
}

pub(crate) struct IntegrityAdmittedStagedCurrentSelector<'bytes> {
    _source: &'bytes [u8],
    validated: IntegrityValidatedCurrentRootSelector<'bytes>,
}

impl<'media> IntegrityAdmittedCurrentRootSelector<'media> {
    pub(super) fn bind(
        source: UntrustedRecoverySource<'media>,
        validated: IntegrityValidatedCurrentRootSelector<'media>,
    ) -> Result<Self, RecoveryIntegrityIngressRejection> {
        require_source(&source, validated.scope(), |input| {
            validated.matches_input(input)
        })?;
        let admitted = Self {
            _source: source.observed(),
            validated,
        };
        if admitted._source.bytes() != Some(admitted.selector().encode().as_slice()) {
            return Err(RecoveryIntegrityIngressRejection::NonCanonicalEncoding);
        }
        Ok(admitted)
    }

    fn selector(&self) -> DurableRootSelector {
        DurableRootSelector::new(
            self.validated.scope().store_identity(),
            self.validated.record_format(),
            self.validated.selector_identity(),
            RootSelectorRole::Current,
            self.validated.root_generation(),
            self.validated.linked_selector(),
            self.validated.linked_root_generation(),
        )
        .expect("sealed current-selector fields preserve the format contract")
    }

    pub(crate) fn project(self) -> DurableRootSelector {
        self.selector()
    }
}

impl<'media> IntegrityAdmittedPreviousRootSelector<'media> {
    pub(super) fn bind(
        source: UntrustedRecoverySource<'media>,
        validated: IntegrityValidatedPreviousRootSelector<'media>,
    ) -> Result<Self, RecoveryIntegrityIngressRejection> {
        require_source(&source, validated.scope(), |input| {
            validated.matches_input(input)
        })?;
        let admitted = Self {
            _source: source.observed(),
            validated,
        };
        if admitted._source.bytes() != Some(admitted.selector().encode().as_slice()) {
            return Err(RecoveryIntegrityIngressRejection::NonCanonicalEncoding);
        }
        Ok(admitted)
    }

    fn selector(&self) -> DurableRootSelector {
        DurableRootSelector::new(
            self.validated.scope().store_identity(),
            self.validated.record_format(),
            self.validated.selector_identity(),
            RootSelectorRole::Previous,
            self.validated.root_generation(),
            self.validated.linked_selector(),
            self.validated.linked_root_generation(),
        )
        .expect("sealed previous-selector fields preserve the format contract")
    }

    pub(crate) fn project(self) -> DurableRootSelector {
        self.selector()
    }
}

impl<'media> IntegrityAdmittedRootManifest<'media> {
    pub(super) fn bind(
        source: UntrustedRecoverySource<'media>,
        validated: IntegrityValidatedRootManifest<'media>,
    ) -> Result<Self, RecoveryIntegrityIngressRejection> {
        require_source(&source, validated.scope(), |input| {
            validated.matches_input(input)
        })?;
        let admitted = Self {
            _source: source.observed(),
            validated,
        };
        let (manifest, format) = admitted.manifest();
        if admitted._source.bytes() != Some(manifest.encode(format).as_slice()) {
            return Err(RecoveryIntegrityIngressRejection::NonCanonicalEncoding);
        }
        Ok(admitted)
    }

    fn manifest(
        &self,
    ) -> (
        DurablePhysicalRootManifest,
        worth_store_physical_format::PhysicalRecordFormatDeclaration,
    ) {
        let manifest = DurablePhysicalRootManifest::builder(
            self.validated.root_generation(),
            self.validated.tree_identity(),
            self.validated.node_capacity(),
            self.validated.free_space_checksum(),
        )
        .record_count(self.validated.record_count())
        .next_block(self.validated.next_block())
        .next_segment_block(self.validated.next_segment_block())
        .routing_root(self.validated.routing_root())
        .segment_root(self.validated.segment_root())
        .free_space_root(self.validated.free_space_root())
        .last_inline_record(self.validated.last_inline_record())
        .last_inline_segment(self.validated.last_inline_segment())
        .admit()
        .expect("sealed root-manifest fields preserve the format contract");
        (manifest, self.validated.record_format())
    }

    pub(crate) fn project(
        self,
    ) -> (
        DurablePhysicalRootManifest,
        worth_store_physical_format::PhysicalRecordFormatDeclaration,
    ) {
        self.manifest()
    }
}

impl<'bytes> IntegrityAdmittedStagedCurrentSelector<'bytes> {
    pub(super) fn bind(
        source: &'bytes [u8],
        validated: IntegrityValidatedCurrentRootSelector<'bytes>,
    ) -> Result<Self, RecoveryIntegrityIngressRejection> {
        let input = UntrustedPhysicalArtifact::from_bounded_bytes(source);
        if !validated.matches_input(input) {
            return Err(RecoveryIntegrityIngressRejection::SourceIncarnationMismatch);
        }
        let admitted = Self {
            _source: source,
            validated,
        };
        if admitted._source != admitted.selector().encode().as_slice() {
            return Err(RecoveryIntegrityIngressRejection::NonCanonicalEncoding);
        }
        Ok(admitted)
    }

    fn selector(&self) -> DurableRootSelector {
        DurableRootSelector::new(
            self.validated.scope().store_identity(),
            self.validated.record_format(),
            self.validated.selector_identity(),
            RootSelectorRole::Current,
            self.validated.root_generation(),
            self.validated.linked_selector(),
            self.validated.linked_root_generation(),
        )
        .expect("sealed staged current-selector fields preserve the format contract")
    }

    pub(crate) fn project(self) -> DurableRootSelector {
        self.selector()
    }
}

fn require_source<'media>(
    source: &UntrustedRecoverySource<'media>,
    validated_scope: worth_store_physical_integrity::PhysicalArtifactScope,
    matches: impl FnOnce(UntrustedPhysicalArtifact<'media>) -> bool,
) -> Result<(), RecoveryIntegrityIngressRejection> {
    if source.scope() != validated_scope {
        return Err(RecoveryIntegrityIngressRejection::ScopeMismatch);
    }
    let input = source
        .input()
        .ok_or(RecoveryIntegrityIngressRejection::MissingBoundedArtifact)?;
    if !matches(input) {
        return Err(RecoveryIntegrityIngressRejection::SourceIncarnationMismatch);
    }
    Ok(())
}
