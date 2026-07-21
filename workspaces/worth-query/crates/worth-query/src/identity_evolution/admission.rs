use crate::identity::FailureDigest;

use super::{
    contracts::IdentityEvolutionComplexityContract,
    performance::{
        IdentityEvolutionBudgetClass, IdentityEvolutionCostClass, IdentityEvolutionPredictionReport,
    },
    request::{
        CorrespondenceIdentityComparison, IdentityEvolutionComparisonBasisFamily,
        IdentityEvolutionQueryContext, LineageTraversalDescriptor,
    },
    synthetic::IdentityEvolutionSyntheticScenario,
    LineageTraversalFamily,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum IdentityEvolutionAdmissionFailureClass {
    UnsupportedQueryFamily,
    UnsupportedLineageTraversalFamily,
    MissingLineageAnchor,
    UnsupportedComparisonBasisFamily,
    AdvisoryAsAuthoritativeForbidden,
    BranchCrossingContinuityForbidden,
    ComparisonBasisPairingRequired,
}

impl IdentityEvolutionAdmissionFailureClass {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::UnsupportedQueryFamily => "unsupported_query_family",
            Self::UnsupportedLineageTraversalFamily => "unsupported_lineage_traversal_family",
            Self::MissingLineageAnchor => "missing_lineage_anchor",
            Self::UnsupportedComparisonBasisFamily => "unsupported_comparison_basis_family",
            Self::AdvisoryAsAuthoritativeForbidden => "advisory_as_authoritative_forbidden",
            Self::BranchCrossingContinuityForbidden => "branch_crossing_continuity_forbidden",
            Self::ComparisonBasisPairingRequired => "comparison_basis_pairing_required",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IdentityEvolutionAdmissionError {
    failure_class: IdentityEvolutionAdmissionFailureClass,
    message: &'static str,
    failure_digest: FailureDigest,
}

impl IdentityEvolutionAdmissionError {
    pub fn failure_class(&self) -> &IdentityEvolutionAdmissionFailureClass {
        &self.failure_class
    }

    pub fn message(&self) -> &'static str {
        self.message
    }

    pub fn failure_digest(&self) -> &FailureDigest {
        &self.failure_digest
    }

    fn new(
        failure_class: IdentityEvolutionAdmissionFailureClass,
        message: &'static str,
        context: &IdentityEvolutionQueryContext,
    ) -> Self {
        let failure_digest = FailureDigest::from_parts(&[
            format!("failure_class:{}", failure_class.as_str()),
            format!("query_digest:{}", context.query_digest().as_str()),
            format!("basis_digest:{}", context.basis_digest().as_str()),
            format!("message:{message}"),
        ]);
        Self {
            failure_class,
            message,
            failure_digest,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) enum AdmittedIdentityEvolutionQueryShape {
    LineageTraversal {
        traversal_descriptor: LineageTraversalDescriptor,
    },
    CorrespondenceIdentityComparison {
        comparison_basis_family: IdentityEvolutionComparisonBasisFamily,
        left_basis_digest: crate::identity::BasisDigest,
        right_basis_digest: crate::identity::BasisDigest,
        comparison: CorrespondenceIdentityComparison,
    },
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AdmittedIdentityEvolutionQuery {
    query_context: IdentityEvolutionQueryContext,
    shape: AdmittedIdentityEvolutionQueryShape,
    complexity_contract: IdentityEvolutionComplexityContract,
    prediction_report: IdentityEvolutionPredictionReport,
    synthetic_scenario: IdentityEvolutionSyntheticScenario,
}

impl AdmittedIdentityEvolutionQuery {
    pub fn query_context(&self) -> &IdentityEvolutionQueryContext {
        &self.query_context
    }

    pub fn traversal_descriptor(&self) -> Option<&LineageTraversalDescriptor> {
        match &self.shape {
            AdmittedIdentityEvolutionQueryShape::LineageTraversal {
                traversal_descriptor,
            } => Some(traversal_descriptor),
            AdmittedIdentityEvolutionQueryShape::CorrespondenceIdentityComparison { .. } => None,
        }
    }

    pub fn correspondence_identity_comparison(
        &self,
    ) -> Option<(
        IdentityEvolutionComparisonBasisFamily,
        &crate::identity::BasisDigest,
        &crate::identity::BasisDigest,
        &CorrespondenceIdentityComparison,
    )> {
        match &self.shape {
            AdmittedIdentityEvolutionQueryShape::LineageTraversal { .. } => None,
            AdmittedIdentityEvolutionQueryShape::CorrespondenceIdentityComparison {
                comparison_basis_family,
                left_basis_digest,
                right_basis_digest,
                comparison,
            } => Some((
                *comparison_basis_family,
                left_basis_digest,
                right_basis_digest,
                comparison,
            )),
        }
    }

    pub fn complexity_contract(&self) -> &IdentityEvolutionComplexityContract {
        &self.complexity_contract
    }

    pub fn prediction_report(&self) -> &IdentityEvolutionPredictionReport {
        &self.prediction_report
    }

    pub(crate) fn synthetic_scenario(&self) -> IdentityEvolutionSyntheticScenario {
        self.synthetic_scenario
    }

    pub(crate) fn new(
        query_context: IdentityEvolutionQueryContext,
        shape: AdmittedIdentityEvolutionQueryShape,
        complexity_contract: IdentityEvolutionComplexityContract,
        prediction_report: IdentityEvolutionPredictionReport,
        synthetic_scenario: IdentityEvolutionSyntheticScenario,
    ) -> Self {
        Self {
            query_context,
            shape,
            complexity_contract,
            prediction_report,
            synthetic_scenario,
        }
    }
}

pub fn admit_identity_evolution_query(
    query_context: IdentityEvolutionQueryContext,
) -> Result<AdmittedIdentityEvolutionQuery, IdentityEvolutionAdmissionError> {
    admit_identity_evolution_query_for_scenario(
        query_context,
        IdentityEvolutionSyntheticScenario::Standard,
    )
}

pub(crate) fn admit_identity_evolution_query_for_scenario(
    query_context: IdentityEvolutionQueryContext,
    synthetic_scenario: IdentityEvolutionSyntheticScenario,
) -> Result<AdmittedIdentityEvolutionQuery, IdentityEvolutionAdmissionError> {
    if let Some(descriptor) = query_context.lineage_traversal_descriptor().cloned() {
        return admit_lineage_traversal(query_context, descriptor, synthetic_scenario);
    }
    if let Some((basis_family, left_basis_digest, right_basis_digest, comparison)) = query_context
        .correspondence_identity_comparison_descriptor()
        .map(
            |(basis_family, left_basis_digest, right_basis_digest, comparison)| {
                (
                    basis_family,
                    left_basis_digest.clone(),
                    right_basis_digest.clone(),
                    comparison.clone(),
                )
            },
        )
    {
        return admit_comparison(
            query_context,
            basis_family,
            left_basis_digest,
            right_basis_digest,
            comparison,
            synthetic_scenario,
        );
    }

    Err(IdentityEvolutionAdmissionError::new(
        IdentityEvolutionAdmissionFailureClass::UnsupportedQueryFamily,
        "identity evolution admission requires a closed lineage or comparison request family",
        &query_context,
    ))
}

fn admit_lineage_traversal(
    query_context: IdentityEvolutionQueryContext,
    descriptor: LineageTraversalDescriptor,
    synthetic_scenario: IdentityEvolutionSyntheticScenario,
) -> Result<AdmittedIdentityEvolutionQuery, IdentityEvolutionAdmissionError> {
    if descriptor.anchor_identity().trim().is_empty() {
        return Err(IdentityEvolutionAdmissionError::new(
            IdentityEvolutionAdmissionFailureClass::MissingLineageAnchor,
            "lineage traversal admission requires one admitted anchor identity",
            &query_context,
        ));
    }
    if synthetic_scenario == IdentityEvolutionSyntheticScenario::UnsupportedLineageTraversal {
        return Err(IdentityEvolutionAdmissionError::new(
            IdentityEvolutionAdmissionFailureClass::UnsupportedLineageTraversalFamily,
            "lineage traversal admission requires one supported direct traversal family",
            &query_context,
        ));
    }

    let family = descriptor.family();
    let complexity_contract = IdentityEvolutionComplexityContract::direct_lineage(family);
    let prediction_report = IdentityEvolutionPredictionReport::zero_work(
        cost_class_for_lineage(family),
        IdentityEvolutionBudgetClass::SingleAnchorDirectOnly,
    );

    Ok(AdmittedIdentityEvolutionQuery::new(
        query_context,
        AdmittedIdentityEvolutionQueryShape::LineageTraversal {
            traversal_descriptor: descriptor,
        },
        complexity_contract,
        prediction_report,
        synthetic_scenario,
    ))
}

fn admit_comparison(
    query_context: IdentityEvolutionQueryContext,
    basis_family: IdentityEvolutionComparisonBasisFamily,
    left_basis_digest: crate::identity::BasisDigest,
    right_basis_digest: crate::identity::BasisDigest,
    comparison: CorrespondenceIdentityComparison,
    synthetic_scenario: IdentityEvolutionSyntheticScenario,
) -> Result<AdmittedIdentityEvolutionQuery, IdentityEvolutionAdmissionError> {
    if left_basis_digest == right_basis_digest
        && basis_family != IdentityEvolutionComparisonBasisFamily::InstalledOperation
    {
        return Err(IdentityEvolutionAdmissionError::new(
            IdentityEvolutionAdmissionFailureClass::ComparisonBasisPairingRequired,
            "comparison admission requires two distinct admitted bases",
            &query_context,
        ));
    }
    if synthetic_scenario == IdentityEvolutionSyntheticScenario::UnsupportedComparisonFamily {
        return Err(IdentityEvolutionAdmissionError::new(
            IdentityEvolutionAdmissionFailureClass::UnsupportedComparisonBasisFamily,
            "comparison admission requires one admitted correspondence family",
            &query_context,
        ));
    }

    let complexity_contract =
        IdentityEvolutionComplexityContract::correspondence_identity_comparison(basis_family);
    let prediction_report = IdentityEvolutionPredictionReport::zero_work(
        IdentityEvolutionCostClass::ConstantMetadataComparison,
        IdentityEvolutionBudgetClass::FixedBasisComparisonOnly,
    );

    Ok(AdmittedIdentityEvolutionQuery::new(
        query_context,
        AdmittedIdentityEvolutionQueryShape::CorrespondenceIdentityComparison {
            comparison_basis_family: basis_family,
            left_basis_digest,
            right_basis_digest,
            comparison,
        },
        complexity_contract,
        prediction_report,
        synthetic_scenario,
    ))
}

fn cost_class_for_lineage(family: LineageTraversalFamily) -> IdentityEvolutionCostClass {
    match family {
        LineageTraversalFamily::DirectPredecessor
        | LineageTraversalFamily::DirectSuccessor
        | LineageTraversalFamily::DirectReplacement
        | LineageTraversalFamily::DirectSplitSuccessors
        | LineageTraversalFamily::DirectMergeSuccessor
        | LineageTraversalFamily::GeneratedIdentity
        | LineageTraversalFamily::RetiredIdentity
        | LineageTraversalFamily::BranchLocalDirectEvolution => {
            IdentityEvolutionCostClass::ConstantDirectLookup
        }
    }
}
