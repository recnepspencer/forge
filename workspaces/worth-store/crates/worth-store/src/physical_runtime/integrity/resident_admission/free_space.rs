use worth_store_buffer_pool::PhysicalFrameLease;
use worth_store_physical_integrity::{
    validate_free_space_header, validate_free_space_membership_block,
    FreeSpaceHeaderIntegrityValidation, FreeSpaceMembershipBlockIntegrityValidation,
    IntegrityValidatedFreeSpaceHeader, IntegrityValidatedFreeSpaceMembershipBlock,
    PhysicalArtifactScope, UntrustedPhysicalArtifact,
};

use super::{
    denial::ResidentIntegrityAdmissionDenial, load::ResidentAdmissionContext,
    record_binding::ResidentIntegrityRecordBinding,
};

pub(in crate::physical_runtime) struct IntegrityAdmittedResidentFreeSpaceHeader<'frame> {
    source: ResidentIntegrityRecordBinding<'frame>,
}

pub(in crate::physical_runtime) struct IntegrityAdmittedResidentFreeSpaceMembershipBlock<'frame> {
    source: ResidentIntegrityRecordBinding<'frame>,
}

pub(in crate::physical_runtime) struct IntegrityAdmittedResidentFreeSpaceHeaderView<'frame> {
    lease: &'frame PhysicalFrameLease,
    scope: PhysicalArtifactScope,
}

pub(in crate::physical_runtime) struct IntegrityAdmittedResidentFreeSpaceMembershipView<'frame> {
    lease: &'frame PhysicalFrameLease,
    scope: PhysicalArtifactScope,
}

pub(in crate::physical_runtime) fn admit_resident_free_space_header<'frame>(
    lease: &'frame PhysicalFrameLease,
    scope: PhysicalArtifactScope,
    context: ResidentAdmissionContext<'_>,
) -> Result<IntegrityAdmittedResidentFreeSpaceHeader<'frame>, ResidentIntegrityAdmissionDenial> {
    if let Some(source) = context.reuse(lease, scope)? {
        return Ok(IntegrityAdmittedResidentFreeSpaceHeader { source });
    }
    let input = context.exact_input(lease, scope)?;
    context.observe_fresh_validation();
    match validate_free_space_header(input, scope).0 {
        FreeSpaceHeaderIntegrityValidation::Intact(validated) => {
            bind_free_space_header(lease, input, validated, context)
        }
        FreeSpaceHeaderIntegrityValidation::Rejected(rejection) => {
            context.validation_rejected(rejection)
        }
    }
}

pub(in crate::physical_runtime) fn admit_resident_free_space_membership_block<'frame>(
    lease: &'frame PhysicalFrameLease,
    scope: PhysicalArtifactScope,
    context: ResidentAdmissionContext<'_>,
) -> Result<
    IntegrityAdmittedResidentFreeSpaceMembershipBlock<'frame>,
    ResidentIntegrityAdmissionDenial,
> {
    if let Some(source) = context.reuse(lease, scope)? {
        return Ok(IntegrityAdmittedResidentFreeSpaceMembershipBlock { source });
    }
    let input = context.exact_input(lease, scope)?;
    context.observe_fresh_validation();
    match validate_free_space_membership_block(input, scope).0 {
        FreeSpaceMembershipBlockIntegrityValidation::Intact(validated) => {
            bind_free_space_membership(lease, input, validated, context)
        }
        FreeSpaceMembershipBlockIntegrityValidation::Rejected(rejection) => {
            context.validation_rejected(rejection)
        }
    }
}

fn bind_free_space_header<'frame>(
    lease: &'frame PhysicalFrameLease,
    input: UntrustedPhysicalArtifact<'frame>,
    validated: IntegrityValidatedFreeSpaceHeader<'frame>,
    context: ResidentAdmissionContext<'_>,
) -> Result<IntegrityAdmittedResidentFreeSpaceHeader<'frame>, ResidentIntegrityAdmissionDenial> {
    if !validated.matches_input(input) {
        return context.deny(ResidentIntegrityAdmissionDenial::SourceIncarnationMismatch);
    }
    let scope = validated.scope();
    let source = context.bind_validated(lease, scope, validated.into_validation_record())?;
    Ok(IntegrityAdmittedResidentFreeSpaceHeader { source })
}

fn bind_free_space_membership<'frame>(
    lease: &'frame PhysicalFrameLease,
    input: UntrustedPhysicalArtifact<'frame>,
    validated: IntegrityValidatedFreeSpaceMembershipBlock<'frame>,
    context: ResidentAdmissionContext<'_>,
) -> Result<
    IntegrityAdmittedResidentFreeSpaceMembershipBlock<'frame>,
    ResidentIntegrityAdmissionDenial,
> {
    if !validated.matches_input(input) {
        return context.deny(ResidentIntegrityAdmissionDenial::SourceIncarnationMismatch);
    }
    let scope = validated.scope();
    let source = context.bind_validated(lease, scope, validated.into_validation_record())?;
    Ok(IntegrityAdmittedResidentFreeSpaceMembershipBlock { source })
}

impl<'frame> IntegrityAdmittedResidentFreeSpaceHeader<'frame> {
    pub(in crate::physical_runtime) fn with_owner_decoder<T>(
        self,
        context: ResidentAdmissionContext<'_>,
        decoder: impl for<'view> FnOnce(IntegrityAdmittedResidentFreeSpaceHeaderView<'view>) -> T,
    ) -> Result<T, ResidentIntegrityAdmissionDenial> {
        context.with_owner_decoder(self.source, |lease, scope| {
            decoder(IntegrityAdmittedResidentFreeSpaceHeaderView { lease, scope })
        })
    }
}

impl<'frame> IntegrityAdmittedResidentFreeSpaceMembershipBlock<'frame> {
    pub(in crate::physical_runtime) fn with_owner_decoder<T>(
        self,
        context: ResidentAdmissionContext<'_>,
        decoder: impl for<'view> FnOnce(IntegrityAdmittedResidentFreeSpaceMembershipView<'view>) -> T,
    ) -> Result<T, ResidentIntegrityAdmissionDenial> {
        context.with_owner_decoder(self.source, |lease, scope| {
            decoder(IntegrityAdmittedResidentFreeSpaceMembershipView { lease, scope })
        })
    }
}

impl IntegrityAdmittedResidentFreeSpaceHeaderView<'_> {
    pub(in crate::physical_runtime) fn bytes(&self) -> &[u8] {
        self.lease
    }

    pub(in crate::physical_runtime) const fn scope(&self) -> PhysicalArtifactScope {
        self.scope
    }
}

impl IntegrityAdmittedResidentFreeSpaceMembershipView<'_> {
    pub(in crate::physical_runtime) fn bytes(&self) -> &[u8] {
        self.lease
    }

    pub(in crate::physical_runtime) const fn scope(&self) -> PhysicalArtifactScope {
        self.scope
    }
}
