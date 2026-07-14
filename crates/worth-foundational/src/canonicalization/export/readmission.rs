use worth_proof::{Artifact, AuthorityWitness};

use super::super::{
    CanonicalExportReadinessProofs, CanonicalExportReady, CanonicalizationRuleVersion,
};
use super::authority::CanonicalExportReadmissionAuthority;
use super::bundle::CanonicalExportBundle;

pub type CanonicalExportReadyArtifact = Artifact<
    CanonicalExportReady,
    CanonicalExportBundle,
    CanonicalExportReadinessProofs,
    worth_proof::FreshnessScopedBasis<
        worth_proof::CurrentValidity,
        worth_proof::AssumptionBasis<CanonicalizationRuleVersion>,
    >,
>;

pub type BoundaryBridgedCanonicalExportArtifact = Artifact<
    CanonicalExportReady,
    CanonicalExportBundle,
    CanonicalExportReadinessProofs,
    worth_proof::BoundaryBridgedAuthorityRevalidationRequiredBasis<CanonicalizationRuleVersion>,
>;

pub fn bridge_canonical_export_trust_boundary(
    export: CanonicalExportReadyArtifact,
) -> BoundaryBridgedCanonicalExportArtifact {
    export.bridge_trust_boundary()
}

pub fn readmit_canonical_export_after_boundary(
    bridged: BoundaryBridgedCanonicalExportArtifact,
    rule_version: CanonicalizationRuleVersion,
    authority: AuthorityWitness<CanonicalExportReadmissionAuthority>,
) -> CanonicalExportReadyArtifact {
    bridged.readmit_with_authority(rule_version, authority)
}

#[cfg(test)]
mod tests {
    use worth_proof::TransitionOutcome;

    use super::{bridge_canonical_export_trust_boundary, readmit_canonical_export_after_boundary};
    use crate::canonicalization::export::{
        prepare_canonical_export_bundle, CanonicalExportReadmissionAuthority,
        CanonicalProducerShape,
    };
    use crate::canonicalization::{
        prepare_canonical_basis_bundle, prepare_canonical_basis_sequence, CanonicalBasisDomain,
        CanonicalBasisEntry, CanonicalBasisEntryKind, CanonicalBasisLocus, CanonicalBasisValue,
        CanonicalEquivalenceBasis, CanonicalIntegerWidth, CanonicalizationRuleVersion,
    };

    fn readmission_test_version() -> CanonicalizationRuleVersion {
        CanonicalizationRuleVersion::new("m2.phase4.internal.readmission").expect("valid version")
    }

    #[test]
    fn export_readmission_requires_milestone_owned_authority_witness() {
        let version = readmission_test_version();
        let entry = CanonicalBasisEntry::new(
            CanonicalBasisDomain::Value,
            CanonicalBasisLocus::Named("alpha".into()),
            CanonicalBasisEntryKind::Value,
            CanonicalBasisValue::SignedInteger {
                width: CanonicalIntegerWidth::Bits64,
                value: 1,
            },
        );
        let sequence = match prepare_canonical_basis_sequence(
            version.clone(),
            CanonicalBasisDomain::Value,
            [entry],
        ) {
            TransitionOutcome::Success(sequence) => sequence,
            _ => panic!("basis should be ready"),
        };
        let bundle = match prepare_canonical_basis_bundle(version.clone(), [sequence]) {
            TransitionOutcome::Success(bundle) => bundle,
            _ => panic!("bundle should be ready"),
        };
        let export = match prepare_canonical_export_bundle(
            "internal-readmission",
            CanonicalProducerShape::GoldenFixture,
            CanonicalEquivalenceBasis::ExactCanonicalBasis,
            bundle,
        ) {
            TransitionOutcome::Success(export) => export,
            _ => panic!("export should be ready"),
        };
        let authority = worth_proof::AuthorityWitness::from_authority_marker(
            CanonicalExportReadmissionAuthority::new(),
        );

        let readmitted = readmit_canonical_export_after_boundary(
            bridge_canonical_export_trust_boundary(export),
            version,
            authority,
        );

        assert_eq!(
            readmitted.strong_basis().value().as_str(),
            "m2.phase4.internal.readmission"
        );
    }
}
