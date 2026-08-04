use super::super::promotion::{LiveQueryFamily, LiveQueryPlan};
use super::super::refresh::{PatchWidthResolution, RefreshFallback};
use super::super::relevance::{
    BridgeChangeSummary, ChangeRelevance, IrrelevantChangeClass, QueryFieldKey, RelevantChangeClass,
};
use super::envelope::LivePatchDigest;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProjectionFieldDelta {
    pub(in crate::live) field: QueryFieldKey,
    pub(in crate::live) old_value: Option<String>,
    pub(in crate::live) new_value: Option<String>,
}

impl ProjectionFieldDelta {
    pub fn field(&self) -> &QueryFieldKey {
        &self.field
    }

    pub fn old_value(&self) -> Option<&str> {
        self.old_value.as_deref()
    }

    pub fn new_value(&self) -> Option<&str> {
        self.new_value.as_deref()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OrderingFieldDelta {
    pub(in crate::live) field: QueryFieldKey,
    pub(in crate::live) old_value: Option<String>,
    pub(in crate::live) new_value: Option<String>,
}

impl OrderingFieldDelta {
    pub fn field(&self) -> &QueryFieldKey {
        &self.field
    }

    pub fn old_value(&self) -> Option<&str> {
        self.old_value.as_deref()
    }

    pub fn new_value(&self) -> Option<&str> {
        self.new_value.as_deref()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DetailPatch {
    pub(in crate::live) digest: LivePatchDigest,
    pub(in crate::live) field_deltas: Vec<ProjectionFieldDelta>,
}

impl DetailPatch {
    pub fn digest(&self) -> &LivePatchDigest {
        &self.digest
    }

    pub fn field_deltas(&self) -> &[ProjectionFieldDelta] {
        &self.field_deltas
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DetailLiveOutcome {
    Patch(DetailPatch),
    Suppressed(SuppressionReason),
    Refresh(RefreshFallback),
}

impl DetailLiveOutcome {
    pub fn suppression_decision(&self) -> SuppressionDecision {
        match self {
            Self::Patch(_) => SuppressionDecision::Deliver,
            Self::Refresh(_) => SuppressionDecision::Deliver,
            Self::Suppressed(reason) => SuppressionDecision::Suppress(reason.clone()),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SuppressionReason {
    IrrelevantChange(IrrelevantChangeClass),
    OffRegionChange {
        scope_kind: super::super::LocalityScopeKind,
        scope: String,
        locality_digest: String,
    },
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SuppressionDecision {
    Deliver,
    Suppress(SuppressionReason),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum LiveDetailPatchError {
    UnsupportedFamily,
    UnsupportedRelevantClass(RelevantChangeClass),
    RelevantChangeWithoutProjectedDelta,
    WidthBudgetExceeded { limit: usize, actual: usize },
    CoalescingRequired,
}

enum DetailChangeClassification {
    Suppressed(IrrelevantChangeClass),
    ProjectedFields(Vec<ProjectionFieldDelta>),
}

impl LiveQueryPlan {
    pub fn detail_live_outcome(
        &self,
        change: &BridgeChangeSummary,
    ) -> Result<DetailLiveOutcome, LiveDetailPatchError> {
        match self.classify_detail_change(change)? {
            DetailChangeClassification::Suppressed(reason) => Ok(DetailLiveOutcome::Suppressed(
                SuppressionReason::IrrelevantChange(reason),
            )),
            DetailChangeClassification::ProjectedFields(field_deltas) => {
                self.assemble_detail_patch_outcome(field_deltas)
            }
        }
    }

    fn classify_detail_change(
        &self,
        change: &BridgeChangeSummary,
    ) -> Result<DetailChangeClassification, LiveDetailPatchError> {
        if self.descriptor.family() != &LiveQueryFamily::Detail {
            return Err(LiveDetailPatchError::UnsupportedFamily);
        }

        match self.classify_change(change) {
            ChangeRelevance::Irrelevant(reason) => {
                Ok(DetailChangeClassification::Suppressed(reason))
            }
            ChangeRelevance::Relevant(RelevantChangeClass::DetailProjectionChange) => {
                let field_deltas = self.projected_detail_field_deltas(change);
                if field_deltas.is_empty() {
                    return Err(LiveDetailPatchError::RelevantChangeWithoutProjectedDelta);
                }
                Ok(DetailChangeClassification::ProjectedFields(field_deltas))
            }
            ChangeRelevance::Relevant(other) => {
                Err(LiveDetailPatchError::UnsupportedRelevantClass(other))
            }
        }
    }

    fn projected_detail_field_deltas(
        &self,
        change: &BridgeChangeSummary,
    ) -> Vec<ProjectionFieldDelta> {
        change
            .field_deltas()
            .iter()
            .filter(|delta| {
                self.descriptor
                    .relevance_contract()
                    .projected_fields()
                    .iter()
                    .any(|field| field.matches(delta))
            })
            .map(|delta| ProjectionFieldDelta {
                field: delta.field_key().clone(),
                old_value: delta.old_value().map(ToOwned::to_owned),
                new_value: delta.new_value().map(ToOwned::to_owned),
            })
            .collect()
    }

    fn assemble_detail_patch_outcome(
        &self,
        field_deltas: Vec<ProjectionFieldDelta>,
    ) -> Result<DetailLiveOutcome, LiveDetailPatchError> {
        let width = self.assess_detail_patch_width(&field_deltas);
        match width.resolution() {
            PatchWidthResolution::Deliver => Ok(DetailLiveOutcome::Patch(
                self.construct_detail_patch(field_deltas),
            )),
            PatchWidthResolution::Reject => Err(LiveDetailPatchError::WidthBudgetExceeded {
                limit: width.budget_limit(),
                actual: width.measured_width(),
            }),
            PatchWidthResolution::Coalesce => Err(LiveDetailPatchError::CoalescingRequired),
            PatchWidthResolution::Refresh(fallback) => {
                Ok(DetailLiveOutcome::Refresh(fallback.clone()))
            }
        }
    }

    fn assess_detail_patch_width(
        &self,
        field_deltas: &[ProjectionFieldDelta],
    ) -> super::super::refresh::PatchWidthAssessment {
        self.evaluate_delivery_width(field_deltas.len())
    }

    fn construct_detail_patch(&self, field_deltas: Vec<ProjectionFieldDelta>) -> DetailPatch {
        let digest_parts = field_deltas.iter().map(|delta| {
            format!(
                "field_delta:{}:{:?}:{:?}",
                delta.field.terminal_digest_part(),
                delta.old_value,
                delta.new_value
            )
        });
        DetailPatch {
            digest: self.patch_digest(&digest_parts.collect::<Vec<_>>()),
            field_deltas,
        }
    }
}
