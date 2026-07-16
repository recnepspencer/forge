#[derive(Clone, Copy)]
pub(super) struct HostileSchedule {
    pub(super) steps: &'static [HostileScheduleStep],
}

#[derive(Clone, Copy)]
pub(super) enum HostileScheduleStep {
    ConsumeUnpublishedDerived,
    OpenBranch(&'static str),
    DiscardPreview {
        label: &'static str,
        identity: &'static str,
        title: &'static str,
    },
    SubmitTask {
        identity: &'static str,
        title: &'static str,
        slot: PublishedArtifactSlot,
    },
    ReconsumePublishedArtifacts {
        current_slot: PublishedArtifactSlot,
        stable_slot: Option<PublishedArtifactSlot>,
    },
    PromotePreview {
        label: &'static str,
        identity: &'static str,
        title: &'static str,
    },
}

#[derive(Clone, Copy)]
pub(super) enum PublishedArtifactSlot {
    First,
    Second,
    Third,
}

pub(super) fn hostile_schedule() -> HostileSchedule {
    HostileSchedule {
        steps: &[
            HostileScheduleStep::ConsumeUnpublishedDerived,
            HostileScheduleStep::OpenBranch("branch-a"),
            HostileScheduleStep::OpenBranch("branch-b"),
            HostileScheduleStep::DiscardPreview {
                label: "preview-discard",
                identity: "preview-discard",
                title: "Preview discard",
            },
            HostileScheduleStep::SubmitTask {
                identity: "task-1",
                title: "Task One",
                slot: PublishedArtifactSlot::First,
            },
            HostileScheduleStep::SubmitTask {
                identity: "task-2",
                title: "Task Two",
                slot: PublishedArtifactSlot::Second,
            },
            HostileScheduleStep::ReconsumePublishedArtifacts {
                current_slot: PublishedArtifactSlot::Second,
                stable_slot: Some(PublishedArtifactSlot::First),
            },
            HostileScheduleStep::PromotePreview {
                label: "preview-promote",
                identity: "task-3",
                title: "Task Three",
            },
            HostileScheduleStep::OpenBranch("branch-c"),
            HostileScheduleStep::ReconsumePublishedArtifacts {
                current_slot: PublishedArtifactSlot::Third,
                stable_slot: None,
            },
        ],
    }
}
