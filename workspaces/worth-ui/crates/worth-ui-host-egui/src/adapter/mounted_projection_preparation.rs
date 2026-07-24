#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiEguiMountedParticipationPreparation {
    mounted_instance: worth_ui_host_contract::UiMountedInstanceIdentity,
    participation: worth_ui_host_contract::UiMountedParticipation,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiEguiMountedProjectionPreparation {
    nodes: Box<[UiEguiMountedParticipationPreparation]>,
}

impl WorthUiEguiMountedProjectionPreparation {
    pub fn prepare(view: &worth_ui_host_contract::UiMountedProjectionView) -> Self {
        Self {
            nodes: view
                .nodes()
                .iter()
                .map(|node| UiEguiMountedParticipationPreparation {
                    mounted_instance: node.mounted_instance(),
                    participation: node.participation(),
                })
                .collect(),
        }
    }

    pub fn nodes(&self) -> &[UiEguiMountedParticipationPreparation] {
        &self.nodes
    }
}

impl UiEguiMountedParticipationPreparation {
    pub fn mounted_instance(self) -> worth_ui_host_contract::UiMountedInstanceIdentity {
        self.mounted_instance
    }

    pub fn participation(self) -> worth_ui_host_contract::UiMountedParticipation {
        self.participation
    }
}
