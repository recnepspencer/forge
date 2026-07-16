use std::path::PathBuf;

#[derive(Debug)]
pub enum PhysicalBackupMaterializationDenial {
    Io {
        path: PathBuf,
        source: std::io::Error,
    },
    InvalidSessionIdentity,
    InvalidBufferBudget,
    CounterOverflow,
    SourceLengthMismatch {
        path: PathBuf,
        expected: u64,
        actual: u64,
    },
    SourceIdentityMismatch {
        path: PathBuf,
    },
    SourceDigestMismatch {
        path: PathBuf,
    },
    SourceInsideSessionOutput {
        source: PathBuf,
        reserved_root: PathBuf,
    },
    OutputAliasesMaterializationFile {
        path: PathBuf,
    },
    UnexpectedStagingEntry {
        path: PathBuf,
    },
    IncompleteSources,
    EmptySourceSet,
    SourceCollectionAllocationFailed,
    DuplicateOutputName {
        output_name: String,
    },
    ReservedOutputName {
        output_name: String,
    },
    SymbolicLinkUnsupported {
        path: PathBuf,
    },
    ConflictingPublicationState {
        staging_root: PathBuf,
        final_root: PathBuf,
    },
    ExistingPublicationMismatch {
        path: PathBuf,
    },
    SessionBusy {
        session_identity: String,
    },
    SessionIdentityMismatch {
        path: PathBuf,
    },
    Cancelled,
}
