use super::{
    admit_certification_basis, admit_inspection_basis, admit_materialization_basis,
    admit_mutation_preparation_basis, admit_observation_basis, admit_preview_closeout_basis,
    admit_replay_basis, admit_subscription_activation_basis, admit_subscription_declaration_basis,
    evaluate_basis_eligibility, normalize_raw_basis, BasisCapabilityAdmission, BasisIntentDenial,
    BasisOperationLaneRequest, CertificationBasisCapability, DeniedBasisCapability,
    InspectionBasisCapability, MaterializationBasisCapability, MutationPreparationBasisCapability,
    ObservationBasisCapability, PreviewCloseoutBasisCapability, RawBasisIntent,
    ReplayBasisCapability, SubscriptionActivationBasisCapability,
    SubscriptionDeclarationBasisCapability,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum BasisScopedAdmissionDenial {
    Intent(BasisIntentDenial),
    Eligibility(DeniedBasisCapability),
}

impl BasisScopedAdmissionDenial {
    pub fn intent_denial(&self) -> Option<&BasisIntentDenial> {
        match self {
            Self::Intent(denial) => Some(denial),
            Self::Eligibility(_) => None,
        }
    }

    pub fn eligibility_denial(&self) -> Option<&DeniedBasisCapability> {
        match self {
            Self::Intent(_) => None,
            Self::Eligibility(denial) => Some(denial),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum BasisScopedAdmissionStatus {
    Admitted,
    Advisory,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct BasisScopedAdmissionFact {
    trace_label: &'static str,
    status: BasisScopedAdmissionStatus,
}

impl BasisScopedAdmissionFact {
    pub(crate) fn trace_label(&self) -> &'static str {
        self.trace_label
    }

    pub(crate) fn status(&self) -> BasisScopedAdmissionStatus {
        self.status
    }
}

pub(crate) fn evaluate_basis_intent_common_path(
    intent: RawBasisIntent,
) -> Result<BasisScopedAdmissionFact, BasisScopedAdmissionDenial> {
    let normalized = normalize_raw_basis(intent).map_err(BasisScopedAdmissionDenial::Intent)?;
    let eligibility =
        evaluate_basis_eligibility(normalized).map_err(BasisScopedAdmissionDenial::Eligibility)?;
    let trace_label = eligibility.trace().rule_label();
    let operation_lane = eligibility.operation_lane().clone();
    let admission = match operation_lane {
        BasisOperationLaneRequest::Observation => admit_observation_basis(eligibility)
            .map_err(BasisScopedAdmissionDenial::Eligibility)?
            .admission()
            .clone(),
        BasisOperationLaneRequest::MutationPreparation => {
            admit_mutation_preparation_basis(eligibility)
                .map_err(BasisScopedAdmissionDenial::Eligibility)?
                .admission()
                .clone()
        }
        BasisOperationLaneRequest::Replay => admit_replay_basis(eligibility)
            .map_err(BasisScopedAdmissionDenial::Eligibility)?
            .admission()
            .clone(),
        BasisOperationLaneRequest::Inspection => admit_inspection_basis(eligibility)
            .map_err(BasisScopedAdmissionDenial::Eligibility)?
            .admission()
            .clone(),
        BasisOperationLaneRequest::Materialization => admit_materialization_basis(eligibility)
            .map_err(BasisScopedAdmissionDenial::Eligibility)?
            .admission()
            .clone(),
        BasisOperationLaneRequest::SubscriptionDeclaration => {
            admit_subscription_declaration_basis(eligibility)
                .map_err(BasisScopedAdmissionDenial::Eligibility)?
                .admission()
                .clone()
        }
        BasisOperationLaneRequest::SubscriptionActivation => {
            admit_subscription_activation_basis(eligibility)
                .map_err(BasisScopedAdmissionDenial::Eligibility)?
                .admission()
                .clone()
        }
        BasisOperationLaneRequest::PreviewCloseout => admit_preview_closeout_basis(eligibility)
            .map_err(BasisScopedAdmissionDenial::Eligibility)?
            .admission()
            .clone(),
        BasisOperationLaneRequest::Certification => admit_certification_basis(eligibility)
            .map_err(BasisScopedAdmissionDenial::Eligibility)?
            .admission()
            .clone(),
    };

    let status = match admission {
        BasisCapabilityAdmission::Admitted(_) => BasisScopedAdmissionStatus::Admitted,
        BasisCapabilityAdmission::Advisory(_) => BasisScopedAdmissionStatus::Advisory,
    };

    Ok(BasisScopedAdmissionFact {
        trace_label,
        status,
    })
}

macro_rules! define_common_path {
    ($fn_name:ident, $wrapper:ident, $admit_fn:ident) => {
        pub fn $fn_name(intent: RawBasisIntent) -> Result<$wrapper, BasisScopedAdmissionDenial> {
            let normalized =
                normalize_raw_basis(intent).map_err(BasisScopedAdmissionDenial::Intent)?;
            let eligibility = evaluate_basis_eligibility(normalized)
                .map_err(BasisScopedAdmissionDenial::Eligibility)?;
            $admit_fn(eligibility).map_err(BasisScopedAdmissionDenial::Eligibility)
        }
    };
}

define_common_path!(
    admit_observation_basis_intent,
    ObservationBasisCapability,
    admit_observation_basis
);
define_common_path!(
    admit_mutation_preparation_basis_intent,
    MutationPreparationBasisCapability,
    admit_mutation_preparation_basis
);
define_common_path!(
    admit_replay_basis_intent,
    ReplayBasisCapability,
    admit_replay_basis
);
define_common_path!(
    admit_inspection_basis_intent,
    InspectionBasisCapability,
    admit_inspection_basis
);
define_common_path!(
    admit_materialization_basis_intent,
    MaterializationBasisCapability,
    admit_materialization_basis
);
define_common_path!(
    admit_subscription_declaration_basis_intent,
    SubscriptionDeclarationBasisCapability,
    admit_subscription_declaration_basis
);
define_common_path!(
    admit_subscription_activation_basis_intent,
    SubscriptionActivationBasisCapability,
    admit_subscription_activation_basis
);
define_common_path!(
    admit_preview_closeout_basis_intent,
    PreviewCloseoutBasisCapability,
    admit_preview_closeout_basis
);
define_common_path!(
    admit_certification_basis_intent,
    CertificationBasisCapability,
    admit_certification_basis
);
