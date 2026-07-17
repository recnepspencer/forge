use crate::declaration::stable_text_digest;

use crate::evidence::{
    UiConstraintPropagationEdgeFamily, UiMeasurementCoordinateSpace, UiMeasurementRoundingPosture,
    UiMeasurementUnitPosture,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiScrollOwnerPlanningInputSolveOrder {
    BeforeDerivedConstraintFamilies,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiScrollOwnerPlanningInputPosture {
    AdmittedPlanningTimeOnly,
    MissingRequiredEvidence,
    IncompatibleMeasurementPosture,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct UiScrollOwnerSourceAdmissionCounters {
    source_inputs_visited: u64,
    admitted_sources: u64,
    duplicates_elided: u64,
}

impl UiScrollOwnerSourceAdmissionCounters {
    pub(crate) const fn new(
        source_inputs_visited: u64,
        admitted_sources: u64,
        duplicates_elided: u64,
    ) -> Self {
        Self {
            source_inputs_visited,
            admitted_sources,
            duplicates_elided,
        }
    }

    pub const fn source_inputs_visited(self) -> u64 {
        self.source_inputs_visited
    }
    pub const fn admitted_sources(self) -> u64 {
        self.admitted_sources
    }
    pub const fn duplicates_elided(self) -> u64 {
        self.duplicates_elided
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiConstraintScrollOwnerPlanningInputResult {
    neighborhood_identity_digest: u64,
    solve_order: UiScrollOwnerPlanningInputSolveOrder,
    posture: UiScrollOwnerPlanningInputPosture,
    source_evidence_identity_digest: Option<u64>,
    source_generation_digest: Option<u64>,
    unit_posture: Option<UiMeasurementUnitPosture>,
    coordinate_space: Option<UiMeasurementCoordinateSpace>,
    rounding_posture: Option<UiMeasurementRoundingPosture>,
    planning_time_only: bool,
    source_evidence: Box<[super::UiScrollOwnerSourceEvidence]>,
    source_admission_counters: UiScrollOwnerSourceAdmissionCounters,
    identity_digest: u64,
}

pub(crate) struct UiConstraintScrollOwnerPlanningInput {
    pub neighborhood_identity_digest: u64,
    pub solve_order: UiScrollOwnerPlanningInputSolveOrder,
    pub posture: UiScrollOwnerPlanningInputPosture,
    pub source_evidence_identity_digest: Option<u64>,
    pub source_generation_digest: Option<u64>,
    pub unit_posture: Option<UiMeasurementUnitPosture>,
    pub coordinate_space: Option<UiMeasurementCoordinateSpace>,
    pub rounding_posture: Option<UiMeasurementRoundingPosture>,
    pub planning_time_only: bool,
    pub source_evidence: Vec<super::UiScrollOwnerSourceEvidence>,
    pub source_admission_counters: UiScrollOwnerSourceAdmissionCounters,
}

impl UiConstraintScrollOwnerPlanningInputResult {
    pub(crate) fn new(
        _mint_authority: &crate::graph::UiGraphConstraintMintAuthority,
        input: UiConstraintScrollOwnerPlanningInput,
    ) -> Self {
        let UiConstraintScrollOwnerPlanningInput {
            neighborhood_identity_digest,
            solve_order,
            posture,
            source_evidence_identity_digest,
            source_generation_digest,
            unit_posture,
            coordinate_space,
            rounding_posture,
            planning_time_only,
            mut source_evidence,
            source_admission_counters,
        } = input;
        source_evidence.sort_unstable();
        source_evidence.dedup();
        let source_set_digest = source_evidence.iter().fold(
            stable_text_digest("worth-ui.scroll-source-set"),
            |digest, source| digest.rotate_left(7) ^ source.identity_digest(),
        );
        let identity_digest = stable_text_digest("worth-ui.constraint-scroll-owner-planning-input")
            ^ neighborhood_identity_digest.rotate_left(7)
            ^ solve_order_digest(solve_order).rotate_left(13)
            ^ posture_digest(posture).rotate_left(19)
            ^ optional_digest(
                source_evidence_identity_digest,
                "worth-ui.constraint-scroll-owner.no-source",
            )
            .rotate_left(23)
            ^ optional_digest(
                source_generation_digest,
                "worth-ui.constraint-scroll-owner.no-generation",
            )
            .rotate_left(29)
            ^ unit_posture_digest(unit_posture).rotate_left(31)
            ^ coordinate_space_digest(coordinate_space).rotate_left(37)
            ^ rounding_posture_digest(rounding_posture).rotate_left(41)
            ^ bool_digest(planning_time_only).rotate_left(43)
            ^ source_set_digest.rotate_left(47)
            ^ source_admission_counters
                .source_inputs_visited()
                .rotate_left(17)
            ^ source_admission_counters.admitted_sources().rotate_left(31)
            ^ source_admission_counters
                .duplicates_elided()
                .rotate_left(53);
        Self {
            neighborhood_identity_digest,
            solve_order,
            posture,
            source_evidence_identity_digest,
            source_generation_digest,
            unit_posture,
            coordinate_space,
            rounding_posture,
            planning_time_only,
            source_evidence: source_evidence.into_boxed_slice(),
            source_admission_counters,
            identity_digest,
        }
    }

    pub fn edge_family(&self) -> UiConstraintPropagationEdgeFamily {
        UiConstraintPropagationEdgeFamily::ScrollViewportInput
    }

    pub fn neighborhood_identity_digest(&self) -> u64 {
        self.neighborhood_identity_digest
    }

    pub fn solve_order(&self) -> UiScrollOwnerPlanningInputSolveOrder {
        self.solve_order
    }

    pub fn posture(&self) -> UiScrollOwnerPlanningInputPosture {
        self.posture
    }

    pub fn source_evidence_identity_digest(&self) -> Option<u64> {
        self.source_evidence_identity_digest
    }

    pub fn source_generation_digest(&self) -> Option<u64> {
        self.source_generation_digest
    }

    pub fn unit_posture(&self) -> Option<UiMeasurementUnitPosture> {
        self.unit_posture
    }

    pub fn coordinate_space(&self) -> Option<UiMeasurementCoordinateSpace> {
        self.coordinate_space
    }

    pub fn rounding_posture(&self) -> Option<UiMeasurementRoundingPosture> {
        self.rounding_posture
    }

    pub fn is_planning_time_only(&self) -> bool {
        self.planning_time_only
    }

    pub fn identity_digest(&self) -> u64 {
        self.identity_digest
    }
    pub fn source_evidence(&self) -> &[super::UiScrollOwnerSourceEvidence] {
        &self.source_evidence
    }
    pub fn source_set_digest(&self) -> u64 {
        self.source_evidence.iter().fold(
            stable_text_digest("worth-ui.scroll-source-set"),
            |digest, source| digest.rotate_left(7) ^ source.identity_digest(),
        )
    }

    pub fn source_admission_counters(&self) -> UiScrollOwnerSourceAdmissionCounters {
        self.source_admission_counters
    }
}

fn solve_order_digest(order: UiScrollOwnerPlanningInputSolveOrder) -> u64 {
    match order {
        UiScrollOwnerPlanningInputSolveOrder::BeforeDerivedConstraintFamilies => {
            stable_text_digest("worth-ui.constraint-scroll-owner.solve-order.before-derived")
        }
    }
}

fn posture_digest(posture: UiScrollOwnerPlanningInputPosture) -> u64 {
    stable_text_digest(match posture {
        UiScrollOwnerPlanningInputPosture::AdmittedPlanningTimeOnly => {
            "worth-ui.constraint-scroll-owner.posture.admitted-planning-time-only"
        }
        UiScrollOwnerPlanningInputPosture::MissingRequiredEvidence => {
            "worth-ui.constraint-scroll-owner.posture.missing-required-evidence"
        }
        UiScrollOwnerPlanningInputPosture::IncompatibleMeasurementPosture => {
            "worth-ui.constraint-scroll-owner.posture.incompatible-measurement-posture"
        }
    })
}

fn unit_posture_digest(posture: Option<UiMeasurementUnitPosture>) -> u64 {
    stable_text_digest(match posture {
        Some(UiMeasurementUnitPosture::LogicalPx) => {
            "worth-ui.constraint-scroll-owner.unit.logical-px"
        }
        Some(UiMeasurementUnitPosture::PhysicalPx) => {
            "worth-ui.constraint-scroll-owner.unit.physical-px"
        }
        Some(UiMeasurementUnitPosture::UnitlessScale) => {
            "worth-ui.constraint-scroll-owner.unit.unitless-scale"
        }
        None => "worth-ui.constraint-scroll-owner.unit.none",
    })
}

fn coordinate_space_digest(space: Option<UiMeasurementCoordinateSpace>) -> u64 {
    stable_text_digest(match space {
        Some(UiMeasurementCoordinateSpace::Viewport) => {
            "worth-ui.constraint-scroll-owner.coordinate.viewport"
        }
        Some(UiMeasurementCoordinateSpace::Window) => {
            "worth-ui.constraint-scroll-owner.coordinate.window"
        }
        Some(UiMeasurementCoordinateSpace::GraphNodeLocal) => {
            "worth-ui.constraint-scroll-owner.coordinate.graph-node-local"
        }
        Some(UiMeasurementCoordinateSpace::HostSurface) => {
            "worth-ui.constraint-scroll-owner.coordinate.host-surface"
        }
        Some(UiMeasurementCoordinateSpace::PortalLayer) => {
            "worth-ui.constraint-scroll-owner.coordinate.portal-layer"
        }
        None => "worth-ui.constraint-scroll-owner.coordinate.none",
    })
}

fn rounding_posture_digest(posture: Option<UiMeasurementRoundingPosture>) -> u64 {
    stable_text_digest(match posture {
        Some(UiMeasurementRoundingPosture::ExactFloat) => {
            "worth-ui.constraint-scroll-owner.rounding.exact-float"
        }
        Some(UiMeasurementRoundingPosture::HostRounded) => {
            "worth-ui.constraint-scroll-owner.rounding.host-rounded"
        }
        Some(UiMeasurementRoundingPosture::RuntimeRounded) => {
            "worth-ui.constraint-scroll-owner.rounding.runtime-rounded"
        }
        Some(UiMeasurementRoundingPosture::DeferredToAllocation) => {
            "worth-ui.constraint-scroll-owner.rounding.deferred"
        }
        None => "worth-ui.constraint-scroll-owner.rounding.none",
    })
}

fn optional_digest(value: Option<u64>, none_text: &str) -> u64 {
    value.unwrap_or_else(|| stable_text_digest(none_text))
}

fn bool_digest(value: bool) -> u64 {
    stable_text_digest(if value {
        "worth-ui.constraint-scroll-owner.planning-time-only"
    } else {
        "worth-ui.constraint-scroll-owner.not-planning-time-only"
    })
}
