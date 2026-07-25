/// Prepared application ownership transition committed inside the runtime's
/// complete publication transaction.
pub(crate) enum WorthUiPreparedApplicationPublication {
    Replacement(Box<crate::facade::WorthUiApp>),
    MountedGraph(
        Box<
            crate::facade::prepared_application_authority::WorthUiPreparedApplicationGraphSuccessor,
        >,
    ),
}

impl WorthUiPreparedApplicationPublication {
    pub(crate) fn new(successor: crate::facade::WorthUiApp) -> Self {
        Self::Replacement(Box::new(successor))
    }

    pub(crate) fn mounted_graph(
        successor: crate::facade::prepared_application_authority::WorthUiPreparedApplicationGraphSuccessor,
    ) -> Self {
        Self::MountedGraph(Box::new(successor))
    }

    pub(super) fn commit_once(self, active: &mut crate::facade::WorthUiApp) {
        match self {
            Self::Replacement(successor) => *active = *successor,
            Self::MountedGraph(successor) => {
                active
                    .commit_graph_successor(*successor)
                    .expect("prepared mounted graph successor retains its predecessor");
            }
        }
    }
}
