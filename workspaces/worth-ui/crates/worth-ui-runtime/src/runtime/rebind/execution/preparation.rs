use super::state::UiRebindReservation;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiPreparedRebindPosture {
    ChangedPresentation,
    EvidenceOnlyPublication,
}

pub struct UiPreparedRebind<'session> {
    plan: crate::runtime::rebind::UiRebindPlan,
    reservation: UiRebindReservation,
    kind: UiPreparedRebindKind<'session>,
}

enum UiPreparedRebindKind<'session> {
    Changed(Box<crate::facade::WorthUiPreparedMountedApplicationReplacement<'session>>),
    EvidenceOnly(crate::facade::entry::WorthUiPreparedEvidenceOnlyApplicationRebind<'session>),
}

impl<'session> UiPreparedRebind<'session> {
    pub(crate) fn changed(
        plan: crate::runtime::rebind::UiRebindPlan,
        reservation: UiRebindReservation,
        replacement: Box<crate::facade::WorthUiPreparedMountedApplicationReplacement<'session>>,
    ) -> Self {
        Self {
            plan,
            reservation,
            kind: UiPreparedRebindKind::Changed(replacement),
        }
    }

    pub(crate) fn evidence_only(
        plan: crate::runtime::rebind::UiRebindPlan,
        reservation: UiRebindReservation,
        prepared: crate::facade::entry::WorthUiPreparedEvidenceOnlyApplicationRebind<'session>,
    ) -> Result<Self, crate::runtime::rebind::UiRebindPreparationDenial> {
        Ok(Self {
            plan,
            reservation,
            kind: UiPreparedRebindKind::EvidenceOnly(prepared),
        })
    }

    pub const fn plan(&self) -> &crate::runtime::rebind::UiRebindPlan {
        &self.plan
    }

    pub const fn reservation_identity(&self) -> u64 {
        self.reservation.identity()
    }

    pub const fn posture(&self) -> UiPreparedRebindPosture {
        match &self.kind {
            UiPreparedRebindKind::Changed(_) => UiPreparedRebindPosture::ChangedPresentation,
            UiPreparedRebindKind::EvidenceOnly(_) => {
                UiPreparedRebindPosture::EvidenceOnlyPublication
            }
        }
    }

    pub fn candidate_generation(
        &self,
    ) -> &crate::facade::prepared_application_authority::WorthUiPreparedApplicationGenerationIdentity
    {
        match &self.kind {
            UiPreparedRebindKind::Changed(_) => self.plan.basis().candidate_generation(),
            UiPreparedRebindKind::EvidenceOnly(prepared) => prepared.generation_identity(),
        }
    }

    pub fn prepared_frame(&self) -> Option<&crate::mounting::UiPreparedMountedFrame> {
        match &self.kind {
            UiPreparedRebindKind::Changed(replacement) => Some(replacement.frame()),
            UiPreparedRebindKind::EvidenceOnly(_) => None,
        }
    }

    pub fn execute(self, now_tick: u64) -> super::UiRebindOutcome<'session> {
        let Self {
            plan,
            mut reservation,
            kind,
        } = self;
        match kind {
            UiPreparedRebindKind::Changed(replacement) => {
                if let Err(denial) = reservation.begin_effecting() {
                    let retry = Self {
                        plan,
                        reservation,
                        kind: UiPreparedRebindKind::Changed(replacement),
                    };
                    return super::UiRebindOutcome::RejectedBeforeEffects(
                        super::UiRebindDenialReceipt::capacity(denial, retry),
                    );
                }
                let deadline = presentation_deadline(&plan);
                let outcome = replacement.present(deadline, now_tick);
                super::outcome::map_changed_first_attempt(plan, reservation, outcome)
            }
            UiPreparedRebindKind::EvidenceOnly(prepared) => {
                if let Err(denial) = reservation.begin_effecting() {
                    let retry = Self {
                        plan,
                        reservation,
                        kind: UiPreparedRebindKind::EvidenceOnly(prepared),
                    };
                    return super::UiRebindOutcome::RejectedBeforeEffects(
                        super::UiRebindDenialReceipt::capacity(denial, retry),
                    );
                }
                let (prior, active) = prepared.commit();
                match super::UiRebindReceipt::evidence_only(plan, reservation, prior, active) {
                    Ok(receipt) => super::UiRebindOutcome::Published(receipt),
                    Err(defect) => super::UiRebindOutcome::InternalDefect(defect),
                }
            }
        }
    }
}

fn presentation_deadline(
    plan: &crate::runtime::rebind::UiRebindPlan,
) -> worth_ui_host_contract::UiPresentationDeadline {
    let tick = match plan.execution_policy().deadline() {
        crate::runtime::rebind::UiRebindDeadlinePolicy::NoDeadline => u64::MAX,
        crate::runtime::rebind::UiRebindDeadlinePolicy::At(deadline) => deadline.tick(),
    };
    worth_ui_host_contract::UiPresentationDeadline::at_tick(tick)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::runtime::observation::UiChangeClassificationOutcome;

    #[test]
    fn changed_plan_prepares_inherited_mounted_transaction_before_effects() {
        let mut session = crate::runtime::tests::active_application_session_test_support::
            source_backed_component_session();
        let predecessor = session.generation_identity().clone();
        let candidate = crate::runtime::tests::active_application_session_test_support::
            component_candidate_submission(
                &session,
                "phase-312-prepared-changed",
                "workspace.component.active_session_candidate",
            );
        let mut turn = session.begin_observation_turn().unwrap();
        turn.admit_source(candidate).unwrap();
        let admitted = turn.seal().unwrap();
        let changed = match session.classify_observations(admitted).unwrap() {
            UiChangeClassificationOutcome::Changed(changed) => changed,
            _ => panic!("candidate changes semantics"),
        };
        let lifecycle = session
            .resolve_affected_scope(changed)
            .unwrap()
            .resolve_identity_lifecycle()
            .unwrap();
        let plan = session
            .compile_rebind_plan(
                lifecycle,
                crate::runtime::rebind::UiRebindExecutionPolicy::ordinary(),
            )
            .unwrap();
        let candidate = plan.basis().candidate_generation().clone();
        let source_digest = plan
            .source_candidate_artifact_digest()
            .expect("source plan retains candidate digest");
        let prepared = session
            .prepare_rebind(
                plan,
                crate::runtime::rebind::UiRebindExecutionRequest::new(1),
            )
            .expect("changed plan prepares every inherited input");
        assert_eq!(
            prepared.posture(),
            UiPreparedRebindPosture::ChangedPresentation
        );
        assert_eq!(prepared.candidate_generation(), &candidate);
        assert!(prepared.prepared_frame().is_some());
        assert!(!prepared.plan().effects().effects().is_empty());
        assert_eq!(
            prepared.plan().source_candidate_artifact_digest(),
            Some(source_digest)
        );
        drop(prepared);
        assert_eq!(session.generation_identity(), &predecessor);
        assert!(session.shutdown().rebind().is_empty());
    }

    #[test]
    fn evidence_only_plan_retains_successor_without_preparing_host_effects() {
        let mut session = crate::runtime::tests::active_application_session_test_support::
            source_backed_component_session();
        let predecessor = session.generation_identity().clone();
        let candidate = crate::runtime::tests::active_application_session_test_support::
            component_candidate_submission(
                &session,
                "phase-312-prepared-evidence-only",
                "workspace.component.active_session_current",
            );
        let mut turn = session.begin_observation_turn().unwrap();
        turn.admit_source(candidate).unwrap();
        let admitted = turn.seal().unwrap();
        let evidence = match session.classify_observations(admitted).unwrap() {
            UiChangeClassificationOutcome::EvidenceOnly(evidence) => evidence,
            _ => panic!("equal semantics with new evidence stays evidence-only"),
        };
        let plan = session
            .compile_preservation_rebind(
                evidence,
                crate::runtime::rebind::UiRebindExecutionPolicy::ordinary(),
            )
            .unwrap();
        let candidate = plan.basis().candidate_generation().clone();
        let source_digest = plan
            .source_candidate_artifact_digest()
            .expect("source plan retains candidate digest");
        let prepared = session
            .prepare_rebind(
                plan,
                crate::runtime::rebind::UiRebindExecutionRequest::new(1),
            )
            .expect("evidence-only plan retains its successor");
        assert_eq!(
            prepared.posture(),
            UiPreparedRebindPosture::EvidenceOnlyPublication
        );
        assert_eq!(prepared.candidate_generation(), &candidate);
        assert!(prepared.prepared_frame().is_none());
        assert_eq!(
            prepared.plan().source_candidate_artifact_digest(),
            Some(source_digest)
        );
        drop(prepared);
        assert_eq!(session.generation_identity(), &predecessor);
        assert!(session.shutdown().rebind().is_empty());
    }

    #[test]
    fn evidence_only_execution_atomically_advances_authored_truth_without_mounting() {
        let mut session = crate::runtime::tests::active_application_session_test_support::
            source_backed_component_session();
        let prior = session.generation_identity().clone();
        let candidate = crate::runtime::tests::active_application_session_test_support::
            component_candidate_submission(
                &session,
                "phase-312-published-evidence-only",
                "workspace.component.active_session_current",
            );
        let mut turn = session.begin_observation_turn().unwrap();
        turn.admit_source(candidate).unwrap();
        let admitted = turn.seal().unwrap();
        let evidence = match session.classify_observations(admitted).unwrap() {
            UiChangeClassificationOutcome::EvidenceOnly(evidence) => evidence,
            _ => panic!("equal semantics with new evidence stays evidence-only"),
        };
        let plan = session
            .compile_preservation_rebind(
                evidence,
                crate::runtime::rebind::UiRebindExecutionPolicy::ordinary(),
            )
            .unwrap();
        let candidate = plan.basis().candidate_generation().clone();
        let prepared = session
            .prepare_rebind(
                plan,
                crate::runtime::rebind::UiRebindExecutionRequest::new(1),
            )
            .unwrap();

        let receipt = match prepared.execute(1) {
            crate::runtime::rebind::UiRebindOutcome::Published(receipt) => receipt,
            _ => panic!("evidence-only publication must complete atomically"),
        };
        assert_eq!(receipt.prior_generation(), &prior);
        assert_eq!(receipt.active_generation(), &candidate);
        assert!(receipt.mounted_publication().is_none());
        assert!(receipt.application_publication().is_none());
        assert_eq!(session.generation_identity(), &candidate);
        drop(receipt);
        assert!(session.shutdown().rebind().is_empty());
    }
}
