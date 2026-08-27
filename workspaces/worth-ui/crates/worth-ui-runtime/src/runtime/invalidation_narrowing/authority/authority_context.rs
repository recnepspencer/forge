use super::{UiAllocationInvalidationAdmissionContext, UiCommittedAllocationInvalidationContext};

impl UiAllocationInvalidationAdmissionContext {
    pub(crate) fn commit(&self) -> UiCommittedAllocationInvalidationContext {
        UiCommittedAllocationInvalidationContext {
            basis: self.basis.clone(),
            neighborhood: self.neighborhood.clone(),
            allocation_plan: self.allocation_plan.clone(),
            replacement_impact: self.replacement_impact.clone(),
            impact_narrowing: self.impact_narrowing.clone(),
            graph_replan_admission: self.graph_replan_admission.clone(),
            scroll_planning: self.scroll_planning.clone(),
            scroll_planning_denial: self.scroll_planning_denial,
            portal_planning: self.portal_planning.clone(),
        }
    }

    pub(crate) fn committed_structural_parts(
        &self,
    ) -> (
        std::rc::Rc<crate::evidence::UiMeasurementBasis>,
        std::rc::Rc<crate::evidence::UiAllocationNeighborhood>,
        Option<u64>,
        crate::graph::UiGraphReplanAdmission,
    ) {
        (
            self.basis.clone(),
            self.neighborhood.clone(),
            self.planning_identity_digest(),
            self.graph_replan_admission(),
        )
    }

    pub(super) fn same_replacement_lineage(&self, other: &Self) -> bool {
        self.replacement_impact == other.replacement_impact
            && self.impact_narrowing == other.impact_narrowing
    }

    pub(super) fn planning_identity_digest(&self) -> Option<u64> {
        self.allocation_plan
            .as_ref()
            .map(crate::graph::UiAdmittedAllocationPlanReference::planning_identity_digest)
    }

    pub(super) fn graph_replan_admission(&self) -> crate::graph::UiGraphReplanAdmission {
        self.graph_replan_admission.clone()
    }

    pub(crate) fn from_planning_basis(
        basis: &crate::runtime::WorthUiAllocationPlanningBasis,
    ) -> Self {
        let (measurement_basis, allocation_neighborhood) = basis.invalidation_authority_parts();
        let mut context = Self {
            basis: measurement_basis,
            neighborhood: allocation_neighborhood,
            allocation_plan: None,
            replacement_impact: None,
            impact_narrowing: None,
            graph_replan_admission: crate::graph::UiGraphReplanAdmission::default(),
            scroll_planning: None,
            scroll_planning_denial: None,
            portal_planning: None,
        };
        context.seal_graph_replan_targets();
        context
    }

    pub(crate) fn with_allocation_candidate(
        mut self,
        candidate: crate::runtime::UiAllocationCandidate,
    ) -> Self {
        if candidate.allocation_constraint_set().is_some() {
            match crate::runtime::scroll::allocation::UiAdmittedScrollPlanningAuthority::seal(
                candidate.planning().basis(),
            ) {
                Ok(planning) => self.scroll_planning = planning,
                Err(denial) => self.scroll_planning_denial = Some(denial),
            }
        }
        self.portal_planning = candidate.allocation_constraint_set().and_then(|set| {
            crate::runtime::portal::anchored_allocation::UiAdmittedPortalPlanningAuthority::seal(
                candidate.measurement_basis(),
                candidate.allocation_neighborhood(),
                set,
            )
        });
        self.allocation_plan =
            Some(crate::graph::UiAdmittedAllocationPlanReference::from_candidate(candidate));
        self.seal_graph_replan_targets();
        self
    }

    pub(super) fn scroll_planning(
        &self,
    ) -> Option<&crate::runtime::scroll::allocation::UiAdmittedScrollPlanningAuthority> {
        self.scroll_planning.as_ref()
    }

    pub(super) fn portal_planning(
        &self,
    ) -> Option<&crate::runtime::portal::anchored_allocation::UiAdmittedPortalPlanningAuthority>
    {
        self.portal_planning.as_ref()
    }

    pub(crate) fn with_replacement_consequences(
        mut self,
        impact: crate::runtime::WorthUiReplacementImpactClassification,
        narrowing: crate::runtime::WorthUiRuntimeImpactNarrowing,
    ) -> Self {
        self.replacement_impact = Some(std::rc::Rc::new(impact));
        self.impact_narrowing = Some(std::rc::Rc::new(narrowing));
        self.seal_graph_replan_targets();
        self
    }

    pub(crate) fn with_replacement_lineage(
        mut self,
        impact: std::rc::Rc<crate::runtime::WorthUiReplacementImpactClassification>,
        narrowing: std::rc::Rc<crate::runtime::WorthUiRuntimeImpactNarrowing>,
    ) -> Self {
        self.replacement_impact = Some(impact);
        self.impact_narrowing = Some(narrowing);
        self.seal_graph_replan_targets();
        self
    }

    fn seal_graph_replan_targets(&mut self) {
        self.graph_replan_admission = crate::graph::UiGraphReplanAdmission::seal(
            &self.neighborhood,
            &self.basis,
            self.allocation_plan.as_ref(),
            self.replacement_impact.as_ref(),
            self.impact_narrowing.as_ref(),
        );
    }
}
