use worth_proof::AuthorityWitness;

use super::super::{
    bridge_canonical_export_trust_boundary, compare_canonical_exports,
    prepare_canonical_export_bundle, readmit_canonical_export_after_boundary,
    BoundaryBridgedCanonicalExportArtifact, CanonicalBundleReadyArtifact,
    CanonicalEquivalenceBasis, CanonicalExportComparisonOutcome, CanonicalExportManifestMismatch,
    CanonicalExportReadmissionAuthority, CanonicalExportReadyArtifact, CanonicalMismatchBasis,
    CanonicalProducerShape, CanonicalizationRuleVersion,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct CanonicalExportFrontDoor;

impl CanonicalExportFrontDoor {
    pub fn from_bundle(self, bundle: CanonicalBundleReadyArtifact) -> CanonicalExportNameStep {
        CanonicalExportNameStep { bundle }
    }

    pub fn compare(
        self,
        left: &CanonicalExportReadyArtifact,
        right: &CanonicalExportReadyArtifact,
    ) -> CanonicalExportComparisonOutcome {
        compare_canonical_exports(left, right)
    }

    pub fn mismatch_basis<'a>(
        self,
        outcome: &'a CanonicalExportComparisonOutcome,
    ) -> Option<&'a CanonicalMismatchBasis> {
        match outcome {
            CanonicalExportComparisonOutcome::Mismatched(mismatch) => Some(mismatch),
            CanonicalExportComparisonOutcome::Equivalent
            | CanonicalExportComparisonOutcome::ManifestMismatch(_) => None,
        }
    }

    pub fn manifest_mismatch<'a>(
        self,
        outcome: &'a CanonicalExportComparisonOutcome,
    ) -> Option<&'a CanonicalExportManifestMismatch> {
        match outcome {
            CanonicalExportComparisonOutcome::ManifestMismatch(mismatch) => Some(mismatch),
            CanonicalExportComparisonOutcome::Equivalent
            | CanonicalExportComparisonOutcome::Mismatched(_) => None,
        }
    }

    pub fn bridge(
        self,
        export: CanonicalExportReadyArtifact,
    ) -> BoundaryBridgedCanonicalExportArtifact {
        bridge_canonical_export_trust_boundary(export)
    }

    pub fn readmit(
        self,
        bridged: BoundaryBridgedCanonicalExportArtifact,
        rule_version: CanonicalizationRuleVersion,
        authority: AuthorityWitness<CanonicalExportReadmissionAuthority>,
    ) -> CanonicalExportReadyArtifact {
        readmit_canonical_export_after_boundary(bridged, rule_version, authority)
    }
}

pub struct CanonicalExportNameStep {
    bundle: CanonicalBundleReadyArtifact,
}

impl CanonicalExportNameStep {
    pub fn named(self, fixture_name: impl Into<String>) -> CanonicalExportShapeStep {
        CanonicalExportShapeStep {
            bundle: self.bundle,
            fixture_name: fixture_name.into(),
        }
    }
}

pub struct CanonicalExportShapeStep {
    bundle: CanonicalBundleReadyArtifact,
    fixture_name: String,
}

impl CanonicalExportShapeStep {
    pub fn for_producer_shape(
        self,
        producer_shape: CanonicalProducerShape,
    ) -> CanonicalExportBasisStep {
        CanonicalExportBasisStep {
            bundle: self.bundle,
            fixture_name: self.fixture_name,
            producer_shape,
        }
    }
}

pub struct CanonicalExportBasisStep {
    bundle: CanonicalBundleReadyArtifact,
    fixture_name: String,
    producer_shape: CanonicalProducerShape,
}

impl CanonicalExportBasisStep {
    pub fn under(
        self,
        equivalence_basis: CanonicalEquivalenceBasis,
    ) -> worth_proof::TransitionOutcome<CanonicalExportReadyArtifact> {
        prepare_canonical_export_bundle(
            self.fixture_name,
            self.producer_shape,
            equivalence_basis,
            self.bundle,
        )
    }
}
