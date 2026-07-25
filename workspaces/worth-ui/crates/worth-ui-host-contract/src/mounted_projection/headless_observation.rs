#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiHeadlessMountedParticipationRecord {
    mounted_instance: crate::UiMountedInstanceIdentity,
    participation: super::UiMountedParticipation,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiHeadlessMountedProjectionRecord {
    nodes: Box<[UiHeadlessMountedParticipationRecord]>,
}

impl WorthUiHeadlessMountedProjectionRecord {
    pub fn observe(view: &super::UiMountedProjectionView) -> Self {
        Self {
            nodes: view
                .nodes()
                .iter()
                .map(|node| UiHeadlessMountedParticipationRecord {
                    mounted_instance: node.mounted_instance(),
                    participation: node.participation(),
                })
                .collect(),
        }
    }

    pub fn nodes(&self) -> &[UiHeadlessMountedParticipationRecord] {
        &self.nodes
    }
}

impl UiHeadlessMountedParticipationRecord {
    pub fn mounted_instance(self) -> crate::UiMountedInstanceIdentity {
        self.mounted_instance
    }

    pub fn participation(self) -> super::UiMountedParticipation {
        self.participation
    }
}
