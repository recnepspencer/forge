use crate::basis_lifecycle::BasisFamily;
use crate::workflow::{admit_query_workflow_declaration, WorkflowAdmissionFailureClass};

use super::eligibility::{
    AdmittedEffectIntent, DeferredEffectEligibility, DeniedEffectEligibility, EffectEligibility,
    EffectEligibilityOutcome, RebindRequiredEffectEligibility,
};
use super::normalized::NormalizedEffectIntent;
use super::support::{support_decision_for, EffectSupportPosture};
use super::taxonomy::DeniedEffectEligibilityKind;

pub fn evaluate_effect_eligibility(normalized: NormalizedEffectIntent) -> EffectEligibilityOutcome {
    let support = support_decision_for(normalized.basis_family(), normalized.family());
    match support.posture() {
        EffectSupportPosture::Admitted => {
            admit_supported_effect(normalized, support.rows_consulted())
        }
        EffectSupportPosture::RebindRequired => {
            EffectEligibilityOutcome::RebindRequired(RebindRequiredEffectEligibility::new(
                &normalized,
                "preview-backed writeback must rebind to an authoritative basis before lowering",
                support.rows_consulted(),
            ))
        }
        EffectSupportPosture::Deferred => {
            EffectEligibilityOutcome::Deferred(DeferredEffectEligibility::new(
                &normalized,
                "effect family is explicitly deferred to a later milestone on this basis family",
                support.rows_consulted(),
            ))
        }
        EffectSupportPosture::Denied | EffectSupportPosture::Unsupported => {
            EffectEligibilityOutcome::Denied(denied_effect(normalized, support.rows_consulted()))
        }
    }
}

fn admit_supported_effect(
    normalized: NormalizedEffectIntent,
    support_row_count: usize,
) -> EffectEligibilityOutcome {
    match admit_query_workflow_declaration(
        normalized.workflow_binding(),
        normalized.workflow_request().clone(),
    ) {
        Ok(workflow_declaration) => EffectEligibilityOutcome::Admitted(EffectEligibility::new(
            normalized,
            workflow_declaration,
            support_row_count,
        )),
        Err(error) => match error.failure_class() {
            WorkflowAdmissionFailureClass::ExplicitRebindRequired => {
                EffectEligibilityOutcome::RebindRequired(RebindRequiredEffectEligibility::new(
                    &normalized,
                    error.message(),
                    support_row_count,
                ))
            }
            _ => EffectEligibilityOutcome::Denied(DeniedEffectEligibility::new(
                DeniedEffectEligibilityKind::WorkflowAdmissionDenied,
                &normalized,
                error.message(),
                support_row_count,
            )),
        },
    }
}

fn denied_effect(
    normalized: NormalizedEffectIntent,
    support_row_count: usize,
) -> DeniedEffectEligibility {
    let message = match (normalized.basis_family(), normalized.family()) {
        (BasisFamily::TenantScoped | BasisFamily::PolicyScoped, super::EffectFamily::Merge) => {
            "merge effects require branch-authoritative basis rather than tenant or policy scoped mutation preparation"
        }
        _ => "effect family is not supported on this basis family",
    };
    DeniedEffectEligibility::new(
        DeniedEffectEligibilityKind::UnsupportedForBasisFamily,
        &normalized,
        message,
        support_row_count,
    )
}

pub fn admit_effect_intent(eligibility: EffectEligibility) -> AdmittedEffectIntent {
    AdmittedEffectIntent::new(eligibility)
}
