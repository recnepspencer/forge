use super::super::WorthUiActiveApplicationSession;

impl WorthUiActiveApplicationSession {
    pub(in crate::facade::entry) fn all_current_command_routing_contexts(
        &self,
    ) -> Box<[crate::runtime::command_routing::UiCommandRoutingContext]> {
        self.mounted
            .current_surfaces()
            .map(|surface| self.current_command_routing_context(surface))
            .collect::<Vec<_>>()
            .into_boxed_slice()
    }

    pub(super) fn current_command_routing_context(
        &self,
        surface: worth_ui_host_contract::UiSemanticSurfaceIdentity,
    ) -> crate::runtime::command_routing::UiCommandRoutingContext {
        let focus = self.focus.as_ref().map(|owner| owner.inspect());
        let semantic_focus = focus.as_ref().and_then(|snapshot| snapshot.current());
        let focused_graph = semantic_focus.map(|current| current.graph_node());
        let focused_scope = focused_graph.and_then(|graph_node| self.command_scope_for(graph_node));
        let mut portal_scopes = self
            .portal
            .as_ref()
            .into_iter()
            .flat_map(|owner| owner.active_portal_owner_graph_nodes())
            .filter_map(|graph_node| self.command_scope_for(graph_node))
            .collect::<Vec<_>>();
        portal_scopes.sort_unstable();
        portal_scopes.dedup();
        let mut context = crate::runtime::command_routing::UiCommandRoutingContext::new(surface)
            .with_focus(
                semantic_focus.map(|current| current.mounted_instance()),
                semantic_focus.map(|current| current.mounted_target()),
                focused_scope,
                focus.as_ref().map_or(0, |snapshot| snapshot.revision()),
            )
            .with_portals(
                portal_scopes.into_boxed_slice(),
                self.portal.as_ref().map_or(0, |owner| owner.revision()),
            );
        if let Some(current) = semantic_focus {
            let graph_node = current.graph_node();
            if let Some((selected_count, revision)) = self.selection.as_ref().and_then(|owner| {
                owner.compact_posture_for(
                    surface,
                    graph_node,
                    crate::runtime::selection::UiSelectionOwnerIncarnation::from_mount_incarnation(
                        current.incarnation(),
                    ),
                )
            }) {
                context = context.with_selection(graph_node, selected_count, revision);
            }
        }
        context
    }

    fn command_scope_for(
        &self,
        graph_node: crate::graph::UiGraphNodeIdentity,
    ) -> Option<crate::capability::UiCommandRouteScopeIdentity> {
        let lookup = self.graph().lookup().graph_node(graph_node)?;
        crate::capability::UiCommandRouteScopeIdentity::from_component_declaration(
            lookup.value_ref().declaration_identity(),
        )
    }
}
