use forge_proof::TransitionOutcome;

use crate::{
    StoreAdmittedSecurityScope, StoreAuthenticityRequirement, StoreCurrentSecurityScopeWitnessSet,
    StoreCustodyPosture, StoreKeyVersionPosture, StoreSecurityScopeAdmissionCounterSnapshot,
    StoreSecurityScopeAdmissionDeferred, StoreSecurityScopeAdmissionDenial,
    StoreSecurityScopeAdmissionFailure, StoreSecurityScopeAdmissionRebindRequired,
    StoreSecurityScopeAdmissionReceipt, StoreSecurityScopeAdmissionRequest,
    StoreSecurityScopeAdmissionStale, StoreSecurityScopeDeclarationProvenance,
    StoreSecurityScopeIdentity,
};

use crate::scope::security_scope_counters::StoreSecurityScopeAdmissionCounters;

pub type StoreSecurityScopeAdmissionOutcome = TransitionOutcome<
    StoreAdmittedSecurityScope,
    StoreSecurityScopeAdmissionDenial,
    StoreSecurityScopeAdmissionDeferred,
    StoreSecurityScopeAdmissionStale,
    StoreSecurityScopeAdmissionRebindRequired,
    StoreSecurityScopeAdmissionFailure,
>;

#[derive(Debug, PartialEq, Eq)]
pub struct StoreSecurityScopeAdmissionEvaluation {
    outcome: StoreSecurityScopeAdmissionOutcome,
    counters: StoreSecurityScopeAdmissionCounterSnapshot,
}

pub fn admit_store_security_scope(
    request: StoreSecurityScopeAdmissionRequest<'_>,
) -> StoreSecurityScopeAdmissionOutcome {
    evaluate_store_security_scope_admission(request).into_outcome()
}

pub fn evaluate_store_security_scope_admission(
    request: StoreSecurityScopeAdmissionRequest<'_>,
) -> StoreSecurityScopeAdmissionEvaluation {
    let mut counters = StoreSecurityScopeAdmissionCounters::start_request();

    if let Some(outcome) = reject_unreadmitted_declaration(&request, &mut counters) {
        return StoreSecurityScopeAdmissionEvaluation::new(outcome, counters.snapshot());
    }

    if let Some(outcome) = reject_physical_authority_drift(&request, &mut counters) {
        return StoreSecurityScopeAdmissionEvaluation::new(outcome, counters.snapshot());
    }

    if let Some(outcome) = reject_key_scope_mismatch(&request, &mut counters) {
        return StoreSecurityScopeAdmissionEvaluation::new(outcome, counters.snapshot());
    }

    if let Some(outcome) = reject_key_version_posture(request.key_version_posture(), &mut counters)
    {
        return StoreSecurityScopeAdmissionEvaluation::new(outcome, counters.snapshot());
    }

    if let Some(outcome) = reject_tenant_scope_mismatch(&request, &mut counters) {
        return StoreSecurityScopeAdmissionEvaluation::new(outcome, counters.snapshot());
    }

    let authenticity_requirement = match admit_authenticity_requirement(&request, &mut counters) {
        Ok(requirement) => requirement,
        Err(outcome) => {
            return StoreSecurityScopeAdmissionEvaluation::new(outcome, counters.snapshot());
        }
    };

    let custody_posture = match admit_custody_posture(&request, &mut counters) {
        Ok(posture) => posture,
        Err(outcome) => {
            return StoreSecurityScopeAdmissionEvaluation::new(outcome, counters.snapshot());
        }
    };

    let declaration = request.declaration();
    let identity = StoreSecurityScopeIdentity::from_physical_security_scope(
        declaration.physical_witness(),
        declaration.key_scope(),
        declaration.key_version_posture(),
        declaration.tenant_scope(),
        authenticity_requirement,
        custody_posture,
    );
    counters.record_witnesses_issued();
    let snapshot = counters.snapshot();
    let receipt = StoreSecurityScopeAdmissionReceipt::new(
        identity,
        request.basis().proof_progression_identity(),
        snapshot,
    );
    let witnesses = StoreCurrentSecurityScopeWitnessSet::new(identity);

    StoreSecurityScopeAdmissionEvaluation::new(
        TransitionOutcome::success(StoreAdmittedSecurityScope::new(witnesses, receipt)),
        snapshot,
    )
}

impl StoreSecurityScopeAdmissionEvaluation {
    const fn new(
        outcome: StoreSecurityScopeAdmissionOutcome,
        counters: StoreSecurityScopeAdmissionCounterSnapshot,
    ) -> Self {
        Self { outcome, counters }
    }

    pub fn into_outcome(self) -> StoreSecurityScopeAdmissionOutcome {
        self.outcome
    }

    pub const fn counters(&self) -> StoreSecurityScopeAdmissionCounterSnapshot {
        self.counters
    }
}

fn reject_unreadmitted_declaration(
    request: &StoreSecurityScopeAdmissionRequest<'_>,
    counters: &mut StoreSecurityScopeAdmissionCounters,
) -> Option<StoreSecurityScopeAdmissionOutcome> {
    match request.declaration().provenance() {
        StoreSecurityScopeDeclarationProvenance::NativeStoreDeclaration
        | StoreSecurityScopeDeclarationProvenance::StoreReadmitted => None,
        StoreSecurityScopeDeclarationProvenance::DeserializedUnadmitted => {
            counters.record_denial();
            counters.record_readmission_required();
            Some(TransitionOutcome::denied(
                StoreSecurityScopeAdmissionDenial::DeserializedSecurityScopeRequiresReadmission,
            ))
        }
        StoreSecurityScopeDeclarationProvenance::ReplayedAdmissionEvidence => {
            counters.record_denial();
            counters.record_replayed_admission_evidence();
            Some(TransitionOutcome::denied(
                StoreSecurityScopeAdmissionDenial::ReplayedAdmissionEvidence,
            ))
        }
    }
}

fn reject_physical_authority_drift(
    request: &StoreSecurityScopeAdmissionRequest<'_>,
    counters: &mut StoreSecurityScopeAdmissionCounters,
) -> Option<StoreSecurityScopeAdmissionOutcome> {
    counters.check_physical_binding();
    if request.declaration().physical_witness() == request.current_authority().physical_witness() {
        None
    } else {
        counters.record_denial();
        counters.record_wrong_physical_scope();
        Some(TransitionOutcome::denied(
            StoreSecurityScopeAdmissionDenial::WrongPhysicalSecurityScope,
        ))
    }
}

fn reject_key_scope_mismatch(
    request: &StoreSecurityScopeAdmissionRequest<'_>,
    counters: &mut StoreSecurityScopeAdmissionCounters,
) -> Option<StoreSecurityScopeAdmissionOutcome> {
    counters.check_key_scope();
    if request.declaration().key_scope() == request.basis().expectation().key_scope() {
        None
    } else {
        counters.record_denial();
        counters.record_wrong_key_scope();
        Some(TransitionOutcome::denied(
            StoreSecurityScopeAdmissionDenial::WrongKeyScope,
        ))
    }
}

fn reject_key_version_posture(
    posture: StoreKeyVersionPosture,
    counters: &mut StoreSecurityScopeAdmissionCounters,
) -> Option<StoreSecurityScopeAdmissionOutcome> {
    counters.check_key_version();
    match posture {
        StoreKeyVersionPosture::Current => None,
        StoreKeyVersionPosture::Stale => {
            counters.record_stale_key_posture();
            Some(TransitionOutcome::stale(
                StoreSecurityScopeAdmissionStale::StaleKeyVersionPosture(posture),
            ))
        }
        StoreKeyVersionPosture::RebindRequired => {
            counters.record_rebind_required_key_posture();
            Some(TransitionOutcome::rebind_required(
                StoreSecurityScopeAdmissionRebindRequired::KeyVersionRebindRequired(posture),
            ))
        }
        StoreKeyVersionPosture::Unsupported => {
            counters.record_denial();
            counters.record_unsupported_posture();
            Some(TransitionOutcome::denied(
                StoreSecurityScopeAdmissionDenial::UnsupportedKeyVersionPosture,
            ))
        }
        StoreKeyVersionPosture::Unavailable => {
            counters.record_denial();
            counters.record_unavailable_posture();
            Some(TransitionOutcome::denied(
                StoreSecurityScopeAdmissionDenial::UnavailableKeyVersionPosture,
            ))
        }
        StoreKeyVersionPosture::Denied => {
            counters.record_denial();
            Some(TransitionOutcome::denied(
                StoreSecurityScopeAdmissionDenial::DeniedKeyVersionPosture,
            ))
        }
    }
}

fn reject_tenant_scope_mismatch(
    request: &StoreSecurityScopeAdmissionRequest<'_>,
    counters: &mut StoreSecurityScopeAdmissionCounters,
) -> Option<StoreSecurityScopeAdmissionOutcome> {
    counters.check_tenant_scope();
    if request.declaration().tenant_scope() == request.basis().expectation().tenant_scope() {
        None
    } else {
        counters.record_denial();
        counters.record_wrong_tenant_scope();
        Some(TransitionOutcome::denied(
            StoreSecurityScopeAdmissionDenial::WrongTenantScope,
        ))
    }
}

fn admit_authenticity_requirement(
    request: &StoreSecurityScopeAdmissionRequest<'_>,
    counters: &mut StoreSecurityScopeAdmissionCounters,
) -> Result<StoreAuthenticityRequirement, StoreSecurityScopeAdmissionOutcome> {
    counters.check_authenticity_requirement();
    let Some(requirement) = request.declaration().authenticity_requirement() else {
        counters.record_denial();
        counters.record_missing_authenticity_requirement();
        return Err(TransitionOutcome::denied(
            StoreSecurityScopeAdmissionDenial::MissingAuthenticityRequirement,
        ));
    };
    if requirement == request.basis().expectation().authenticity_requirement() {
        Ok(requirement)
    } else {
        counters.record_denial();
        counters.record_unsupported_authenticity_requirement();
        counters.record_unsupported_posture();
        Err(TransitionOutcome::denied(
            StoreSecurityScopeAdmissionDenial::UnsupportedAuthenticityRequirement,
        ))
    }
}

fn admit_custody_posture(
    request: &StoreSecurityScopeAdmissionRequest<'_>,
    counters: &mut StoreSecurityScopeAdmissionCounters,
) -> Result<StoreCustodyPosture, StoreSecurityScopeAdmissionOutcome> {
    counters.check_custody_posture();
    let Some(posture) = request.declaration().custody_posture() else {
        counters.record_denial();
        counters.record_missing_custody_posture();
        return Err(TransitionOutcome::denied(
            StoreSecurityScopeAdmissionDenial::MissingCustodyPosture,
        ));
    };
    match posture {
        StoreCustodyPosture::InternalStoreCustody
        | StoreCustodyPosture::ExportPrepared
        | StoreCustodyPosture::Readmitted => {
            if posture == request.basis().expectation().custody_posture() {
                Ok(posture)
            } else {
                counters.record_denial();
                Err(TransitionOutcome::denied(
                    StoreSecurityScopeAdmissionDenial::WrongCustodyPosture,
                ))
            }
        }
        StoreCustodyPosture::ExportedOutOfCustody => {
            counters.record_denial();
            counters.record_readmission_required();
            Err(TransitionOutcome::denied(
                StoreSecurityScopeAdmissionDenial::ExportedCustodyRequiresReadmission,
            ))
        }
        StoreCustodyPosture::ImportedUnreadmitted => {
            counters.record_denial();
            counters.record_readmission_required();
            Err(TransitionOutcome::denied(
                StoreSecurityScopeAdmissionDenial::ImportedCustodyRequiresReadmission,
            ))
        }
        StoreCustodyPosture::CustodyUnavailable => {
            counters.record_denial();
            counters.record_unavailable_posture();
            Err(TransitionOutcome::denied(
                StoreSecurityScopeAdmissionDenial::UnavailableCustodyPosture,
            ))
        }
        StoreCustodyPosture::CustodyDenied => {
            counters.record_denial();
            Err(TransitionOutcome::denied(
                StoreSecurityScopeAdmissionDenial::DeniedCustodyPosture,
            ))
        }
        StoreCustodyPosture::CustodyUnsupported => {
            counters.record_denial();
            counters.record_unsupported_posture();
            Err(TransitionOutcome::denied(
                StoreSecurityScopeAdmissionDenial::UnsupportedCustodyPosture,
            ))
        }
    }
}

pub const fn admission_counter_snapshot(
    admitted: &StoreAdmittedSecurityScope,
) -> StoreSecurityScopeAdmissionCounterSnapshot {
    admitted.receipt().counters()
}
