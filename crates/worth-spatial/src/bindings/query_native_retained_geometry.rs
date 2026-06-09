use forge_query::facade::{
    ForgeQueryDeclarationAspectContract, ForgeQueryDeclarationEnvelopeChecked,
    ForgeQueryDeclarationFamilyMarker, ForgeQueryDeclarationLegalityContract,
    ForgeQueryDeclarationRouteContract, ForgeQueryRelationalTruthAuthority,
    ForgeQuerySignalNotCompatiblePosture, ForgeQuerySingleOnlyGrouping,
};
use worth_primitives::{truth_digest_parts, TruthDigestScope};

use crate::bindings::authority::SpatialBindingKind;
use crate::bindings::query_native_rebinding::PrimitiveRebindingQueryDomain;
use crate::bindings::query_native_rebinding_authoring::PrimitiveRebindingDeclarationEntry;
use crate::bindings::rebinding::PrimitiveRebindingRetainedFactSource;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PrimitiveRebindingRetainedSubject {
    binding_kind: SpatialBindingKind,
    declaration_digest: String,
    progression_digest: Option<String>,
    route_plan_digest: Option<String>,
    receipt_digest: Option<String>,
    envelope_digest: String,
}

impl PrimitiveRebindingRetainedSubject {
    pub fn binding_kind(&self) -> SpatialBindingKind {
        self.binding_kind
    }

    pub fn declaration_digest(&self) -> &str {
        &self.declaration_digest
    }

    pub fn progression_digest(&self) -> Option<&str> {
        self.progression_digest.as_deref()
    }

    pub fn route_plan_digest(&self) -> Option<&str> {
        self.route_plan_digest.as_deref()
    }

    pub fn receipt_digest(&self) -> Option<&str> {
        self.receipt_digest.as_deref()
    }

    pub fn envelope_digest(&self) -> &str {
        &self.envelope_digest
    }
}

pub fn primitive_rebinding_retained_subject(
    binding_kind: SpatialBindingKind,
    checked: &ForgeQueryDeclarationEnvelopeChecked<
        PrimitiveRebindingQueryDomain,
        PrimitiveRebindingDeclarationEntry,
    >,
) -> PrimitiveRebindingRetainedSubject {
    PrimitiveRebindingRetainedSubject {
        binding_kind,
        declaration_digest: checked_declaration_digest(checked).to_string(),
        progression_digest: checked_progression_digest(checked).map(ToOwned::to_owned),
        route_plan_digest: checked_route_plan_digest(checked).map(ToOwned::to_owned),
        receipt_digest: checked_receipt_digest(checked),
        envelope_digest: checked_envelope_digest(checked).to_string(),
    }
}

pub(crate) fn retained_source_digest(source: &PrimitiveRebindingRetainedFactSource) -> String {
    let receipt = source.receipt();
    truth_digest_parts(
        TruthDigestScope::ArtifactIdentity,
        &[
            format!("binding_kind:{:?}", source.binding_kind()),
            format!("outcome:{:?}", receipt.outcome_class()),
            format!("continuity:{:?}", receipt.continuity_class()),
            format!("motion:{:?}", receipt.motion_posture()),
            format!("family:{:?}", receipt.neighborhood_family()),
            format!("prior:{}", receipt.prior_binding_identity()),
            format!("prior_site:{}", receipt.prior_site_identity()),
            format!(
                "selected_identity:{}",
                receipt.selected_candidate_identity().unwrap_or("none")
            ),
            format!(
                "selected_label:{}",
                receipt.selected_candidate_label().unwrap_or("none")
            ),
            format!("unsupported:{:?}", receipt.unsupported_reason()),
        ],
    )
}

pub(crate) fn subject_kind_label(subject: &PrimitiveRebindingRetainedSubject) -> &'static str {
    match subject.binding_kind() {
        SpatialBindingKind::FaceSurface => "face_surface",
        SpatialBindingKind::EdgeCurve => "edge_curve",
        SpatialBindingKind::CoedgePCurve => "coedge_pcurve",
        SpatialBindingKind::VertexGeometry => "vertex_geometry",
    }
}

pub(crate) fn checked_declaration_digest(
    checked: &ForgeQueryDeclarationEnvelopeChecked<
        PrimitiveRebindingQueryDomain,
        PrimitiveRebindingDeclarationEntry,
    >,
) -> &str {
    match checked {
        ForgeQueryDeclarationEnvelopeChecked::Enveloped(envelope) => envelope.declaration_digest(),
        ForgeQueryDeclarationEnvelopeChecked::Deferred(envelope) => {
            envelope.envelope().declaration_digest()
        }
        ForgeQueryDeclarationEnvelopeChecked::Denied(envelope) => {
            envelope.envelope().declaration_digest()
        }
        ForgeQueryDeclarationEnvelopeChecked::Failed(envelope) => {
            envelope.envelope().declaration_digest()
        }
    }
}

pub(crate) fn checked_progression_digest(
    checked: &ForgeQueryDeclarationEnvelopeChecked<
        PrimitiveRebindingQueryDomain,
        PrimitiveRebindingDeclarationEntry,
    >,
) -> Option<&str> {
    match checked {
        ForgeQueryDeclarationEnvelopeChecked::Enveloped(envelope) => envelope.progression_digest(),
        ForgeQueryDeclarationEnvelopeChecked::Deferred(envelope) => {
            envelope.envelope().progression_digest()
        }
        ForgeQueryDeclarationEnvelopeChecked::Denied(envelope) => {
            envelope.envelope().progression_digest()
        }
        ForgeQueryDeclarationEnvelopeChecked::Failed(envelope) => {
            envelope.envelope().progression_digest()
        }
    }
}

pub(crate) fn checked_route_plan_digest(
    checked: &ForgeQueryDeclarationEnvelopeChecked<
        PrimitiveRebindingQueryDomain,
        PrimitiveRebindingDeclarationEntry,
    >,
) -> Option<&str> {
    match checked {
        ForgeQueryDeclarationEnvelopeChecked::Enveloped(envelope) => envelope.route_plan_digest(),
        ForgeQueryDeclarationEnvelopeChecked::Deferred(envelope) => {
            envelope.envelope().route_plan_digest()
        }
        ForgeQueryDeclarationEnvelopeChecked::Denied(envelope) => {
            envelope.envelope().route_plan_digest()
        }
        ForgeQueryDeclarationEnvelopeChecked::Failed(envelope) => {
            envelope.envelope().route_plan_digest()
        }
    }
}

pub(crate) fn checked_receipt_digest(
    checked: &ForgeQueryDeclarationEnvelopeChecked<
        PrimitiveRebindingQueryDomain,
        PrimitiveRebindingDeclarationEntry,
    >,
) -> Option<String> {
    match checked {
        ForgeQueryDeclarationEnvelopeChecked::Enveloped(envelope) => {
            Some(format!("{:?}", envelope.receipt_digest()))
        }
        ForgeQueryDeclarationEnvelopeChecked::Deferred(envelope) => {
            Some(format!("{:?}", envelope.envelope().receipt_digest()))
        }
        ForgeQueryDeclarationEnvelopeChecked::Denied(envelope) => {
            Some(format!("{:?}", envelope.envelope().receipt_digest()))
        }
        ForgeQueryDeclarationEnvelopeChecked::Failed(envelope) => {
            Some(format!("{:?}", envelope.envelope().receipt_digest()))
        }
    }
}

pub(crate) fn checked_envelope_digest(
    checked: &ForgeQueryDeclarationEnvelopeChecked<
        PrimitiveRebindingQueryDomain,
        PrimitiveRebindingDeclarationEntry,
    >,
) -> String {
    match checked {
        ForgeQueryDeclarationEnvelopeChecked::Enveloped(envelope) => {
            format!("{:?}", envelope.envelope_digest())
        }
        ForgeQueryDeclarationEnvelopeChecked::Deferred(envelope) => {
            format!("{:?}", envelope.envelope().envelope_digest())
        }
        ForgeQueryDeclarationEnvelopeChecked::Denied(envelope) => {
            format!("{:?}", envelope.envelope().envelope_digest())
        }
        ForgeQueryDeclarationEnvelopeChecked::Failed(envelope) => {
            format!("{:?}", envelope.envelope().envelope_digest())
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct HistoricalGeometryInspectionDeclarationFamily;

impl ForgeQueryDeclarationFamilyMarker<PrimitiveRebindingQueryDomain>
    for HistoricalGeometryInspectionDeclarationFamily
{
    type PrimaryAuthority = ForgeQueryRelationalTruthAuthority;
    type SignalCompatibility = ForgeQuerySignalNotCompatiblePosture;
    type GroupedPosture = ForgeQuerySingleOnlyGrouping;

    fn semantic_family_key() -> &'static str {
        "HistoricalGeometryInspection"
    }

    fn aspect_contract() -> ForgeQueryDeclarationAspectContract {
        ForgeQueryDeclarationAspectContract::from_slices(
            &["geometry.retained.source", "geometry.retained.subject"],
            &["geometry.retained.historical_inspection"],
            &[],
            &[],
            &[],
        )
    }

    fn legality_contract() -> ForgeQueryDeclarationLegalityContract {
        ForgeQueryDeclarationLegalityContract::authoritative_hot_artifact()
    }

    fn route_contract() -> ForgeQueryDeclarationRouteContract {
        ForgeQueryDeclarationRouteContract::relational_only()
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct BranchLocalGeometryInspectionDeclarationFamily;

impl ForgeQueryDeclarationFamilyMarker<PrimitiveRebindingQueryDomain>
    for BranchLocalGeometryInspectionDeclarationFamily
{
    type PrimaryAuthority = ForgeQueryRelationalTruthAuthority;
    type SignalCompatibility = ForgeQuerySignalNotCompatiblePosture;
    type GroupedPosture = ForgeQuerySingleOnlyGrouping;

    fn semantic_family_key() -> &'static str {
        "BranchLocalGeometryInspection"
    }

    fn aspect_contract() -> ForgeQueryDeclarationAspectContract {
        ForgeQueryDeclarationAspectContract::from_slices(
            &[
                "geometry.retained.source",
                "geometry.retained.subject",
                "geometry.retained.branch_basis",
            ],
            &["geometry.retained.branch_local_inspection"],
            &[],
            &[],
            &[],
        )
    }

    fn legality_contract() -> ForgeQueryDeclarationLegalityContract {
        ForgeQueryDeclarationLegalityContract::authoritative_hot_artifact()
    }

    fn route_contract() -> ForgeQueryDeclarationRouteContract {
        ForgeQueryDeclarationRouteContract::relational_only()
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct GeometryReplayParityDeclarationFamily;

impl ForgeQueryDeclarationFamilyMarker<PrimitiveRebindingQueryDomain>
    for GeometryReplayParityDeclarationFamily
{
    type PrimaryAuthority = ForgeQueryRelationalTruthAuthority;
    type SignalCompatibility = ForgeQuerySignalNotCompatiblePosture;
    type GroupedPosture = ForgeQuerySingleOnlyGrouping;

    fn semantic_family_key() -> &'static str {
        "GeometryReplayParity"
    }

    fn aspect_contract() -> ForgeQueryDeclarationAspectContract {
        ForgeQueryDeclarationAspectContract::from_slices(
            &["geometry.retained.left", "geometry.retained.right"],
            &["geometry.retained.replay_parity"],
            &[],
            &[],
            &[],
        )
    }

    fn legality_contract() -> ForgeQueryDeclarationLegalityContract {
        ForgeQueryDeclarationLegalityContract::authoritative_hot_artifact()
    }

    fn route_contract() -> ForgeQueryDeclarationRouteContract {
        ForgeQueryDeclarationRouteContract::relational_only()
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct GeometryRecoveryActionDeclarationFamily;

impl ForgeQueryDeclarationFamilyMarker<PrimitiveRebindingQueryDomain>
    for GeometryRecoveryActionDeclarationFamily
{
    type PrimaryAuthority = ForgeQueryRelationalTruthAuthority;
    type SignalCompatibility = ForgeQuerySignalNotCompatiblePosture;
    type GroupedPosture = ForgeQuerySingleOnlyGrouping;

    fn semantic_family_key() -> &'static str {
        "GeometryRecoveryAction"
    }

    fn aspect_contract() -> ForgeQueryDeclarationAspectContract {
        ForgeQueryDeclarationAspectContract::from_slices(
            &["geometry.recovery.source"],
            &["geometry.recovery.action"],
            &[],
            &[],
            &[],
        )
    }

    fn legality_contract() -> ForgeQueryDeclarationLegalityContract {
        ForgeQueryDeclarationLegalityContract::authoritative_hot_artifact()
    }

    fn route_contract() -> ForgeQueryDeclarationRouteContract {
        ForgeQueryDeclarationRouteContract::relational_only()
    }
}
