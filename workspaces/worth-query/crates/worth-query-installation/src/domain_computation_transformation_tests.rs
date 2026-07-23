use crate::domain_computation_artifact_fixture::*;
use crate::facade::*;

#[test]
fn every_transformation_semantic_dimension_changes_contract_identity() {
    let baseline = transformation_identity(
        "worth.routing.raw-occurrence",
        "worth.routing.import",
        1,
        WorthQueryTransformationOutcomeContract::new(
            WorthQuerySourceOutputCorrespondence::OneToMany,
            WorthQueryTransformationDisposition::Split,
            WorthQueryTransformationErrorPosture::Bounded,
            WorthQueryTransformationLossPosture::DeclaredLossy,
        ),
    );
    let drifted = [
        transformation_identity(
            "worth.routing.normalized-occurrence",
            "worth.routing.import",
            1,
            standard_outcome(),
        ),
        transformation_identity(
            "worth.routing.raw-occurrence",
            "worth.routing.repair",
            1,
            standard_outcome(),
        ),
        transformation_identity(
            "worth.routing.raw-occurrence",
            "worth.routing.import",
            2,
            standard_outcome(),
        ),
        transformation_identity(
            "worth.routing.raw-occurrence",
            "worth.routing.import",
            1,
            WorthQueryTransformationOutcomeContract::new(
                WorthQuerySourceOutputCorrespondence::ManyToOne,
                WorthQueryTransformationDisposition::Split,
                WorthQueryTransformationErrorPosture::Bounded,
                WorthQueryTransformationLossPosture::DeclaredLossy,
            ),
        ),
        transformation_identity(
            "worth.routing.raw-occurrence",
            "worth.routing.import",
            1,
            WorthQueryTransformationOutcomeContract::new(
                WorthQuerySourceOutputCorrespondence::OneToMany,
                WorthQueryTransformationDisposition::Merged,
                WorthQueryTransformationErrorPosture::Bounded,
                WorthQueryTransformationLossPosture::DeclaredLossy,
            ),
        ),
        transformation_identity(
            "worth.routing.raw-occurrence",
            "worth.routing.import",
            1,
            WorthQueryTransformationOutcomeContract::new(
                WorthQuerySourceOutputCorrespondence::OneToMany,
                WorthQueryTransformationDisposition::Split,
                WorthQueryTransformationErrorPosture::Estimated,
                WorthQueryTransformationLossPosture::DeclaredLossy,
            ),
        ),
        transformation_identity(
            "worth.routing.raw-occurrence",
            "worth.routing.import",
            1,
            WorthQueryTransformationOutcomeContract::new(
                WorthQuerySourceOutputCorrespondence::OneToMany,
                WorthQueryTransformationDisposition::Split,
                WorthQueryTransformationErrorPosture::Bounded,
                WorthQueryTransformationLossPosture::LossClassifiedByDomain,
            ),
        ),
    ];

    for identity in drifted {
        assert_ne!(baseline, identity);
    }
}

fn standard_outcome() -> WorthQueryTransformationOutcomeContract {
    WorthQueryTransformationOutcomeContract::new(
        WorthQuerySourceOutputCorrespondence::OneToMany,
        WorthQueryTransformationDisposition::Split,
        WorthQueryTransformationErrorPosture::Bounded,
        WorthQueryTransformationLossPosture::DeclaredLossy,
    )
}

fn transformation_identity(
    source: &str,
    family: &str,
    version: u32,
    outcome: WorthQueryTransformationOutcomeContract,
) -> String {
    base_builder()
        .transformation(WorthQueryTransformationEvidenceContract::declared(
            WorthQueryImmutableSourceOccurrenceContract::new(source),
            WorthQueryTransformationIdentity::new(family, version),
            outcome,
        ))
        .compatibility(active_compatibility())
        .finish()
        .unwrap()
        .identity()
        .as_str()
        .to_string()
}
