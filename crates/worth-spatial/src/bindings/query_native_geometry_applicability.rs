use crate::bindings::query_native_geometry_applicability_planar::classify_planar_contract_surface;
use crate::bindings::query_native_geometry_inventory::GeometryPublicSurface;
use worth_primitives::{truth_digest_parts, TruthDigestScope};
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum GeometryRuntimeConcern {
    GroupedNeighborhoodWorkflow,
    ContributionComposition,
    LowerRuntimeRouting,
    RecoveryAction,
    MutationEvidence,
    ProjectionConsumption,
    SignalContinuation,
    HistoricalInspection,
    BranchLocalInspection,
    ReplayParity,
    BooleanReadinessCertification,
}

impl GeometryRuntimeConcern {
    pub const fn all() -> [Self; 11] {
        [
            Self::GroupedNeighborhoodWorkflow,
            Self::ContributionComposition,
            Self::LowerRuntimeRouting,
            Self::RecoveryAction,
            Self::MutationEvidence,
            Self::ProjectionConsumption,
            Self::SignalContinuation,
            Self::HistoricalInspection,
            Self::BranchLocalInspection,
            Self::ReplayParity,
            Self::BooleanReadinessCertification,
        ]
    }

    pub const fn as_str(self) -> &'static str {
        match self {
            Self::GroupedNeighborhoodWorkflow => "grouped-neighborhood-workflow",
            Self::ContributionComposition => "contribution-composition",
            Self::LowerRuntimeRouting => "lower-runtime-routing",
            Self::RecoveryAction => "recovery-action",
            Self::MutationEvidence => "mutation-evidence",
            Self::ProjectionConsumption => "projection-consumption",
            Self::SignalContinuation => "signal-continuation",
            Self::HistoricalInspection => "historical-inspection",
            Self::BranchLocalInspection => "branch-local-inspection",
            Self::ReplayParity => "replay-parity",
            Self::BooleanReadinessCertification => "boolean-readiness-certification",
        }
    }
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum GeometryApplicabilityStatus {
    RequiredNow,
    NotApplicable,
    DeniedForThisRuntime,
}
impl GeometryApplicabilityStatus {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RequiredNow => "required-now",
            Self::NotApplicable => "not-applicable",
            Self::DeniedForThisRuntime => "denied-for-this-runtime",
        }
    }
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GeometryApplicabilityRow {
    surface: GeometryPublicSurface,
    concern: GeometryRuntimeConcern,
    status: GeometryApplicabilityStatus,
    rationale: &'static str,
    row_digest: String,
}
impl GeometryApplicabilityRow {
    fn new(surface: GeometryPublicSurface, concern: GeometryRuntimeConcern) -> Self {
        let (status, rationale) = classify(surface, concern);
        let row_digest = hash_parts(&[
            format!("surface:{}", surface.as_str()),
            format!("concern:{}", concern.as_str()),
            format!("status:{}", status.as_str()),
            format!("rationale:{rationale}"),
        ]);
        Self {
            surface,
            concern,
            status,
            rationale,
            row_digest,
        }
    }

    pub fn surface(&self) -> GeometryPublicSurface {
        self.surface
    }

    pub fn concern(&self) -> GeometryRuntimeConcern {
        self.concern
    }

    pub fn status(&self) -> GeometryApplicabilityStatus {
        self.status
    }

    pub fn rationale(&self) -> &'static str {
        self.rationale
    }

    pub fn row_digest(&self) -> &str {
        &self.row_digest
    }
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GeometryApplicabilityMatrix {
    rows: Vec<GeometryApplicabilityRow>,
    matrix_digest: String,
}

impl GeometryApplicabilityMatrix {
    pub fn rows(&self) -> &[GeometryApplicabilityRow] {
        &self.rows
    }

    pub fn matrix_digest(&self) -> &str {
        &self.matrix_digest
    }

    pub fn row(
        &self,
        surface: GeometryPublicSurface,
        concern: GeometryRuntimeConcern,
    ) -> Option<&GeometryApplicabilityRow> {
        self.rows
            .iter()
            .find(|row| row.surface() == surface && row.concern() == concern)
    }
}
pub fn geometry_applicability_matrix() -> GeometryApplicabilityMatrix {
    let rows = GeometryPublicSurface::all()
        .into_iter()
        .flat_map(|surface| {
            GeometryRuntimeConcern::all()
                .into_iter()
                .map(move |concern| GeometryApplicabilityRow::new(surface, concern))
        })
        .collect::<Vec<_>>();
    let matrix_digest = hash_parts(
        &rows
            .iter()
            .map(|row| row.row_digest().to_string())
            .collect::<Vec<_>>(),
    );
    GeometryApplicabilityMatrix {
        rows,
        matrix_digest,
    }
}
fn classify(
    surface: GeometryPublicSurface,
    concern: GeometryRuntimeConcern,
) -> (GeometryApplicabilityStatus, &'static str) {
    use GeometryApplicabilityStatus::{
        DeniedForThisRuntime as Denied, NotApplicable as NA, RequiredNow as Required,
    };
    use GeometryPublicSurface as Surface;
    use GeometryRuntimeConcern as Concern;

    if let Some(classification) = classify_planar_contract_surface(surface, concern) {
        return classification;
    }

    match (surface, concern) {
        (Surface::GeometryTargetIdentity, Concern::HistoricalInspection)
        | (Surface::GeometryTargetIdentity, Concern::BranchLocalInspection) => (
            Required,
            "target identity must survive retained-view inspection without local semantic replay",
        ),
        (Surface::SpatialAnchorSelection, Concern::LowerRuntimeRouting) => (
            Required,
            "anchor selection is an admitted declaration family with an explicit relational route",
        ),
        (Surface::PrimitiveBinding, Concern::LowerRuntimeRouting)
        | (Surface::PrimitiveBinding, Concern::MutationEvidence)
        | (Surface::PrimitiveAnchorBinding, Concern::LowerRuntimeRouting)
        | (Surface::PrimitiveAnchorBinding, Concern::MutationEvidence) => (
            Required,
            "binding-family workflow already exposes admitted routing and mutation evidence on the ordinary path",
        ),
        (Surface::PrimitiveRebinding, Concern::GroupedNeighborhoodWorkflow)
        | (Surface::PrimitiveRebinding, Concern::ContributionComposition)
        | (Surface::PrimitiveRebinding, Concern::LowerRuntimeRouting)
        | (Surface::PrimitiveRebinding, Concern::RecoveryAction)
        | (Surface::PrimitiveRebinding, Concern::MutationEvidence)
        | (Surface::PrimitiveRebinding, Concern::ProjectionConsumption)
        | (Surface::PrimitiveRebinding, Concern::HistoricalInspection)
        | (Surface::PrimitiveRebinding, Concern::BranchLocalInspection)
        | (Surface::PrimitiveRebinding, Concern::ReplayParity) => (
            Required,
            "primitive rebinding already owns this admitted ordinary runtime lane through real Query families and retained artifacts",
        ),
        (Surface::PrimitiveRebinding, Concern::SignalContinuation) => (
            Required,
            "primitive rebinding now exposes explicit signal-compatibility and bridge-continuation workflow through real Query contracts instead of inheriting a denied default",
        ),
        (Surface::TopologyNeighborhoodReplacement, Concern::GroupedNeighborhoodWorkflow)
        | (Surface::TopologyNeighborhoodReplacement, Concern::LowerRuntimeRouting) => (
            Required,
            "replacement scope is part of the admitted neighborhood-bearing rebinding runtime story",
        ),
        (Surface::ToleranceAndPrecisionCertification, Concern::LowerRuntimeRouting) => (
            Required,
            "tolerance certification is an admitted declaration family with an explicit route contract",
        ),
        (Surface::ToleranceAndPrecisionCertification, Concern::RecoveryAction) => (
            Denied,
            "tolerance escalation or synthesis recovery remains a typed future lane rather than an admitted ordinary runtime family today",
        ),
        (Surface::HistoricalGeometryInspection, Concern::LowerRuntimeRouting)
        | (Surface::HistoricalGeometryInspection, Concern::HistoricalInspection)
        | (Surface::BranchLocalGeometryInspection, Concern::LowerRuntimeRouting)
        | (Surface::BranchLocalGeometryInspection, Concern::BranchLocalInspection)
        | (Surface::GeometryReplayParity, Concern::LowerRuntimeRouting)
        | (Surface::GeometryReplayParity, Concern::ReplayParity) => (
            Required,
            "retained-view families are admitted and explicit for this retained runtime responsibility",
        ),
        (Surface::GeometryRecoveryAction, Concern::LowerRuntimeRouting)
        | (Surface::GeometryRecoveryAction, Concern::RecoveryAction)
        | (Surface::GeometryRecoveryAction, Concern::MutationEvidence) => (
            Required,
            "recovery is a real admitted family and mutating recovery preserves canonical evidence rather than summary-only denial folklore",
        ),
        (Surface::GeometryRecoveryAction, Concern::ReplayParity) => (
            Denied,
            "recovery does not yet admit replay-bearing retained equivalence as an ordinary runtime responsibility",
        ),
        (Surface::GeometryProjectionConsumption, Concern::LowerRuntimeRouting)
        | (Surface::GeometryProjectionConsumption, Concern::ProjectionConsumption) => (
            Required,
            "projection consumption is admitted as the receipt-backed downstream fact delivery lane",
        ),
        (_, Concern::SignalContinuation) => (
            NA,
            "this surface does not currently own a signal-facing or continuation-bearing runtime responsibility",
        ),
        (_, Concern::GroupedNeighborhoodWorkflow)
        | (_, Concern::ContributionComposition)
        | (_, Concern::RecoveryAction)
        | (_, Concern::MutationEvidence)
        | (_, Concern::ProjectionConsumption)
        | (_, Concern::HistoricalInspection)
        | (_, Concern::BranchLocalInspection)
        | (_, Concern::ReplayParity)
        | (_, Concern::BooleanReadinessCertification)
        | (_, Concern::LowerRuntimeRouting) => (
            NA,
            "this runtime concern is not part of the admitted responsibility of this public geometry surface today",
        ),
    }
}

fn hash_parts(parts: &[String]) -> String {
    truth_digest_parts(TruthDigestScope::ArtifactIdentity, parts)
}
