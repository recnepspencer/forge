use crate::identity::hash_parts;

use super::{
    denied_basis_capability_for_lane_mismatch, BasisAuthorityPosture, BasisEligibility,
    BasisEligibilityCounters, BasisEligibilityDisposition, BasisOperationLaneRequest,
    BasisTenantSchemaPosture, DeniedBasisCapability, NormalizedBasisFamily,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BasisVisibility {
    CurrentHead,
    BranchScoped,
    SnapshotScoped,
    Historical,
    PreviewScoped,
}

impl BasisVisibility {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::CurrentHead => "current_head",
            Self::BranchScoped => "branch_scoped",
            Self::SnapshotScoped => "snapshot_scoped",
            Self::Historical => "historical",
            Self::PreviewScoped => "preview_scoped",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BasisLifecyclePosture {
    Authoritative,
    MutablePreparation,
    HistoricalReplay,
    AdvisoryPreview,
}

impl BasisLifecyclePosture {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Authoritative => "authoritative",
            Self::MutablePreparation => "mutable_preparation",
            Self::HistoricalReplay => "historical_replay",
            Self::AdvisoryPreview => "advisory_preview",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UnboundLowerRuntimeEvidencePlaceholder {
    BridgeTruthViewAuthority,
    BridgeContinuityAuthority,
    BridgeSubscriptionAuthority,
    BridgePreviewSubscriptionAuthority,
    BridgeWritebackAuthority,
    BridgeCausalEnvelopeAuthority,
    RelationalTruthAuthority,
    SignalObservationAuthority,
}

impl UnboundLowerRuntimeEvidencePlaceholder {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::BridgeTruthViewAuthority => "bridge_truth_view_authority",
            Self::BridgeContinuityAuthority => "bridge_continuity_authority",
            Self::BridgeSubscriptionAuthority => "bridge_subscription_authority",
            Self::BridgePreviewSubscriptionAuthority => "bridge_preview_subscription_authority",
            Self::BridgeWritebackAuthority => "bridge_writeback_authority",
            Self::BridgeCausalEnvelopeAuthority => "bridge_causal_envelope_authority",
            Self::RelationalTruthAuthority => "relational_truth_authority",
            Self::SignalObservationAuthority => "signal_observation_authority",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AdmittedBasisCapability {
    normalized_basis_intent_digest: String,
    family: NormalizedBasisFamily,
    authority_posture: BasisAuthorityPosture,
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
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AdvisoryBasisCapability {
    normalized_basis_intent_digest: String,
    family: NormalizedBasisFamily,
    authority_posture: BasisAuthorityPosture,
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
                scope_label: eligibility.normalized_label().to_string(),
                visibility,
                lifecycle_posture,
                operation_lane: eligibility.operation_lane().clone(),
                tenant_schema_posture: eligibility.tenant_schema_posture().clone(),
                lower_runtime_evidence_placeholders,
                permitted_lanes,
                counters: eligibility.counters().clone(),
                capability_digest: hash_parts(&[
                    format!(
                        "normalized_basis_intent_digest:{}",
                        eligibility.normalized_basis_intent_digest()
                    ),
                    format!("family:{}", eligibility.family().as_str()),
                    format!("operation_lane:{}", eligibility.operation_lane().as_str()),
                    format!("visibility:{}", visibility.as_str()),
                    format!("lifecycle_posture:{}", lifecycle_posture.as_str()),
                    "disposition:success".to_string(),
                ]),
            })
        }
        BasisEligibilityDisposition::Advisory => {
            BasisCapabilityAdmission::Advisory(AdvisoryBasisCapability {
                normalized_basis_intent_digest: eligibility
                    .normalized_basis_intent_digest()
                    .to_string(),
                family: eligibility.family().clone(),
                authority_posture: eligibility.authority_posture().clone(),
                scope_label: eligibility.normalized_label().to_string(),
                visibility,
                lifecycle_posture,
                operation_lane: eligibility.operation_lane().clone(),
                tenant_schema_posture: eligibility.tenant_schema_posture().clone(),
                lower_runtime_evidence_placeholders,
                permitted_lanes,
                counters: eligibility.counters().clone(),
                advisory_digest: hash_parts(&[
                    format!(
                        "normalized_basis_intent_digest:{}",
                        eligibility.normalized_basis_intent_digest()
                    ),
                    format!("family:{}", eligibility.family().as_str()),
                    format!("operation_lane:{}", eligibility.operation_lane().as_str()),
                    format!("visibility:{}", visibility.as_str()),
                    format!("lifecycle_posture:{}", lifecycle_posture.as_str()),
                    "disposition:advisory".to_string(),
                ]),
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

fn visibility_for_family(family: &NormalizedBasisFamily) -> BasisVisibility {
    match family {
        NormalizedBasisFamily::CurrentHead => BasisVisibility::CurrentHead,
        NormalizedBasisFamily::BranchHead => BasisVisibility::BranchScoped,
        NormalizedBasisFamily::BranchSnapshot | NormalizedBasisFamily::RuntimeSnapshot => {
            BasisVisibility::SnapshotScoped
        }
        NormalizedBasisFamily::HistoricalSnapshot | NormalizedBasisFamily::HistoricalCommit => {
            BasisVisibility::Historical
        }
        NormalizedBasisFamily::Preview | NormalizedBasisFamily::PreviewDerivedHistorical => {
            BasisVisibility::PreviewScoped
        }
    }
}

fn lifecycle_posture_for_admission(
    lane: &BasisOperationLaneRequest,
    disposition: &BasisEligibilityDisposition,
) -> BasisLifecyclePosture {
    match disposition {
        BasisEligibilityDisposition::Advisory => BasisLifecyclePosture::AdvisoryPreview,
        BasisEligibilityDisposition::Success => match lane {
            BasisOperationLaneRequest::MutationPreparation => {
                BasisLifecyclePosture::MutablePreparation
            }
            BasisOperationLaneRequest::Replay => BasisLifecyclePosture::HistoricalReplay,
            _ => BasisLifecyclePosture::Authoritative,
        },
    }
}

fn placeholders_for_lane(
    lane: &BasisOperationLaneRequest,
    disposition: &BasisEligibilityDisposition,
) -> &'static [UnboundLowerRuntimeEvidencePlaceholder] {
    match (lane, disposition) {
        (BasisOperationLaneRequest::Observation, _) => &[
            UnboundLowerRuntimeEvidencePlaceholder::BridgeTruthViewAuthority,
            UnboundLowerRuntimeEvidencePlaceholder::RelationalTruthAuthority,
            UnboundLowerRuntimeEvidencePlaceholder::SignalObservationAuthority,
        ],
        (BasisOperationLaneRequest::Inspection, _) => &[
            UnboundLowerRuntimeEvidencePlaceholder::BridgeContinuityAuthority,
            UnboundLowerRuntimeEvidencePlaceholder::RelationalTruthAuthority,
            UnboundLowerRuntimeEvidencePlaceholder::SignalObservationAuthority,
        ],
        (
            BasisOperationLaneRequest::SubscriptionDeclaration
            | BasisOperationLaneRequest::SubscriptionActivation,
            BasisEligibilityDisposition::Advisory,
        ) => &[UnboundLowerRuntimeEvidencePlaceholder::BridgePreviewSubscriptionAuthority],
        (
            BasisOperationLaneRequest::SubscriptionDeclaration
            | BasisOperationLaneRequest::SubscriptionActivation,
            BasisEligibilityDisposition::Success,
        ) => &[UnboundLowerRuntimeEvidencePlaceholder::BridgeSubscriptionAuthority],
        (BasisOperationLaneRequest::Materialization, _) => &[
            UnboundLowerRuntimeEvidencePlaceholder::BridgeTruthViewAuthority,
            UnboundLowerRuntimeEvidencePlaceholder::BridgeCausalEnvelopeAuthority,
        ],
        (BasisOperationLaneRequest::MutationPreparation, _) => {
            &[UnboundLowerRuntimeEvidencePlaceholder::BridgeWritebackAuthority]
        }
        _ => &[],
    }
}

fn permitted_lanes_for_family(
    family: &NormalizedBasisFamily,
) -> &'static [BasisOperationLaneRequest] {
    match family {
        NormalizedBasisFamily::CurrentHead => &[
            BasisOperationLaneRequest::Observation,
            BasisOperationLaneRequest::Inspection,
            BasisOperationLaneRequest::Materialization,
            BasisOperationLaneRequest::SubscriptionDeclaration,
            BasisOperationLaneRequest::SubscriptionActivation,
            BasisOperationLaneRequest::Certification,
            BasisOperationLaneRequest::MutationPreparation,
        ],
        NormalizedBasisFamily::BranchHead => &[
            BasisOperationLaneRequest::Observation,
            BasisOperationLaneRequest::Inspection,
            BasisOperationLaneRequest::Materialization,
            BasisOperationLaneRequest::SubscriptionDeclaration,
            BasisOperationLaneRequest::SubscriptionActivation,
            BasisOperationLaneRequest::Certification,
            BasisOperationLaneRequest::MutationPreparation,
        ],
        NormalizedBasisFamily::BranchSnapshot | NormalizedBasisFamily::RuntimeSnapshot => &[
            BasisOperationLaneRequest::Observation,
            BasisOperationLaneRequest::Inspection,
            BasisOperationLaneRequest::Materialization,
            BasisOperationLaneRequest::Certification,
        ],
        NormalizedBasisFamily::HistoricalSnapshot | NormalizedBasisFamily::HistoricalCommit => &[
            BasisOperationLaneRequest::Observation,
            BasisOperationLaneRequest::Inspection,
            BasisOperationLaneRequest::Materialization,
            BasisOperationLaneRequest::Replay,
            BasisOperationLaneRequest::Certification,
        ],
        NormalizedBasisFamily::Preview => &[
            BasisOperationLaneRequest::Observation,
            BasisOperationLaneRequest::Inspection,
            BasisOperationLaneRequest::PreviewCloseout,
        ],
        NormalizedBasisFamily::PreviewDerivedHistorical => &[
            BasisOperationLaneRequest::Observation,
            BasisOperationLaneRequest::Inspection,
            BasisOperationLaneRequest::Certification,
        ],
    }
}
