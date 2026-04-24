use crate::identity::hash_parts;

use super::super::activation::SubscriptionActivationInput;
use super::super::bridge_lowering::BridgeSubscriptionLoweringPlan;
use super::super::declaration::QuerySubscriptionDeclarationArtifact;
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
    witness_digest: String,
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

    pub fn witness_digest(&self) -> &str {
        &self.witness_digest
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
    let mut digest_parts = vec![
        "query_subscription_manual_bridge_witness_v1".to_string(),
        declaration.family().as_str().to_string(),
        lowering.bridge_family().as_str().to_string(),
        declaration.basis_posture().as_str().to_string(),
        lowering
            .signal_strategy_request()
            .request_kind()
            .as_str()
            .to_string(),
        declaration.declaration_digest().as_str().to_string(),
        lowering.bridge_declaration_digest().to_string(),
        lowering.basis_request().digest().to_string(),
        lowering.signal_strategy_request().digest().to_string(),
        activation.activation_digest().to_string(),
        assembly_posture.as_str().to_string(),
    ];
    digest_parts.extend(
        bridge_slice_labels
            .iter()
            .enumerate()
            .map(|(index, label)| format!("bridge_slice:{index}:{label}")),
    );
    let witness_digest = hash_parts(&digest_parts);

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
        bridge_declaration_digest: lowering.bridge_declaration_digest().to_string(),
        basis_binding_digest: lowering.basis_request().digest().to_string(),
        signal_strategy_digest: lowering.signal_strategy_request().digest().to_string(),
        activation_digest: activation.activation_digest().to_string(),
        assembly_posture,
        witness_digest,
    })
}

fn validate_authoritative_sources(
    declaration: &QuerySubscriptionDeclarationArtifact,
    lowering: &BridgeSubscriptionLoweringPlan,
    activation: &SubscriptionActivationInput,
) -> Result<(), QuerySubscriptionBridgeParityError> {
    if declaration.declaration_digest().as_str() != lowering.query_declaration_digest()
        || declaration.declaration_digest().as_str() != activation.query_declaration_digest()
    {
        return Err(QuerySubscriptionBridgeParityError::new(
            QuerySubscriptionBridgeParityFailure::new(
                QuerySubscriptionBridgeParityFailureKind::DeclarationMismatch,
                QuerySubscriptionBridgeParityClass::DeniedSourceMismatch,
                "manual bridge witness requires declaration, lowering, and activation to bind the same canonical declaration digest",
                declaration.declaration_digest().as_str(),
                &[
                    format!("declaration:{}", declaration.declaration_digest().as_str()),
                    format!("lowering:{}", lowering.query_declaration_digest()),
                    format!("activation:{}", activation.query_declaration_digest()),
                ],
            ),
            QuerySubscriptionBridgeParityCounters::denied(),
        ));
    }

    if lowering.bridge_declaration_digest() != activation.bridge_declaration_digest() {
        return Err(QuerySubscriptionBridgeParityError::new(
            QuerySubscriptionBridgeParityFailure::new(
                QuerySubscriptionBridgeParityFailureKind::BridgeMismatch,
                QuerySubscriptionBridgeParityClass::DeniedSourceMismatch,
                "manual bridge witness requires lowering and activation to bind the same bridge declaration digest",
                lowering.bridge_declaration_digest(),
                &[
                    format!("lowering:{}", lowering.bridge_declaration_digest()),
                    format!("activation:{}", activation.bridge_declaration_digest()),
                ],
            ),
            QuerySubscriptionBridgeParityCounters::denied(),
        ));
    }

    if lowering.basis_request().digest() != activation.basis_binding_digest() {
        return Err(QuerySubscriptionBridgeParityError::new(
            QuerySubscriptionBridgeParityFailure::new(
                QuerySubscriptionBridgeParityFailureKind::BasisMismatch,
                QuerySubscriptionBridgeParityClass::DeniedSourceMismatch,
                "manual bridge witness requires lowering and activation to bind the same basis request digest",
                lowering.basis_request().digest(),
                &[
                    format!("lowering:{}", lowering.basis_request().digest()),
                    format!("activation:{}", activation.basis_binding_digest()),
                ],
            ),
            QuerySubscriptionBridgeParityCounters::denied(),
        ));
    }

    if lowering.signal_strategy_request().digest() != activation.signal_strategy_digest() {
        return Err(QuerySubscriptionBridgeParityError::new(
            QuerySubscriptionBridgeParityFailure::new(
                QuerySubscriptionBridgeParityFailureKind::SignalStrategyMismatch,
                QuerySubscriptionBridgeParityClass::DeniedSourceMismatch,
                "manual bridge witness requires lowering and activation to bind the same signal strategy digest",
                lowering.signal_strategy_request().digest(),
                &[
                    format!("lowering:{}", lowering.signal_strategy_request().digest()),
                    format!("activation:{}", activation.signal_strategy_digest()),
                ],
            ),
            QuerySubscriptionBridgeParityCounters::denied(),
        ));
    }

    Ok(())
}
