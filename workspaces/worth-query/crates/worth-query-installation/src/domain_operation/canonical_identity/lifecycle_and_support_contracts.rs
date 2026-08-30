use crate::canonical_hash_encoding::CanonicalHashSink;

use crate::canonical_hash_encoding::hash_text_field;

use super::{bool_name, hash_sequence, WorthQueryDomainOperationCanonicalSemantics};
use crate::domain_operation::*;

pub(super) fn hash_lifecycle_and_support_contracts(
    hasher: &mut impl CanonicalHashSink,
    semantics: &impl WorthQueryDomainOperationCanonicalSemantics,
) {
    hash_replay(hasher, semantics.replay());
    hash_aftermath(hasher, semantics.aftermath());
    hash_text_field(hasher, "lineage", lineage_name(semantics.lineage()));
    hash_text_field(hasher, "promotion", promotion_name(semantics.promotion()));
    hash_publication(hasher, semantics.publication());
    hash_text_field(
        hasher,
        "projection-consumption",
        match semantics.projection_consumption() {
            WorthQueryOperationProjectionConsumptionContract::NotRequired => "not-required",
            WorthQueryOperationProjectionConsumptionContract::QueryReadAuthority => {
                "query-read-authority"
            }
        },
    );
    hash_terminal(hasher, semantics.terminal());
    hash_cost(hasher, semantics.cost());
    hash_support(hasher, semantics.support());
    hash_text_field(hasher, "lowering-family", &semantics.lowering().family);
    hash_text_field(
        hasher,
        "lowering-deterministic",
        bool_name(semantics.lowering().deterministic),
    );
}

fn hash_aftermath(
    hasher: &mut impl CanonicalHashSink,
    contract: Option<&crate::application_aftermath::WorthQueryInstalledAftermathContract>,
) {
    match contract {
        None => hash_text_field(hasher, "aftermath", "none"),
        Some(contract) => {
            hash_text_field(hasher, "aftermath", "installed");
            hasher.write(&32u64.to_le_bytes());
            hasher.write(contract.identity().bytes());
            hash_text_field(hasher, "aftermath-operation", contract.operation_slot());
            hash_text_field(
                hasher,
                "aftermath-posture",
                match contract.published_posture() {
                    crate::application_aftermath::PublishedAftermathPosture::Reversible => {
                        "reversible"
                    }
                    crate::application_aftermath::PublishedAftermathPosture::Compensatable => {
                        "compensatable"
                    }
                    crate::application_aftermath::PublishedAftermathPosture::Reconcilable => {
                        "reconcilable"
                    }
                    crate::application_aftermath::PublishedAftermathPosture::Irreversible => {
                        "irreversible"
                    }
                },
            );
        }
    }
}

fn hash_publication(
    hasher: &mut impl CanonicalHashSink,
    contract: &WorthQueryOperationPublicationContract,
) {
    match contract {
        WorthQueryOperationPublicationContract::NotRequired => {
            hash_text_field(hasher, "publication", "not-required");
        }
        WorthQueryOperationPublicationContract::DerivedProjection { projection_role } => {
            hash_text_field(hasher, "publication", "derived-projection");
            hash_text_field(hasher, "publication-role", projection_role.as_str());
        }
    }
}

fn hash_terminal(
    hasher: &mut impl CanonicalHashSink,
    contract: &WorthQueryOperationTerminalContract,
) {
    hash_sequence(
        hasher,
        "result-state",
        contract
            .result_states
            .iter()
            .map(|state| result_state_name(*state)),
    );
    for failure in &contract.failure_classes {
        match failure {
            WorthQueryOperationFailureClass::Domain(name) => {
                hash_text_field(hasher, "failure-class", "domain");
                hash_text_field(hasher, "domain-failure", name);
            }
            failure => hash_text_field(hasher, "failure-class", failure_name(failure)),
        }
    }
}

fn hash_cost(hasher: &mut impl CanonicalHashSink, contract: WorthQueryOperationCostContract) {
    hash_text_field(hasher, "lookup-cost", cost_name(contract.lookup));
    hash_text_field(hasher, "execution-cost", cost_name(contract.execution));
    hash_text_field(
        hasher,
        "result-width-cost",
        cost_name(contract.result_width),
    );
}

fn hash_support(
    hasher: &mut impl CanonicalHashSink,
    support: WorthQueryOperationSupportRequirements,
) {
    for (dimension, requirement) in [
        ("live", support.live),
        ("continuation", support.continuation),
        ("async-result-state", support.async_result_state),
        ("recovery", support.recovery),
        ("inspection", support.inspection),
        ("projection-consumption", support.projection_consumption),
        ("dependency-impact", support.dependency_impact),
        ("sharing", support.sharing),
        ("invalidation", support.invalidation),
        ("collection-delivery", support.collection_delivery),
        ("conditional-evaluation", support.conditional_evaluation),
        ("conditional-comparator", support.conditional_comparator),
        ("conditional-trigger", support.conditional_trigger),
        (
            "conditional-temporal-or-on-demand",
            support.conditional_temporal_or_on_demand,
        ),
    ] {
        hash_text_field(hasher, dimension, support_name(requirement));
    }
}

fn hash_replay(hasher: &mut impl CanonicalHashSink, posture: &WorthQueryOperationReplayContract) {
    match posture {
        WorthQueryOperationReplayContract::NotSupported => {
            hash_text_field(hasher, "replay", "not-supported");
        }
        WorthQueryOperationReplayContract::ReExecutable => {
            hash_text_field(hasher, "replay", "re-executable");
        }
        WorthQueryOperationReplayContract::CertReplayable { comparator } => {
            hash_text_field(hasher, "replay", "cert-replayable");
            hash_text_field(hasher, "replay-comparator-family", comparator.family());
        }
        WorthQueryOperationReplayContract::CertReplayableWithNoise { comparator, noise } => {
            hash_text_field(hasher, "replay", "cert-replayable-with-noise");
            hash_text_field(hasher, "replay-comparator-family", comparator.family());
            hash_text_field(
                hasher,
                "replay-diagnostic-warnings",
                bool_name(noise.diagnostic_warnings),
            );
        }
    }
}

fn lineage_name(posture: WorthQueryOperationLineageContract) -> &'static str {
    match posture {
        WorthQueryOperationLineageContract::NotRequired => "not-required",
        WorthQueryOperationLineageContract::Preserve => "preserve",
        WorthQueryOperationLineageContract::Evolve => "evolve",
    }
}

fn promotion_name(posture: WorthQueryOperationPromotionContract) -> &'static str {
    match posture {
        WorthQueryOperationPromotionContract::NotRequired => "not-required",
        WorthQueryOperationPromotionContract::OnDurableReference => "on-durable-reference",
    }
}

fn result_state_name(state: WorthQueryOperationResultState) -> &'static str {
    match state {
        WorthQueryOperationResultState::Ready => "ready",
        WorthQueryOperationResultState::Advisory => "advisory",
        WorthQueryOperationResultState::Pending => "pending",
        WorthQueryOperationResultState::Partial => "partial",
        WorthQueryOperationResultState::Violation => "violation",
    }
}

fn failure_name(failure: &WorthQueryOperationFailureClass) -> &'static str {
    match failure {
        WorthQueryOperationFailureClass::InvalidInput => "invalid-input",
        WorthQueryOperationFailureClass::Unsupported => "unsupported",
        WorthQueryOperationFailureClass::Conflict => "conflict",
        WorthQueryOperationFailureClass::Dependency => "dependency",
        WorthQueryOperationFailureClass::Indeterminate => "indeterminate",
        WorthQueryOperationFailureClass::Domain(_) => "domain",
    }
}

fn cost_name(cost: WorthQueryOperationCostClass) -> &'static str {
    match cost {
        WorthQueryOperationCostClass::Constant => "constant",
        WorthQueryOperationCostClass::DeclaredWidth => "declared-width",
        WorthQueryOperationCostClass::GraphBreadth => "graph-breadth",
        WorthQueryOperationCostClass::ExternalBoundary => "external-boundary",
    }
}

fn support_name(requirement: WorthQuerySupportRequirement) -> &'static str {
    match requirement {
        WorthQuerySupportRequirement::NotRequired => "not-required",
        WorthQuerySupportRequirement::Required => "required",
    }
}
