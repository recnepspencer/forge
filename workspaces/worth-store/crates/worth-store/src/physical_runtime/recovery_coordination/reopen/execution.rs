use worth_store_physical_backend::CompletedScheduledRecoveryReopenRead;
use worth_store_physical_format::{
    DurablePhysicalRootManifest, DurableRootSelector, RootSelectorRole,
};

use crate::physical_runtime::recovery_coordination::{
    PerformedRecoveryPhysicalEffect, PhysicalRecoveryCoordination, RecoveryFreshReopenOccurrence,
};

use super::{
    CompletedPhysicalRecoveryFreshReopen, PhysicalRecoveryFreshReopenCommand,
    PhysicalRecoveryFreshReopenDenial, PhysicalRecoveryFreshReopenDenialKind,
    PhysicalRecoveryFreshReopenOutcome, PhysicalRecoveryFreshReopenStage,
};

mod read;

pub(super) fn execute(
    coordination: &PhysicalRecoveryCoordination,
    media: &worth_store_physical_backend::AdmittedRecoveryFilesystemMedia,
    command: PhysicalRecoveryFreshReopenCommand,
) -> PhysicalRecoveryFreshReopenOutcome {
    let generation = command.expected_root.generation();
    let selector = match read::execute(
        coordination,
        media,
        PhysicalRecoveryFreshReopenStage::CurrentSelector,
        generation,
        worth_store_physical_format::ROOT_SELECTOR_BYTES as u64,
    ) {
        Ok(read) => read,
        Err(denial) => return PhysicalRecoveryFreshReopenOutcome::Denied(denial),
    };
    let observed_selector = match DurableRootSelector::decode(selector.physical.bytes()) {
        Ok(selector)
            if selector == command.expected_selector
                && selector.store_identity() == media.store_identity()
                && selector.format() == command.format
                && selector.role() == RootSelectorRole::Current
                && selector.root_generation() == generation =>
        {
            selector
        }
        _ => {
            return denied_binding(
                selector.physical,
                None,
                PhysicalRecoveryFreshReopenDenialKind::InvalidSelector,
            );
        }
    };
    let root_bytes = command.expected_root.encode(command.format).len() as u64;
    let root = match read::execute(
        coordination,
        media,
        PhysicalRecoveryFreshReopenStage::RootManifest,
        generation,
        root_bytes,
    ) {
        Ok(read) => read,
        Err(mut denial) => {
            denial.selector = Some(selector.physical);
            return PhysicalRecoveryFreshReopenOutcome::Denied(denial);
        }
    };
    let observed_root = match DurablePhysicalRootManifest::decode(
        root.physical.bytes(),
        command.expected_root.node_capacity(),
    ) {
        Ok((root, format)) if format == command.format => root,
        _ => {
            return denied_binding(
                selector.physical,
                Some(root.physical),
                PhysicalRecoveryFreshReopenDenialKind::InvalidRoot,
            )
        }
    };
    if observed_selector != command.expected_selector || observed_root != command.expected_root {
        return denied_binding(
            selector.physical,
            Some(root.physical),
            PhysicalRecoveryFreshReopenDenialKind::BindingMismatch,
        );
    }
    let wait = coordination.pause_at(
        crate::physical_runtime::PhysicalRecoveryYieldpointStage::FreshReopenExactBinding,
    );
    if wait.is_interrupted() {
        return PhysicalRecoveryFreshReopenOutcome::Denied(PhysicalRecoveryFreshReopenDenial::new(
            PhysicalRecoveryFreshReopenStage::ExactBinding,
            PhysicalRecoveryFreshReopenDenialKind::Yieldpoint(wait),
            Some(selector.physical.clone()),
            Some(root.physical.clone()),
            None,
        ));
    }
    let performed =
        PerformedRecoveryPhysicalEffect::record_fresh_reopen(RecoveryFreshReopenOccurrence::new(
            coordination.session_identity(),
            command.plan,
            generation,
            selector.physical,
            root.physical,
            selector.work,
            root.work,
            selector.signal,
            root.signal,
        ));
    PhysicalRecoveryFreshReopenOutcome::Completed(CompletedPhysicalRecoveryFreshReopen::new(
        observed_root,
        performed,
    ))
}

fn denied_binding(
    selector: CompletedScheduledRecoveryReopenRead,
    root: Option<CompletedScheduledRecoveryReopenRead>,
    kind: PhysicalRecoveryFreshReopenDenialKind,
) -> PhysicalRecoveryFreshReopenOutcome {
    PhysicalRecoveryFreshReopenOutcome::Denied(PhysicalRecoveryFreshReopenDenial::new(
        PhysicalRecoveryFreshReopenStage::ExactBinding,
        kind,
        Some(selector),
        root,
        None,
    ))
}
