use crate::canonical_hash_encoding::CanonicalHashSink;

use crate::canonical_hash_encoding::hash_text_field;
use crate::domain_computation::{
    WorthQueryArtifactClassification, WorthQueryDecisionCausalParentShape,
    WorthQueryDecisionRecordContract,
};

pub(crate) fn hash_decision_record_contract(
    hash: &mut impl CanonicalHashSink,
    contract: &WorthQueryDecisionRecordContract,
) {
    match contract {
        WorthQueryDecisionRecordContract::NotRequired => {
            hash_text_field(hash, "decision-record", "not-required");
        }
        WorthQueryDecisionRecordContract::Declared { schemas } => {
            hash_text_field(hash, "decision-record", "declared");
            for schema in schemas {
                hash_text_field(hash, "decision-kind", schema.kind().as_str());
                hash_text_field(
                    hash,
                    "decision-reason-family",
                    schema.reason_family().as_str(),
                );
                hash_text_field(
                    hash,
                    "decision-artifact-key-family",
                    schema.affected_artifact_key_family().as_str(),
                );
                hash_text_field(
                    hash,
                    "decision-causal-parent",
                    causal_parent_name(schema.causal_parent()),
                );
                hash_text_field(
                    hash,
                    "decision-payload-version",
                    &schema.payload_version().get().to_string(),
                );
                hash_text_field(
                    hash,
                    "decision-classification",
                    classification_name(schema.classification()),
                );
                hash_text_field(
                    hash,
                    "decision-retention",
                    retention_name(schema.retention()),
                );
            }
        }
    }
}

fn causal_parent_name(value: WorthQueryDecisionCausalParentShape) -> &'static str {
    match value {
        WorthQueryDecisionCausalParentShape::None => "none",
        WorthQueryDecisionCausalParentShape::OptionalSingle => "optional-single",
        WorthQueryDecisionCausalParentShape::RequiredSingle => "required-single",
        WorthQueryDecisionCausalParentShape::OrderedMany => "ordered-many",
    }
}

fn classification_name(value: WorthQueryArtifactClassification) -> &'static str {
    match value {
        WorthQueryArtifactClassification::Public => "public",
        WorthQueryArtifactClassification::Internal => "internal",
        WorthQueryArtifactClassification::Confidential => "confidential",
        WorthQueryArtifactClassification::Restricted => "restricted",
    }
}

fn retention_name(value: worth_foundational::facade::RetentionDeliveryProfile) -> &'static str {
    match value {
        worth_foundational::facade::RetentionDeliveryProfile::Ephemeral => "ephemeral",
        worth_foundational::facade::RetentionDeliveryProfile::Retained => "retained",
        worth_foundational::facade::RetentionDeliveryProfile::Durable => "durable",
    }
}
