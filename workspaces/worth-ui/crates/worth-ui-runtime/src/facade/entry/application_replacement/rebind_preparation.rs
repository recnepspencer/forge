use super::{
    WorthUiPreparedApplicationReplacement, WorthUiPreparedApplicationReplacementBasis,
    WorthUiPreparedReplacementSemanticInput,
};
use crate::facade::{WorthUiActiveApplicationSessionIdentity, WorthUiApp};

impl WorthUiPreparedApplicationReplacement {
    pub(crate) fn from_changed_rebind_plan(
        session: WorthUiActiveApplicationSessionIdentity,
        host_session_plan: crate::facade::prepared_application_authority::WorthUiHostSessionPlan,
        font_collection: std::sync::Arc<worth_ui_text::UiGlobalFontCollection>,
        changed: crate::runtime::rebind::UiChangedRebindSemanticProof,
    ) -> Option<Self> {
        let next_app = WorthUiApp::from_prepared_authority(
            changed.successor_authority,
            host_session_plan,
            font_collection,
        );
        let candidate_query_binding = next_app
            .prepared_authority()
            .query_binding_plan()
            .prepare_downstream_state();
        let basis = WorthUiPreparedApplicationReplacementBasis::bind(
            session,
            &next_app,
            changed.lowering.admitted(),
        )?;
        Some(Self {
            next_app,
            semantic_input: WorthUiPreparedReplacementSemanticInput::Prelowered(changed.lowering),
            basis,
            candidate_query_binding,
            candidate_graph_changed_nodes: changed.candidate_graph_changed_nodes,
        })
    }
}
