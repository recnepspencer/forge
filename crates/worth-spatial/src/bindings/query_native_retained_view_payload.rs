use forge_query::facade::{
    ForgeQueryDeclarationEntryInspection, ForgeQueryOrdinaryNextStep,
    ForgeQueryOrdinaryPostureKind, ScopedInspectionBasis,
};
use worth_primitives::{truth_digest_parts, TruthDigestScope};

use crate::bindings::authority::SpatialBindingKind;
use crate::bindings::query_native_geometry_replay_parity_artifact::RebindingOrdinaryOutcomeShape;
use crate::bindings::query_native_rebinding::PrimitiveRebindingQueryDomain;
use crate::bindings::query_native_rebinding_authoring::PrimitiveRebindingDeclarationEntry;
use crate::bindings::rebinding::{
    BindingContinuityClass, MotionAwareBindingPosture, NeighborhoodBindingFamily,
    PrimitiveRebindingRetainedFactSource, RebindingOutcomeClass, UnsupportedRebindingReason,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PrimitiveRebindingRetainedViewPayload {
    binding_kind: SpatialBindingKind,
    outcome_class: RebindingOutcomeClass,
    continuity_class: BindingContinuityClass,
    motion_posture: MotionAwareBindingPosture,
    neighborhood_family: NeighborhoodBindingFamily,
    prior_binding_identity: String,
    prior_site_identity: String,
    selected_candidate_identity: Option<String>,
    selected_candidate_label: Option<String>,
    candidate_identities: Vec<String>,
    candidate_labels: Vec<String>,
    candidate_site_identities: Vec<String>,
    unsupported_reason: Option<UnsupportedRebindingReason>,
}

impl PrimitiveRebindingRetainedViewPayload {
    pub(crate) fn from_retained_source(source: &PrimitiveRebindingRetainedFactSource) -> Self {
        let receipt = source.receipt();
        Self {
            binding_kind: source.binding_kind(),
            outcome_class: receipt.outcome_class(),
            continuity_class: receipt.continuity_class(),
            motion_posture: receipt.motion_posture(),
            neighborhood_family: receipt.neighborhood_family(),
            prior_binding_identity: receipt.prior_binding_identity().to_string(),
            prior_site_identity: receipt.prior_site_identity().to_string(),
            selected_candidate_identity: receipt.selected_candidate_identity().map(str::to_string),
            selected_candidate_label: receipt.selected_candidate_label().map(str::to_string),
            candidate_identities: receipt.candidate_identities().to_vec(),
            candidate_labels: receipt.candidate_labels().to_vec(),
            candidate_site_identities: receipt.candidate_site_identities().to_vec(),
            unsupported_reason: receipt.unsupported_reason(),
        }
    }

    pub fn binding_kind(&self) -> SpatialBindingKind {
        self.binding_kind
    }

    pub fn outcome_class(&self) -> RebindingOutcomeClass {
        self.outcome_class
    }

    pub fn continuity_class(&self) -> BindingContinuityClass {
        self.continuity_class
    }

    pub fn motion_posture(&self) -> MotionAwareBindingPosture {
        self.motion_posture.clone()
    }

    pub fn neighborhood_family(&self) -> NeighborhoodBindingFamily {
        self.neighborhood_family
    }

    pub fn prior_binding_identity(&self) -> &str {
        &self.prior_binding_identity
    }

    pub fn prior_site_identity(&self) -> &str {
        &self.prior_site_identity
    }

    pub fn selected_candidate_identity(&self) -> Option<&str> {
        self.selected_candidate_identity.as_deref()
    }

    pub fn selected_candidate_label(&self) -> Option<&str> {
        self.selected_candidate_label.as_deref()
    }

    pub fn candidate_identities(&self) -> &[String] {
        &self.candidate_identities
    }

    pub fn candidate_labels(&self) -> &[String] {
        &self.candidate_labels
    }

    pub fn candidate_site_identities(&self) -> &[String] {
        &self.candidate_site_identities
    }

    pub fn unsupported_reason(&self) -> Option<UnsupportedRebindingReason> {
        self.unsupported_reason
    }

    pub(crate) fn ordinary_shape(&self) -> RebindingOrdinaryOutcomeShape {
        match self.outcome_class {
            RebindingOutcomeClass::Preserved
            | RebindingOutcomeClass::ExactReattachment
            | RebindingOutcomeClass::ContinuityJustifiedReattachment
            | RebindingOutcomeClass::CorrespondenceOnly => {
                RebindingOrdinaryOutcomeShape::new("bound", None, None)
            }
            RebindingOutcomeClass::Ambiguous => RebindingOrdinaryOutcomeShape::new(
                "ambiguous",
                Some(ForgeQueryOrdinaryPostureKind::Ambiguous),
                Some(ForgeQueryOrdinaryNextStep::NarrowInput),
            ),
            RebindingOutcomeClass::Orphaned => RebindingOrdinaryOutcomeShape::new(
                "rebind_required",
                Some(ForgeQueryOrdinaryPostureKind::RebindRequired),
                Some(ForgeQueryOrdinaryNextStep::RebindContext),
            ),
            RebindingOutcomeClass::Unsupported => RebindingOrdinaryOutcomeShape::new(
                "unsupported",
                Some(ForgeQueryOrdinaryPostureKind::Unsupported),
                Some(ForgeQueryOrdinaryNextStep::CheckSupport),
            ),
        }
    }

    pub(crate) fn historical_digest(
        &self,
        inspection: &ForgeQueryDeclarationEntryInspection<
            PrimitiveRebindingQueryDomain,
            PrimitiveRebindingDeclarationEntry,
        >,
    ) -> String {
        truth_digest_parts(
            TruthDigestScope::ArtifactIdentity,
            &[
                format!("declaration:{}", inspection.declaration_digest()),
                format!(
                    "progression:{}",
                    inspection.progression_digest().unwrap_or("none")
                ),
                format!("receipt:{}", inspection.receipt_digest().unwrap_or("none")),
                format!("envelope:{}", inspection.envelope_digest()),
                format!("outcome:{:?}", self.outcome_class()),
                format!("continuity:{:?}", self.continuity_class()),
                format!("motion:{:?}", self.motion_posture()),
                format!("family:{:?}", self.neighborhood_family()),
                format!("prior:{}", self.prior_binding_identity()),
                format!("prior_site:{}", self.prior_site_identity()),
                format!(
                    "selected_identity:{}",
                    self.selected_candidate_identity().unwrap_or("none")
                ),
                format!(
                    "selected_label:{}",
                    self.selected_candidate_label().unwrap_or("none")
                ),
                format!("unsupported:{:?}", self.unsupported_reason()),
            ],
        )
    }

    pub(crate) fn branch_local_digest(
        &self,
        inspection: &ForgeQueryDeclarationEntryInspection<
            PrimitiveRebindingQueryDomain,
            PrimitiveRebindingDeclarationEntry,
        >,
        scoped_basis: &ScopedInspectionBasis,
    ) -> String {
        truth_digest_parts(
            TruthDigestScope::ArtifactIdentity,
            &[
                format!("declaration:{}", inspection.declaration_digest()),
                format!(
                    "progression:{}",
                    inspection.progression_digest().unwrap_or("none")
                ),
                format!("receipt:{}", inspection.receipt_digest().unwrap_or("none")),
                format!("envelope:{}", inspection.envelope_digest()),
                format!("branch_family:{}", scoped_basis.family().as_str()),
                format!("branch_basis:{}", scoped_basis.scoped_basis_digest()),
                format!("outcome:{:?}", self.outcome_class()),
                format!("continuity:{:?}", self.continuity_class()),
                format!("motion:{:?}", self.motion_posture()),
                format!("family:{:?}", self.neighborhood_family()),
                format!("prior:{}", self.prior_binding_identity()),
                format!("prior_site:{}", self.prior_site_identity()),
                format!(
                    "selected_identity:{}",
                    self.selected_candidate_identity().unwrap_or("none")
                ),
                format!(
                    "selected_label:{}",
                    self.selected_candidate_label().unwrap_or("none")
                ),
                format!("unsupported:{:?}", self.unsupported_reason()),
            ],
        )
    }

    pub(crate) fn replay_source_fact_digest(&self, source_kind: &str) -> String {
        truth_digest_parts(
            TruthDigestScope::ArtifactIdentity,
            &[
                format!("source_kind:{source_kind}"),
                format!("binding_kind:{:?}", self.binding_kind()),
                format!("outcome:{:?}", self.outcome_class()),
                format!("continuity:{:?}", self.continuity_class()),
                format!("motion:{:?}", self.motion_posture()),
                format!("family:{:?}", self.neighborhood_family()),
                format!("prior:{}", self.prior_binding_identity()),
                format!("prior_site:{}", self.prior_site_identity()),
                format!(
                    "selected_identity:{}",
                    self.selected_candidate_identity().unwrap_or("none")
                ),
                format!(
                    "selected_label:{}",
                    self.selected_candidate_label().unwrap_or("none")
                ),
                format!("unsupported:{:?}", self.unsupported_reason()),
            ],
        )
    }
}
