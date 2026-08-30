use worth_store_buffer_pool::{PhysicalFrameLease, PhysicalResidentFrameGeneration};
use worth_store_physical_format::RecordArtifactFile;
use worth_store_physical_integrity::{
    validate_inline_page, InlinePageIntegrityValidation, IntegrityValidatedPageFrame,
    PhysicalArtifactScope,
};

use super::{
    denial::ResidentIntegrityAdmissionDenial, load::ResidentAdmissionContext,
    record_binding::ResidentIntegrityRecordBinding, source_scope::require_exact_page_source,
};

pub(in crate::physical_runtime) struct IntegrityAdmittedResidentPage<'frame> {
    source: ResidentIntegrityRecordBinding<'frame>,
}

pub(in crate::physical_runtime) struct IntegrityAdmittedResidentPageView<'frame> {
    lease: &'frame PhysicalFrameLease,
    scope: PhysicalArtifactScope,
}

pub(in crate::physical_runtime) fn admit_resident_page<'frame>(
    lease: &'frame PhysicalFrameLease,
    scope: PhysicalArtifactScope,
    expected_segment: RecordArtifactFile,
    context: ResidentAdmissionContext<'_>,
) -> Result<IntegrityAdmittedResidentPage<'frame>, ResidentIntegrityAdmissionDenial> {
    require_exact_page_source(lease, scope, expected_segment)
        .map_err(|denial| context.reject_source(denial))?;
    if let Some(source) = context.reuse(lease, scope)? {
        return Ok(IntegrityAdmittedResidentPage { source });
    }
    let input = context.exact_input(lease, scope)?;
    context.observe_fresh_validation();
    match validate_inline_page(input, scope).0 {
        InlinePageIntegrityValidation::Intact(validated) => {
            bind_validated_page(lease, input, validated, context)
        }
        InlinePageIntegrityValidation::Rejected(rejection) => {
            context.validation_rejected(rejection)
        }
    }
}

fn bind_validated_page<'frame>(
    lease: &'frame PhysicalFrameLease,
    input: worth_store_physical_integrity::UntrustedPhysicalArtifact<'frame>,
    validated: IntegrityValidatedPageFrame<'frame>,
    context: ResidentAdmissionContext<'_>,
) -> Result<IntegrityAdmittedResidentPage<'frame>, ResidentIntegrityAdmissionDenial> {
    if !validated.matches_input(input) {
        return context.deny(ResidentIntegrityAdmissionDenial::SourceIncarnationMismatch);
    }
    let scope = validated.scope();
    let source = context.bind_validated(lease, scope, validated.into_validation_record())?;
    Ok(IntegrityAdmittedResidentPage { source })
}

impl<'frame> IntegrityAdmittedResidentPage<'frame> {
    pub(in crate::physical_runtime) fn with_owner_decoder<T>(
        self,
        context: ResidentAdmissionContext<'_>,
        decoder: impl for<'view> FnOnce(IntegrityAdmittedResidentPageView<'view>) -> T,
    ) -> Result<T, ResidentIntegrityAdmissionDenial> {
        context.with_owner_decoder(self.source, |lease, scope| {
            decoder(IntegrityAdmittedResidentPageView { lease, scope })
        })
    }
}

impl IntegrityAdmittedResidentPageView<'_> {
    pub(in crate::physical_runtime) fn bytes(&self) -> &[u8] {
        self.lease
    }

    pub(in crate::physical_runtime) const fn scope(&self) -> PhysicalArtifactScope {
        self.scope
    }

    pub(in crate::physical_runtime) const fn frame_generation(
        &self,
    ) -> PhysicalResidentFrameGeneration {
        self.lease.resident_generation()
    }
}
