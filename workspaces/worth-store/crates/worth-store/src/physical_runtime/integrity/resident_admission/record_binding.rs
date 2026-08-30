use worth_store_buffer_pool::{PhysicalFrameLease, PhysicalResidentFrameGeneration};
use worth_store_physical_format::{RecordArtifactFile, RecordFrameCoordinate};
use worth_store_physical_integrity::{
    IntegrityValidatedCurrentRootSelector, IntegrityValidatedPreviousRootSelector,
    IntegrityValidatedRootManifest, PhysicalArtifactScope, PhysicalIntegrityValidationRecord,
    UntrustedPhysicalArtifact,
};

use super::denial::ResidentIntegrityAdmissionDenial;
use crate::physical_runtime::LifecycleGeneration;

/// Store-lifecycle binding for a descriptive record retained by the exact C.6 frame.
///
/// This value grants no decoder access. Family-specific admitted views are
/// introduced only by their production cutover phases.
pub(in crate::physical_runtime) struct ResidentIntegrityRecordBinding<'lease> {
    lease: &'lease PhysicalFrameLease,
    lifecycle_generation: LifecycleGeneration,
    frame_generation: PhysicalResidentFrameGeneration,
    scope: PhysicalArtifactScope,
}

impl<'lease> ResidentIntegrityRecordBinding<'lease> {
    pub(in crate::physical_runtime) fn bind_current_root_selector(
        lease: &'lease PhysicalFrameLease,
        lifecycle_generation: LifecycleGeneration,
        validated: IntegrityValidatedCurrentRootSelector<'lease>,
    ) -> Result<Self, ResidentIntegrityAdmissionDenial> {
        let scope = validated.scope();
        let input = require_source(lease, scope)?;
        if !validated.matches_input(input) {
            return Err(ResidentIntegrityAdmissionDenial::SourceIncarnationMismatch);
        }
        commit(
            lease,
            lifecycle_generation,
            scope,
            validated.into_validation_record(),
        )
    }

    pub(in crate::physical_runtime) fn bind_previous_root_selector(
        lease: &'lease PhysicalFrameLease,
        lifecycle_generation: LifecycleGeneration,
        validated: IntegrityValidatedPreviousRootSelector<'lease>,
    ) -> Result<Self, ResidentIntegrityAdmissionDenial> {
        let scope = validated.scope();
        let input = require_source(lease, scope)?;
        if !validated.matches_input(input) {
            return Err(ResidentIntegrityAdmissionDenial::SourceIncarnationMismatch);
        }
        commit(
            lease,
            lifecycle_generation,
            scope,
            validated.into_validation_record(),
        )
    }

    pub(in crate::physical_runtime) fn bind_root_manifest(
        lease: &'lease PhysicalFrameLease,
        lifecycle_generation: LifecycleGeneration,
        validated: IntegrityValidatedRootManifest<'lease>,
    ) -> Result<Self, ResidentIntegrityAdmissionDenial> {
        let scope = validated.scope();
        let input = require_source(lease, scope)?;
        if !validated.matches_input(input) {
            return Err(ResidentIntegrityAdmissionDenial::SourceIncarnationMismatch);
        }
        commit(
            lease,
            lifecycle_generation,
            scope,
            validated.into_validation_record(),
        )
    }

    pub(in crate::physical_runtime) fn from_retained_record(
        lease: &'lease PhysicalFrameLease,
        lifecycle_generation: LifecycleGeneration,
        scope: PhysicalArtifactScope,
    ) -> Option<Self> {
        let record = lease.integrity_validation()?;
        require_source(lease, scope).ok()?;
        record.matches_scope(scope).then(|| Self {
            lease,
            lifecycle_generation,
            frame_generation: lease.resident_generation(),
            scope,
        })
    }

    pub(in crate::physical_runtime) const fn lifecycle_generation(&self) -> LifecycleGeneration {
        self.lifecycle_generation
    }

    pub(in crate::physical_runtime) const fn frame_generation(
        &self,
    ) -> PhysicalResidentFrameGeneration {
        self.frame_generation
    }

    pub(in crate::physical_runtime) const fn scope(&self) -> PhysicalArtifactScope {
        self.scope
    }

    pub(in crate::physical_runtime) const fn lease(&self) -> &'lease PhysicalFrameLease {
        self.lease
    }
}

fn require_source<'lease>(
    lease: &'lease PhysicalFrameLease,
    scope: PhysicalArtifactScope,
) -> Result<UntrustedPhysicalArtifact<'lease>, ResidentIntegrityAdmissionDenial> {
    let coordinate = lease.key().coordinate();
    let range = scope.byte_range();
    if scope.store_identity() != lease.key().store()
        || !coordinate_matches_scope(coordinate, scope)
        || range.offset() != coordinate.offset()
        || range.length() != u64::from(coordinate.length())
    {
        return Err(ResidentIntegrityAdmissionDenial::SourceScopeMismatch);
    }
    Ok(UntrustedPhysicalArtifact::from_bounded_bytes(lease))
}

fn coordinate_matches_scope(
    coordinate: RecordFrameCoordinate,
    scope: PhysicalArtifactScope,
) -> bool {
    coordinate.artifact() == resident_artifact_for_scope(scope)
}

fn resident_artifact_for_scope(scope: PhysicalArtifactScope) -> RecordArtifactFile {
    use worth_store_physical_format::integrity_declarations::PhysicalIntegrityArtifactFamily;

    match scope.artifact_family() {
        PhysicalIntegrityArtifactFamily::CurrentRootSelector => {
            RecordArtifactFile::CurrentRootSelector
        }
        PhysicalIntegrityArtifactFamily::PreviousRootSelector => {
            RecordArtifactFile::PreviousRootSelector
        }
        PhysicalIntegrityArtifactFamily::RootManifest => RecordArtifactFile::RootManifest {
            generation: scope
                .root_generation()
                .expect("root-manifest scope carries generation"),
        },
        _ => unreachable!("current resident integrity scopes are root-family only"),
    }
}

fn commit<'lease>(
    lease: &'lease PhysicalFrameLease,
    lifecycle_generation: LifecycleGeneration,
    scope: PhysicalArtifactScope,
    record: PhysicalIntegrityValidationRecord,
) -> Result<ResidentIntegrityRecordBinding<'lease>, ResidentIntegrityAdmissionDenial> {
    if !record.matches_scope(scope) {
        return Err(ResidentIntegrityAdmissionDenial::SourceScopeMismatch);
    }
    lease
        .commit_integrity_validation(record)
        .map_err(ResidentIntegrityAdmissionDenial::Frame)?;
    Ok(ResidentIntegrityRecordBinding {
        lease,
        lifecycle_generation,
        frame_generation: lease.resident_generation(),
        scope,
    })
}

#[cfg(test)]
mod tests {
    use worth_store_physical_format::store_namespace::{
        ProposedStoreIdentity, StoreNamespaceIdentityRecord, StoreNamespaceVersion,
    };
    use worth_store_physical_integrity::PhysicalByteRange;

    use super::*;

    fn store() -> worth_store_physical_format::store_namespace::StableStoreIdentity {
        StoreNamespaceIdentityRecord::new(
            StoreNamespaceVersion::CURRENT,
            ProposedStoreIdentity::from_nonzero_bytes([17; 16]).unwrap(),
        )
        .published_identity()
    }

    fn coordinate(artifact: RecordArtifactFile, length: u32) -> RecordFrameCoordinate {
        RecordFrameCoordinate::new(artifact, 0, length).unwrap()
    }

    fn format() -> worth_store_physical_format::PhysicalRecordFormatDeclaration {
        worth_store_physical_format::PhysicalRecordFormatDeclaration::builder()
            .admit()
            .unwrap()
    }

    #[test]
    fn resident_owner_mapping_rejects_root_family_substitution() {
        let scope = PhysicalArtifactScope::current_root_selector(
            store(),
            format(),
            PhysicalByteRange::new(0, 107).unwrap(),
        );

        assert!(coordinate_matches_scope(
            coordinate(RecordArtifactFile::CurrentRootSelector, 107),
            scope,
        ));
        assert!(!coordinate_matches_scope(
            coordinate(RecordArtifactFile::PreviousRootSelector, 107),
            scope,
        ));
    }

    #[test]
    fn resident_owner_mapping_rejects_root_generation_substitution() {
        let scope = PhysicalArtifactScope::root_manifest(
            store(),
            format(),
            7,
            PhysicalByteRange::new(0, 368).unwrap(),
        )
        .unwrap();

        assert!(coordinate_matches_scope(
            coordinate(RecordArtifactFile::RootManifest { generation: 7 }, 368),
            scope,
        ));
        assert!(!coordinate_matches_scope(
            coordinate(RecordArtifactFile::RootManifest { generation: 8 }, 368),
            scope,
        ));
    }
}
