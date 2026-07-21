use super::WorthUiPlanRegionExecutable;

#[derive(Clone, Debug)]
pub(in crate::runtime::planning::plan_topology::region) struct WorthUiPlanRegionRecord {
    pub(super) schema: super::WorthUiPlanRegionSchema,
    pub(super) handle: super::WorthUiPlanRegionHandle,
    pub(super) executable: WorthUiPlanRegionExecutable,
}

impl WorthUiPlanRegionRecord {
    pub(super) fn new(
        schema: super::WorthUiPlanRegionSchema,
        handle: super::WorthUiPlanRegionHandle,
        executable: WorthUiPlanRegionExecutable,
    ) -> Self {
        Self {
            schema,
            handle,
            executable,
        }
    }

    pub(super) fn semantic_digest(&self) -> u64 {
        self.executable.semantic_digest(
            self.schema.identity().routing_fingerprint()
                ^ self.schema.narrowing_fingerprint().rotate_left(29),
        )
    }
}
