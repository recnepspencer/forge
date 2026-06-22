use forge_proof::TransitionOutcome;
use forge_server::{
    ForgeServerBinaryDownload, ForgeServerBinaryDownloadOutcome,
    ForgeServerCompatibilityInspection, ForgeServerCompatibilityRead,
    ForgeServerCompatibilityUpload, ForgeServerCompatibilityUploadOutcome,
    ForgeServerQueryHandoffDenial,
};

pub(crate) fn compat_read_success(
    outcome: forge_server::ForgeServerCompatibilityExecutionOutcome<ForgeServerCompatibilityRead>,
) -> ForgeServerCompatibilityRead {
    match outcome {
        TransitionOutcome::Success(value) => value,
        other => panic!("expected compatibility read success, got {other:?}"),
    }
}

pub(crate) fn compat_inspection_success(
    outcome: forge_server::ForgeServerCompatibilityExecutionOutcome<
        ForgeServerCompatibilityInspection,
    >,
) -> ForgeServerCompatibilityInspection {
    match outcome {
        TransitionOutcome::Success(value) => value,
        other => panic!("expected compatibility inspection success, got {other:?}"),
    }
}

pub(crate) fn compat_upload_success(
    outcome: ForgeServerCompatibilityUploadOutcome<ForgeServerCompatibilityUpload>,
) -> ForgeServerCompatibilityUpload {
    match outcome {
        TransitionOutcome::Success(value) => value,
        other => panic!("expected compatibility upload success, got {other:?}"),
    }
}

pub(crate) fn compat_upload_denied(
    outcome: ForgeServerCompatibilityUploadOutcome<ForgeServerCompatibilityUpload>,
) -> ForgeServerQueryHandoffDenial {
    match outcome {
        TransitionOutcome::Denied(value) => value,
        other => panic!("expected compatibility upload denial, got {other:?}"),
    }
}

pub(crate) fn compat_download_success(
    outcome: ForgeServerBinaryDownloadOutcome<ForgeServerBinaryDownload>,
) -> ForgeServerBinaryDownload {
    match outcome {
        TransitionOutcome::Success(value) => value,
        other => panic!("expected binary download success, got {other:?}"),
    }
}

pub(crate) fn compat_download_denied(
    outcome: ForgeServerBinaryDownloadOutcome<ForgeServerBinaryDownload>,
) -> ForgeServerQueryHandoffDenial {
    match outcome {
        TransitionOutcome::Denied(value) => value,
        other => panic!("expected binary download denial, got {other:?}"),
    }
}
