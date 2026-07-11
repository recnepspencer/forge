use forge_store_layout_indexes::layout_certification::{
    S9FormalModelTarget, S9LayoutStateMachine, StorageFoundationS9LayoutHandoff,
    S9_DOWNSTREAM_PROTOCOL_DESTINATIONS, S9_REQUIRED_LAYOUT_MACHINES,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum S8LayoutHandoffReadinessDenial {
    MissingStateMachine(S9LayoutStateMachine),
    MissingDownstreamProtocolDestination(S9FormalModelTarget),
}

/// Readiness consumption of an already-admitted S.9 handoff. This is not an
/// execution, freshness, or counter witness.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct S8LayoutHandoffReadiness {
    handoff: StorageFoundationS9LayoutHandoff,
}

pub fn admit_s8_layout_handoff_readiness(
    handoff: StorageFoundationS9LayoutHandoff,
) -> Result<S8LayoutHandoffReadiness, S8LayoutHandoffReadinessDenial> {
    for machine in S9_REQUIRED_LAYOUT_MACHINES {
        if !handoff.requires(machine) {
            return Err(S8LayoutHandoffReadinessDenial::MissingStateMachine(machine));
        }
    }
    for target in S9_DOWNSTREAM_PROTOCOL_DESTINATIONS {
        if !handoff.declares_pending_protocol_target(target) {
            return Err(
                S8LayoutHandoffReadinessDenial::MissingDownstreamProtocolDestination(target),
            );
        }
    }
    Ok(S8LayoutHandoffReadiness { handoff })
}

impl S8LayoutHandoffReadiness {
    pub const fn handoff(&self) -> &StorageFoundationS9LayoutHandoff {
        &self.handoff
    }

    pub fn into_handoff(self) -> StorageFoundationS9LayoutHandoff {
        self.handoff
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use forge_store_layout_indexes::layout_closeout::layout_closeout;

    #[test]
    fn readiness_does_not_construct_lower_handoff_authority() {
        assert_eq!(
            S8LayoutHandoffReadinessDenial::MissingStateMachine(
                S9LayoutStateMachine::HiddenScanDenial
            ),
            S8LayoutHandoffReadinessDenial::MissingStateMachine(
                S9LayoutStateMachine::HiddenScanDenial
            )
        );
    }

    #[test]
    fn readiness_preserves_machine_and_formal_target_coverage() {
        let readiness =
            admit_s8_layout_handoff_readiness(layout_closeout().admit_s9_layout_handoff().unwrap())
                .unwrap();
        assert!(readiness
            .handoff()
            .requires(S9LayoutStateMachine::DegradedExactScan));
        for target in S9_DOWNSTREAM_PROTOCOL_DESTINATIONS {
            assert!(readiness.handoff().declares_pending_protocol_target(target));
        }
    }
}
