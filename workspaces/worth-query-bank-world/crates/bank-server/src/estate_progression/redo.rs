//! Production redo progression through ordinary mutation entry (R8.42 / R8.43).
//!
//! Intent is descriptive. Admission requires fresh authority and linear-lane
//! divergence policy. Progression re-enters ordinary disbursement — never a
//! parallel mutator and never authority from the proved undo alone.

use bank_domain::estate::EstateAction;
use worth_query_host::facade::admission::authenticated_principal::WorthQueryRequestScope;
use worth_query_host::facade::provisional_aftermath::{
    progress_admitted_redo, WorthQueryProvedUndo, WorthQueryRedoDenial, WorthQueryRedoIntent,
    WorthQueryRedoRecovery,
};

use super::{BankDisbursementRedoAdmission, BankEstateProgressionDenial};
use crate::{BankAuthenticatedPrincipal, BankIdentityRuntime, BankMutationCommitOutcome};

impl BankIdentityRuntime {
    /// Derive a descriptive redo intent bound to the current linear head.
    pub fn derive_redo_intent(
        &self,
        proved: &WorthQueryProvedUndo,
    ) -> Result<WorthQueryRedoIntent, BankEstateProgressionDenial> {
        self.application_runtime()
            .derive_redo_intent(proved)
            .map_err(|_| BankEstateProgressionDenial::Redo(WorthQueryRedoDenial::stale()))
    }

    /// Fresh redo admission through current authority (R8.43).
    ///
    /// Re-admits capability/operation first — the proved undo authorizes nothing
    /// about the current world.
    pub fn admit_redo_disbursement_recovery<'context>(
        &self,
        recovery: WorthQueryRedoRecovery,
        principal: &'context BankAuthenticatedPrincipal,
        request: &'context WorthQueryRequestScope,
        intent: &WorthQueryRedoIntent,
    ) -> Result<BankDisbursementRedoAdmission, BankEstateProgressionDenial> {
        let action = original_redo_disbursement(&recovery)?;
        let admission = self
            .admit_estate_disbursement(principal, action, request)
            .map_err(map_redo_admission_denial)?;
        let authority = self
            .application_runtime()
            .admit_recovery_effect_authority(recovery.handle(), &admission)
            .map_err(|denial| {
                map_redo_path_recovery_denial(BankEstateProgressionDenial::Recovery(denial))
            })?;
        let query = self
            .application_runtime()
            .admit_redo(recovery, &authority, intent)
            .map_err(BankEstateProgressionDenial::Redo)?;
        Ok(BankDisbursementRedoAdmission::new(query, admission))
    }

    /// Progress an admitted redo by re-entering ordinary disbursement.
    pub fn progress_redo_disbursement(
        &self,
        admission: BankDisbursementRedoAdmission,
    ) -> Result<BankMutationCommitOutcome, BankEstateProgressionDenial> {
        let (query, ordinary) = admission.into_parts();
        let handoff = progress_admitted_redo(query).map_err(BankEstateProgressionDenial::Redo)?;
        let idempotency = handoff.idempotency_binding();
        let program = self.materialize_estate_disbursement(ordinary, idempotency)?;
        Ok(self
            .application_runtime()
            .compare_and_commit_redo_application(program, idempotency, &handoff)
            .into())
    }

    // Relational owns branch lineage; Query co-commits only causal facts.
}

fn original_redo_disbursement(
    recovery: &WorthQueryRedoRecovery,
) -> Result<EstateAction, BankEstateProgressionDenial> {
    let action = recovery
        .handle()
        .binding()
        .original_input::<EstateAction>()
        .copied()
        .ok_or(BankEstateProgressionDenial::CommandInput(
            "retained redo input",
        ))?;
    match action {
        EstateAction::DisburseEstate(_) => Ok(action),
        _ => Err(BankEstateProgressionDenial::CommandInput(
            "retained redo disbursement input",
        )),
    }
}

fn map_redo_admission_denial(denial: BankEstateProgressionDenial) -> BankEstateProgressionDenial {
    match denial {
        BankEstateProgressionDenial::Authorization(_) => {
            BankEstateProgressionDenial::Redo(WorthQueryRedoDenial::newly_unauthorized())
        }
        other => other,
    }
}

fn map_redo_path_recovery_denial(
    denial: BankEstateProgressionDenial,
) -> BankEstateProgressionDenial {
    use worth_query_host::facade::primary_graph::WorthQueryRecoveryHandleDenialKind as K;
    match denial {
        BankEstateProgressionDenial::Recovery(inner) => match inner.kind() {
            K::Expired => BankEstateProgressionDenial::Redo(WorthQueryRedoDenial::stale()),
            K::AlreadyTerminal => {
                BankEstateProgressionDenial::Redo(WorthQueryRedoDenial::duplicate_redo())
            }
            K::CurrentPolicyDenied | K::FreshAuthorityDenied => {
                BankEstateProgressionDenial::Redo(WorthQueryRedoDenial::newly_unauthorized())
            }
            K::ForeignRuntime | K::ForeignPrincipal => {
                BankEstateProgressionDenial::Redo(WorthQueryRedoDenial::foreign_principal())
            }
            _ => BankEstateProgressionDenial::Recovery(inner),
        },
        other => other,
    }
}
