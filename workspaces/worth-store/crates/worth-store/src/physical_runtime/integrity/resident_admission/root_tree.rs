use worth_store_buffer_pool::PhysicalFrameLease;
use worth_store_physical_integrity::{
    validate_root_routing_block, validate_segment_membership_block,
    IntegrityValidatedRootRoutingBlock, IntegrityValidatedSegmentMembershipBlock,
    PhysicalArtifactScope, RootRoutingBlockIntegrityValidation,
    SegmentMembershipBlockIntegrityValidation, UntrustedPhysicalArtifact,
};

use super::{
    denial::ResidentIntegrityAdmissionDenial, load::ResidentAdmissionContext,
    record_binding::ResidentIntegrityRecordBinding,
};

pub(in crate::physical_runtime) struct IntegrityAdmittedResidentRootRoutingBlock<'frame> {
    source: ResidentIntegrityRecordBinding<'frame>,
}

pub(in crate::physical_runtime) struct IntegrityAdmittedResidentSegmentMembershipBlock<'frame> {
    source: ResidentIntegrityRecordBinding<'frame>,
}

pub(in crate::physical_runtime) struct IntegrityAdmittedResidentRootRoutingView<'frame> {
    lease: &'frame PhysicalFrameLease,
    scope: PhysicalArtifactScope,
}

pub(in crate::physical_runtime) struct IntegrityAdmittedResidentSegmentMembershipView<'frame> {
    lease: &'frame PhysicalFrameLease,
    scope: PhysicalArtifactScope,
}

pub(in crate::physical_runtime) fn admit_resident_root_routing_block<'frame>(
    lease: &'frame PhysicalFrameLease,
    scope: PhysicalArtifactScope,
    context: ResidentAdmissionContext<'_>,
) -> Result<IntegrityAdmittedResidentRootRoutingBlock<'frame>, ResidentIntegrityAdmissionDenial> {
    if let Some(source) = context.reuse(lease, scope)? {
        return Ok(IntegrityAdmittedResidentRootRoutingBlock { source });
    }
    let input = context.exact_input(lease, scope)?;
    context.observe_fresh_validation();
    match validate_root_routing_block(input, scope).0 {
        RootRoutingBlockIntegrityValidation::Intact(validated) => {
            bind_root_routing(lease, input, validated, context)
        }
        RootRoutingBlockIntegrityValidation::Rejected(rejection) => {
            context.validation_rejected(rejection)
        }
    }
}

pub(in crate::physical_runtime) fn admit_resident_segment_membership_block<'frame>(
    lease: &'frame PhysicalFrameLease,
    scope: PhysicalArtifactScope,
    context: ResidentAdmissionContext<'_>,
) -> Result<IntegrityAdmittedResidentSegmentMembershipBlock<'frame>, ResidentIntegrityAdmissionDenial>
{
    if let Some(source) = context.reuse(lease, scope)? {
        return Ok(IntegrityAdmittedResidentSegmentMembershipBlock { source });
    }
    let input = context.exact_input(lease, scope)?;
    context.observe_fresh_validation();
    match validate_segment_membership_block(input, scope).0 {
        SegmentMembershipBlockIntegrityValidation::Intact(validated) => {
            bind_segment_membership(lease, input, validated, context)
        }
        SegmentMembershipBlockIntegrityValidation::Rejected(rejection) => {
            context.validation_rejected(rejection)
        }
    }
}

fn bind_root_routing<'frame>(
    lease: &'frame PhysicalFrameLease,
    input: UntrustedPhysicalArtifact<'frame>,
    validated: IntegrityValidatedRootRoutingBlock<'frame>,
    context: ResidentAdmissionContext<'_>,
) -> Result<IntegrityAdmittedResidentRootRoutingBlock<'frame>, ResidentIntegrityAdmissionDenial> {
    if !validated.matches_input(input) {
        return context.deny(ResidentIntegrityAdmissionDenial::SourceIncarnationMismatch);
    }
    let scope = validated.scope();
    let source = context.bind_validated(lease, scope, validated.into_validation_record())?;
    Ok(IntegrityAdmittedResidentRootRoutingBlock { source })
}

fn bind_segment_membership<'frame>(
    lease: &'frame PhysicalFrameLease,
    input: UntrustedPhysicalArtifact<'frame>,
    validated: IntegrityValidatedSegmentMembershipBlock<'frame>,
    context: ResidentAdmissionContext<'_>,
) -> Result<IntegrityAdmittedResidentSegmentMembershipBlock<'frame>, ResidentIntegrityAdmissionDenial>
{
    if !validated.matches_input(input) {
        return context.deny(ResidentIntegrityAdmissionDenial::SourceIncarnationMismatch);
    }
    let scope = validated.scope();
    let source = context.bind_validated(lease, scope, validated.into_validation_record())?;
    Ok(IntegrityAdmittedResidentSegmentMembershipBlock { source })
}

impl<'frame> IntegrityAdmittedResidentRootRoutingBlock<'frame> {
    pub(in crate::physical_runtime) fn with_owner_decoder<T>(
        self,
        context: ResidentAdmissionContext<'_>,
        decoder: impl for<'view> FnOnce(IntegrityAdmittedResidentRootRoutingView<'view>) -> T,
    ) -> Result<T, ResidentIntegrityAdmissionDenial> {
        context.with_owner_decoder(self.source, |lease, scope| {
            decoder(IntegrityAdmittedResidentRootRoutingView { lease, scope })
        })
    }
}

impl<'frame> IntegrityAdmittedResidentSegmentMembershipBlock<'frame> {
    pub(in crate::physical_runtime) fn with_owner_decoder<T>(
        self,
        context: ResidentAdmissionContext<'_>,
        decoder: impl for<'view> FnOnce(IntegrityAdmittedResidentSegmentMembershipView<'view>) -> T,
    ) -> Result<T, ResidentIntegrityAdmissionDenial> {
        context.with_owner_decoder(self.source, |lease, scope| {
            decoder(IntegrityAdmittedResidentSegmentMembershipView { lease, scope })
        })
    }
}

macro_rules! resident_tree_view {
    ($view:ty) => {
        impl $view {
            pub(in crate::physical_runtime) fn bytes(&self) -> &[u8] {
                self.lease
            }

            pub(in crate::physical_runtime) const fn scope(&self) -> PhysicalArtifactScope {
                self.scope
            }
        }
    };
}

resident_tree_view!(IntegrityAdmittedResidentRootRoutingView<'_>);
resident_tree_view!(IntegrityAdmittedResidentSegmentMembershipView<'_>);
