use super::{WorthUiPlanRegionIdentity, WorthUiPlanRegionSchema};

#[derive(Clone, Debug)]
pub(crate) enum WorthUiPlanRegionMutation {
    Upsert(WorthUiPlanRegionSchema),
    Insert(WorthUiPlanRegionSchema),
    Replace(WorthUiPlanRegionSchema),
    Reparent(WorthUiPlanRegionSchema),
    Rebind(WorthUiPlanRegionSchema),
    LaneTransition(WorthUiPlanRegionSchema),
    Retire(WorthUiPlanRegionIdentity),
    OwnerBundle {
        root: WorthUiPlanRegionIdentity,
        schemas: Vec<WorthUiPlanRegionSchema>,
    },
    RetireOwner(WorthUiPlanRegionIdentity),
}

impl WorthUiPlanRegionMutation {
    pub(super) fn identity(&self) -> &WorthUiPlanRegionIdentity {
        match self {
            Self::Upsert(schema)
            | Self::Insert(schema)
            | Self::Replace(schema)
            | Self::Reparent(schema)
            | Self::Rebind(schema)
            | Self::LaneTransition(schema) => schema.identity(),
            Self::Retire(identity) | Self::RetireOwner(identity) => identity,
            Self::OwnerBundle { root, .. } => root,
        }
    }
}
