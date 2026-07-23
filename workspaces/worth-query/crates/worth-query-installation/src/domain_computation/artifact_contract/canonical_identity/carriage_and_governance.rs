use sha2::Sha256;

use crate::canonical_hash_encoding::hash_text_field;
use crate::domain_computation::*;

use super::super::WorthQueryPortableArtifactContract;
use super::vocabulary::*;

pub(super) fn hash_carriage(hash: &mut Sha256, contract: &WorthQueryPortableArtifactContract) {
    let carriage = contract.carriage;
    hash_text_field(hash, "movement", move_name(carriage.movement()));
    hash_text_field(hash, "borrowing", borrow_name(carriage.borrowing()));
    let clone_posture = carriage.clone_posture();
    hash_text_field(hash, "clone", clone_posture_name(clone_posture));
    if let WorthQueryArtifactClonePosture::Declared {
        mechanism,
        boundary,
    } = clone_posture
    {
        hash_text_field(hash, "clone-mechanism", clone_mechanism_name(mechanism));
        hash_text_field(hash, "clone-boundary", clone_boundary_name(boundary));
    }
    hash_text_field(
        hash,
        "provider-transfer",
        provider_transfer_name(carriage.provider_transfer()),
    );
    hash_text_field(
        hash,
        "serialization",
        serialization_name(carriage.serialization()),
    );
    hash_text_field(hash, "lifecycle", lifecycle_name(contract.lifecycle));
    for (label, counter) in [
        ("byte-counter", contract.counters.byte_counter()),
        ("element-counter", contract.counters.element_counter()),
        (
            "structural-counter",
            contract.counters.structural_work_counter(),
        ),
    ] {
        hash_text_field(hash, label, counter.as_str());
    }
}

pub(super) fn hash_governance(hash: &mut Sha256, contract: &WorthQueryPortableArtifactContract) {
    for audience in contract.governance.audiences() {
        hash_text_field(hash, "audience", audience);
    }
    hash_text_field(
        hash,
        "classification",
        classification_name(contract.governance.classification()),
    );
    hash_text_field(
        hash,
        "redaction",
        redaction_name(contract.governance.redaction()),
    );
    hash_text_field(
        hash,
        "retention",
        retention_name(contract.governance.retention()),
    );
    hash_text_field(
        hash,
        "deletion",
        deletion_name(contract.governance.deletion()),
    );
    hash_text_field(
        hash,
        "legal-hold",
        legal_hold_name(contract.governance.legal_hold()),
    );
}

pub(super) fn hash_compatibility(
    hash: &mut Sha256,
    value: &WorthQueryArtifactCompatibilityContract,
) {
    hash_text_field(
        hash,
        "minimum-schema",
        &value.minimum_schema().get().to_string(),
    );
    hash_text_field(
        hash,
        "maximum-schema",
        &value.maximum_schema().get().to_string(),
    );
    hash_text_field(
        hash,
        "minimum-protocol",
        &value.minimum_protocol().get().to_string(),
    );
    hash_text_field(
        hash,
        "maximum-protocol",
        &value.maximum_protocol().get().to_string(),
    );
    for owner in value.migration_owners() {
        hash_text_field(hash, "migration-owner", owner);
    }
    match value.retirement() {
        WorthQueryArtifactRetirementRule::Active => hash_text_field(hash, "retirement", "active"),
        WorthQueryArtifactRetirementRule::Retired => hash_text_field(hash, "retirement", "retired"),
        WorthQueryArtifactRetirementRule::RetiredThroughSchema(version) => {
            hash_text_field(hash, "retirement", "retired-through-schema");
            hash_text_field(hash, "retirement-version", &version.get().to_string());
        }
    }
    match value.downgrade() {
        WorthQueryArtifactDowngradePosture::Denied => hash_text_field(hash, "downgrade", "denied"),
        WorthQueryArtifactDowngradePosture::SupportedBy { family } => {
            hash_text_field(hash, "downgrade", "supported");
            hash_text_field(hash, "downgrade-family", family);
        }
    }
}
