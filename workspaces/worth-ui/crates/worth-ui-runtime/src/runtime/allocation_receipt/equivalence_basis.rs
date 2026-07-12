use crate::evidence::{
    UiAllocationConstraintSummary, UiConstraintChildIntrinsicContribution,
    UiConstraintCycleParticipationPosture, UiConstraintHostIntrinsicKind,
    UiConstraintIntrinsicSourcePosture, UiConstraintPropagationEdgeFamily,
    UiConstraintPropagationEdgePayload, UiLayoutOperatorContractIdentity,
    UiMeasurementCoordinateSpace, UiMeasurementRoundingPosture, UiMeasurementUnitPosture,
};

/// Reuse-relevant payload semantics. Child intrinsic extents are deliberately
/// absent: they are the one admitted partial-reuse delta; every other payload
/// field remains part of structural equivalence.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiAllocationConstraintPayloadShape {
    ParentAvailableSpace(crate::evidence::UiConstraintParentAvailableSpace),
    ChildIntrinsicContribution {
        axis_scope: crate::evidence::UiConstraintAxisScope,
        source_posture: UiConstraintIntrinsicSourcePosture,
        host_kind: UiConstraintHostIntrinsicKind,
        unit_posture: UiMeasurementUnitPosture,
        coordinate_space: UiMeasurementCoordinateSpace,
        rounding_posture: UiMeasurementRoundingPosture,
    },
    SiblingNegotiation {
        axis_scope: crate::evidence::UiConstraintAxisScope,
        group_identity_digest: u64,
        negotiation_identity_digest: u64,
        fixed_point_policy: crate::evidence::UiConstraintSiblingNegotiationFixedPointPolicy,
        solve_order: crate::evidence::UiConstraintSiblingNegotiationSolveOrder,
    },
    EqualShareDistribution {
        axis_scope: crate::evidence::UiConstraintAxisScope,
        policy: crate::evidence::UiConstraintEqualShareDistributionPolicy,
        group_identity_digest: u64,
        distribution_identity_digest: u64,
        solve_order: crate::evidence::UiConstraintEqualShareSolveOrder,
        posture: crate::evidence::UiConstraintEqualSharePosture,
    },
    BoundedReconciliation {
        axis_scope: crate::evidence::UiConstraintAxisScope,
        reconciliation_identity_digest: u64,
        solve_order: crate::evidence::UiBoundReconciliationSolveOrder,
        posture: crate::evidence::UiBoundReconciliationPosture,
    },
    ViewportInput {
        viewport_identity_digest: u64,
        solve_order: crate::evidence::UiViewportPlanningInputSolveOrder,
        posture: crate::evidence::UiViewportPlanningInputPosture,
        planning_time_only: bool,
    },
    ScrollViewportInput {
        scroll_identity_digest: u64,
        solve_order: crate::evidence::UiScrollOwnerPlanningInputSolveOrder,
        posture: crate::evidence::UiScrollOwnerPlanningInputPosture,
        planning_time_only: bool,
    },
    PortalAnchorInput {
        portal_identity_digest: u64,
        solve_order: crate::evidence::UiPortalAnchorPlanningInputSolveOrder,
        posture: crate::evidence::UiPortalAnchorPlanningInputPosture,
        planning_time_only: bool,
    },
    DurableResizeInput {
        durable_identity_digest: u64,
        axis_scope: crate::evidence::UiConstraintAxisScope,
        posture: crate::evidence::UiConstraintResizeInputPosture,
        planning_time_only: bool,
    },
}

impl UiAllocationConstraintPayloadShape {
    fn from_payload(payload: UiConstraintPropagationEdgePayload) -> Self {
        match payload {
            UiConstraintPropagationEdgePayload::ParentAvailableSpace(value) => {
                Self::ParentAvailableSpace(value)
            }
            UiConstraintPropagationEdgePayload::ChildIntrinsicContribution(value) => {
                Self::child_intrinsic(value)
            }
            UiConstraintPropagationEdgePayload::SiblingNegotiation {
                axis_scope,
                group_identity_digest,
                negotiation_identity_digest,
                fixed_point_policy,
                solve_order,
            } => Self::SiblingNegotiation {
                axis_scope,
                group_identity_digest,
                negotiation_identity_digest,
                fixed_point_policy,
                solve_order,
            },
            UiConstraintPropagationEdgePayload::EqualShareDistribution {
                axis_scope,
                policy,
                group_identity_digest,
                distribution_identity_digest,
                solve_order,
                posture,
            } => Self::EqualShareDistribution {
                axis_scope,
                policy,
                group_identity_digest,
                distribution_identity_digest,
                solve_order,
                posture,
            },
            UiConstraintPropagationEdgePayload::BoundedReconciliation {
                axis_scope,
                reconciliation_identity_digest,
                solve_order,
                posture,
            } => Self::BoundedReconciliation {
                axis_scope,
                reconciliation_identity_digest,
                solve_order,
                posture,
            },
            UiConstraintPropagationEdgePayload::ViewportInput {
                viewport_identity_digest,
                solve_order,
                posture,
                planning_time_only,
            } => Self::ViewportInput {
                viewport_identity_digest,
                solve_order,
                posture,
                planning_time_only,
            },
            UiConstraintPropagationEdgePayload::ScrollViewportInput {
                scroll_identity_digest,
                solve_order,
                posture,
                planning_time_only,
            } => Self::ScrollViewportInput {
                scroll_identity_digest,
                solve_order,
                posture,
                planning_time_only,
            },
            UiConstraintPropagationEdgePayload::PortalAnchorInput {
                portal_identity_digest,
                solve_order,
                posture,
                planning_time_only,
            } => Self::PortalAnchorInput {
                portal_identity_digest,
                solve_order,
                posture,
                planning_time_only,
            },
            UiConstraintPropagationEdgePayload::DurableResizeInput {
                durable_identity_digest,
                axis_scope,
                posture,
                planning_time_only,
            } => Self::DurableResizeInput {
                durable_identity_digest,
                axis_scope,
                posture,
                planning_time_only,
            },
        }
    }

    fn child_intrinsic(value: UiConstraintChildIntrinsicContribution) -> Self {
        Self::ChildIntrinsicContribution {
            axis_scope: value.axis_scope(),
            source_posture: value.source_posture(),
            host_kind: value.host_kind(),
            unit_posture: value.unit_posture(),
            coordinate_space: value.coordinate_space(),
            rounding_posture: value.rounding_posture(),
        }
    }
}

/// Reuse-relevant propagation topology, excluding leaf contribution values.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiAllocationConstraintPropagationShape {
    family: UiConstraintPropagationEdgeFamily,
    source_member_identity_digest: u64,
    target_member_identity_digest: u64,
    cycle_participation_posture: UiConstraintCycleParticipationPosture,
    payload: UiAllocationConstraintPayloadShape,
}

impl UiAllocationConstraintPropagationShape {
    fn from_edge(edge: &crate::evidence::UiConstraintPropagationEdge) -> Self {
        Self {
            family: edge.family(),
            source_member_identity_digest: edge.source_member_identity_digest(),
            target_member_identity_digest: edge.target_member_identity_digest(),
            cycle_participation_posture: edge.cycle_participation_posture(),
            payload: UiAllocationConstraintPayloadShape::from_payload(edge.payload()),
        }
    }

    pub fn family(&self) -> UiConstraintPropagationEdgeFamily {
        self.family
    }
    pub fn source_member_identity_digest(&self) -> u64 {
        self.source_member_identity_digest
    }
    pub fn target_member_identity_digest(&self) -> u64 {
        self.target_member_identity_digest
    }
    pub fn cycle_participation_posture(&self) -> UiConstraintCycleParticipationPosture {
        self.cycle_participation_posture
    }

    pub fn payload(&self) -> UiAllocationConstraintPayloadShape {
        self.payload
    }
}

/// Reuse-relevant constraint topology, deliberately excluding leaf values.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiAllocationReceiptConstraintShape {
    summary: UiAllocationConstraintSummary,
    propagation_edges: Box<[UiAllocationConstraintPropagationShape]>,
}

impl UiAllocationReceiptConstraintShape {
    fn from_constraint_set(set: &crate::evidence::UiAllocationConstraintSet) -> Self {
        let mut propagation_edges = set
            .propagation_edges()
            .iter()
            .map(UiAllocationConstraintPropagationShape::from_edge)
            .collect::<Vec<_>>();
        propagation_edges.sort_unstable_by_key(|edge| {
            (
                edge.family,
                edge.source_member_identity_digest,
                edge.target_member_identity_digest,
                edge.cycle_participation_posture.rank(),
                payload_rank(edge.payload),
            )
        });
        Self {
            summary: set.summary(),
            propagation_edges: propagation_edges.into_boxed_slice(),
        }
    }

    pub fn summary(&self) -> UiAllocationConstraintSummary {
        self.summary
    }

    pub fn propagation_edges(&self) -> &[UiAllocationConstraintPropagationShape] {
        &self.propagation_edges
    }
}

fn payload_rank(payload: UiAllocationConstraintPayloadShape) -> u8 {
    match payload {
        UiAllocationConstraintPayloadShape::ParentAvailableSpace(_) => 0,
        UiAllocationConstraintPayloadShape::ChildIntrinsicContribution { .. } => 1,
        UiAllocationConstraintPayloadShape::SiblingNegotiation { .. } => 2,
        UiAllocationConstraintPayloadShape::EqualShareDistribution { .. } => 3,
        UiAllocationConstraintPayloadShape::BoundedReconciliation { .. } => 4,
        UiAllocationConstraintPayloadShape::ViewportInput { .. } => 5,
        UiAllocationConstraintPayloadShape::ScrollViewportInput { .. } => 6,
        UiAllocationConstraintPayloadShape::PortalAnchorInput { .. } => 7,
        UiAllocationConstraintPayloadShape::DurableResizeInput { .. } => 8,
    }
}

/// Admitted operator-family basis used to evaluate allocation receipt reuse.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiAllocationReceiptEquivalenceBasis {
    coordinate_ownership: UiLayoutOperatorContractIdentity,
    constraint_shape: Option<UiAllocationReceiptConstraintShape>,
    resize_basis: Option<crate::runtime::UiResizeAllocationPlanningBasis>,
    portal_basis_identity_digest: Option<u64>,
}

impl UiAllocationReceiptEquivalenceBasis {
    pub(crate) fn identity_digest(&self) -> u64 {
        self.coordinate_ownership.identity_digest()
            ^ self
                .resize_basis
                .as_ref()
                .map_or(0, |basis| basis.identity_digest().rotate_left(23))
            ^ self
                .portal_basis_identity_digest
                .map_or(0, |digest| digest.rotate_left(41))
    }
    pub(crate) fn from_candidate(candidate: &super::UiAllocationCandidate) -> Self {
        Self {
            coordinate_ownership: candidate
                .allocation_neighborhood()
                .identity()
                .layout_operator_contract_identity(),
            constraint_shape: candidate
                .allocation_constraint_set()
                .map(UiAllocationReceiptConstraintShape::from_constraint_set),
            resize_basis: candidate.resize_basis().cloned(),
            portal_basis_identity_digest: candidate
                .portal_allocation_input()
                .map(crate::runtime::UiPortalAllocationPlanningBasis::identity_digest),
        }
    }
    pub fn coordinate_ownership(&self) -> UiLayoutOperatorContractIdentity {
        self.coordinate_ownership
    }
    pub fn constraint_shape(&self) -> Option<&UiAllocationReceiptConstraintShape> {
        self.constraint_shape.as_ref()
    }
    pub fn resize_basis(&self) -> Option<&crate::runtime::UiResizeAllocationPlanningBasis> {
        self.resize_basis.as_ref()
    }
}
