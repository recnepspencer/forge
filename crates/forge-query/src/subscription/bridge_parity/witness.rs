use crate::evidence_identity::ForgeQueryEvidenceIdentity;

use super::super::activation::SubscriptionActivationInput;
use super::super::bridge_lowering::BridgeSubscriptionLoweringPlan;
use super::super::declaration::QuerySubscriptionDeclarationArtifact;
use super::super::evidence_identities::{manual_bridge_witness_identity, typed_identity_drift};
use super::explanation::{
    QuerySubscriptionBridgeParityClass, QuerySubscriptionBridgeParityCounters,
    QuerySubscriptionBridgeParityError, QuerySubscriptionBridgeParityFailure,
    QuerySubscriptionBridgeParityFailureKind,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BridgeWitnessAssemblyPosture {
    PreLoweredWitness,
    CanonicalComposition,
    SemanticRediscoveryDebtExplicit,
    SemanticRediscoveryDenied,
}

impl BridgeWitnessAssemblyPosture {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::PreLoweredWitness => "pre_lowered_witness",
            Self::CanonicalComposition => "canonical_composition",
            Self::SemanticRediscoveryDebtExplicit => "semantic_rediscovery_debt_explicit",
            Self::SemanticRediscoveryDenied => "semantic_rediscovery_denied",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct QuerySubscriptionManualBridgeWitness {
    query_family_label: String,
    declaration_family_label: String,
    bridge_family_label: String,
    bridge_slice_labels: Vec<String>,
    basis_posture_label: String,
    signal_strategy_class_label: String,
    query_declaration_digest: String,
    bridge_declaration_digest: String,
    basis_binding_digest: String,
    signal_strategy_digest: String,
    activation_digest: String,
    assembly_posture: BridgeWitnessAssemblyPosture,
    witness_identity: ForgeQueryEvidenceIdentity,
}

impl QuerySubscriptionManualBridgeWitness {
    pub fn query_family_label(&self) -> &str {
        &self.query_family_label
    }

    pub fn declaration_family_label(&self) -> &str {
        &self.declaration_family_label
    }

    pub fn bridge_family_label(&self) -> &str {
        &self.bridge_family_label
    }

    pub fn bridge_slice_labels(&self) -> &[String] {
        &self.bridge_slice_labels
    }

    pub fn basis_posture_label(&self) -> &str {
        &self.basis_posture_label
    }

    pub fn signal_strategy_class_label(&self) -> &str {
        &self.signal_strategy_class_label
    }

    pub fn query_declaration_digest(&self) -> &str {
        &self.query_declaration_digest
    }

    pub fn bridge_declaration_digest(&self) -> &str {
        &self.bridge_declaration_digest
    }

    pub fn basis_binding_digest(&self) -> &str {
        &self.basis_binding_digest
    }

    pub fn signal_strategy_digest(&self) -> &str {
        &self.signal_strategy_digest
    }

    pub fn activation_digest(&self) -> &str {
        &self.activation_digest
    }

    pub fn assembly_posture(&self) -> &BridgeWitnessAssemblyPosture {
        &self.assembly_posture
    }

    pub fn evidence_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.witness_identity
    }

    pub fn witness_for_reporting(&self) -> &str {
        self.witness_identity.as_str()
    }

    pub fn witness_digest(&self) -> &str {
        self.witness_for_reporting()
    }
}

pub fn build_query_subscription_manual_bridge_witness(
    declaration: &QuerySubscriptionDeclarationArtifact,
    lowering: &BridgeSubscriptionLoweringPlan,
    activation: &SubscriptionActivationInput,
) -> Result<QuerySubscriptionManualBridgeWitness, QuerySubscriptionBridgeParityError> {
    validate_authoritative_sources(declaration, lowering, activation)?;

    let bridge_slice_labels = lowering
        .bridge_slices()
        .iter()
        .map(|slice| slice.as_str().to_string())
        .collect::<Vec<_>>();
    let assembly_posture = BridgeWitnessAssemblyPosture::PreLoweredWitness;
    let witness_identity = manual_bridge_witness_identity(
        declaration.family().as_str(),
        lowering.bridge_family().as_str(),
        declaration.basis_posture().as_str(),
        lowering
            .signal_strategy_request()
            .request_kind()
            .as_str(),
        declaration.declaration_identity(),
        lowering.bridge_declaration_identity(),
        lowering.basis_request().evidence_identity(),
        lowering.signal_strategy_request().evidence_identity(),
        activation.evidence_identity(),
        assembly_posture.as_str(),
        lowering.bridge_slices(),
    );

    Ok(QuerySubscriptionManualBridgeWitness {
        query_family_label: declaration.family().as_str().to_string(),
        declaration_family_label: declaration.family().as_str().to_string(),
        bridge_family_label: lowering.bridge_family().as_str().to_string(),
        bridge_slice_labels,
        basis_posture_label: declaration.basis_posture().as_str().to_string(),
        signal_strategy_class_label: lowering
            .signal_strategy_request()
            .request_kind()
            .as_str()
            .to_string(),
        query_declaration_digest: declaration.declaration_digest().as_str().to_string(),
        bridge_declaration_digest: lowering.bridge_declaration_for_reporting().to_string(),
        basis_binding_digest: lowering.basis_request().evidence_identity().as_str().to_string(),
        signal_strategy_digest: lowering
            .signal_strategy_request()
            .evidence_identity()
            .as_str()
            .to_string(),
        activation_digest: activation.activation_for_reporting().to_string(),
        assembly_posture,
        witness_identity,
    })
}

fn validate_authoritative_sources(
    declaration: &QuerySubscriptionDeclarationArtifact,
    lowering: &BridgeSubscriptionLoweringPlan,
    activation: &SubscriptionActivationInput,
) -> Result<(), QuerySubscriptionBridgeParityError> {
    if typed_identity_drift(
        declaration.declaration_identity(),
        lowering.query_declaration_identity(),
    ) || typed_identity_drift(
        declaration.declaration_identity(),
        activation.query_declaration_identity(),
    ) {
        return Err(QuerySubscriptionBridgeParityError::new(
            QuerySubscriptionBridgeParityFailure::new(
                QuerySubscriptionBridgeParityFailureKind::DeclarationMismatch,
                QuerySubscriptionBridgeParityClass::DeniedSourceMismatch,
                "manual bridge witness requires declaration, lowering, and activation to bind the same canonical declaration identity",
                declaration.declaration_for_reporting(),
                &[
                    format!("declaration:{}", declaration.declaration_for_reporting()),
                    format!("lowering:{}", lowering.query_declaration_for_reporting()),
                    format!(
                        "activation:{}",
                        activation.query_declaration_for_reporting()
                    ),
                ],
            ),
            QuerySubscriptionBridgeParityCounters::denied(),
        ));
    }

    if typed_identity_drift(
        lowering.bridge_declaration_identity(),
        activation.bridge_declaration_identity(),
    ) {
        return Err(QuerySubscriptionBridgeParityError::new(
            QuerySubscriptionBridgeParityFailure::new(
                QuerySubscriptionBridgeParityFailureKind::BridgeMismatch,
                QuerySubscriptionBridgeParityClass::DeniedSourceMismatch,
                "manual bridge witness requires lowering and activation to bind the same bridge declaration digest",
                lowering.bridge_declaration_for_reporting(),
                &[
                    format!("lowering:{}", lowering.bridge_declaration_for_reporting()),
                    format!("activation:{}", activation.bridge_declaration_for_reporting()),
                ],
            ),
            QuerySubscriptionBridgeParityCounters::denied(),
        ));
    }

    if typed_identity_drift(
        lowering.basis_request().evidence_identity(),
        activation.basis_binding_identity(),
    ) {
        return Err(QuerySubscriptionBridgeParityError::new(
            QuerySubscriptionBridgeParityFailure::new(
                QuerySubscriptionBridgeParityFailureKind::BasisMismatch,
                QuerySubscriptionBridgeParityClass::DeniedSourceMismatch,
                "manual bridge witness requires lowering and activation to bind the same basis request digest",
                lowering.basis_request().evidence_identity().as_str(),
                &[
                    format!(
                        "lowering:{}",
                        lowering.basis_request().evidence_identity().as_str()
                    ),
                    format!(
                        "activation:{}",
                        activation.basis_binding_for_reporting()
                    ),
                ],
            ),
            QuerySubscriptionBridgeParityCounters::denied(),
        ));
    }

    if typed_identity_drift(
        lowering.signal_strategy_request().evidence_identity(),
        activation.signal_strategy_identity(),
    ) {
        return Err(QuerySubscriptionBridgeParityError::new(
            QuerySubscriptionBridgeParityFailure::new(
                QuerySubscriptionBridgeParityFailureKind::SignalStrategyMismatch,
                QuerySubscriptionBridgeParityClass::DeniedSourceMismatch,
                "manual bridge witness requires lowering and activation to bind the same signal strategy digest",
                lowering.signal_strategy_request().evidence_identity().as_str(),
                &[
                    format!(
                        "lowering:{}",
                        lowering.signal_strategy_request().evidence_identity().as_str()
                    ),
                    format!(
                        "activation:{}",
                        activation.signal_strategy_for_reporting()
                    ),
                ],
            ),
            QuerySubscriptionBridgeParityCounters::denied(),
        ));
    }

    Ok(())
}
