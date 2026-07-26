use super::{
    WorthUiActiveApplicationSessionIdentity, WorthUiApp,
    WorthUiPreparedApplicationGenerationIdentity, WorthUiPreparedApplicationReplacementBasis,
};

impl WorthUiPreparedApplicationReplacementBasis {
    pub(super) fn bind(
        origin_session: WorthUiActiveApplicationSessionIdentity,
        next_app: &WorthUiApp,
        admitted: &crate::runtime::WorthUiAdmittedReplacementCandidate,
    ) -> Option<Self> {
        let candidate_basis = admitted.candidate().basis();
        (next_app
            .prepared_authority()
            .source_backed_candidate_basis()
            == candidate_basis)
            .then(|| Self {
                origin_session,
                next_generation: next_app.generation_identity().clone(),
                candidate_basis,
                graph_authority_identity: next_app
                    .prepared_authority()
                    .graph_snapshot()
                    .authority_identity(),
                candidate_application_authority: next_app.prepared_authority().lowering_authority(),
            })
    }

    pub(super) fn admits_session(&self, session: WorthUiActiveApplicationSessionIdentity) -> bool {
        self.origin_session == session
    }

    pub(super) fn rebind_graph(&mut self, next_app: &WorthUiApp) {
        self.next_generation = next_app.generation_identity().clone();
        self.graph_authority_identity = next_app.graph_snapshot().authority_identity();
        self.candidate_application_authority = next_app.prepared_authority().lowering_authority();
    }

    pub(crate) fn next_generation(&self) -> &WorthUiPreparedApplicationGenerationIdentity {
        &self.next_generation
    }

    pub(super) fn admits_catalog_delta(
        &self,
        delta: &crate::graph::UiAdmittedAllocationCatalogDelta,
    ) -> bool {
        self.graph_authority_identity == delta.graph_authority_identity()
    }

    pub(super) fn admits_application_authority(
        &self,
        authority: &crate::facade::prepared_application_authority::WorthUiPreparedApplicationLoweringAuthority,
    ) -> bool {
        self.candidate_application_authority
            .shares_authority_with(authority)
    }
}
