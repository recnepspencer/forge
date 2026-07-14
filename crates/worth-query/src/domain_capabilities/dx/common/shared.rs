use crate::domain_capabilities::denials::WorthQueryDomainCapabilityTransitionOutcome;
use crate::domain_capabilities::dx::checked::WorthQueryCheckedDomainCapabilityOutcome;
use crate::domain_capabilities::WorthQueryDomainCapabilityTargetKind;

pub(super) fn materialize_common_lane<Request, Eligible, Admitted, Ready, Success>(
    category: &'static str,
    target_kind: WorthQueryDomainCapabilityTargetKind,
    semantic_posture: &'static str,
    requested: Request,
    evaluate: impl FnOnce(Request) -> WorthQueryDomainCapabilityTransitionOutcome<Eligible>,
    admit: impl FnOnce(Eligible) -> WorthQueryDomainCapabilityTransitionOutcome<Admitted>,
    prepare: impl FnOnce(Admitted) -> WorthQueryDomainCapabilityTransitionOutcome<Ready>,
    materialize: impl FnOnce(Ready) -> WorthQueryDomainCapabilityTransitionOutcome<Success>,
) -> WorthQueryCheckedDomainCapabilityOutcome<Success> {
    let outcome = match evaluate(requested) {
        worth_proof::TransitionOutcome::Success(eligible) => match admit(eligible) {
            worth_proof::TransitionOutcome::Success(admitted) => match prepare(admitted) {
                worth_proof::TransitionOutcome::Success(ready) => materialize(ready),
                worth_proof::TransitionOutcome::Denied(denial) => {
                    worth_proof::TransitionOutcome::Denied(denial)
                }
                worth_proof::TransitionOutcome::Stale(stale) => {
                    worth_proof::TransitionOutcome::Stale(stale)
                }
                worth_proof::TransitionOutcome::RebindRequired(rebind) => {
                    worth_proof::TransitionOutcome::RebindRequired(rebind)
                }
                worth_proof::TransitionOutcome::Failed(failure) => {
                    worth_proof::TransitionOutcome::Failed(failure)
                }
                worth_proof::TransitionOutcome::Deferred(never) => match never {},
            },
            worth_proof::TransitionOutcome::Denied(denial) => {
                worth_proof::TransitionOutcome::Denied(denial)
            }
            worth_proof::TransitionOutcome::Stale(stale) => {
                worth_proof::TransitionOutcome::Stale(stale)
            }
            worth_proof::TransitionOutcome::RebindRequired(rebind) => {
                worth_proof::TransitionOutcome::RebindRequired(rebind)
            }
            worth_proof::TransitionOutcome::Failed(failure) => {
                worth_proof::TransitionOutcome::Failed(failure)
            }
            worth_proof::TransitionOutcome::Deferred(never) => match never {},
        },
        worth_proof::TransitionOutcome::Denied(denial) => {
            worth_proof::TransitionOutcome::Denied(denial)
        }
        worth_proof::TransitionOutcome::Stale(stale) => {
            worth_proof::TransitionOutcome::Stale(stale)
        }
        worth_proof::TransitionOutcome::RebindRequired(rebind) => {
            worth_proof::TransitionOutcome::RebindRequired(rebind)
        }
        worth_proof::TransitionOutcome::Failed(failure) => {
            worth_proof::TransitionOutcome::Failed(failure)
        }
        worth_proof::TransitionOutcome::Deferred(never) => match never {},
    };

    WorthQueryCheckedDomainCapabilityOutcome::from_transition_outcome(
        category,
        target_kind,
        semantic_posture,
        outcome,
    )
}

pub(super) fn qualify_semantic_code(
    authority: &crate::domain_installation::WorthQueryInstalledDomainAuthority,
    semantic_code: &str,
) -> String {
    format!("{}.{}", authority.domain_owner(), semantic_code)
}
