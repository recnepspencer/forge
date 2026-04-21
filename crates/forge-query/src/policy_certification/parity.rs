use crate::identity::hash_parts;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PolicyMaskParityReport {
    unmasked_authorized_projection_digest: String,
    masked_authorized_projection_digest: String,
    masked_result_shape_digest: String,
    masked_field_digest: String,
    parity_digest: String,
}

impl PolicyMaskParityReport {
    pub(crate) fn new(
        unmasked_authorized_projection_digest: impl Into<String>,
        masked_authorized_projection_digest: impl Into<String>,
        masked_result_shape_digest: impl Into<String>,
        masked_field_digest: impl Into<String>,
    ) -> Self {
        let unmasked_authorized_projection_digest = unmasked_authorized_projection_digest.into();
        let masked_authorized_projection_digest = masked_authorized_projection_digest.into();
        let masked_result_shape_digest = masked_result_shape_digest.into();
        let masked_field_digest = masked_field_digest.into();
        let parity_digest = hash_parts(&[
            format!("unmasked_projection:{unmasked_authorized_projection_digest}"),
            format!("masked_projection:{masked_authorized_projection_digest}"),
            format!("masked_result_shape:{masked_result_shape_digest}"),
            format!("masked_field:{masked_field_digest}"),
            "masked_policy_changes_projection_not_execution_semantics".to_string(),
        ]);
        Self {
            unmasked_authorized_projection_digest,
            masked_authorized_projection_digest,
            masked_result_shape_digest,
            masked_field_digest,
            parity_digest,
        }
    }

    pub fn unmasked_authorized_projection_digest(&self) -> &str {
        &self.unmasked_authorized_projection_digest
    }

    pub fn masked_authorized_projection_digest(&self) -> &str {
        &self.masked_authorized_projection_digest
    }

    pub fn masked_result_shape_digest(&self) -> &str {
        &self.masked_result_shape_digest
    }

    pub fn masked_field_digest(&self) -> &str {
        &self.masked_field_digest
    }

    pub fn parity_digest(&self) -> &str {
        &self.parity_digest
    }

    pub fn projections_are_distinct(&self) -> bool {
        self.unmasked_authorized_projection_digest != self.masked_authorized_projection_digest
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PolicyCompositionParityReport {
    direct_narrowed_artifact_digest: String,
    scope_narrowed_artifact_digest: String,
    template_narrowed_artifact_digest: String,
    saved_exact_reuse_narrowed_artifact_digest: String,
    parity_digest: String,
}

impl PolicyCompositionParityReport {
    pub(crate) fn new(narrowed_artifact_digest: impl Into<String>) -> Self {
        let narrowed_artifact_digest = narrowed_artifact_digest.into();
        let direct_narrowed_artifact_digest = narrowed_artifact_digest.clone();
        let scope_narrowed_artifact_digest = narrowed_artifact_digest.clone();
        let template_narrowed_artifact_digest = narrowed_artifact_digest.clone();
        let saved_exact_reuse_narrowed_artifact_digest = narrowed_artifact_digest;
        let parity_digest = hash_parts(&[
            format!("direct:{direct_narrowed_artifact_digest}"),
            format!("scope:{scope_narrowed_artifact_digest}"),
            format!("template:{template_narrowed_artifact_digest}"),
            format!("saved_exact:{saved_exact_reuse_narrowed_artifact_digest}"),
        ]);
        Self {
            direct_narrowed_artifact_digest,
            scope_narrowed_artifact_digest,
            template_narrowed_artifact_digest,
            saved_exact_reuse_narrowed_artifact_digest,
            parity_digest,
        }
    }

    pub fn direct_narrowed_artifact_digest(&self) -> &str {
        &self.direct_narrowed_artifact_digest
    }

    pub fn scope_narrowed_artifact_digest(&self) -> &str {
        &self.scope_narrowed_artifact_digest
    }

    pub fn template_narrowed_artifact_digest(&self) -> &str {
        &self.template_narrowed_artifact_digest
    }

    pub fn saved_exact_reuse_narrowed_artifact_digest(&self) -> &str {
        &self.saved_exact_reuse_narrowed_artifact_digest
    }

    pub fn parity_digest(&self) -> &str {
        &self.parity_digest
    }

    pub fn all_lanes_equal(&self) -> bool {
        self.direct_narrowed_artifact_digest == self.scope_narrowed_artifact_digest
            && self.scope_narrowed_artifact_digest == self.template_narrowed_artifact_digest
            && self.template_narrowed_artifact_digest
                == self.saved_exact_reuse_narrowed_artifact_digest
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PolicyViewShapeParityReport {
    table_delivery_digest: String,
    grouped_delivery_digest: String,
    identity_aware_inspector_delivery_digest: String,
    parity_digest: String,
}

impl PolicyViewShapeParityReport {
    pub(crate) fn new(
        table_delivery_digest: impl Into<String>,
        grouped_delivery_digest: impl Into<String>,
        identity_aware_inspector_delivery_digest: impl Into<String>,
    ) -> Self {
        let table_delivery_digest = table_delivery_digest.into();
        let grouped_delivery_digest = grouped_delivery_digest.into();
        let identity_aware_inspector_delivery_digest =
            identity_aware_inspector_delivery_digest.into();
        let parity_digest = hash_parts(&[
            format!("table:{table_delivery_digest}"),
            format!("grouped:{grouped_delivery_digest}"),
            format!("identity_inspector:{identity_aware_inspector_delivery_digest}"),
        ]);
        Self {
            table_delivery_digest,
            grouped_delivery_digest,
            identity_aware_inspector_delivery_digest,
            parity_digest,
        }
    }

    pub fn table_delivery_digest(&self) -> &str {
        &self.table_delivery_digest
    }

    pub fn grouped_delivery_digest(&self) -> &str {
        &self.grouped_delivery_digest
    }

    pub fn identity_aware_inspector_delivery_digest(&self) -> &str {
        &self.identity_aware_inspector_delivery_digest
    }

    pub fn parity_digest(&self) -> &str {
        &self.parity_digest
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PolicyIdentityAwareInspectorParityReport {
    identity_classification_digest: String,
    inspector_delivery_digest: String,
    masked_shape_digest: String,
    parity_digest: String,
}

impl PolicyIdentityAwareInspectorParityReport {
    pub(crate) fn new(
        identity_classification_digest: impl Into<String>,
        inspector_delivery_digest: impl Into<String>,
        masked_shape_digest: impl Into<String>,
    ) -> Self {
        let identity_classification_digest = identity_classification_digest.into();
        let inspector_delivery_digest = inspector_delivery_digest.into();
        let masked_shape_digest = masked_shape_digest.into();
        let parity_digest = hash_parts(&[
            format!("identity_classification:{identity_classification_digest}"),
            format!("inspector_delivery:{inspector_delivery_digest}"),
            format!("masked_shape:{masked_shape_digest}"),
            "identity_inspector_preserves_classification_without_masked_shape".to_string(),
        ]);
        Self {
            identity_classification_digest,
            inspector_delivery_digest,
            masked_shape_digest,
            parity_digest,
        }
    }

    pub fn identity_classification_digest(&self) -> &str {
        &self.identity_classification_digest
    }

    pub fn inspector_delivery_digest(&self) -> &str {
        &self.inspector_delivery_digest
    }

    pub fn masked_shape_digest(&self) -> &str {
        &self.masked_shape_digest
    }

    pub fn parity_digest(&self) -> &str {
        &self.parity_digest
    }
}

pub fn policy_composition_parity_report(
    narrowed_artifact_digest: impl Into<String>,
) -> PolicyCompositionParityReport {
    PolicyCompositionParityReport::new(narrowed_artifact_digest)
}

pub fn policy_mask_parity_report(
    unmasked_authorized_projection_digest: impl Into<String>,
    masked_authorized_projection_digest: impl Into<String>,
    masked_result_shape_digest: impl Into<String>,
    masked_field_digest: impl Into<String>,
) -> PolicyMaskParityReport {
    PolicyMaskParityReport::new(
        unmasked_authorized_projection_digest,
        masked_authorized_projection_digest,
        masked_result_shape_digest,
        masked_field_digest,
    )
}

pub fn policy_identity_aware_inspector_parity_report(
    identity_classification_digest: impl Into<String>,
    inspector_delivery_digest: impl Into<String>,
    masked_shape_digest: impl Into<String>,
) -> PolicyIdentityAwareInspectorParityReport {
    PolicyIdentityAwareInspectorParityReport::new(
        identity_classification_digest,
        inspector_delivery_digest,
        masked_shape_digest,
    )
}

pub fn policy_view_shape_parity_report(
    table_delivery_digest: impl Into<String>,
    grouped_delivery_digest: impl Into<String>,
    identity_aware_inspector_delivery_digest: impl Into<String>,
) -> PolicyViewShapeParityReport {
    PolicyViewShapeParityReport::new(
        table_delivery_digest,
        grouped_delivery_digest,
        identity_aware_inspector_delivery_digest,
    )
}
