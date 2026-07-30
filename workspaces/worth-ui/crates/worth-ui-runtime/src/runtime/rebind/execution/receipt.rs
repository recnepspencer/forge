use super::state::UiRebindReservation;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiRebindDisposition {
    Complete,
}

pub struct UiRebindReceipt {
    plan: crate::runtime::rebind::UiRebindPlan,
    publication: UiRebindPublication,
    disposition: UiRebindDisposition,
    _registration: UiRebindReservation,
}

enum UiRebindPublication {
    Changed {
        application: crate::facade::WorthUiApplicationCutoverReceipt,
        mounted: crate::mounting::UiMountedFramePublicationReceipt,
    },
    EvidenceOnly {
        prior: crate::facade::prepared_application_authority::
            WorthUiPreparedApplicationGenerationIdentity,
        active: crate::facade::prepared_application_authority::
            WorthUiPreparedApplicationGenerationIdentity,
    },
    Content {
        generation: crate::facade::prepared_application_authority::
            WorthUiPreparedApplicationGenerationIdentity,
        mounted: crate::mounting::UiMountedFramePublicationReceipt,
    },
}

impl UiRebindReceipt {
    pub(crate) fn changed(
        plan: crate::runtime::rebind::UiRebindPlan,
        mut registration: UiRebindReservation,
        application: crate::facade::WorthUiApplicationCutoverReceipt,
        mounted: crate::mounting::UiMountedFramePublicationReceipt,
    ) -> Result<Self, super::UiRebindInternalDefectOutcome> {
        let matches_plan = application.prior_generation()
            == plan.basis().classification().predecessor_generation()
            && application.active_generation() == plan.basis().candidate_generation()
            && mounted.generation() == plan.basis().candidate_generation();
        if !matches_plan {
            registration
                .retain_recovery()
                .expect("pre-effect admission reserved recovery capacity");
            return Err(super::UiRebindInternalDefectOutcome::published_mismatch(
                plan,
                registration,
                application,
                mounted,
            ));
        }
        registration
            .retain_receipt()
            .expect("pre-effect admission reserved receipt capacity");
        Ok(Self {
            plan,
            publication: UiRebindPublication::Changed {
                application,
                mounted,
            },
            disposition: UiRebindDisposition::Complete,
            _registration: registration,
        })
    }

    pub(crate) fn evidence_only(
        plan: crate::runtime::rebind::UiRebindPlan,
        mut registration: UiRebindReservation,
        prior: crate::facade::prepared_application_authority::
            WorthUiPreparedApplicationGenerationIdentity,
        active: crate::facade::prepared_application_authority::
            WorthUiPreparedApplicationGenerationIdentity,
    ) -> Result<Self, super::UiRebindInternalDefectOutcome> {
        let matches_plan = &prior == plan.basis().classification().predecessor_generation()
            && &active == plan.basis().candidate_generation();
        if !matches_plan {
            registration
                .retain_recovery()
                .expect("pre-publication admission reserved recovery capacity");
            return Err(super::UiRebindInternalDefectOutcome::evidence_mismatch(
                plan,
                registration,
                prior,
                active,
            ));
        }
        registration
            .retain_receipt()
            .expect("pre-publication admission reserved receipt capacity");
        Ok(Self {
            plan,
            publication: UiRebindPublication::EvidenceOnly { prior, active },
            disposition: UiRebindDisposition::Complete,
            _registration: registration,
        })
    }

    pub(crate) fn content(
        plan: crate::runtime::rebind::UiRebindPlan,
        mut registration: UiRebindReservation,
        generation: crate::facade::prepared_application_authority::
            WorthUiPreparedApplicationGenerationIdentity,
        mounted: crate::mounting::UiMountedFramePublicationReceipt,
    ) -> Result<Self, super::UiRebindInternalDefectOutcome> {
        let matches_plan = &generation == plan.basis().classification().predecessor_generation()
            && &generation == plan.basis().candidate_generation()
            && mounted.generation() == &generation;
        if !matches_plan {
            registration
                .retain_recovery()
                .expect("pre-effect admission reserved recovery capacity");
            return Err(super::UiRebindInternalDefectOutcome::content_mismatch(
                plan,
                registration,
                generation,
                mounted,
            ));
        }
        registration
            .retain_receipt()
            .expect("pre-publication admission reserved receipt capacity");
        Ok(Self {
            plan,
            publication: UiRebindPublication::Content {
                generation,
                mounted,
            },
            disposition: UiRebindDisposition::Complete,
            _registration: registration,
        })
    }

    pub const fn plan(&self) -> &crate::runtime::rebind::UiRebindPlan {
        &self.plan
    }

    pub fn planned_effects(&self) -> &[crate::runtime::rebind::UiRebindDeclarativeEffect] {
        self.plan.effects().effects()
    }

    pub const fn planned_cost(&self) -> crate::runtime::rebind::UiRebindPlanCost {
        self.plan.cost()
    }

    pub fn realized_bindings(&self) -> &[worth_ui_host_contract::UiSurfaceBindingGeneration] {
        match &self.publication {
            UiRebindPublication::Changed { mounted, .. } => mounted.bindings(),
            UiRebindPublication::Content { mounted, .. } => mounted.bindings(),
            UiRebindPublication::EvidenceOnly { .. } => &[],
        }
    }

    pub fn realized_mount_cost(&self) -> Option<crate::mounting::UiMountCostReport> {
        match &self.publication {
            UiRebindPublication::Changed { mounted, .. } => Some(mounted.cost_report()),
            UiRebindPublication::Content { mounted, .. } => Some(mounted.cost_report()),
            UiRebindPublication::EvidenceOnly { .. } => None,
        }
    }

    pub const fn retains_terminal_decision_record(&self) -> bool {
        true
    }

    pub const fn retains_recovery_authority(&self) -> bool {
        false
    }

    pub const fn disposition(&self) -> UiRebindDisposition {
        self.disposition
    }

    pub fn decision_record(&self) -> worth_ui_inspection::UiRebindDecisionRecord {
        super::inspection_projection::project_rebind_decision(self)
    }

    pub fn decision_index(&self) -> worth_ui_inspection::UiRebindDecisionIndex {
        super::inspection_projection::project_rebind_decision_index(self)
    }

    pub(crate) const fn decision_key(&self) -> u64 {
        self._registration.identity()
    }

    pub(crate) const fn session_identity(
        &self,
    ) -> crate::facade::WorthUiActiveApplicationSessionIdentity {
        self.plan.basis().classification().session()
    }

    pub(crate) const fn inspection_disposition(
        &self,
    ) -> worth_ui_inspection::UiRebindDecisionDisposition {
        match &self.publication {
            UiRebindPublication::Changed { .. } => {
                worth_ui_inspection::UiRebindDecisionDisposition::Changed
            }
            UiRebindPublication::Content { .. } => {
                worth_ui_inspection::UiRebindDecisionDisposition::Changed
            }
            UiRebindPublication::EvidenceOnly { .. } => {
                worth_ui_inspection::UiRebindDecisionDisposition::EvidenceOnly
            }
        }
    }

    pub fn prior_generation(
        &self,
    ) -> &crate::facade::prepared_application_authority::WorthUiPreparedApplicationGenerationIdentity
    {
        match &self.publication {
            UiRebindPublication::Changed { application, .. } => application.prior_generation(),
            UiRebindPublication::Content { generation, .. } => generation,
            UiRebindPublication::EvidenceOnly { prior, .. } => prior,
        }
    }

    pub fn active_generation(
        &self,
    ) -> &crate::facade::prepared_application_authority::WorthUiPreparedApplicationGenerationIdentity
    {
        match &self.publication {
            UiRebindPublication::Changed { application, .. } => application.active_generation(),
            UiRebindPublication::Content { generation, .. } => generation,
            UiRebindPublication::EvidenceOnly { active, .. } => active,
        }
    }

    pub fn mounted_publication(
        &self,
    ) -> Option<&crate::mounting::UiMountedFramePublicationReceipt> {
        match &self.publication {
            UiRebindPublication::Changed { mounted, .. } => Some(mounted),
            UiRebindPublication::Content { mounted, .. } => Some(mounted),
            UiRebindPublication::EvidenceOnly { .. } => None,
        }
    }

    pub fn application_publication(
        &self,
    ) -> Option<&crate::facade::WorthUiApplicationCutoverReceipt> {
        match &self.publication {
            UiRebindPublication::Changed { application, .. } => Some(application),
            UiRebindPublication::Content { .. } | UiRebindPublication::EvidenceOnly { .. } => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::runtime::observation::UiChangeClassificationOutcome;

    #[test]
    fn planned_realized_mismatch_is_a_recoverable_internal_defect() {
        let mut session = crate::runtime::tests::active_application_session_test_support::
            source_backed_component_session();
        let foreign = crate::runtime::tests::active_application_session_test_support::
            source_backed_component_session();
        let candidate = crate::runtime::tests::active_application_session_test_support::
            component_candidate_submission(
                &session,
                "phase-312-receipt-mismatch",
                "workspace.component.active_session_current",
            );
        let mut turn = session.begin_observation_turn().unwrap();
        turn.admit_source(candidate).unwrap();
        let admitted = turn.seal().unwrap();
        let evidence = match session.classify_observations(admitted).unwrap() {
            UiChangeClassificationOutcome::EvidenceOnly(evidence) => evidence,
            _ => panic!("equal semantics with fresh evidence stays evidence-only"),
        };
        let plan = session
            .compile_preservation_rebind(
                evidence,
                crate::runtime::rebind::UiRebindExecutionPolicy::ordinary(),
            )
            .unwrap();
        let prior = plan
            .basis()
            .classification()
            .predecessor_generation()
            .clone();
        let state = super::super::UiRebindRuntimeState::new(
            crate::runtime::rebind::UiRebindProfile::platform_pulse(),
        );
        let basis = super::super::UiRebindFinalAdmissionBasis::new(
            plan.basis().classification().session(),
            plan.basis().classification().source_basis(),
            plan.basis().classification().predecessor_generation(),
        );
        let reservation = super::super::admit_plan(
            &state,
            basis,
            &plan,
            crate::runtime::rebind::UiRebindExecutionRequest::new(1),
        )
        .expect("current plan reserves terminal capacity");

        let defect = match UiRebindReceipt::evidence_only(
            plan,
            reservation,
            prior,
            foreign.generation_identity().clone(),
        ) {
            Err(defect) => defect,
            Ok(receipt) => {
                drop(receipt);
                panic!("crossed realized generation must not become a receipt");
            }
        };
        assert_eq!(
            defect.kind(),
            super::super::UiRebindInternalDefectKind::PlannedRealizedMismatch
        );
        assert!(defect.publication_occurred());
        assert!(defect.retains_recovery_authority());
        assert_eq!(
            defect.valid_next_action(),
            super::super::UiRebindValidNextAction::ReportDefect
        );
        drop(defect);
        assert!(state.shutdown().is_empty());
        assert!(session.shutdown().rebind().is_empty());
        assert!(foreign.shutdown().rebind().is_empty());
    }
}
