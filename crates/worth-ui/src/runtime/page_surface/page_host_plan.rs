use crate::runtime::{WorthUiProjectionDependencySet, WorthUiRuntimeFactId, WorthUiRuntimeHost};

use super::{WorthUiPageHostFrameReceipt, WorthUiPageHostRequest, WorthUiPageHostSlotReceipt};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiPageHostPlan {
    page_name: String,
    slots: Vec<WorthUiPageHostSlotReceipt>,
    frame_digest: u64,
    dependencies: WorthUiProjectionDependencySet,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorthUiPageHostPlanDenial {
    MissingAuthoringSnapshot,
    MissingPage(String),
}

impl WorthUiPageHostPlan {
    pub fn from_runtime(
        runtime: &WorthUiRuntimeHost,
        request: WorthUiPageHostRequest,
    ) -> Result<Self, WorthUiPageHostPlanDenial> {
        let authoring = runtime
            .active_authoring_snapshot()
            .ok_or(WorthUiPageHostPlanDenial::MissingAuthoringSnapshot)?;
        let content_slots = authoring
            .content_slots()
            .page(request.page_name())
            .ok_or_else(|| {
                WorthUiPageHostPlanDenial::MissingPage(request.page_name().to_owned())
            })?;
        let slots = content_slots
            .assignments()
            .iter()
            .map(|assignment| {
                WorthUiPageHostSlotReceipt::new(assignment.slot_name(), assignment.surface_id())
            })
            .collect::<Vec<_>>();
        let dependencies = page_dependencies(request.page_name(), &slots);
        let frame_digest = digest_page_host_frame(request.page_name(), &slots);

        Ok(Self {
            page_name: request.page_name().to_owned(),
            slots,
            frame_digest,
            dependencies,
        })
    }

    pub fn execute_frame(&self) -> WorthUiPageHostFrameReceipt {
        WorthUiPageHostFrameReceipt::new(
            self.page_name.clone(),
            self.slots.clone(),
            self.frame_digest,
        )
    }

    pub fn frame_digest(&self) -> u64 {
        self.frame_digest
    }

    pub fn dependencies(&self) -> &WorthUiProjectionDependencySet {
        &self.dependencies
    }
}

fn page_dependencies(
    page_name: &str,
    slots: &[WorthUiPageHostSlotReceipt],
) -> WorthUiProjectionDependencySet {
    let mut dependencies = WorthUiProjectionDependencySet::empty()
        .depends_on(WorthUiRuntimeFactId::active_artifact())
        .depends_on(WorthUiRuntimeFactId::execution_plan())
        .depends_on(WorthUiRuntimeFactId::layout_topology(page_name));
    for slot in slots {
        dependencies = dependencies.depends_on(WorthUiRuntimeFactId::content_mount(format!(
            "{page_name}.{}",
            slot.slot_name()
        )));
    }
    dependencies
}

fn digest_page_host_frame(page_name: &str, slots: &[WorthUiPageHostSlotReceipt]) -> u64 {
    let mut digest = fold_bytes(0xcbf2_9ce4_8422_2325, page_name.as_bytes());
    for slot in slots {
        digest = fold_bytes(digest, slot.slot_name().as_bytes());
        digest = fold_bytes(digest, slot.surface_id().as_bytes());
    }
    digest
}

fn fold_bytes(mut accumulator: u64, bytes: &[u8]) -> u64 {
    for byte in bytes {
        accumulator ^= u64::from(*byte);
        accumulator = accumulator.wrapping_mul(0x0000_0100_0000_01b3);
    }
    accumulator
}
