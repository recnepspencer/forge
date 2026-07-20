use sha2::Sha256;

use crate::canonical_hash_encoding::hash_text_field;

use super::{bool_name, hash_sequence};
use crate::domain_operation::*;

pub(super) fn hash_lifecycle_and_support_contracts(
    hasher: &mut Sha256,
    semantics: &WorthQueryDomainOperationSemanticClosure,
) {
    hash_text_field(hasher, "replay", replay_name(semantics.replay));
    hash_reversal(hasher, &semantics.reversal);
    hash_text_field(hasher, "lineage", lineage_name(semantics.lineage));
    hash_text_field(hasher, "promotion", promotion_name(semantics.promotion));
    hash_publication(hasher, &semantics.publication);
    hash_text_field(
        hasher,
        "projection-consumption",
        match semantics.projection_consumption {
            WorthQueryOperationProjectionConsumptionContract::NotRequired => "not-required",
            WorthQueryOperationProjectionConsumptionContract::QueryReadAuthority => {
                "query-read-authority"
            }
        },
    );
    hash_terminal(hasher, &semantics.terminal);
    hash_cost(hasher, semantics.cost);
    hash_support(hasher, semantics.support);
    hash_text_field(hasher, "lowering-family", &semantics.lowering.family);
    hash_text_field(
        hasher,
        "lowering-deterministic",
        bool_name(semantics.lowering.deterministic),
    );
}

fn hash_reversal(hasher: &mut Sha256, contract: &WorthQueryOperationReversalContract) {
    match contract {
        WorthQueryOperationReversalContract::Irreversible => {
            hash_text_field(hasher, "reversal", "irreversible");
        }
        WorthQueryOperationReversalContract::ProvisionalDiscard => {
            hash_text_field(hasher, "reversal", "provisional-discard");
        }
        WorthQueryOperationReversalContract::ExactInverse { lowering_family } => {
            hash_text_field(hasher, "reversal", "exact-inverse");
            hash_text_field(hasher, "reversal-lowering", lowering_family);
        }
        WorthQueryOperationReversalContract::Compensation { operation } => {
            hash_text_field(hasher, "reversal", "compensation");
            hash_text_field(hasher, "compensation-operation", operation);
        }
        WorthQueryOperationReversalContract::RebuildRequired { recovery_family } => {
            hash_text_field(hasher, "reversal", "rebuild-required");
            hash_text_field(hasher, "recovery-family", recovery_family);
        }
    }
}

fn hash_publication(hasher: &mut Sha256, contract: &WorthQueryOperationPublicationContract) {
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

fn hash_terminal(hasher: &mut Sha256, contract: &WorthQueryOperationTerminalContract) {
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

fn hash_cost(hasher: &mut Sha256, contract: WorthQueryOperationCostContract) {
    hash_text_field(hasher, "lookup-cost", cost_name(contract.lookup));
    hash_text_field(hasher, "execution-cost", cost_name(contract.execution));
    hash_text_field(
        hasher,
        "result-width-cost",
        cost_name(contract.result_width),
    );
}

fn hash_support(hasher: &mut Sha256, support: WorthQueryOperationSupportRequirements) {
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

fn replay_name(posture: WorthQueryOperationReplayContract) -> &'static str {
    match posture {
        WorthQueryOperationReplayContract::NotSupported => "not-supported",
        WorthQueryOperationReplayContract::ReExecutable => "re-executable",
        WorthQueryOperationReplayContract::CertReplayable => "cert-replayable",
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
