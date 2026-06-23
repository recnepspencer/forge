use worth_ui::facade::{WorthUiCapabilityReloadEvidence, WorthUiRuntimeFactSet};

use crate::reload::ValidationReloadEvidence;
use crate::runtime_workbench::ValidationRuntimeWorkbench;

pub(super) fn combined_changed_facts(
    source_evidence: &ValidationReloadEvidence,
    capability_evidence: &WorthUiCapabilityReloadEvidence,
) -> WorthUiRuntimeFactSet {
    let mut changed_facts = source_evidence.changed_facts().clone();
    changed_facts.extend(capability_evidence.changed_facts().facts().cloned());
    changed_facts
}

pub(super) fn visible_result_digest(workbench: &ValidationRuntimeWorkbench) -> u64 {
    let header = workbench.header_frame_plan().execute_frame();
    let page_host = workbench.page_host_plan().execute_frame();
    let mut rows = vec![
        format!("header_frame={}", header.frame_digest()),
        format!("page_host_frame={}", page_host.frame_digest()),
    ];
    for group in header.menu().groups() {
        rows.push(format!(
            "{}|{:?}|{:?}|{}",
            group.projection_id(),
            group.selection_mode(),
            group.selection_reconciliation().status(),
            group.commands().len()
        ));
    }
    for slot in page_host.slots() {
        let surface_id = worth_ui::facade::SurfaceId::new(slot.surface_id())
            .expect("page-host uses surface ids");
        let component_id = workbench
            .runtime()
            .inspect_active_authored_surface_component_id(&surface_id)
            .unwrap_or("missing");
        rows.push(format!("slot|{}|{}", slot.surface_id(), component_id));
    }
    fold_texts(rows)
}

pub(super) fn fold_texts(texts: impl IntoIterator<Item = impl AsRef<str>>) -> u64 {
    let mut digest = 0xcbf2_9ce4_8422_2325u64;
    for text in texts {
        for byte in text.as_ref().as_bytes() {
            digest ^= u64::from(*byte);
            digest = digest.wrapping_mul(0x0000_0100_0000_01b3);
        }
    }
    digest
}
