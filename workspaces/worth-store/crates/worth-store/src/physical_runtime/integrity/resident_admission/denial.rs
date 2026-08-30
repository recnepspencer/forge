use worth_store_buffer_pool::CleanFrameIntegrityValidationDenial;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::physical_runtime) enum ResidentIntegrityAdmissionDenial {
    SourceScopeMismatch,
    SourceIncarnationMismatch,
    Frame(CleanFrameIntegrityValidationDenial),
}
