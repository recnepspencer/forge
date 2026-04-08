use std::sync::Arc;

use sha2::{Digest, Sha256};

use crate::diagnostics::BridgeRouteRecord;
use crate::error::{BridgeContinuityError, BridgeContinuityErrorKind};
use crate::mapping::SubscriptionSliceKind;
use crate::routing::{
    BridgeRouteIdentity, BridgeSubscriptionSlice, BridgeSubscriptionSliceIdentity,
    FineGrainedMatchStatus,
};

use super::BridgeContinuityAuthorityBasis;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct PriorSubscriptionSlice {
    prior_subscription_slice_identity: BridgeSubscriptionSliceIdentity,
    entity_identity: Arc<str>,
    aspect_label: Arc<str>,
    surface_label: Arc<str>,
    slice_kind: SubscriptionSliceKind,
    match_status: FineGrainedMatchStatus,
}

impl PriorSubscriptionSlice {
    pub(crate) fn from_parts(
        prior_subscription_slice_identity: BridgeSubscriptionSliceIdentity,
        entity_identity: impl Into<Arc<str>>,
        aspect_label: impl Into<Arc<str>>,
        surface_label: impl Into<Arc<str>>,
        slice_kind: SubscriptionSliceKind,
        match_status: FineGrainedMatchStatus,
    ) -> Self {
        Self {
            prior_subscription_slice_identity,
            entity_identity: entity_identity.into(),
            aspect_label: aspect_label.into(),
            surface_label: surface_label.into(),
            slice_kind,
            match_status,
        }
    }

    pub(crate) fn new(
        prior_subscription_slice_identity: BridgeSubscriptionSliceIdentity,
        slice: &BridgeSubscriptionSlice,
    ) -> Self {
        Self::from_parts(
            prior_subscription_slice_identity,
            slice.entity_identity(),
            slice.aspect_label(),
            slice.surface_label(),
            slice.slice_kind().clone(),
            slice.match_status(),
        )
    }

    pub fn prior_subscription_slice_identity(&self) -> &BridgeSubscriptionSliceIdentity {
        &self.prior_subscription_slice_identity
    }

    pub fn entity_identity(&self) -> &str {
        self.entity_identity.as_ref()
    }

    pub fn aspect_label(&self) -> &str {
        self.aspect_label.as_ref()
    }

    pub fn surface_label(&self) -> &str {
        self.surface_label.as_ref()
    }

    pub fn slice_kind(&self) -> SubscriptionSliceKind {
        self.slice_kind.clone()
    }

    pub fn match_status(&self) -> FineGrainedMatchStatus {
        self.match_status
    }

    pub fn canonical_basis(&self) -> String {
        format!(
            "prior-slice|slice-set={}|entity={}|aspect={}|surface={}|kind={:?}|match={:?}",
            self.prior_subscription_slice_identity.as_str(),
            self.entity_identity(),
            self.aspect_label(),
            self.surface_label(),
            self.slice_kind(),
            self.match_status(),
        )
    }

    pub(crate) fn logical_dedup_basis(&self) -> String {
        format!(
            "prior-slice-logical|entity={}|aspect={}|surface={}|kind={:?}|match={:?}",
            self.entity_identity(),
            self.aspect_label(),
            self.surface_label(),
            self.slice_kind(),
            self.match_status(),
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgePlannedContinuityRequest {
    request_key: Arc<str>,
    prior_route_identity: BridgeRouteIdentity,
    prior_slice: PriorSubscriptionSlice,
}

impl BridgePlannedContinuityRequest {
    fn new(prior_route_identity: BridgeRouteIdentity, prior_slice: PriorSubscriptionSlice) -> Self {
        let canonical_basis = format!(
            "planned-continuity-request|route={}|{}",
            prior_route_identity.as_str(),
            prior_slice.canonical_basis()
        );
        let digest = Sha256::digest(canonical_basis.as_bytes());
        Self {
            request_key: Arc::from(format!("continuity-request:sha256:{digest:x}")),
            prior_route_identity,
            prior_slice,
        }
    }

    pub fn request_key(&self) -> &str {
        self.request_key.as_ref()
    }

    pub fn prior_route_identity(&self) -> &BridgeRouteIdentity {
        &self.prior_route_identity
    }

    pub fn prior_slice(&self) -> &PriorSubscriptionSlice {
        &self.prior_slice
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgePlannedContinuityRequestSet {
    prior_route_identity: BridgeRouteIdentity,
    authority_basis: BridgeContinuityAuthorityBasis,
    prior_subscription_slice_identity: BridgeSubscriptionSliceIdentity,
    prior_slice_count: usize,
    requests: Arc<[BridgePlannedContinuityRequest]>,
    digest: Arc<str>,
}

impl BridgePlannedContinuityRequestSet {
    pub(crate) fn from_route_record(
        route_record: &BridgeRouteRecord,
    ) -> Result<Self, BridgeContinuityError> {
        let lineage_context = route_record
            .mapping_context()
            .lineage_context()
            .ok_or_else(|| {
                BridgeContinuityError::new(
                    BridgeContinuityErrorKind::MissingLineageContext,
                    "Bridge continuity planning requires an explicit lineage context in the mapping context.",
                )
            })?;
        let authority_basis = lineage_context.authority_basis().clone();
        if authority_basis.snapshot_identity() != route_record.source_snapshot() {
            return Err(BridgeContinuityError::new(
                BridgeContinuityErrorKind::LineageAuthorityMismatch,
                format!(
                    "Bridge continuity lineage context was bound to snapshot `{}` but route record source snapshot was `{}`.",
                    authority_basis.snapshot_identity().as_str(),
                    route_record.source_snapshot().as_str(),
                ),
            ));
        }
        if authority_basis.branch_identity() != route_record.source_branch() {
            return Err(BridgeContinuityError::new(
                BridgeContinuityErrorKind::LineageAuthorityMismatch,
                format!(
                    "Bridge continuity lineage context was bound to branch `{}` but route record source branch was `{}`.",
                    authority_basis.branch_identity().as_str(),
                    route_record.source_branch().as_str(),
                ),
            ));
        }
        let prior_route_identity = route_record.route_identity().clone();
        let prior_subscription_slice_identity = route_record.subscription_slice_identity().clone();
        let prior_slice_count = route_record.subscription_slices().len();
        let mut prior_slices = route_record
            .subscription_slices()
            .iter()
            .map(|slice| {
                PriorSubscriptionSlice::new(prior_subscription_slice_identity.clone(), slice)
            })
            .collect::<Vec<_>>();
        prior_slices.sort_by(|left, right| {
            left.logical_dedup_basis()
                .cmp(&right.logical_dedup_basis())
                .then_with(|| {
                    left.prior_subscription_slice_identity()
                        .as_str()
                        .cmp(right.prior_subscription_slice_identity().as_str())
                })
        });
        prior_slices.dedup_by(|left, right| left.logical_dedup_basis() == right.logical_dedup_basis());
        let requests = prior_slices
            .into_iter()
            .map(|prior_slice| {
                BridgePlannedContinuityRequest::new(prior_route_identity.clone(), prior_slice)
            })
            .collect::<Vec<_>>();
        let mut requests = requests;
        requests.sort_by(|left, right| left.request_key().cmp(right.request_key()));
        let canonical_basis = format!(
            "planned-continuity-request-set|route={}|slice-set={}|authority={}|prior-slice-count={}|request-count={}|requests={}",
            prior_route_identity.as_str(),
            prior_subscription_slice_identity.as_str(),
            authority_basis.digest(),
            prior_slice_count,
            requests.len(),
            requests
                .iter()
                .map(|request| request.request_key())
                .collect::<Vec<_>>()
                .join(","),
        );
        let digest = Sha256::digest(canonical_basis.as_bytes());

        Ok(Self {
            prior_route_identity,
            authority_basis,
            prior_subscription_slice_identity,
            prior_slice_count,
            requests: Arc::from(requests),
            digest: Arc::from(format!("planned-continuity-set:sha256:{digest:x}")),
        })
    }

    pub fn prior_route_identity(&self) -> &BridgeRouteIdentity {
        &self.prior_route_identity
    }

    pub fn authority_basis(&self) -> &BridgeContinuityAuthorityBasis {
        &self.authority_basis
    }

    pub fn prior_subscription_slice_identity(&self) -> &BridgeSubscriptionSliceIdentity {
        &self.prior_subscription_slice_identity
    }

    pub fn prior_slice_count(&self) -> usize {
        self.prior_slice_count
    }

    pub fn requests(&self) -> &[BridgePlannedContinuityRequest] {
        &self.requests
    }

    pub fn digest(&self) -> &str {
        self.digest.as_ref()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeEligibleContinuityRequestSet {
    planned: BridgePlannedContinuityRequestSet,
}

impl BridgeEligibleContinuityRequestSet {
    pub(crate) fn from_planned(
        planned: BridgePlannedContinuityRequestSet,
    ) -> Result<Self, BridgeContinuityError> {
        let mut request_keys = planned
            .requests()
            .iter()
            .map(|request| request.request_key())
            .collect::<Vec<_>>();
        let mut sorted = request_keys.clone();
        sorted.sort_unstable();
        sorted.dedup();
        if sorted.len() != request_keys.len() {
            return Err(BridgeContinuityError::new(
                BridgeContinuityErrorKind::InvalidContinuityRequestSet,
                "Bridge continuity request-set contained duplicate request keys after canonical planning.",
            ));
        }
        request_keys.sort_unstable();
        if request_keys != sorted {
            return Err(BridgeContinuityError::new(
                BridgeContinuityErrorKind::InvalidContinuityRequestSet,
                "Bridge continuity request-set was not emitted in canonical request-key order.",
            ));
        }
        Ok(Self { planned })
    }

    pub fn prior_route_identity(&self) -> &BridgeRouteIdentity {
        self.planned.prior_route_identity()
    }

    pub fn authority_basis(&self) -> &BridgeContinuityAuthorityBasis {
        self.planned.authority_basis()
    }

    pub fn prior_subscription_slice_identity(&self) -> &BridgeSubscriptionSliceIdentity {
        self.planned.prior_subscription_slice_identity()
    }

    pub fn prior_slice_count(&self) -> usize {
        self.planned.prior_slice_count()
    }

    pub fn requests(&self) -> &[BridgePlannedContinuityRequest] {
        self.planned.requests()
    }

    pub fn digest(&self) -> &str {
        self.planned.digest()
    }
}
