use worth_store_buffer_pool::CleanFrameIntegrityValidationDenial;
use worth_store_physical_integrity::PhysicalIntegrityRejection;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::physical_runtime) enum ResidentIntegrityAdmissionDenial {
    SourceScopeMismatch,
    SourceIncarnationMismatch,
    LifecycleGenerationChanged,
    FrameGenerationChanged,
    RetainedRecordInvalidated,
    RetainedRecordChanged,
    Validation(PhysicalIntegrityRejection),
    Frame(CleanFrameIntegrityValidationDenial),
}
