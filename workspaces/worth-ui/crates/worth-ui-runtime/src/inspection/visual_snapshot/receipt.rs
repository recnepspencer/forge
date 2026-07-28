use std::marker::PhantomData;

pub struct UiVisualSnapshotReceipt<ArtifactPosture: worth_ui_inspection::UiVisualArtifactPolicy> {
    session: crate::lifecycle::WorthUiActiveApplicationSessionIdentity,
    identity: super::UiVisualSnapshotIdentity,
    parent_snapshot: Option<super::UiVisualSnapshotIdentity>,
    captured_client_extent: worth_ui_inspection::UiClientPhysicalRect,
    presentation: super::UiVisualSurfaceCaptureBasis,
    host_coordinate_transform: worth_ui_host_contract::UiHostCoordinateTransform,
    pixel_artifact: Option<worth_ui_inspection::UiVisualPixelArtifact>,
    evidence: worth_ui_inspection::UiVisualSnapshotEvidence,
    visible_index: super::UiVisibleRegionIndex,
    hit_test_index: super::UiHitTestRegionIndex,
    retained_identity_trace_basis: crate::mounting::UiMountedIdentityTraceBasis,
    _snapshot_lease: crate::mounting::UiMountedVisualSnapshotLease,
    resource_lease: super::UiVisualSnapshotResourceLease,
    _artifact_posture: PhantomData<ArtifactPosture>,
}

pub(crate) struct UiVisualSnapshotSealInput {
    pub(crate) session: crate::lifecycle::WorthUiActiveApplicationSessionIdentity,
    pub(crate) identity: super::UiVisualSnapshotIdentity,
    pub(crate) parent_snapshot: Option<super::UiVisualSnapshotIdentity>,
    pub(crate) captured_client_extent: worth_ui_inspection::UiClientPhysicalRect,
    pub(crate) presentation: super::UiVisualSurfaceCaptureBasis,
    pub(crate) affinity: worth_ui_inspection::UiVisualSnapshotAffinity,
    pub(crate) coordinates: worth_ui_inspection::UiVisualCoordinateObservation,
    pub(crate) host_coordinate_transform: worth_ui_host_contract::UiHostCoordinateTransform,
    pub(crate) pixel_artifact: Option<worth_ui_inspection::UiVisualPixelArtifact>,
    pub(crate) disclosure: worth_ui_inspection::UiVisualInspectionDisclosure,
    pub(crate) cost: worth_ui_inspection::UiVisualInspectionCostReceipt,
    pub(crate) query_budget: worth_ui_inspection::UiVisualQueryBudget,
    pub(crate) visible_index: super::UiVisibleRegionIndex,
    pub(crate) hit_test_index: super::UiHitTestRegionIndex,
    pub(crate) identity_trace_basis: crate::mounting::UiMountedIdentityTraceBasis,
    pub(crate) snapshot_lease: crate::mounting::UiMountedVisualSnapshotLease,
    pub(crate) resource_lease: super::UiVisualSnapshotResourceLease,
}

pub(crate) struct UiRetainedVisualSnapshotSource {
    pub(crate) session: crate::lifecycle::WorthUiActiveApplicationSessionIdentity,
    pub(crate) identity: super::UiVisualSnapshotIdentity,
    pub(crate) captured_client_extent: worth_ui_inspection::UiClientPhysicalRect,
    pub(crate) presentation: super::UiVisualSurfaceCaptureBasis,
    pub(crate) host_coordinate_transform: worth_ui_host_contract::UiHostCoordinateTransform,
    pub(crate) pixel_artifact: Option<worth_ui_inspection::UiVisualPixelArtifact>,
    pub(crate) evidence: worth_ui_inspection::UiVisualSnapshotEvidence,
    pub(crate) visible_index: super::UiVisibleRegionIndex,
    pub(crate) hit_test_index: super::UiHitTestRegionIndex,
    pub(crate) identity_trace_basis: crate::mounting::UiMountedIdentityTraceBasis,
    pub(crate) snapshot_lease: crate::mounting::UiMountedVisualSnapshotLease,
    pub(crate) resource_lease: super::UiVisualSnapshotResourceLease,
}

pub struct UiVisualCoordinateScope<'snapshot> {
    snapshot: super::UiVisualSnapshotIdentity,
    captured_client_extent: worth_ui_inspection::UiClientPhysicalRect,
    visible_index: &'snapshot super::UiVisibleRegionIndex,
    hit_test_index: &'snapshot super::UiHitTestRegionIndex,
    trace_basis: &'snapshot crate::mounting::UiMountedIdentityTraceBasis,
    query_budget: worth_ui_inspection::UiVisualQueryBudget,
    _invariant: PhantomData<&'snapshot mut &'snapshot ()>,
}

pub struct UiSnapshotClientPixel<'snapshot> {
    point: worth_ui_inspection::UiClientPhysicalPixel,
    _invariant: PhantomData<&'snapshot mut &'snapshot ()>,
}

pub struct UiSnapshotClientRegion<'snapshot> {
    region: worth_ui_inspection::UiClientPhysicalRect,
    _invariant: PhantomData<&'snapshot mut &'snapshot ()>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiVisualSnapshotDisposalReceipt {
    identity: super::UiVisualSnapshotIdentity,
    released_registered_resource: bool,
}

impl<Posture: worth_ui_inspection::UiVisualArtifactPolicy> UiVisualSnapshotReceipt<Posture> {
    pub(crate) fn seal(input: UiVisualSnapshotSealInput) -> Self {
        let artifact = match (
            Posture::PIXELS_REQUESTED,
            Posture::PIXELS_REQUIRED,
            input.pixel_artifact.is_some(),
        ) {
            (false, false, false) => {
                worth_ui_inspection::UiVisualSnapshotArtifactPosture::GeometryOnly
            }
            (true, false, false) => {
                worth_ui_inspection::UiVisualSnapshotArtifactPosture::PixelsOptionalOmitted
            }
            (true, false, true) => {
                worth_ui_inspection::UiVisualSnapshotArtifactPosture::PixelsOptionalCaptured
            }
            (true, true, true) => {
                worth_ui_inspection::UiVisualSnapshotArtifactPosture::PixelsRequiredCaptured
            }
            _ => unreachable!("sealed artifact posture and captured bytes must agree"),
        };
        let evidence = worth_ui_inspection::UiVisualSnapshotEvidence::from_runtime_projection(
            worth_ui_inspection::UiVisualSnapshotEvidenceInput {
                affinity: input.affinity,
                coordinates: input.coordinates,
                visible_index: input.visible_index.identity(),
                hit_test_index: input.hit_test_index.identity(),
                artifact,
                disclosure: input.disclosure,
                query_budget: input.query_budget,
                cost: input.cost,
            },
        );
        Self {
            session: input.session,
            identity: input.identity,
            parent_snapshot: input.parent_snapshot,
            captured_client_extent: input.captured_client_extent,
            presentation: input.presentation,
            host_coordinate_transform: input.host_coordinate_transform,
            pixel_artifact: input.pixel_artifact,
            evidence,
            visible_index: input.visible_index,
            hit_test_index: input.hit_test_index,
            retained_identity_trace_basis: input.identity_trace_basis,
            _snapshot_lease: input.snapshot_lease,
            resource_lease: input.resource_lease,
            _artifact_posture: PhantomData,
        }
    }

    pub const fn identity(&self) -> super::UiVisualSnapshotIdentity {
        self.identity
    }

    pub const fn parent_snapshot(&self) -> Option<super::UiVisualSnapshotIdentity> {
        self.parent_snapshot
    }

    pub const fn captured_client_extent(&self) -> worth_ui_inspection::UiClientPhysicalRect {
        self.captured_client_extent
    }

    pub const fn affinity(&self) -> worth_ui_inspection::UiVisualSnapshotAffinity {
        self.evidence.affinity()
    }

    pub const fn coordinates(&self) -> worth_ui_inspection::UiVisualCoordinateObservation {
        self.evidence.coordinates()
    }

    pub const fn cost(&self) -> worth_ui_inspection::UiVisualInspectionCostReceipt {
        self.evidence.cost()
    }

    pub const fn evidence(&self) -> worth_ui_inspection::UiVisualSnapshotEvidence {
        self.evidence
    }

    pub fn overlay_target(
        &self,
        target: &worth_ui_inspection::UiVisualHitTestTarget,
    ) -> Result<super::UiVisualOverlayTarget, worth_ui_inspection::UiVisualOverlayDenial> {
        let trace = target.identity_trace();
        let mounted = trace.mounted_node();
        let record = self
            .hit_test_index
            .target_record(mounted.node_receipt(), target.total_order())
            .ok_or(worth_ui_inspection::UiVisualOverlayDenial::ForeignSnapshotTarget)?;
        match self
            ._snapshot_lease
            .relation()
            .map_err(super::overlay::map_overlay_retention_denial)?
        {
            worth_ui_inspection::UiVisualSnapshotRelation::Current => {}
            worth_ui_inspection::UiVisualSnapshotRelation::RetainedPredecessor
            | worth_ui_inspection::UiVisualSnapshotRelation::Historical => {
                return Err(worth_ui_inspection::UiVisualOverlayDenial::Superseded);
            }
        }
        let lease = self
            ._snapshot_lease
            .derive_overlay()
            .map_err(super::overlay::map_overlay_retention_denial)?;
        Ok(super::seal_overlay_target(
            super::UiVisualOverlayTargetInput {
                session: self.session,
                base_snapshot: self.identity,
                presentation: self.presentation,
                target_receipt: record.node_receipt(),
                target_region: record.inspection_region(),
                host_coordinate_transform: self.host_coordinate_transform,
                trace: trace.clone(),
                lease,
            },
        ))
    }

    pub fn into_client_region_target(
        self,
        select: impl for<'scope> FnOnce(
            UiVisualCoordinateScope<'scope>,
        ) -> UiSnapshotClientRegion<'scope>,
    ) -> Result<super::UiClientRegionVisualTarget, worth_ui_inspection::UiVisualSnapshotDenial>
    {
        let region = select(UiVisualCoordinateScope {
            snapshot: self.identity,
            captured_client_extent: self.captured_client_extent,
            visible_index: &self.visible_index,
            hit_test_index: &self.hit_test_index,
            trace_basis: &self.retained_identity_trace_basis,
            query_budget: self.evidence.query_budget(),
            _invariant: PhantomData,
        })
        .region;
        let extent = self.captured_client_extent;
        if region.left() < extent.left()
            || region.top() < extent.top()
            || region.right() > extent.right()
            || region.bottom() > extent.bottom()
        {
            return Err(worth_ui_inspection::UiVisualSnapshotDenial::OutsideCapturedPixelExtent);
        }
        Ok(super::seal_region_target(
            self.into_retained_source(),
            region,
        ))
    }

    pub const fn visible_region_index_identity(
        &self,
    ) -> worth_ui_inspection::UiVisibleRegionIndexIdentity {
        self.visible_index.identity()
    }

    pub const fn hit_test_region_index_identity(
        &self,
    ) -> worth_ui_inspection::UiHitTestRegionIndexIdentity {
        self.hit_test_index.identity()
    }

    pub fn visible_region_count(&self) -> usize {
        self.visible_index.len()
    }

    pub fn hit_test_region_count(&self) -> usize {
        self.hit_test_index.len()
    }

    pub fn with_coordinate_scope<Result>(
        &self,
        use_scope: impl for<'scope> FnOnce(UiVisualCoordinateScope<'scope>) -> Result,
    ) -> Result {
        use_scope(UiVisualCoordinateScope {
            snapshot: self.identity,
            captured_client_extent: self.captured_client_extent,
            visible_index: &self.visible_index,
            hit_test_index: &self.hit_test_index,
            trace_basis: &self.retained_identity_trace_basis,
            query_budget: self.evidence.query_budget(),
            _invariant: PhantomData,
        })
    }

    pub(crate) fn dispose(self) -> UiVisualSnapshotDisposalReceipt {
        UiVisualSnapshotDisposalReceipt {
            identity: self.identity,
            released_registered_resource: self.resource_lease.dispose(),
        }
    }

    fn into_retained_source(self) -> UiRetainedVisualSnapshotSource {
        UiRetainedVisualSnapshotSource {
            session: self.session,
            identity: self.identity,
            captured_client_extent: self.captured_client_extent,
            presentation: self.presentation,
            host_coordinate_transform: self.host_coordinate_transform,
            pixel_artifact: self.pixel_artifact,
            evidence: self.evidence,
            visible_index: self.visible_index,
            hit_test_index: self.hit_test_index,
            identity_trace_basis: self.retained_identity_trace_basis,
            snapshot_lease: self._snapshot_lease,
            resource_lease: self.resource_lease,
        }
    }
}

impl UiRetainedVisualSnapshotSource {
    pub(crate) fn replace_registered_resource(
        mut self,
        identity: super::UiVisualSnapshotIdentity,
        usage: super::UiVisualRetainedResourceUsage,
    ) -> (Self, worth_ui_inspection::UiVisualPixelArtifactValidity) {
        self.resource_lease = self
            .resource_lease
            .replace(identity.diagnostic_value(), usage);
        let validity = self.resource_lease.pixel_validity();
        (self, validity)
    }
}

impl UiVisualSnapshotDisposalReceipt {
    pub const fn identity(self) -> super::UiVisualSnapshotIdentity {
        self.identity
    }

    pub const fn released_registered_resource(self) -> bool {
        self.released_registered_resource
    }
}

impl UiVisualSnapshotReceipt<worth_ui_inspection::UiPixelsRequired> {
    pub fn pixel_artifact(&self) -> &worth_ui_inspection::UiVisualPixelArtifact {
        self.pixel_artifact
            .as_ref()
            .expect("pixels-required receipts are sealed only with an artifact")
    }
}

impl UiVisualSnapshotReceipt<worth_ui_inspection::UiPixelsOptional> {
    pub const fn optional_pixel_artifact(
        &self,
    ) -> Option<&worth_ui_inspection::UiVisualPixelArtifact> {
        self.pixel_artifact.as_ref()
    }
}

impl<'snapshot> UiVisualCoordinateScope<'snapshot> {
    pub const fn snapshot(&self) -> super::UiVisualSnapshotIdentity {
        self.snapshot
    }

    pub fn client_pixel(
        &self,
        point: worth_ui_inspection::UiClientPhysicalPixel,
    ) -> Result<UiSnapshotClientPixel<'snapshot>, worth_ui_inspection::UiVisualSnapshotDenial> {
        if !self.captured_client_extent.contains(point) {
            return Err(worth_ui_inspection::UiVisualSnapshotDenial::OutsideCapturedPixelExtent);
        }
        Ok(UiSnapshotClientPixel {
            point,
            _invariant: PhantomData,
        })
    }

    pub const fn client_region(
        &self,
        region: worth_ui_inspection::UiClientPhysicalRect,
    ) -> UiSnapshotClientRegion<'snapshot> {
        UiSnapshotClientRegion {
            region,
            _invariant: PhantomData,
        }
    }

    pub fn adjudicate_point(
        &self,
        point: UiSnapshotClientPixel<'snapshot>,
    ) -> Result<
        worth_ui_inspection::UiVisualPointAdjudication,
        worth_ui_inspection::UiVisualSnapshotOmission,
    > {
        Ok(super::adjudicate_point(super::UiPointAdjudicationInput {
            point: point.point,
            visible_index: self.visible_index,
            hit_test_index: self.hit_test_index,
            trace_basis: self.trace_basis,
            budget: self.query_budget,
        }))
    }

    pub fn adjudicate_region(
        &self,
        region: UiSnapshotClientRegion<'snapshot>,
    ) -> worth_ui_inspection::UiVisualRegionAdjudication {
        super::adjudicate_region(super::UiRegionAdjudicationInput {
            region: region.region,
            visible_index: self.visible_index,
            trace_basis: self.trace_basis,
            budget: self.query_budget,
        })
    }
}

impl UiSnapshotClientPixel<'_> {
    pub const fn point(&self) -> worth_ui_inspection::UiClientPhysicalPixel {
        self.point
    }
}

impl UiSnapshotClientRegion<'_> {
    pub const fn region(&self) -> worth_ui_inspection::UiClientPhysicalRect {
        self.region
    }
}
