use crate::runtime::{
    WorthQueryAspectMutationOperation, WorthQueryAspectTouch,
    WorthQueryGraphObligationExecutionStatus, WorthQueryGraphObligationKind,
    WorthQueryGraphObligationOperatingWorldDescriptor,
    WorthQueryGraphObligationOperatingWorldSelector, WorthQueryGraphObligationRegistration,
    WorthQueryGraphObligationRuleIdentity, WorthQueryGraphObligationSupportLane,
    WorthQueryGraphObligationSupportMatrix, WorthQueryGraphObligationSupportMatrixRow,
    WorthQueryGraphObligationSupportPosture, WorthQueryGraphObligationSupportStatus,
    WorthQueryGraphTouchDescriptor, WorthQueryGraphTouchReadVerb, WorthQueryGraphTouchSelector,
    WorthQueryMutationFamily,
};
use worth_foundational::facade::AspectKey;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryGraphObligationMatrixCertificationCase {
    row: WorthQueryGraphObligationSupportMatrixRow,
    selector: WorthQueryGraphTouchSelector,
    touch_descriptor: WorthQueryGraphTouchDescriptor,
    operating_world: WorthQueryGraphObligationOperatingWorldDescriptor,
    expected_execution_status: WorthQueryGraphObligationExecutionStatus,
}

impl WorthQueryGraphObligationMatrixCertificationCase {
    pub fn milestone_9_9_authority_cases() -> Vec<Self> {
        WorthQueryGraphObligationSupportMatrix::milestone_9_9_authority_surface()
            .rows()
            .iter()
            .map(Self::from_matrix_row)
            .collect()
    }

    pub fn from_matrix_row(row: &WorthQueryGraphObligationSupportMatrixRow) -> Self {
        let selector = representative_selector_for_lane(row.support_lane());
        let touch_descriptor = representative_touch_for_lane(row.support_lane());
        let expected_execution_status = expected_execution_status_for_row(row);
        Self {
            row: row.clone(),
            selector,
            touch_descriptor,
            operating_world:
                WorthQueryGraphObligationOperatingWorldDescriptor::any_committed_authority(),
            expected_execution_status,
        }
    }

    pub fn row(&self) -> &WorthQueryGraphObligationSupportMatrixRow {
        &self.row
    }

    pub fn selector(&self) -> &WorthQueryGraphTouchSelector {
        &self.selector
    }

    pub fn touch_descriptor(&self) -> &WorthQueryGraphTouchDescriptor {
        &self.touch_descriptor
    }

    pub fn operating_world(&self) -> &WorthQueryGraphObligationOperatingWorldDescriptor {
        &self.operating_world
    }

    pub fn expected_execution_status(&self) -> WorthQueryGraphObligationExecutionStatus {
        self.expected_execution_status
    }

    pub fn registration(&self) -> WorthQueryGraphObligationRegistration {
        WorthQueryGraphObligationRegistration::new(
            self.row.obligation_kind(),
            rule_for_row(&self.row),
            self.selector.clone(),
            WorthQueryGraphObligationOperatingWorldSelector::any_committed_authority(),
        )
        .with_support_posture(support_posture_for_row(&self.row))
        .with_execution_budget(self.row.execution_budget().clone())
    }
}

fn representative_selector_for_lane(
    lane: WorthQueryGraphObligationSupportLane,
) -> WorthQueryGraphTouchSelector {
    match lane {
        WorthQueryGraphObligationSupportLane::ReadFamily => {
            WorthQueryGraphTouchSelector::read_verb(
                WorthQueryGraphTouchReadVerb::ObservesRelationKind,
            )
        }
        WorthQueryGraphObligationSupportLane::LiveRead => WorthQueryGraphTouchSelector::read_verb(
            WorthQueryGraphTouchReadVerb::RetainsLiveSubscription,
        ),
        _ => WorthQueryGraphTouchSelector::collection("topology.edge")
            .expect("static certification selector is non-empty"),
    }
}

fn representative_touch_for_lane(
    lane: WorthQueryGraphObligationSupportLane,
) -> WorthQueryGraphTouchDescriptor {
    match lane {
        WorthQueryGraphObligationSupportLane::ReadFamily => {
            WorthQueryGraphTouchDescriptor::read_family(
                "topology.edge",
                [WorthQueryGraphTouchReadVerb::ObservesRelationKind],
            )
            .expect("static read certification touch is valid")
        }
        WorthQueryGraphObligationSupportLane::LiveRead => {
            WorthQueryGraphTouchDescriptor::live_read("topology.edge")
                .expect("static live-read certification touch is valid")
        }
        _ => WorthQueryGraphTouchDescriptor::declared_mutation_collection(
            "topology.edge",
            WorthQueryMutationFamily::Update,
            None,
            [WorthQueryAspectMutationOperation::set(
                capacity_aspect_touch(),
            )],
            [capacity_aspect_touch()],
        )
        .expect("static mutation certification touch is valid"),
    }
}

fn capacity_aspect_touch() -> WorthQueryAspectTouch {
    WorthQueryAspectTouch::whole_aspect(
        AspectKey::new("capacity").expect("static support matrix aspect key should admit"),
    )
}

fn expected_execution_status_for_row(
    row: &WorthQueryGraphObligationSupportMatrixRow,
) -> WorthQueryGraphObligationExecutionStatus {
    match row.status() {
        WorthQueryGraphObligationSupportStatus::Supported => {
            if row.obligation_kind() == WorthQueryGraphObligationKind::PreflightSequencingObligation
            {
                WorthQueryGraphObligationExecutionStatus::BlockedByPrerequisite
            } else {
                WorthQueryGraphObligationExecutionStatus::Executed
            }
        }
        WorthQueryGraphObligationSupportStatus::Unsupported => {
            WorthQueryGraphObligationExecutionStatus::Unsupported
        }
        WorthQueryGraphObligationSupportStatus::NotApplicable => {
            WorthQueryGraphObligationExecutionStatus::NotApplicableAfterStateLoad
        }
        WorthQueryGraphObligationSupportStatus::DiagnosticOnly => {
            WorthQueryGraphObligationExecutionStatus::DiagnosticOnly
        }
        WorthQueryGraphObligationSupportStatus::DeferredToBackstop => {
            WorthQueryGraphObligationExecutionStatus::DeferredToBackstop
        }
    }
}

fn support_posture_for_row(
    row: &WorthQueryGraphObligationSupportMatrixRow,
) -> WorthQueryGraphObligationSupportPosture {
    match row.status() {
        WorthQueryGraphObligationSupportStatus::Supported => {
            WorthQueryGraphObligationSupportPosture::supported(row.support_lane())
        }
        WorthQueryGraphObligationSupportStatus::Unsupported => {
            WorthQueryGraphObligationSupportPosture::unsupported(row.support_lane())
        }
        WorthQueryGraphObligationSupportStatus::NotApplicable => {
            WorthQueryGraphObligationSupportPosture::not_applicable(row.support_lane())
        }
        WorthQueryGraphObligationSupportStatus::DiagnosticOnly => {
            WorthQueryGraphObligationSupportPosture::diagnostic_only(row.support_lane())
        }
        WorthQueryGraphObligationSupportStatus::DeferredToBackstop => {
            WorthQueryGraphObligationSupportPosture::deferred_to_backstop(row.support_lane())
        }
    }
}

fn rule_for_row(
    row: &WorthQueryGraphObligationSupportMatrixRow,
) -> WorthQueryGraphObligationRuleIdentity {
    WorthQueryGraphObligationRuleIdentity::new(
        "milestone-9.9.authority-certification",
        format!(
            "{}:{}",
            row.obligation_kind().as_str(),
            row.support_lane().as_str()
        ),
        "v1",
    )
    .expect("static certification rule identity is non-empty")
}
