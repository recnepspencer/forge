use crate::runtime::{
    ForgeQueryAspectMutationOperation, ForgeQueryAspectTouch,
    ForgeQueryGraphObligationExecutionStatus, ForgeQueryGraphObligationKind,
    ForgeQueryGraphObligationOperatingWorldDescriptor,
    ForgeQueryGraphObligationOperatingWorldSelector, ForgeQueryGraphObligationRegistration,
    ForgeQueryGraphObligationRuleIdentity, ForgeQueryGraphObligationSupportLane,
    ForgeQueryGraphObligationSupportMatrix, ForgeQueryGraphObligationSupportMatrixRow,
    ForgeQueryGraphObligationSupportPosture, ForgeQueryGraphObligationSupportStatus,
    ForgeQueryGraphTouchDescriptor, ForgeQueryGraphTouchReadVerb, ForgeQueryGraphTouchSelector,
    ForgeQueryMutationFamily,
};
use forge_foundational::facade::AspectKey;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryGraphObligationMatrixCertificationCase {
    row: ForgeQueryGraphObligationSupportMatrixRow,
    selector: ForgeQueryGraphTouchSelector,
    touch_descriptor: ForgeQueryGraphTouchDescriptor,
    operating_world: ForgeQueryGraphObligationOperatingWorldDescriptor,
    expected_execution_status: ForgeQueryGraphObligationExecutionStatus,
}

impl ForgeQueryGraphObligationMatrixCertificationCase {
    pub fn milestone_9_9_authority_cases() -> Vec<Self> {
        ForgeQueryGraphObligationSupportMatrix::milestone_9_9_authority_surface()
            .rows()
            .iter()
            .map(Self::from_matrix_row)
            .collect()
    }

    pub fn from_matrix_row(row: &ForgeQueryGraphObligationSupportMatrixRow) -> Self {
        let selector = representative_selector_for_lane(row.support_lane());
        let touch_descriptor = representative_touch_for_lane(row.support_lane());
        let expected_execution_status = expected_execution_status_for_row(row);
        Self {
            row: row.clone(),
            selector,
            touch_descriptor,
            operating_world:
                ForgeQueryGraphObligationOperatingWorldDescriptor::any_committed_authority(),
            expected_execution_status,
        }
    }

    pub fn row(&self) -> &ForgeQueryGraphObligationSupportMatrixRow {
        &self.row
    }

    pub fn selector(&self) -> &ForgeQueryGraphTouchSelector {
        &self.selector
    }

    pub fn touch_descriptor(&self) -> &ForgeQueryGraphTouchDescriptor {
        &self.touch_descriptor
    }

    pub fn operating_world(&self) -> &ForgeQueryGraphObligationOperatingWorldDescriptor {
        &self.operating_world
    }

    pub fn expected_execution_status(&self) -> ForgeQueryGraphObligationExecutionStatus {
        self.expected_execution_status
    }

    pub fn registration(&self) -> ForgeQueryGraphObligationRegistration {
        ForgeQueryGraphObligationRegistration::new(
            self.row.obligation_kind(),
            rule_for_row(&self.row),
            self.selector.clone(),
            ForgeQueryGraphObligationOperatingWorldSelector::any_committed_authority(),
        )
        .with_support_posture(support_posture_for_row(&self.row))
        .with_execution_budget(self.row.execution_budget().clone())
    }
}

fn representative_selector_for_lane(
    lane: ForgeQueryGraphObligationSupportLane,
) -> ForgeQueryGraphTouchSelector {
    match lane {
        ForgeQueryGraphObligationSupportLane::ReadFamily => {
            ForgeQueryGraphTouchSelector::read_verb(
                ForgeQueryGraphTouchReadVerb::ObservesRelationKind,
            )
        }
        ForgeQueryGraphObligationSupportLane::LiveRead => ForgeQueryGraphTouchSelector::read_verb(
            ForgeQueryGraphTouchReadVerb::RetainsLiveSubscription,
        ),
        _ => ForgeQueryGraphTouchSelector::collection("topology.edge")
            .expect("static certification selector is non-empty"),
    }
}

fn representative_touch_for_lane(
    lane: ForgeQueryGraphObligationSupportLane,
) -> ForgeQueryGraphTouchDescriptor {
    match lane {
        ForgeQueryGraphObligationSupportLane::ReadFamily => {
            ForgeQueryGraphTouchDescriptor::read_family(
                "topology.edge",
                [ForgeQueryGraphTouchReadVerb::ObservesRelationKind],
            )
            .expect("static read certification touch is valid")
        }
        ForgeQueryGraphObligationSupportLane::LiveRead => {
            ForgeQueryGraphTouchDescriptor::live_read("topology.edge")
                .expect("static live-read certification touch is valid")
        }
        _ => ForgeQueryGraphTouchDescriptor::declared_mutation_collection(
            "topology.edge",
            ForgeQueryMutationFamily::Update,
            None,
            [ForgeQueryAspectMutationOperation::set(
                capacity_aspect_touch(),
            )],
            [capacity_aspect_touch()],
        )
        .expect("static mutation certification touch is valid"),
    }
}

fn capacity_aspect_touch() -> ForgeQueryAspectTouch {
    ForgeQueryAspectTouch::whole_aspect(
        AspectKey::new("capacity").expect("static support matrix aspect key should admit"),
    )
}

fn expected_execution_status_for_row(
    row: &ForgeQueryGraphObligationSupportMatrixRow,
) -> ForgeQueryGraphObligationExecutionStatus {
    match row.status() {
        ForgeQueryGraphObligationSupportStatus::Supported => {
            if row.obligation_kind() == ForgeQueryGraphObligationKind::PreflightSequencingObligation
            {
                ForgeQueryGraphObligationExecutionStatus::BlockedByPrerequisite
            } else {
                ForgeQueryGraphObligationExecutionStatus::Executed
            }
        }
        ForgeQueryGraphObligationSupportStatus::Unsupported => {
            ForgeQueryGraphObligationExecutionStatus::Unsupported
        }
        ForgeQueryGraphObligationSupportStatus::NotApplicable => {
            ForgeQueryGraphObligationExecutionStatus::NotApplicableAfterStateLoad
        }
        ForgeQueryGraphObligationSupportStatus::DiagnosticOnly => {
            ForgeQueryGraphObligationExecutionStatus::DiagnosticOnly
        }
        ForgeQueryGraphObligationSupportStatus::DeferredToBackstop => {
            ForgeQueryGraphObligationExecutionStatus::DeferredToBackstop
        }
    }
}

fn support_posture_for_row(
    row: &ForgeQueryGraphObligationSupportMatrixRow,
) -> ForgeQueryGraphObligationSupportPosture {
    match row.status() {
        ForgeQueryGraphObligationSupportStatus::Supported => {
            ForgeQueryGraphObligationSupportPosture::supported(row.support_lane())
        }
        ForgeQueryGraphObligationSupportStatus::Unsupported => {
            ForgeQueryGraphObligationSupportPosture::unsupported(row.support_lane())
        }
        ForgeQueryGraphObligationSupportStatus::NotApplicable => {
            ForgeQueryGraphObligationSupportPosture::not_applicable(row.support_lane())
        }
        ForgeQueryGraphObligationSupportStatus::DiagnosticOnly => {
            ForgeQueryGraphObligationSupportPosture::diagnostic_only(row.support_lane())
        }
        ForgeQueryGraphObligationSupportStatus::DeferredToBackstop => {
            ForgeQueryGraphObligationSupportPosture::deferred_to_backstop(row.support_lane())
        }
    }
}

fn rule_for_row(
    row: &ForgeQueryGraphObligationSupportMatrixRow,
) -> ForgeQueryGraphObligationRuleIdentity {
    ForgeQueryGraphObligationRuleIdentity::new(
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
