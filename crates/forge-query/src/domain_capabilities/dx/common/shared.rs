use crate::domain_capabilities::denials::ForgeQueryDomainCapabilityTransitionOutcome;
use crate::domain_capabilities::dx::checked::ForgeQueryCheckedDomainCapabilityOutcome;
use crate::domain_capabilities::ForgeQueryDomainCapabilityTargetKind;

pub(super) fn materialize_common_lane<Request, Eligible, Admitted, Ready, Success>(
    category: &'static str,
    target_kind: ForgeQueryDomainCapabilityTargetKind,
    semantic_posture: &'static str,
    requested: Request,
    evaluate: impl FnOnce(Request) -> ForgeQueryDomainCapabilityTransitionOutcome<Eligible>,
    admit: impl FnOnce(Eligible) -> ForgeQueryDomainCapabilityTransitionOutcome<Admitted>,
    prepare: impl FnOnce(Admitted) -> ForgeQueryDomainCapabilityTransitionOutcome<Ready>,
    materialize: impl FnOnce(Ready) -> ForgeQueryDomainCapabilityTransitionOutcome<Success>,
) -> ForgeQueryCheckedDomainCapabilityOutcome<Success> {
    let outcome = match evaluate(requested) {
        forge_proof::TransitionOutcome::Success(eligible) => match admit(eligible) {
            forge_proof::TransitionOutcome::Success(admitted) => match prepare(admitted) {
                forge_proof::TransitionOutcome::Success(ready) => materialize(ready),
                forge_proof::TransitionOutcome::Denied(denial) => {
                    forge_proof::TransitionOutcome::Denied(denial)
                }
                forge_proof::TransitionOutcome::Stale(stale) => {
                    forge_proof::TransitionOutcome::Stale(stale)
                }
                forge_proof::TransitionOutcome::RebindRequired(rebind) => {
                    forge_proof::TransitionOutcome::RebindRequired(rebind)
                }
                forge_proof::TransitionOutcome::Failed(failure) => {
                    forge_proof::TransitionOutcome::Failed(failure)
                }
                forge_proof::TransitionOutcome::Deferred(never) => match never {},
            },
            forge_proof::TransitionOutcome::Denied(denial) => {
                forge_proof::TransitionOutcome::Denied(denial)
            }
            forge_proof::TransitionOutcome::Stale(stale) => {
                forge_proof::TransitionOutcome::Stale(stale)
            }
            forge_proof::TransitionOutcome::RebindRequired(rebind) => {
                forge_proof::TransitionOutcome::RebindRequired(rebind)
            }
            forge_proof::TransitionOutcome::Failed(failure) => {
                forge_proof::TransitionOutcome::Failed(failure)
            }
            forge_proof::TransitionOutcome::Deferred(never) => match never {},
        },
        forge_proof::TransitionOutcome::Denied(denial) => {
            forge_proof::TransitionOutcome::Denied(denial)
        }
        forge_proof::TransitionOutcome::Stale(stale) => {
            forge_proof::TransitionOutcome::Stale(stale)
        }
        forge_proof::TransitionOutcome::RebindRequired(rebind) => {
            forge_proof::TransitionOutcome::RebindRequired(rebind)
        }
        forge_proof::TransitionOutcome::Failed(failure) => {
            forge_proof::TransitionOutcome::Failed(failure)
        }
        forge_proof::TransitionOutcome::Deferred(never) => match never {},
    };

    ForgeQueryCheckedDomainCapabilityOutcome::from_transition_outcome(
        category,
        target_kind,
        semantic_posture,
        outcome,
    )
}

pub(super) fn qualify_semantic_code(domain: &str, semantic_code: &str) -> String {
    format!("{domain}.{semantic_code}")
}
