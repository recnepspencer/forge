use forge_query::facade::{
    ForgeQueryAdmittedConfiguredDomainHandle, ForgeQueryDomainOperatingContext,
    ForgeQueryOrdinaryNextStep, ForgeQueryOrdinaryOutcome, ForgeQueryOrdinaryPosture,
    ForgeQueryOrdinaryPostureKind,
};
use worth_math::arithmetic::precision::PrecisionEscalation;
use worth_math::sign::CertifiedTriSign;
use worth_math::{MathError, NumericContractKind};

use crate::bindings::query_native_planar_predicate::authoring::PlanarPredicateAuthorityEntry;
use crate::bindings::query_native_planar_predicate::domain::PlanarPredicateAuthorityQueryDomain;
use crate::planar_contracts::predicate_authority::{
    digest_parts, evaluate_planar_predicate_authority, PlanarPredicateAuthorityDenial,
    PlanarPredicateCoincidencePolicy, PlanarPredicateEvaluationFailureKind,
    PlanarPredicateFactReceipt, PlanarPredicateInputBasis, PlanarPredicateKind,
    PlanarPredicateMathEvaluation, PlanarPredicatePerformanceCounters,
};

#[derive(Clone, Debug, PartialEq)]
pub enum PlanarPredicateAuthorityFactError {
    OutcomeNotBound {
        kind: ForgeQueryOrdinaryPostureKind,
        reason: String,
        next_step: ForgeQueryOrdinaryNextStep,
    },
    PredicateEvaluation {
        kind: PlanarPredicateEvaluationFailureKind,
        reason: String,
    },
    PredicateUncertain {
        denial: PlanarPredicateAuthorityDenial,
        certified_sign: CertifiedTriSign,
        precision_escalation: PrecisionEscalation,
        counters: PlanarPredicatePerformanceCounters,
    },
}

impl PlanarPredicateAuthorityFactError {
    fn outcome_not_bound(posture: &ForgeQueryOrdinaryPosture) -> Self {
        Self::OutcomeNotBound {
            kind: posture.kind(),
            reason: posture.reason().to_string(),
            next_step: posture.next_step(),
        }
    }
}

pub fn planar_predicate_authority_facts<C>(
    entry: &PlanarPredicateAuthorityEntry,
    handle: &ForgeQueryAdmittedConfiguredDomainHandle<PlanarPredicateAuthorityQueryDomain, C>,
) -> Result<PlanarPredicateFactReceipt, PlanarPredicateAuthorityFactError>
where
    C: ForgeQueryDomainOperatingContext<PlanarPredicateAuthorityQueryDomain>,
{
    match handle.orchestrate_declaration_entry_outcome(entry.clone()) {
        ForgeQueryOrdinaryOutcome::Bound(envelope) => {
            let basis = entry.case().input_basis();
            let evaluation =
                evaluate_planar_predicate_authority(entry.case().predicate_kind(), basis)
                    .map_err(predicate_evaluation_error)?;
            if basis.coincidence_policy()
                == PlanarPredicateCoincidencePolicy::DenyCertifiedZeroBeforeRepair
                && evaluation.certified_sign.is_zero()
            {
                return Err(predicate_uncertain_before_repair(evaluation));
            }
            Ok(receipt_from_bound_evaluation(
                entry.case().predicate_kind(),
                basis,
                &evaluation,
                envelope.declaration_digest(),
                &format!("{:?}", envelope.envelope_digest()),
            ))
        }
        ForgeQueryOrdinaryOutcome::Ambiguous(posture)
        | ForgeQueryOrdinaryOutcome::AspectConflict(posture)
        | ForgeQueryOrdinaryOutcome::AuthorityMismatch(posture)
        | ForgeQueryOrdinaryOutcome::BasisMismatch(posture)
        | ForgeQueryOrdinaryOutcome::Deferred(posture)
        | ForgeQueryOrdinaryOutcome::Denied(posture)
        | ForgeQueryOrdinaryOutcome::ExplicitNarrowingRequired(posture)
        | ForgeQueryOrdinaryOutcome::Failed(posture)
        | ForgeQueryOrdinaryOutcome::MissingRequiredAspect(posture)
        | ForgeQueryOrdinaryOutcome::RebindRequired(posture)
        | ForgeQueryOrdinaryOutcome::Refused(posture)
        | ForgeQueryOrdinaryOutcome::Stale(posture)
        | ForgeQueryOrdinaryOutcome::Unavailable(posture)
        | ForgeQueryOrdinaryOutcome::Unsupported(posture)
        | ForgeQueryOrdinaryOutcome::WrongHandle(posture)
        | ForgeQueryOrdinaryOutcome::WrongWorld(posture) => Err(
            PlanarPredicateAuthorityFactError::outcome_not_bound(&posture),
        ),
    }
}

fn predicate_uncertain_before_repair(
    evaluation: PlanarPredicateMathEvaluation,
) -> PlanarPredicateAuthorityFactError {
    let counters =
        PlanarPredicatePerformanceCounters::orient2d(evaluation.basis_digest_parts.len());
    PlanarPredicateAuthorityFactError::PredicateUncertain {
        denial: PlanarPredicateAuthorityDenial::CertifiedZeroDeniedBeforeRepair,
        certified_sign: evaluation.certified_sign,
        precision_escalation: evaluation.precision_escalation,
        counters,
    }
}

fn receipt_from_bound_evaluation(
    kind: PlanarPredicateKind,
    basis: &PlanarPredicateInputBasis,
    evaluation: &PlanarPredicateMathEvaluation,
    declaration_digest: &str,
    envelope_digest: &str,
) -> PlanarPredicateFactReceipt {
    let mut fact_parts = evaluation.basis_digest_parts.clone();
    fact_parts.push(format!("sign:{:?}", evaluation.certified_sign.sign()));
    fact_parts.push(format!(
        "resolved_at:{:?}",
        evaluation.precision_escalation.get_resolved_at()
    ));
    fact_parts.push(format!(
        "float_agreed:{}",
        evaluation.precision_escalation.get_float_agreed()
    ));
    fact_parts.push(format!(
        "expansion_length:{:?}",
        evaluation.precision_escalation.get_expansion_length()
    ));
    fact_parts.push(format!(
        "target:{}",
        evaluation.precision_escalation.get_target_triple()
    ));
    fact_parts.push(format!("declaration:{declaration_digest}"));
    fact_parts.push(format!("envelope:{envelope_digest}"));
    PlanarPredicateFactReceipt::new(
        kind,
        basis.clone(),
        evaluation.certified_sign,
        evaluation.precision_escalation.clone(),
        declaration_digest.to_string(),
        envelope_digest.to_string(),
        digest_parts(&fact_parts),
        PlanarPredicatePerformanceCounters::orient2d(evaluation.basis_digest_parts.len()),
    )
}

fn predicate_evaluation_error(error: MathError) -> PlanarPredicateAuthorityFactError {
    let kind = match error {
        MathError::NumericContractViolation {
            kind: NumericContractKind::FinitePoint2,
            ..
        } => PlanarPredicateEvaluationFailureKind::NonFiniteProjectedPoint2,
        _ => PlanarPredicateEvaluationFailureKind::CertifiedPredicateMathFailure,
    };
    PlanarPredicateAuthorityFactError::PredicateEvaluation {
        kind,
        reason: error.to_string(),
    }
}
