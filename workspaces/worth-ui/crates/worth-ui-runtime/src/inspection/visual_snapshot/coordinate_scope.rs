use std::marker::PhantomData;

pub struct UiVisualCoordinateScope<'snapshot> {
    snapshot: super::UiVisualSnapshotIdentity,
    captured_client_extent: worth_ui_inspection::UiClientPhysicalRect,
    visible_index: &'snapshot super::UiVisibleRegionIndex,
    hit_test_index: &'snapshot super::UiHitTestRegionIndex,
    trace_basis: &'snapshot crate::mounting::UiMountedIdentityTraceBasis,
    query_budget: worth_ui_inspection::UiVisualQueryBudget,
    _invariant: PhantomData<&'snapshot mut &'snapshot ()>,
}

pub(super) struct UiVisualCoordinateScopeInput<'snapshot> {
    pub(super) snapshot: super::UiVisualSnapshotIdentity,
    pub(super) captured_client_extent: worth_ui_inspection::UiClientPhysicalRect,
    pub(super) visible_index: &'snapshot super::UiVisibleRegionIndex,
    pub(super) hit_test_index: &'snapshot super::UiHitTestRegionIndex,
    pub(super) trace_basis: &'snapshot crate::mounting::UiMountedIdentityTraceBasis,
    pub(super) query_budget: worth_ui_inspection::UiVisualQueryBudget,
}

pub struct UiSnapshotClientPixel<'snapshot> {
    point: worth_ui_inspection::UiClientPhysicalPixel,
    _invariant: PhantomData<&'snapshot mut &'snapshot ()>,
}

pub struct UiSnapshotClientRegion<'snapshot> {
    region: worth_ui_inspection::UiClientPhysicalRect,
    _invariant: PhantomData<&'snapshot mut &'snapshot ()>,
}

impl<'snapshot> UiVisualCoordinateScope<'snapshot> {
    pub(super) const fn new(input: UiVisualCoordinateScopeInput<'snapshot>) -> Self {
        Self {
            snapshot: input.snapshot,
            captured_client_extent: input.captured_client_extent,
            visible_index: input.visible_index,
            hit_test_index: input.hit_test_index,
            trace_basis: input.trace_basis,
            query_budget: input.query_budget,
            _invariant: PhantomData,
        }
    }

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
