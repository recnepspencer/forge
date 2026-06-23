use super::identity::basis_lifecycle_digest;
use super::{
    AdmittedBasisCapability, AdvisoryBasisCapability, BasisCapabilityAdmission,
    BasisScopedAdmissionDenial, CertificationBasisCapability, DeniedBasisCapability,
    InspectionBasisCapability, MaterializationBasisCapability, MutationPreparationBasisCapability,
    ObservationBasisCapability, PreviewCloseoutBasisCapability, ReplayBasisCapability,
    SubscriptionActivationBasisCapability, SubscriptionDeclarationBasisCapability,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ScopedBasisConstructionCounters {
    admitted_evidence_width: usize,
    lane_witness_width: usize,
    lower_runtime_placeholder_width: usize,
}

impl ScopedBasisConstructionCounters {
    pub fn admitted_evidence_width(&self) -> usize {
        self.admitted_evidence_width
    }

    pub fn lane_witness_width(&self) -> usize {
        self.lane_witness_width
    }

    pub fn lower_runtime_placeholder_width(&self) -> usize {
        self.lower_runtime_placeholder_width
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ScopedObservationBasis {
    admission: BasisCapabilityAdmission,
    counters: ScopedBasisConstructionCounters,
    scoped_digest: String,
}

impl ScopedObservationBasis {
    pub fn admission(&self) -> &BasisCapabilityAdmission {
        &self.admission
    }

    pub fn counters(&self) -> &ScopedBasisConstructionCounters {
        &self.counters
    }

    pub fn scoped_digest(&self) -> &str {
        &self.scoped_digest
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ScopedInspectionBasis {
    admission: BasisCapabilityAdmission,
    counters: ScopedBasisConstructionCounters,
    scoped_digest: String,
}

impl ScopedInspectionBasis {
    pub fn admission(&self) -> &BasisCapabilityAdmission {
        &self.admission
    }

    pub fn counters(&self) -> &ScopedBasisConstructionCounters {
        &self.counters
    }

    pub fn scoped_digest(&self) -> &str {
        &self.scoped_digest
    }
}

macro_rules! define_admitted_scoped_wrapper {
    ($name:ident) => {
        #[derive(Clone, Debug, Eq, PartialEq)]
        pub struct $name {
            capability: AdmittedBasisCapability,
            counters: ScopedBasisConstructionCounters,
            scoped_digest: String,
        }

        impl $name {
            pub fn capability(&self) -> &AdmittedBasisCapability {
                &self.capability
            }

            pub fn counters(&self) -> &ScopedBasisConstructionCounters {
                &self.counters
            }

            pub fn scoped_digest(&self) -> &str {
                &self.scoped_digest
            }
        }
    };
}

define_admitted_scoped_wrapper!(ScopedMutationPreparationBasis);
define_admitted_scoped_wrapper!(ScopedReplayBasis);
define_admitted_scoped_wrapper!(ScopedMaterializationBasis);
define_admitted_scoped_wrapper!(ScopedSubscriptionDeclarationBasis);
define_admitted_scoped_wrapper!(ScopedSubscriptionActivationBasis);
define_admitted_scoped_wrapper!(ScopedPreviewCloseoutBasis);
define_admitted_scoped_wrapper!(ScopedCertificationBasis);

pub fn scope_observation_basis(
    capability: ObservationBasisCapability,
) -> Result<ScopedObservationBasis, DeniedBasisCapability> {
    let admission = capability.admission().clone();
    Ok(ScopedObservationBasis {
        counters: counters_for_admission(&admission),
        scoped_digest: digest_for_admission("scoped_observation_basis", &admission),
        admission,
    })
}

pub fn scope_inspection_basis(
    capability: InspectionBasisCapability,
) -> Result<ScopedInspectionBasis, DeniedBasisCapability> {
    let admission = capability.admission().clone();
    Ok(ScopedInspectionBasis {
        counters: counters_for_admission(&admission),
        scoped_digest: digest_for_admission("scoped_inspection_basis", &admission),
        admission,
    })
}

macro_rules! define_scoped_transition {
    ($fn_name:ident, $input:ident, $output:ident, $label:literal) => {
        pub fn $fn_name(capability: $input) -> Result<$output, DeniedBasisCapability> {
            let admitted = require_admitted_capability(capability.admission(), $label)?;
            Ok($output {
                counters: counters_for_admitted(&admitted),
                scoped_digest: digest_for_admitted($label, &admitted),
                capability: admitted,
            })
        }
    };
}

define_scoped_transition!(
    scope_mutation_preparation_basis,
    MutationPreparationBasisCapability,
    ScopedMutationPreparationBasis,
    "scoped_mutation_preparation_basis"
);
define_scoped_transition!(
    scope_replay_basis,
    ReplayBasisCapability,
    ScopedReplayBasis,
    "scoped_replay_basis"
);
define_scoped_transition!(
    scope_materialization_basis,
    MaterializationBasisCapability,
    ScopedMaterializationBasis,
    "scoped_materialization_basis"
);
define_scoped_transition!(
    scope_subscription_declaration_basis,
    SubscriptionDeclarationBasisCapability,
    ScopedSubscriptionDeclarationBasis,
    "scoped_subscription_declaration_basis"
);
define_scoped_transition!(
    scope_subscription_activation_basis,
    SubscriptionActivationBasisCapability,
    ScopedSubscriptionActivationBasis,
    "scoped_subscription_activation_basis"
);
define_scoped_transition!(
    scope_preview_closeout_basis,
    PreviewCloseoutBasisCapability,
    ScopedPreviewCloseoutBasis,
    "scoped_preview_closeout_basis"
);
define_scoped_transition!(
    scope_certification_basis,
    CertificationBasisCapability,
    ScopedCertificationBasis,
    "scoped_certification_basis"
);

fn require_admitted_capability(
    admission: &BasisCapabilityAdmission,
    scoped_label: &'static str,
) -> Result<AdmittedBasisCapability, DeniedBasisCapability> {
    match admission {
        BasisCapabilityAdmission::Admitted(admitted) => Ok(admitted.clone()),
        BasisCapabilityAdmission::Advisory(advisory) => Err(
            super::eligibility::denied_basis_capability_for_scoped_use_requires_admitted_capability(
                advisory.normalized_basis_intent_digest(),
                advisory.family(),
                advisory.operation_lane(),
                advisory.counters().clone(),
                scoped_label,
            ),
        ),
    }
}

fn counters_for_admission(admission: &BasisCapabilityAdmission) -> ScopedBasisConstructionCounters {
    match admission {
        BasisCapabilityAdmission::Admitted(admitted) => counters_for_admitted(admitted),
        BasisCapabilityAdmission::Advisory(advisory) => counters_for_advisory(advisory),
    }
}

fn counters_for_admitted(admitted: &AdmittedBasisCapability) -> ScopedBasisConstructionCounters {
    ScopedBasisConstructionCounters {
        admitted_evidence_width: admitted.counters().consulted_row_count(),
        lane_witness_width: 1,
        lower_runtime_placeholder_width: admitted.lower_runtime_evidence_placeholders().len(),
    }
}

fn counters_for_advisory(advisory: &AdvisoryBasisCapability) -> ScopedBasisConstructionCounters {
    ScopedBasisConstructionCounters {
        admitted_evidence_width: advisory.counters().consulted_row_count(),
        lane_witness_width: 1,
        lower_runtime_placeholder_width: advisory.lower_runtime_evidence_placeholders().len(),
    }
}

fn digest_for_admission(label: &'static str, admission: &BasisCapabilityAdmission) -> String {
    match admission {
        BasisCapabilityAdmission::Admitted(admitted) => digest_for_admitted(label, admitted),
        BasisCapabilityAdmission::Advisory(advisory) => basis_lifecycle_digest(
            "scoped_basis_advisory_v1",
            [
                ("scoped_label", label.to_string()),
                (
                    "normalized_basis_intent_digest",
                    advisory.normalized_basis_intent_digest().to_string(),
                ),
                (
                    "operation_lane",
                    advisory.operation_lane().as_str().to_string(),
                ),
                ("disposition", "advisory".to_string()),
            ],
        ),
    }
}

fn digest_for_admitted(label: &'static str, admitted: &AdmittedBasisCapability) -> String {
    basis_lifecycle_digest(
        "scoped_basis_admitted_v1",
        [
            ("scoped_label", label.to_string()),
            (
                "normalized_basis_intent_digest",
                admitted.normalized_basis_intent_digest().to_string(),
            ),
            (
                "operation_lane",
                admitted.operation_lane().as_str().to_string(),
            ),
            ("disposition", "admitted".to_string()),
        ],
    )
}

macro_rules! define_scoped_common_path {
    ($fn_name:ident, $admit_fn:ident, $scope_fn:ident, $output:ident) => {
        pub fn $fn_name(
            intent: super::RawBasisIntent,
        ) -> Result<$output, BasisScopedAdmissionDenial> {
            let capability = super::common_paths::$admit_fn(intent)?;
            $scope_fn(capability).map_err(BasisScopedAdmissionDenial::Eligibility)
        }
    };
}

define_scoped_common_path!(
    scope_observation_basis_intent,
    admit_observation_basis_intent,
    scope_observation_basis,
    ScopedObservationBasis
);
define_scoped_common_path!(
    scope_inspection_basis_intent,
    admit_inspection_basis_intent,
    scope_inspection_basis,
    ScopedInspectionBasis
);
define_scoped_common_path!(
    scope_mutation_preparation_basis_intent,
    admit_mutation_preparation_basis_intent,
    scope_mutation_preparation_basis,
    ScopedMutationPreparationBasis
);
define_scoped_common_path!(
    scope_replay_basis_intent,
    admit_replay_basis_intent,
    scope_replay_basis,
    ScopedReplayBasis
);
define_scoped_common_path!(
    scope_materialization_basis_intent,
    admit_materialization_basis_intent,
    scope_materialization_basis,
    ScopedMaterializationBasis
);
define_scoped_common_path!(
    scope_subscription_declaration_basis_intent,
    admit_subscription_declaration_basis_intent,
    scope_subscription_declaration_basis,
    ScopedSubscriptionDeclarationBasis
);
define_scoped_common_path!(
    scope_subscription_activation_basis_intent,
    admit_subscription_activation_basis_intent,
    scope_subscription_activation_basis,
    ScopedSubscriptionActivationBasis
);
define_scoped_common_path!(
    scope_preview_closeout_basis_intent,
    admit_preview_closeout_basis_intent,
    scope_preview_closeout_basis,
    ScopedPreviewCloseoutBasis
);
define_scoped_common_path!(
    scope_certification_basis_intent,
    admit_certification_basis_intent,
    scope_certification_basis,
    ScopedCertificationBasis
);
