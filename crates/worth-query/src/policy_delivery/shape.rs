use crate::authorized_projection::AuthorizedProjectionFieldPath;
use crate::identity::hash_parts;
use crate::policy_execution_seam::{
    PolicyAwareExecutionMode, PolicyAwareExecutionSeam, PolicyAwareExecutionSeamError,
    PolicyAwareExecutionSeamFailureClass, PolicyAwareSeamCounters,
};
use crate::policy_narrowing::NarrowedPolicyQueryArtifact;

use super::DeliveryWidthClass;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PolicyAwareDeliveryDigest(String);

impl PolicyAwareDeliveryDigest {
    pub(crate) fn new(value: String) -> Self {
        Self(value)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PolicyAwareDeliveryReport {
    digest: String,
    width_class: DeliveryWidthClass,
    delivery_width: usize,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PolicyPlaceholderMaskingRequest {
    requested_placeholder_fields: Vec<AuthorizedProjectionFieldPath>,
}

impl PolicyPlaceholderMaskingRequest {
    pub fn from_authorized_field_paths(
        requested_placeholder_fields: Vec<AuthorizedProjectionFieldPath>,
    ) -> Self {
        Self {
            requested_placeholder_fields,
        }
    }

    pub fn requested_placeholder_field_paths(&self) -> &[AuthorizedProjectionFieldPath] {
        &self.requested_placeholder_fields
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PolicyPlaceholderMaskingDenial {
    requested_placeholder_fields: Vec<AuthorizedProjectionFieldPath>,
    failure_digest: String,
}

impl PolicyPlaceholderMaskingDenial {
    pub fn requested_placeholder_field_paths(&self) -> &[AuthorizedProjectionFieldPath] {
        &self.requested_placeholder_fields
    }

    pub fn failure_digest(&self) -> &str {
        &self.failure_digest
    }
}

impl PolicyAwareDeliveryReport {
    pub fn digest(&self) -> &str {
        &self.digest
    }

    pub fn width_class(&self) -> DeliveryWidthClass {
        self.width_class
    }

    pub fn delivery_width(&self) -> usize {
        self.delivery_width
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PolicyAwareDeliveryShape {
    digest: PolicyAwareDeliveryDigest,
    seam: PolicyAwareExecutionSeam,
    narrowed_result_shape_digest: String,
    delivered_fields: Vec<AuthorizedProjectionFieldPath>,
    width_class: DeliveryWidthClass,
    report: PolicyAwareDeliveryReport,
}

impl PolicyAwareDeliveryShape {
    pub fn digest(&self) -> &PolicyAwareDeliveryDigest {
        &self.digest
    }

    pub fn seam(&self) -> &PolicyAwareExecutionSeam {
        &self.seam
    }

    pub fn narrowed_result_shape_digest(&self) -> &str {
        &self.narrowed_result_shape_digest
    }

    pub fn delivered_field_paths(&self) -> &[AuthorizedProjectionFieldPath] {
        &self.delivered_fields
    }

    pub fn width_class(&self) -> DeliveryWidthClass {
        self.width_class
    }

    pub fn report(&self) -> &PolicyAwareDeliveryReport {
        &self.report
    }
}

pub fn lower_policy_aware_delivery_shape(
    artifact: &NarrowedPolicyQueryArtifact,
    width_class: DeliveryWidthClass,
) -> Result<PolicyAwareDeliveryShape, PolicyAwareExecutionSeamError> {
    let delivered_fields = artifact
        .authorized_projection()
        .visible_field_paths()
        .to_vec();
    let delivery_width = delivered_fields.len();
    if width_class == DeliveryWidthClass::DeniedWidthInflation
        || delivery_width > width_class.budget_limit()
    {
        return Err(PolicyAwareExecutionSeamError::new(
            PolicyAwareExecutionSeamFailureClass::DeliveryShapeOverexposure,
            "policy-aware delivery width must be admitted before payload emission",
            PolicyAwareSeamCounters::denied_delivery_overexposure(),
        ));
    }
    if delivered_fields.iter().any(|field| {
        artifact
            .authorized_projection()
            .masked_projection()
            .masked_field_paths()
            .contains(field)
    }) {
        return Err(PolicyAwareExecutionSeamError::new(
            PolicyAwareExecutionSeamFailureClass::DeliveryShapeOverexposure,
            "policy-aware delivery cannot expose masked placeholder fields",
            PolicyAwareSeamCounters::denied_delivery_overexposure(),
        ));
    }
    let counters = PolicyAwareSeamCounters::admitted(
        artifact.authorized_projection().visible_field_paths().len(),
        artifact.relationship_proof().topology_classes().len(),
        delivery_width,
        0,
        12,
    );
    let seam = PolicyAwareExecutionSeam::from_narrowed(
        artifact,
        PolicyAwareExecutionMode::DeliveryShape,
        counters,
    );
    let digest = PolicyAwareDeliveryDigest::new(hash_parts(&[
        format!("seam:{}", seam.identity().as_str()),
        format!("narrowed_shape:{}", artifact.narrowed_result_shape_digest()),
        format!("width_class:{}", width_class.as_str()),
        format!(
            "fields:{}",
            hash_parts(&terminal_field_projections(&delivered_fields))
        ),
    ]));
    let report = PolicyAwareDeliveryReport {
        digest: hash_parts(&[
            format!("delivery:{}", digest.as_str()),
            format!("width_class:{}", width_class.as_str()),
            format!("width:{delivery_width}"),
        ]),
        width_class,
        delivery_width,
    };
    Ok(PolicyAwareDeliveryShape {
        digest,
        seam,
        narrowed_result_shape_digest: artifact.narrowed_result_shape_digest().to_string(),
        delivered_fields,
        width_class,
        report,
    })
}

pub fn deny_policy_placeholder_masking(
    artifact: &NarrowedPolicyQueryArtifact,
    request: PolicyPlaceholderMaskingRequest,
) -> Result<PolicyPlaceholderMaskingDenial, PolicyAwareExecutionSeamError> {
    let masked_fields = artifact
        .authorized_projection()
        .masked_projection()
        .masked_field_paths();
    if request
        .requested_placeholder_field_paths()
        .iter()
        .any(|requested| masked_fields.iter().any(|masked| masked == requested))
    {
        return Err(PolicyAwareExecutionSeamError::new(
            PolicyAwareExecutionSeamFailureClass::PlaceholderMaskingForbidden,
            "masked fields cannot be preserved as caller-visible placeholder delivery fields",
            PolicyAwareSeamCounters::denied_placeholder_masking(),
        ));
    }

    Ok(PolicyPlaceholderMaskingDenial {
        failure_digest: hash_parts(&[
            format!("narrowed:{}", artifact.digest()),
            format!(
                "placeholder_fields:{}",
                hash_parts(&terminal_field_projections(
                    request.requested_placeholder_field_paths()
                ))
            ),
            "no_placeholder_masking_denial".to_string(),
        ]),
        requested_placeholder_fields: request.requested_placeholder_fields,
    })
}

fn terminal_field_projections(fields: &[AuthorizedProjectionFieldPath]) -> Vec<String> {
    fields
        .iter()
        .map(|field| field.terminal_projection_for_boundary().to_string())
        .collect()
}
