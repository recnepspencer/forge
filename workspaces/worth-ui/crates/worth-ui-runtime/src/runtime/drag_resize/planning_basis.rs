#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiResizeAllocationPlanningBasis {
    target: crate::graph::UiGraphNodeIdentity,
    durable_identity_digest: u64,
    axis_scope: crate::evidence::UiConstraintAxisScope,
    extent: super::UiResizeLogicalExtent,
    identity_digest: u64,
}

impl UiResizeAllocationPlanningBasis {
    pub(crate) fn seal(
        selection: &crate::graph::UiAdmittedReplanNeighborhoodSet,
        target: crate::graph::UiGraphNodeIdentity,
        durable_identity_digest: Option<u64>,
        extent: super::UiResizeLogicalExtent,
    ) -> Option<Self> {
        let measurement_basis = selection
            .primary()
            .allocation_candidate()
            .measurement_basis();
        let admitted_identity =
            durable_identity_digest.or_else(|| measurement_basis.durable_resize_inputs().next())?;
        let support = measurement_basis.durable_resize_support(admitted_identity)?;
        let axis_scope = support.axis_scope();
        let identity_digest =
            crate::declaration::stable_text_digest("worth-ui.resize-allocation-planning-basis")
                ^ target.digest().rotate_left(7)
                ^ admitted_identity.rotate_left(13)
                ^ u64::from(extent.subpixels()).rotate_left(19)
                ^ (axis_scope as u64).rotate_left(23);
        Some(Self {
            target,
            durable_identity_digest: admitted_identity,
            axis_scope,
            extent,
            identity_digest,
        })
    }
    pub fn target(&self) -> crate::graph::UiGraphNodeIdentity {
        self.target
    }
    pub fn durable_identity_digest(&self) -> u64 {
        self.durable_identity_digest
    }
    pub fn axis_scope(&self) -> crate::evidence::UiConstraintAxisScope {
        self.axis_scope
    }
    pub fn extent(&self) -> super::UiResizeLogicalExtent {
        self.extent
    }
    pub fn identity_digest(&self) -> u64 {
        self.identity_digest
    }
}
