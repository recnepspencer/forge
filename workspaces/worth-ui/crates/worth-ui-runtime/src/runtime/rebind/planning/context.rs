pub(crate) struct UiRebindPlanningContext<'runtime> {
    runtime: &'runtime crate::runtime::WorthUiRuntime,
    session: crate::facade::WorthUiActiveApplicationSessionIdentity,
    predecessor: &'runtime crate::facade::prepared_application_authority::
        WorthUiPreparedApplicationAuthority,
}

impl<'runtime> UiRebindPlanningContext<'runtime> {
    pub(crate) const fn new(
        runtime: &'runtime crate::runtime::WorthUiRuntime,
        session: crate::facade::WorthUiActiveApplicationSessionIdentity,
        predecessor: &'runtime crate::facade::prepared_application_authority::
            WorthUiPreparedApplicationAuthority,
    ) -> Self {
        Self {
            runtime,
            session,
            predecessor,
        }
    }

    pub(super) const fn runtime(&self) -> &'runtime crate::runtime::WorthUiRuntime {
        self.runtime
    }

    pub(super) const fn session(&self) -> crate::facade::WorthUiActiveApplicationSessionIdentity {
        self.session
    }

    pub(super) fn current_generation(
        &self,
    ) -> &'runtime crate::facade::prepared_application_authority::
    WorthUiPreparedApplicationGenerationIdentity{
        self.predecessor.generation_identity()
    }

    pub(super) fn current_source_basis(&self) -> u64 {
        self.predecessor.capabilities().digest().as_u64()
    }

    pub(super) fn current_graph(&self) -> crate::graph::UiGraphFactIndexBasis {
        crate::graph::UiGraphFactIndexBasis::from_generation(
            self.predecessor.graph_snapshot(),
            self.predecessor.capabilities(),
        )
    }

    pub(super) fn budget(&self) -> crate::runtime::rebind::UiRebindBudgetInput {
        self.predecessor.change_profile().rebind().budget()
    }
}
