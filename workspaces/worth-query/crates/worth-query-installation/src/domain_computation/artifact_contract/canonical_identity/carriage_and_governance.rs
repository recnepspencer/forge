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
    for counter in contract.counters.rows() {
        hash_text_field(hash, "counter-name", counter.name().as_str());
        hash_text_field(hash, "counter-role", counter_role_name(counter.role()));
        hash_text_field(hash, "counter-unit", &counter_unit_name(counter.unit()));
        hash_text_field(
            hash,
            "counter-aggregation",
            counter_aggregation_name(counter.aggregation()),
        );
        for source in counter.aggregation().sources() {
            hash_text_field(hash, "counter-aggregation-source", source.as_str());
        }
        hash_text_field(
            hash,
            "counter-monotonicity",
            counter_monotonicity_name(counter.monotonicity()),
        );
        hash_text_field(hash, "counter-scope", counter_scope_name(counter.scope()));
        hash_text_field(
            hash,
            "counter-reset",
            counter_reset_name(counter.reset_boundary()),
        );
        hash_text_field(
            hash,
            "counter-requiredness",
            counter_requiredness_name(counter.requiredness()),
        );
        hash_text_field(
            hash,
            "counter-replay",
            counter_replay_name(counter.replay()),
        );
    }
}

fn counter_role_name(value: WorthQueryStructuralCounterRole) -> &'static str {
    match value {
        WorthQueryStructuralCounterRole::Bytes => "bytes",
        WorthQueryStructuralCounterRole::Elements => "elements",
        WorthQueryStructuralCounterRole::StructuralWork => "structural-work",
        WorthQueryStructuralCounterRole::DomainWork => "domain-work",
    }
}

fn counter_unit_name(value: &WorthQueryStructuralCounterUnit) -> String {
    match value {
        WorthQueryStructuralCounterUnit::Bytes => "bytes".into(),
        WorthQueryStructuralCounterUnit::Elements => "elements".into(),
        WorthQueryStructuralCounterUnit::Operations => "operations".into(),
        WorthQueryStructuralCounterUnit::Comparisons => "comparisons".into(),
        WorthQueryStructuralCounterUnit::Iterations => "iterations".into(),
        WorthQueryStructuralCounterUnit::Neighborhoods => "neighborhoods".into(),
        WorthQueryStructuralCounterUnit::Domain(identity) => format!("domain:{identity}"),
    }
}

fn counter_aggregation_name(value: &WorthQueryStructuralCounterAggregation) -> &'static str {
    match value {
        WorthQueryStructuralCounterAggregation::Independent => "independent",
        WorthQueryStructuralCounterAggregation::SumOf(_) => "sum-of",
        WorthQueryStructuralCounterAggregation::MaximumOf(_) => "maximum-of",
        WorthQueryStructuralCounterAggregation::MinimumOf(_) => "minimum-of",
    }
}

fn counter_monotonicity_name(value: WorthQueryStructuralCounterMonotonicity) -> &'static str {
    match value {
        WorthQueryStructuralCounterMonotonicity::Unconstrained => "unconstrained",
        WorthQueryStructuralCounterMonotonicity::NonDecreasing => "non-decreasing",
    }
}

fn counter_scope_name(value: WorthQueryStructuralCounterScope) -> &'static str {
    match value {
        WorthQueryStructuralCounterScope::Operation => "operation",
        WorthQueryStructuralCounterScope::Run => "run",
        WorthQueryStructuralCounterScope::Stage => "stage",
        WorthQueryStructuralCounterScope::Attempt => "attempt",
        WorthQueryStructuralCounterScope::ArtifactOccurrence => "artifact-occurrence",
    }
}

fn counter_reset_name(value: WorthQueryStructuralCounterResetBoundary) -> &'static str {
    match value {
        WorthQueryStructuralCounterResetBoundary::Operation => "operation",
        WorthQueryStructuralCounterResetBoundary::Run => "run",
        WorthQueryStructuralCounterResetBoundary::Stage => "stage",
        WorthQueryStructuralCounterResetBoundary::Attempt => "attempt",
        WorthQueryStructuralCounterResetBoundary::ArtifactOccurrence => "artifact-occurrence",
    }
}

fn counter_requiredness_name(value: WorthQueryStructuralCounterRequiredness) -> &'static str {
    match value {
        WorthQueryStructuralCounterRequiredness::RequiredCore => "required-core",
        WorthQueryStructuralCounterRequiredness::OptionalSidecar => "optional-sidecar",
    }
}

fn counter_replay_name(value: WorthQueryStructuralCounterReplayPosture) -> &'static str {
    match value {
        WorthQueryStructuralCounterReplayPosture::Exact => "exact",
        WorthQueryStructuralCounterReplayPosture::NonDecreasing => "non-decreasing",
        WorthQueryStructuralCounterReplayPosture::ProviderCertified => "provider-certified",
        WorthQueryStructuralCounterReplayPosture::NotCompared => "not-compared",
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
