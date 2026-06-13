use crate::identity::hash_parts;

use super::super::activation::SubscriptionActivationInput;
use super::super::bridge_lowering::BridgeSubscriptionLoweringPlan;
use super::super::declaration::QuerySubscriptionDeclarationArtifact;
use super::super::family::QuerySubscriptionFamily;
use super::witness::{BridgeWitnessAssemblyPosture, QuerySubscriptionManualBridgeWitness};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum QuerySubscriptionBridgeParityClass {
    ExactParity,
    FamilyDistinctBridgeShared,
    DeniedSourceMismatch,
    DeniedUnsupported,
}

impl QuerySubscriptionBridgeParityClass {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::ExactParity => "exact_parity",
            Self::FamilyDistinctBridgeShared => "family_distinct_bridge_shared",
            Self::DeniedSourceMismatch => "denied_source_mismatch",
            Self::DeniedUnsupported => "denied_unsupported",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum QuerySubscriptionBridgeParityFailureKind {
    DeclarationMismatch,
    BridgeMismatch,
    BasisMismatch,
    SignalStrategyMismatch,
    ActivationMismatch,
}

impl QuerySubscriptionBridgeParityFailureKind {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::DeclarationMismatch => "declaration_mismatch",
            Self::BridgeMismatch => "bridge_mismatch",
            Self::BasisMismatch => "basis_mismatch",
            Self::SignalStrategyMismatch => "signal_strategy_mismatch",
            Self::ActivationMismatch => "activation_mismatch",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct CanonicalBridgeParitySemantics {
    query_family_label: String,
    declaration_family_label: String,
    bridge_family_label: String,
    bridge_slice_labels: Vec<String>,
    basis_posture_label: String,
    signal_strategy_class_label: String,
}

impl CanonicalBridgeParitySemantics {
    fn from_authoritative_sources(
        declaration: &QuerySubscriptionDeclarationArtifact,
        lowering: &BridgeSubscriptionLoweringPlan,
    ) -> Self {
        Self {
            query_family_label: declaration.family().as_str().to_string(),
            declaration_family_label: declaration.family().as_str().to_string(),
            bridge_family_label: lowering.bridge_family().as_str().to_string(),
            bridge_slice_labels: lowering
                .bridge_slices()
                .iter()
                .map(|slice| slice.as_str().to_string())
                .collect(),
            basis_posture_label: declaration.basis_posture().as_str().to_string(),
            signal_strategy_class_label: lowering
                .signal_strategy_request()
                .request_kind()
                .as_str()
                .to_string(),
        }
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct QuerySubscriptionBridgeParityCounters {
    subscription_bridge_parity_comparison_count: u64,
    subscription_bridge_parity_admitted_count: u64,
    subscription_bridge_parity_denial_count: u64,
    subscription_bridge_family_distinction_preservation_count: u64,
}

impl QuerySubscriptionBridgeParityCounters {
    pub fn digest(&self) -> String {
        hash_parts(&[
            format!(
                "subscription_bridge_parity_comparison_count:{}",
                self.subscription_bridge_parity_comparison_count
            ),
            format!(
                "subscription_bridge_parity_admitted_count:{}",
                self.subscription_bridge_parity_admitted_count
            ),
            format!(
                "subscription_bridge_parity_denial_count:{}",
                self.subscription_bridge_parity_denial_count
            ),
            format!(
                "subscription_bridge_family_distinction_preservation_count:{}",
                self.subscription_bridge_family_distinction_preservation_count
            ),
        ])
    }

    pub fn subscription_bridge_parity_comparison_count(&self) -> u64 {
        self.subscription_bridge_parity_comparison_count
    }

    pub fn subscription_bridge_parity_admitted_count(&self) -> u64 {
        self.subscription_bridge_parity_admitted_count
    }

    pub fn subscription_bridge_parity_denial_count(&self) -> u64 {
        self.subscription_bridge_parity_denial_count
    }

    pub fn subscription_bridge_family_distinction_preservation_count(&self) -> u64 {
        self.subscription_bridge_family_distinction_preservation_count
    }

    pub(crate) fn admitted(parity_class: QuerySubscriptionBridgeParityClass) -> Self {
        Self {
            subscription_bridge_parity_comparison_count: 1,
            subscription_bridge_parity_admitted_count: 1,
            subscription_bridge_family_distinction_preservation_count: u64::from(
                parity_class == QuerySubscriptionBridgeParityClass::FamilyDistinctBridgeShared,
            ),
            ..Default::default()
        }
    }

    pub(crate) fn denied() -> Self {
        Self {
            subscription_bridge_parity_comparison_count: 1,
            subscription_bridge_parity_denial_count: 1,
            ..Default::default()
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SubscriptionBridgeParityWidth {
    compared_family_dimension_count: usize,
    compared_slice_dimension_count: usize,
    compared_basis_dimension_count: usize,
    compared_signal_dimension_count: usize,
    digest: String,
}

impl SubscriptionBridgeParityWidth {
    fn new(
        compared_family_dimension_count: usize,
        compared_slice_dimension_count: usize,
        compared_basis_dimension_count: usize,
        compared_signal_dimension_count: usize,
    ) -> Self {
        let digest = hash_parts(&[
            "subscription_bridge_parity_width_v1".to_string(),
            format!("family:{compared_family_dimension_count}"),
            format!("slice:{compared_slice_dimension_count}"),
            format!("basis:{compared_basis_dimension_count}"),
            format!("signal:{compared_signal_dimension_count}"),
        ]);
        Self {
            compared_family_dimension_count,
            compared_slice_dimension_count,
            compared_basis_dimension_count,
            compared_signal_dimension_count,
            digest,
        }
    }

    pub fn compared_family_dimension_count(&self) -> usize {
        self.compared_family_dimension_count
    }

    pub fn compared_slice_dimension_count(&self) -> usize {
        self.compared_slice_dimension_count
    }

    pub fn compared_basis_dimension_count(&self) -> usize {
        self.compared_basis_dimension_count
    }

    pub fn compared_signal_dimension_count(&self) -> usize {
        self.compared_signal_dimension_count
    }

    pub fn digest(&self) -> &str {
        &self.digest
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BridgeParityReceipt {
    witness_assembly_posture: BridgeWitnessAssemblyPosture,
    parity_class: QuerySubscriptionBridgeParityClass,
    comparison_width: SubscriptionBridgeParityWidth,
    semantic_rebuild_count: usize,
    digest: String,
}

impl BridgeParityReceipt {
    fn new(
        witness_assembly_posture: BridgeWitnessAssemblyPosture,
        parity_class: QuerySubscriptionBridgeParityClass,
        comparison_width: SubscriptionBridgeParityWidth,
        semantic_rebuild_count: usize,
    ) -> Self {
        let digest = hash_parts(&[
            "query_subscription_bridge_parity_receipt_v1".to_string(),
            witness_assembly_posture.as_str().to_string(),
            parity_class.as_str().to_string(),
            comparison_width.digest().to_string(),
            format!("semantic_rebuild_count:{semantic_rebuild_count}"),
        ]);
        Self {
            witness_assembly_posture,
            parity_class,
            comparison_width,
            semantic_rebuild_count,
            digest,
        }
    }

    pub fn witness_assembly_posture(&self) -> &BridgeWitnessAssemblyPosture {
        &self.witness_assembly_posture
    }

    pub fn parity_class(&self) -> &QuerySubscriptionBridgeParityClass {
        &self.parity_class
    }

    pub fn comparison_width(&self) -> &SubscriptionBridgeParityWidth {
        &self.comparison_width
    }

    pub fn semantic_rebuild_count(&self) -> usize {
        self.semantic_rebuild_count
    }

    pub fn digest(&self) -> &str {
        &self.digest
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct QuerySubscriptionBridgeParityFailure {
    failure_kind: QuerySubscriptionBridgeParityFailureKind,
    parity_class: QuerySubscriptionBridgeParityClass,
    reason: String,
    source_digest: String,
    failure_digest: String,
}

impl QuerySubscriptionBridgeParityFailure {
    pub(crate) fn new(
        failure_kind: QuerySubscriptionBridgeParityFailureKind,
        parity_class: QuerySubscriptionBridgeParityClass,
        reason: impl Into<String>,
        source_digest: impl Into<String>,
        evidence_parts: &[String],
    ) -> Self {
        let reason = reason.into();
        let source_digest = source_digest.into();
        let mut failure_parts = vec![
            "query_subscription_bridge_parity_failure_v1".to_string(),
            failure_kind.as_str().to_string(),
            parity_class.as_str().to_string(),
            reason.clone(),
            source_digest.clone(),
        ];
        failure_parts.extend(evidence_parts.iter().cloned());
        let failure_digest = hash_parts(&failure_parts);
        Self {
            failure_kind,
            parity_class,
            reason,
            source_digest,
            failure_digest,
        }
    }

    pub fn failure_kind(&self) -> &QuerySubscriptionBridgeParityFailureKind {
        &self.failure_kind
    }

    pub fn parity_class(&self) -> &QuerySubscriptionBridgeParityClass {
        &self.parity_class
    }

    pub fn reason(&self) -> &str {
        &self.reason
    }

    pub fn source_digest(&self) -> &str {
        &self.source_digest
    }

    pub fn failure_digest(&self) -> &str {
        &self.failure_digest
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct QuerySubscriptionBridgeParityError {
    failure: QuerySubscriptionBridgeParityFailure,
    counters: QuerySubscriptionBridgeParityCounters,
}

impl QuerySubscriptionBridgeParityError {
    pub(crate) fn new(
        failure: QuerySubscriptionBridgeParityFailure,
        counters: QuerySubscriptionBridgeParityCounters,
    ) -> Self {
        Self { failure, counters }
    }

    pub fn failure(&self) -> &QuerySubscriptionBridgeParityFailure {
        &self.failure
    }

    pub fn counters(&self) -> &QuerySubscriptionBridgeParityCounters {
        &self.counters
    }

    pub fn message(&self) -> &str {
        self.failure.reason()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct QuerySubscriptionBridgeParityComparison {
    parity_class: QuerySubscriptionBridgeParityClass,
    query_declaration_digest: String,
    bridge_declaration_digest: String,
    witness_digest: String,
    activation_digest: String,
    comparison_digest: String,
}

impl QuerySubscriptionBridgeParityComparison {
    pub fn parity_class(&self) -> &QuerySubscriptionBridgeParityClass {
        &self.parity_class
    }

    pub fn query_declaration_digest(&self) -> &str {
        &self.query_declaration_digest
    }

    pub fn bridge_declaration_digest(&self) -> &str {
        &self.bridge_declaration_digest
    }

    pub fn witness_digest(&self) -> &str {
        &self.witness_digest
    }

    pub fn activation_digest(&self) -> &str {
        &self.activation_digest
    }

    pub fn comparison_digest(&self) -> &str {
        &self.comparison_digest
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct QuerySubscriptionBridgeParityExplanation {
    comparison: QuerySubscriptionBridgeParityComparison,
    witness: QuerySubscriptionManualBridgeWitness,
    query_family_label: String,
    declaration_family_label: String,
    bridge_family_label: String,
    bridge_slice_labels: Vec<String>,
    basis_posture_label: String,
    signal_strategy_class_label: String,
    counter_snapshot: String,
    explanation_digest: String,
    counters: QuerySubscriptionBridgeParityCounters,
}

impl QuerySubscriptionBridgeParityExplanation {
    pub fn comparison(&self) -> &QuerySubscriptionBridgeParityComparison {
        &self.comparison
    }

    pub fn witness(&self) -> &QuerySubscriptionManualBridgeWitness {
        &self.witness
    }

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

    pub fn counter_snapshot(&self) -> &str {
        &self.counter_snapshot
    }

    pub fn explanation_digest(&self) -> &str {
        &self.explanation_digest
    }

    pub fn counters(&self) -> &QuerySubscriptionBridgeParityCounters {
        &self.counters
    }
}

pub fn explain_query_subscription_bridge_parity(
    declaration: &QuerySubscriptionDeclarationArtifact,
    lowering: &BridgeSubscriptionLoweringPlan,
    activation: &SubscriptionActivationInput,
    witness: QuerySubscriptionManualBridgeWitness,
) -> Result<
    (
        QuerySubscriptionBridgeParityExplanation,
        BridgeParityReceipt,
    ),
    QuerySubscriptionBridgeParityError,
> {
    let semantics =
        CanonicalBridgeParitySemantics::from_authoritative_sources(declaration, lowering);
    validate_parity_sources(declaration, lowering, activation, &witness, &semantics)?;

    let parity_class = parity_class_for_family(declaration.family());
    let comparison_width =
        SubscriptionBridgeParityWidth::new(3, lowering.bridge_slices().len(), 1, 1);
    let receipt = BridgeParityReceipt::new(
        *witness.assembly_posture(),
        parity_class,
        comparison_width,
        0,
    );
    let counters = QuerySubscriptionBridgeParityCounters::admitted(parity_class);
    let counter_snapshot = counters.digest();
    let comparison = QuerySubscriptionBridgeParityComparison {
        parity_class,
        query_declaration_digest: declaration.declaration_digest().as_str().to_string(),
        bridge_declaration_digest: lowering.bridge_declaration_for_reporting().to_string(),
        witness_digest: witness.witness_digest().to_string(),
        activation_digest: activation.activation_for_reporting().to_string(),
        comparison_digest: hash_parts(&[
            "query_subscription_bridge_parity_comparison_v1".to_string(),
            parity_class.as_str().to_string(),
            declaration.declaration_digest().as_str().to_string(),
            lowering.bridge_declaration_for_reporting().to_string(),
            witness.witness_digest().to_string(),
            activation.activation_for_reporting().to_string(),
        ]),
    };
    let explanation_digest = hash_parts(&[
        "query_subscription_bridge_parity_explanation_v1".to_string(),
        comparison.comparison_digest().to_string(),
        witness.witness_digest().to_string(),
        receipt.digest().to_string(),
        counter_snapshot.clone(),
    ]);

    Ok((
        QuerySubscriptionBridgeParityExplanation {
            comparison,
            query_family_label: semantics.query_family_label,
            declaration_family_label: semantics.declaration_family_label,
            bridge_family_label: semantics.bridge_family_label,
            bridge_slice_labels: semantics.bridge_slice_labels,
            basis_posture_label: semantics.basis_posture_label,
            signal_strategy_class_label: semantics.signal_strategy_class_label,
            counter_snapshot,
            explanation_digest,
            counters,
            witness,
        },
        receipt,
    ))
}

fn validate_parity_sources(
    declaration: &QuerySubscriptionDeclarationArtifact,
    lowering: &BridgeSubscriptionLoweringPlan,
    activation: &SubscriptionActivationInput,
    witness: &QuerySubscriptionManualBridgeWitness,
    semantics: &CanonicalBridgeParitySemantics,
) -> Result<(), QuerySubscriptionBridgeParityError> {
    if declaration.declaration_digest().as_str() != witness.query_declaration_digest()
        || lowering.query_declaration_for_reporting() != witness.query_declaration_digest()
        || activation.query_declaration_for_reporting() != witness.query_declaration_digest()
    {
        return Err(QuerySubscriptionBridgeParityError::new(
            QuerySubscriptionBridgeParityFailure::new(
                QuerySubscriptionBridgeParityFailureKind::DeclarationMismatch,
                QuerySubscriptionBridgeParityClass::DeniedSourceMismatch,
                "bridge parity explanation requires declaration, lowering, activation, and witness to preserve canonical declaration identity",
                witness.query_declaration_digest(),
                &[
                    format!("declaration:{}", declaration.declaration_digest().as_str()),
                    format!("lowering:{}", lowering.query_declaration_for_reporting()),
                    format!("activation:{}", activation.query_declaration_for_reporting()),
                    format!("witness:{}", witness.query_declaration_digest()),
                ],
            ),
            QuerySubscriptionBridgeParityCounters::denied(),
        ));
    }

    if activation.activation_for_reporting() != witness.activation_digest() {
        return Err(QuerySubscriptionBridgeParityError::new(
            QuerySubscriptionBridgeParityFailure::new(
                QuerySubscriptionBridgeParityFailureKind::ActivationMismatch,
                QuerySubscriptionBridgeParityClass::DeniedSourceMismatch,
                "bridge parity explanation requires activation and witness to preserve the same runtime activation identity",
                witness.activation_digest(),
                &[
                    format!("activation:{}", activation.activation_for_reporting()),
                    format!("witness:{}", witness.activation_digest()),
                ],
            ),
            QuerySubscriptionBridgeParityCounters::denied(),
        ));
    }

    if declaration.family().as_str() != witness.query_family_label()
        || declaration.family().as_str() != witness.declaration_family_label()
        || semantics.query_family_label != witness.query_family_label()
        || semantics.declaration_family_label != witness.declaration_family_label()
    {
        return Err(QuerySubscriptionBridgeParityError::new(
            QuerySubscriptionBridgeParityFailure::new(
                QuerySubscriptionBridgeParityFailureKind::DeclarationMismatch,
                QuerySubscriptionBridgeParityClass::DeniedSourceMismatch,
                "bridge parity explanation requires witness family labels to preserve canonical query and declaration family semantics",
                witness.query_declaration_digest(),
                &[
                    format!("declaration_family:{}", declaration.family().as_str()),
                    format!("witness_query_family:{}", witness.query_family_label()),
                    format!(
                        "witness_declaration_family:{}",
                        witness.declaration_family_label()
                    ),
                ],
            ),
            QuerySubscriptionBridgeParityCounters::denied(),
        ));
    }

    if lowering.bridge_declaration_for_reporting() != witness.bridge_declaration_digest()
        || activation.bridge_declaration_for_reporting() != witness.bridge_declaration_digest()
    {
        return Err(QuerySubscriptionBridgeParityError::new(
            QuerySubscriptionBridgeParityFailure::new(
                QuerySubscriptionBridgeParityFailureKind::BridgeMismatch,
                QuerySubscriptionBridgeParityClass::DeniedSourceMismatch,
                "bridge parity explanation requires lowering, activation, and witness to preserve bridge declaration identity",
                witness.bridge_declaration_digest(),
                &[
                    format!("lowering:{}", lowering.bridge_declaration_for_reporting()),
                    format!("activation:{}", activation.bridge_declaration_for_reporting()),
                    format!("witness:{}", witness.bridge_declaration_digest()),
                ],
            ),
            QuerySubscriptionBridgeParityCounters::denied(),
        ));
    }

    if lowering.bridge_family().as_str() != witness.bridge_family_label()
        || semantics.bridge_family_label != witness.bridge_family_label()
        || semantics.bridge_slice_labels.as_slice() != witness.bridge_slice_labels()
    {
        return Err(QuerySubscriptionBridgeParityError::new(
            QuerySubscriptionBridgeParityFailure::new(
                QuerySubscriptionBridgeParityFailureKind::BridgeMismatch,
                QuerySubscriptionBridgeParityClass::DeniedSourceMismatch,
                "bridge parity explanation requires witness bridge family and slice labels to preserve canonical lowering semantics",
                witness.bridge_declaration_digest(),
                &[
                    format!("lowering_bridge_family:{}", lowering.bridge_family().as_str()),
                    format!("witness_bridge_family:{}", witness.bridge_family_label()),
                    format!(
                        "lowering_bridge_slices:{}",
                        semantics.bridge_slice_labels.join("|")
                    ),
                    format!(
                        "witness_bridge_slices:{}",
                        witness.bridge_slice_labels().join("|")
                    ),
                ],
            ),
            QuerySubscriptionBridgeParityCounters::denied(),
        ));
    }

    if lowering.basis_request().digest() != witness.basis_binding_digest()
        || activation.basis_binding_for_reporting() != witness.basis_binding_digest()
    {
        return Err(QuerySubscriptionBridgeParityError::new(
            QuerySubscriptionBridgeParityFailure::new(
                QuerySubscriptionBridgeParityFailureKind::BasisMismatch,
                QuerySubscriptionBridgeParityClass::DeniedSourceMismatch,
                "bridge parity explanation requires lowering, activation, and witness to preserve basis request identity",
                witness.basis_binding_digest(),
                &[
                    format!("lowering:{}", lowering.basis_request().digest()),
                    format!("activation:{}", activation.basis_binding_for_reporting()),
                    format!("witness:{}", witness.basis_binding_digest()),
                ],
            ),
            QuerySubscriptionBridgeParityCounters::denied(),
        ));
    }

    if declaration.basis_posture().as_str() != witness.basis_posture_label()
        || semantics.basis_posture_label != witness.basis_posture_label()
    {
        return Err(QuerySubscriptionBridgeParityError::new(
            QuerySubscriptionBridgeParityFailure::new(
                QuerySubscriptionBridgeParityFailureKind::BasisMismatch,
                QuerySubscriptionBridgeParityClass::DeniedSourceMismatch,
                "bridge parity explanation requires witness basis posture labels to preserve canonical declaration semantics",
                witness.basis_binding_digest(),
                &[
                    format!("declaration_basis:{}", declaration.basis_posture().as_str()),
                    format!("witness_basis:{}", witness.basis_posture_label()),
                ],
            ),
            QuerySubscriptionBridgeParityCounters::denied(),
        ));
    }

    if lowering.signal_strategy_request().digest() != witness.signal_strategy_digest()
        || activation.signal_strategy_for_reporting() != witness.signal_strategy_digest()
    {
        return Err(QuerySubscriptionBridgeParityError::new(
            QuerySubscriptionBridgeParityFailure::new(
                QuerySubscriptionBridgeParityFailureKind::SignalStrategyMismatch,
                QuerySubscriptionBridgeParityClass::DeniedSourceMismatch,
                "bridge parity explanation requires lowering, activation, and witness to preserve signal strategy identity",
                witness.signal_strategy_digest(),
                &[
                    format!("lowering:{}", lowering.signal_strategy_request().digest()),
                    format!("activation:{}", activation.signal_strategy_for_reporting()),
                    format!("witness:{}", witness.signal_strategy_digest()),
                ],
            ),
            QuerySubscriptionBridgeParityCounters::denied(),
        ));
    }

    if lowering.signal_strategy_request().request_kind().as_str()
        != witness.signal_strategy_class_label()
        || semantics.signal_strategy_class_label != witness.signal_strategy_class_label()
    {
        return Err(QuerySubscriptionBridgeParityError::new(
            QuerySubscriptionBridgeParityFailure::new(
                QuerySubscriptionBridgeParityFailureKind::SignalStrategyMismatch,
                QuerySubscriptionBridgeParityClass::DeniedSourceMismatch,
                "bridge parity explanation requires witness signal strategy labels to preserve canonical lowering semantics",
                witness.signal_strategy_digest(),
                &[
                    format!(
                        "lowering_signal_strategy:{}",
                        lowering.signal_strategy_request().request_kind().as_str()
                    ),
                    format!(
                        "witness_signal_strategy:{}",
                        witness.signal_strategy_class_label()
                    ),
                ],
            ),
            QuerySubscriptionBridgeParityCounters::denied(),
        ));
    }

    Ok(())
}

fn parity_class_for_family(family: &QuerySubscriptionFamily) -> QuerySubscriptionBridgeParityClass {
    match family {
        QuerySubscriptionFamily::InspectorDetailExact
        | QuerySubscriptionFamily::GroupedCollectionMembership
        | QuerySubscriptionFamily::BoundedMaterialization => {
            QuerySubscriptionBridgeParityClass::FamilyDistinctBridgeShared
        }
        QuerySubscriptionFamily::DetailExact | QuerySubscriptionFamily::CollectionMembership => {
            QuerySubscriptionBridgeParityClass::ExactParity
        }
    }
}
