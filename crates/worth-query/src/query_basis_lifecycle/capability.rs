use super::identity::basis_lifecycle_digest;
use crate::evidence_identity::{
    worth_query_evidence_identity, WorthQueryEvidenceIdentity, WorthQueryEvidenceScope,
    WorthQueryEvidenceTag,
};

use super::{
    denied_basis_capability_for_lane_mismatch, lifecycle_posture_for_admission,
    permitted_lanes_for_family, placeholders_for_lane, visibility_for_family,
    BasisAuthorityPosture, BasisEligibility, BasisEligibilityCounters, BasisEligibilityDisposition,
    BasisLifecyclePosture, BasisOperationLaneRequest, BasisTenantSchemaPosture, BasisVisibility,
    DeniedBasisCapability, NormalizedBasisFamily, NormalizedBasisSubject,
    UnboundLowerRuntimeEvidencePlaceholder,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AdmittedBasisCapability {
    normalized_basis_intent_digest: String,
    family: NormalizedBasisFamily,
    authority_posture: BasisAuthorityPosture,
    scope_subject: NormalizedBasisSubject,
    scope_label: String,
    visibility: BasisVisibility,
    lifecycle_posture: BasisLifecyclePosture,
    operation_lane: BasisOperationLaneRequest,
    tenant_schema_posture: BasisTenantSchemaPosture,
    lower_runtime_evidence_placeholders: &'static [UnboundLowerRuntimeEvidencePlaceholder],
    permitted_lanes: &'static [BasisOperationLaneRequest],
    counters: BasisEligibilityCounters,
    capability_digest: String,
}

impl AdmittedBasisCapability {
    pub fn normalized_basis_intent_digest(&self) -> &str {
        &self.normalized_basis_intent_digest
    }

    pub fn family(&self) -> &NormalizedBasisFamily {
        &self.family
    }

    pub fn authority_posture(&self) -> &BasisAuthorityPosture {
        &self.authority_posture
    }

    pub fn scope_subject(&self) -> &NormalizedBasisSubject {
        &self.scope_subject
    }

    pub fn scope_label(&self) -> &str {
        &self.scope_label
    }

    pub fn visibility(&self) -> BasisVisibility {
        self.visibility
    }

    pub fn lifecycle_posture(&self) -> BasisLifecyclePosture {
        self.lifecycle_posture
    }

    pub fn operation_lane(&self) -> &BasisOperationLaneRequest {
        &self.operation_lane
    }

    pub fn tenant_schema_posture(&self) -> &BasisTenantSchemaPosture {
        &self.tenant_schema_posture
    }

    pub fn lower_runtime_evidence_placeholders(
        &self,
    ) -> &'static [UnboundLowerRuntimeEvidencePlaceholder] {
        self.lower_runtime_evidence_placeholders
    }

    pub fn permitted_lanes(&self) -> &'static [BasisOperationLaneRequest] {
        self.permitted_lanes
    }

    pub fn counters(&self) -> &BasisEligibilityCounters {
        &self.counters
    }

    pub fn capability_digest(&self) -> &str {
        &self.capability_digest
    }

    pub fn snapshot_basis_identity(&self) -> WorthQueryEvidenceIdentity {
        compose_admitted_capability_snapshot_basis_identity(self)
    }

    pub fn snapshot_result_shape_identity(&self) -> WorthQueryEvidenceIdentity {
        compose_admitted_capability_snapshot_result_shape_identity(self)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AdvisoryBasisCapability {
    normalized_basis_intent_digest: String,
    family: NormalizedBasisFamily,
    authority_posture: BasisAuthorityPosture,
    scope_subject: NormalizedBasisSubject,
    scope_label: String,
    visibility: BasisVisibility,
    lifecycle_posture: BasisLifecyclePosture,
    operation_lane: BasisOperationLaneRequest,
    tenant_schema_posture: BasisTenantSchemaPosture,
    lower_runtime_evidence_placeholders: &'static [UnboundLowerRuntimeEvidencePlaceholder],
    permitted_lanes: &'static [BasisOperationLaneRequest],
    counters: BasisEligibilityCounters,
    advisory_digest: String,
}

impl AdvisoryBasisCapability {
    pub fn normalized_basis_intent_digest(&self) -> &str {
        &self.normalized_basis_intent_digest
    }

    pub fn family(&self) -> &NormalizedBasisFamily {
        &self.family
    }

    pub fn authority_posture(&self) -> &BasisAuthorityPosture {
        &self.authority_posture
    }

    pub fn scope_subject(&self) -> &NormalizedBasisSubject {
        &self.scope_subject
    }

    pub fn scope_label(&self) -> &str {
        &self.scope_label
    }

    pub fn visibility(&self) -> BasisVisibility {
        self.visibility
    }

    pub fn lifecycle_posture(&self) -> BasisLifecyclePosture {
        self.lifecycle_posture
    }

    pub fn operation_lane(&self) -> &BasisOperationLaneRequest {
        &self.operation_lane
    }

    pub fn tenant_schema_posture(&self) -> &BasisTenantSchemaPosture {
        &self.tenant_schema_posture
    }

    pub fn lower_runtime_evidence_placeholders(
        &self,
    ) -> &'static [UnboundLowerRuntimeEvidencePlaceholder] {
        self.lower_runtime_evidence_placeholders
    }

    pub fn permitted_lanes(&self) -> &'static [BasisOperationLaneRequest] {
        self.permitted_lanes
    }

    pub fn counters(&self) -> &BasisEligibilityCounters {
        &self.counters
    }

    pub fn advisory_digest(&self) -> &str {
        &self.advisory_digest
    }

    pub fn snapshot_basis_identity(&self) -> WorthQueryEvidenceIdentity {
        compose_advisory_capability_snapshot_basis_identity(self)
    }

    pub fn snapshot_result_shape_identity(&self) -> WorthQueryEvidenceIdentity {
        compose_advisory_capability_snapshot_result_shape_identity(self)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum BasisCapabilityAdmission {
    Admitted(AdmittedBasisCapability),
    Advisory(AdvisoryBasisCapability),
}

impl BasisCapabilityAdmission {
    pub fn operation_lane(&self) -> &BasisOperationLaneRequest {
        match self {
            Self::Admitted(capability) => capability.operation_lane(),
            Self::Advisory(capability) => capability.operation_lane(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ObservationBasisCapability {
    admission: BasisCapabilityAdmission,
}

impl ObservationBasisCapability {
    pub fn admission(&self) -> &BasisCapabilityAdmission {
        &self.admission
    }
}

macro_rules! define_lane_wrapper {
    ($name:ident) => {
        #[derive(Clone, Debug, Eq, PartialEq)]
        pub struct $name {
            admission: BasisCapabilityAdmission,
        }

        impl $name {
            pub fn admission(&self) -> &BasisCapabilityAdmission {
                &self.admission
            }
        }
    };
}

define_lane_wrapper!(MutationPreparationBasisCapability);
define_lane_wrapper!(ReplayBasisCapability);
define_lane_wrapper!(InspectionBasisCapability);
define_lane_wrapper!(MaterializationBasisCapability);
define_lane_wrapper!(SubscriptionDeclarationBasisCapability);
define_lane_wrapper!(SubscriptionActivationBasisCapability);
define_lane_wrapper!(PreviewCloseoutBasisCapability);
define_lane_wrapper!(CertificationBasisCapability);

pub fn admit_basis_capability(eligibility: BasisEligibility) -> BasisCapabilityAdmission {
    let visibility = visibility_for_family(eligibility.family());
    let lifecycle_posture =
        lifecycle_posture_for_admission(eligibility.operation_lane(), eligibility.disposition());
    let lower_runtime_evidence_placeholders =
        placeholders_for_lane(eligibility.operation_lane(), eligibility.disposition());
    let permitted_lanes = permitted_lanes_for_family(eligibility.family());
    match eligibility.disposition() {
        BasisEligibilityDisposition::Success => {
            BasisCapabilityAdmission::Admitted(AdmittedBasisCapability {
                normalized_basis_intent_digest: eligibility
                    .normalized_basis_intent_digest()
                    .to_string(),
                family: eligibility.family().clone(),
                authority_posture: eligibility.authority_posture().clone(),
                scope_subject: eligibility.normalized_subject().clone(),
                scope_label: eligibility.normalized_label().to_string(),
                visibility,
                lifecycle_posture,
                operation_lane: eligibility.operation_lane().clone(),
                tenant_schema_posture: eligibility.tenant_schema_posture().clone(),
                lower_runtime_evidence_placeholders,
                permitted_lanes,
                counters: eligibility.counters().clone(),
                capability_digest: basis_lifecycle_digest(
                    "basis_capability_success_v1",
                    [
                        (
                            "normalized_basis_intent_digest",
                            eligibility.normalized_basis_intent_digest().to_string(),
                        ),
                        ("family", eligibility.family().as_str().to_string()),
                        (
                            "operation_lane",
                            eligibility.operation_lane().as_str().to_string(),
                        ),
                        ("visibility", visibility.as_str().to_string()),
                        ("lifecycle_posture", lifecycle_posture.as_str().to_string()),
                        ("disposition", "success".to_string()),
                    ],
                ),
            })
        }
        BasisEligibilityDisposition::Advisory => {
            BasisCapabilityAdmission::Advisory(AdvisoryBasisCapability {
                normalized_basis_intent_digest: eligibility
                    .normalized_basis_intent_digest()
                    .to_string(),
                family: eligibility.family().clone(),
                authority_posture: eligibility.authority_posture().clone(),
                scope_subject: eligibility.normalized_subject().clone(),
                scope_label: eligibility.normalized_label().to_string(),
                visibility,
                lifecycle_posture,
                operation_lane: eligibility.operation_lane().clone(),
                tenant_schema_posture: eligibility.tenant_schema_posture().clone(),
                lower_runtime_evidence_placeholders,
                permitted_lanes,
                counters: eligibility.counters().clone(),
                advisory_digest: basis_lifecycle_digest(
                    "basis_capability_advisory_v1",
                    [
                        (
                            "normalized_basis_intent_digest",
                            eligibility.normalized_basis_intent_digest().to_string(),
                        ),
                        ("family", eligibility.family().as_str().to_string()),
                        (
                            "operation_lane",
                            eligibility.operation_lane().as_str().to_string(),
                        ),
                        ("visibility", visibility.as_str().to_string()),
                        ("lifecycle_posture", lifecycle_posture.as_str().to_string()),
                        ("disposition", "advisory".to_string()),
                    ],
                ),
            })
        }
    }
}

macro_rules! define_lane_admission {
    ($fn_name:ident, $wrapper:ident, $lane:ident) => {
        pub fn $fn_name(eligibility: BasisEligibility) -> Result<$wrapper, DeniedBasisCapability> {
            if eligibility.operation_lane() != &BasisOperationLaneRequest::$lane {
                return Err(denied_basis_capability_for_lane_mismatch(
                    eligibility.normalized_basis_intent_digest(),
                    eligibility.family(),
                    eligibility.operation_lane(),
                    eligibility.counters().clone(),
                    "lane_specific_admission_requires_matching_eligible_lane",
                    "lane-specific capability wrappers may only be constructed from eligibility proven for the same lane",
                    stringify!($wrapper),
                ));
            }
            Ok($wrapper {
                admission: admit_basis_capability(eligibility),
            })
        }
    };
}

define_lane_admission!(
    admit_observation_basis,
    ObservationBasisCapability,
    Observation
);
define_lane_admission!(
    admit_mutation_preparation_basis,
    MutationPreparationBasisCapability,
    MutationPreparation
);
define_lane_admission!(admit_replay_basis, ReplayBasisCapability, Replay);
define_lane_admission!(
    admit_inspection_basis,
    InspectionBasisCapability,
    Inspection
);
define_lane_admission!(
    admit_materialization_basis,
    MaterializationBasisCapability,
    Materialization
);
define_lane_admission!(
    admit_subscription_declaration_basis,
    SubscriptionDeclarationBasisCapability,
    SubscriptionDeclaration
);
define_lane_admission!(
    admit_subscription_activation_basis,
    SubscriptionActivationBasisCapability,
    SubscriptionActivation
);
define_lane_admission!(
    admit_preview_closeout_basis,
    PreviewCloseoutBasisCapability,
    PreviewCloseout
);
define_lane_admission!(
    admit_certification_basis,
    CertificationBasisCapability,
    Certification
);

fn compose_admitted_capability_snapshot_basis_identity(
    capability: &AdmittedBasisCapability,
) -> WorthQueryEvidenceIdentity {
    worth_query_evidence_identity(WorthQueryEvidenceScope::RawBasisIntent)
        .field_shape(
            WorthQueryEvidenceTag::new("identity_family"),
            "basis_capability_snapshot_basis_v1",
        )
        .field_shape(WorthQueryEvidenceTag::new("disposition"), "admitted")
        .field_shape(
            WorthQueryEvidenceTag::new("family"),
            capability.family().as_str(),
        )
        .field_shape(
            WorthQueryEvidenceTag::new("operation_lane"),
            capability.operation_lane().as_str(),
        )
        .field_shape(
            WorthQueryEvidenceTag::new("visibility"),
            capability.visibility().as_str(),
        )
        .field_shape(
            WorthQueryEvidenceTag::new("lifecycle_posture"),
            capability.lifecycle_posture().as_str(),
        )
        .field_shape(
            WorthQueryEvidenceTag::new("scope_label"),
            capability.scope_label(),
        )
        .seal()
}

fn compose_admitted_capability_snapshot_result_shape_identity(
    capability: &AdmittedBasisCapability,
) -> WorthQueryEvidenceIdentity {
    worth_query_evidence_identity(WorthQueryEvidenceScope::RawBasisIntent)
        .field_shape(
            WorthQueryEvidenceTag::new("identity_family"),
            "basis_capability_snapshot_result_shape_v1",
        )
        .field_shape(WorthQueryEvidenceTag::new("disposition"), "admitted")
        .field_shape(
            WorthQueryEvidenceTag::new("family"),
            capability.family().as_str(),
        )
        .field_shape(
            WorthQueryEvidenceTag::new("operation_lane"),
            capability.operation_lane().as_str(),
        )
        .field_shape(
            WorthQueryEvidenceTag::new("authority"),
            capability.authority_posture().as_str(),
        )
        .seal()
}

fn compose_advisory_capability_snapshot_basis_identity(
    capability: &AdvisoryBasisCapability,
) -> WorthQueryEvidenceIdentity {
    worth_query_evidence_identity(WorthQueryEvidenceScope::RawBasisIntent)
        .field_shape(
            WorthQueryEvidenceTag::new("identity_family"),
            "basis_capability_snapshot_basis_v1",
        )
        .field_shape(WorthQueryEvidenceTag::new("disposition"), "advisory")
        .field_shape(
            WorthQueryEvidenceTag::new("family"),
            capability.family().as_str(),
        )
        .field_shape(
            WorthQueryEvidenceTag::new("operation_lane"),
            capability.operation_lane().as_str(),
        )
        .field_shape(
            WorthQueryEvidenceTag::new("visibility"),
            capability.visibility().as_str(),
        )
        .field_shape(
            WorthQueryEvidenceTag::new("lifecycle_posture"),
            capability.lifecycle_posture().as_str(),
        )
        .field_shape(
            WorthQueryEvidenceTag::new("scope_label"),
            capability.scope_label(),
        )
        .seal()
}

fn compose_advisory_capability_snapshot_result_shape_identity(
    capability: &AdvisoryBasisCapability,
) -> WorthQueryEvidenceIdentity {
    worth_query_evidence_identity(WorthQueryEvidenceScope::RawBasisIntent)
        .field_shape(
            WorthQueryEvidenceTag::new("identity_family"),
            "basis_capability_snapshot_result_shape_v1",
        )
        .field_shape(WorthQueryEvidenceTag::new("disposition"), "advisory")
        .field_shape(
            WorthQueryEvidenceTag::new("family"),
            capability.family().as_str(),
        )
        .field_shape(
            WorthQueryEvidenceTag::new("operation_lane"),
            capability.operation_lane().as_str(),
        )
        .field_shape(
            WorthQueryEvidenceTag::new("authority"),
            capability.authority_posture().as_str(),
        )
        .seal()
}
