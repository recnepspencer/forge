use crate::domain_computation_artifact_fixture::*;
use crate::facade::*;

#[test]
fn unsupported_schema_protocol_retirement_and_migration_are_distinct() {
    let denial = |compatibility| {
        base_builder()
            .compatibility(compatibility)
            .finish()
            .unwrap_err()
            .kind()
    };
    assert_eq!(
        denial(WorthQueryArtifactCompatibilityContract::new(
            WorthQueryArtifactCompatibilityWindow::new(
                WorthQueryArtifactSchemaVersion::new(3),
                WorthQueryArtifactSchemaVersion::new(4),
                WorthQueryArtifactProtocolVersion::new(1),
                WorthQueryArtifactProtocolVersion::new(2),
            ),
            "migration",
            WorthQueryArtifactRetirementRule::Active,
            WorthQueryArtifactDowngradePosture::Denied,
        )),
        WorthQueryArtifactContractValidationDenialKind::UnsupportedSchemaVersion
    );
    assert_eq!(
        denial(WorthQueryArtifactCompatibilityContract::new(
            WorthQueryArtifactCompatibilityWindow::new(
                WorthQueryArtifactSchemaVersion::new(1),
                WorthQueryArtifactSchemaVersion::new(3),
                WorthQueryArtifactProtocolVersion::new(2),
                WorthQueryArtifactProtocolVersion::new(3),
            ),
            "migration",
            WorthQueryArtifactRetirementRule::Active,
            WorthQueryArtifactDowngradePosture::Denied,
        )),
        WorthQueryArtifactContractValidationDenialKind::UnsupportedProtocolVersion
    );
    assert_eq!(
        denial(WorthQueryArtifactCompatibilityContract::new(
            compatibility_window(),
            "migration",
            WorthQueryArtifactRetirementRule::Retired,
            WorthQueryArtifactDowngradePosture::Denied,
        )),
        WorthQueryArtifactContractValidationDenialKind::RetiredSchemaVersion
    );
    assert_eq!(
        denial(
            WorthQueryArtifactCompatibilityContract::new(
                compatibility_window(),
                "migration.alpha",
                WorthQueryArtifactRetirementRule::Active,
                WorthQueryArtifactDowngradePosture::Denied,
            )
            .migration_owner("migration.beta")
        ),
        WorthQueryArtifactContractValidationDenialKind::AmbiguousMigration
    );
}

#[test]
fn retired_through_schema_uses_an_explicit_inclusive_cutoff() {
    let denial = base_builder()
        .compatibility(WorthQueryArtifactCompatibilityContract::new(
            compatibility_window(),
            "migration",
            WorthQueryArtifactRetirementRule::RetiredThroughSchema(
                WorthQueryArtifactSchemaVersion::new(2),
            ),
            WorthQueryArtifactDowngradePosture::Denied,
        ))
        .finish()
        .unwrap_err();

    assert_eq!(
        denial.kind(),
        WorthQueryArtifactContractValidationDenialKind::RetiredSchemaVersion
    );
}

#[test]
fn unversioned_compatibility_bounds_and_invalid_downgrade_family_are_rejected() {
    let zero_schema = base_builder()
        .compatibility(WorthQueryArtifactCompatibilityContract::new(
            WorthQueryArtifactCompatibilityWindow::new(
                WorthQueryArtifactSchemaVersion::new(0),
                WorthQueryArtifactSchemaVersion::new(2),
                WorthQueryArtifactProtocolVersion::new(1),
                WorthQueryArtifactProtocolVersion::new(1),
            ),
            "migration",
            WorthQueryArtifactRetirementRule::Active,
            WorthQueryArtifactDowngradePosture::Denied,
        ))
        .finish()
        .unwrap_err();
    assert_eq!(
        zero_schema.kind(),
        WorthQueryArtifactContractValidationDenialKind::UnsupportedSchemaVersion
    );

    let invalid_downgrade = base_builder()
        .compatibility(WorthQueryArtifactCompatibilityContract::new(
            compatibility_window(),
            "migration",
            WorthQueryArtifactRetirementRule::Active,
            WorthQueryArtifactDowngradePosture::SupportedBy { family: " ".into() },
        ))
        .finish()
        .unwrap_err();
    assert_eq!(
        invalid_downgrade.kind(),
        WorthQueryArtifactContractValidationDenialKind::InvalidSemanticEvidence
    );
}
