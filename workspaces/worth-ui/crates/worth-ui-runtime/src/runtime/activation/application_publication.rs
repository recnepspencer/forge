/// Prepared application ownership transition committed inside the runtime's
/// complete publication transaction.
pub(crate) enum WorthUiPreparedApplicationPublication {
    Replacement {
        successor: Box<crate::facade::WorthUiApp>,
        intent_contract: crate::declaration::UiIntentCatalogSemanticComparison,
    },
    MountedGraph(
        Box<
            crate::facade::prepared_application_authority::WorthUiPreparedApplicationGraphSuccessor,
        >,
    ),
}

impl WorthUiPreparedApplicationPublication {
    pub(crate) fn replacement(
        predecessor: &crate::facade::prepared_application_authority::WorthUiPreparedApplicationAuthority,
        successor: crate::facade::WorthUiApp,
    ) -> Self {
        let intent_contract = predecessor
            .intent_catalog()
            .compare_semantic_contract(successor.prepared_authority().intent_catalog());
        Self::Replacement {
            successor: Box::new(successor),
            intent_contract,
        }
    }

    pub(crate) fn mounted_graph(
        successor: crate::facade::prepared_application_authority::WorthUiPreparedApplicationGraphSuccessor,
    ) -> Self {
        Self::MountedGraph(Box::new(successor))
    }

    pub(super) fn permits_execution_plan_semantic_no_op(&self) -> bool {
        match self {
            Self::Replacement {
                intent_contract, ..
            } => {
                *intent_contract
                    == crate::declaration::UiIntentCatalogSemanticComparison::Equivalent
            }
            Self::MountedGraph(_) => true,
        }
    }

    pub(super) fn commit_once(self, active: &mut crate::facade::WorthUiApp) {
        match self {
            Self::Replacement { successor, .. } => *active = *successor,
            Self::MountedGraph(successor) => {
                active
                    .commit_graph_successor(*successor)
                    .expect("prepared mounted graph successor retains its predecessor");
            }
        }
    }
}
