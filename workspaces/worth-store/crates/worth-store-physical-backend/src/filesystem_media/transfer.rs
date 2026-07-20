#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MediaTransferPosition {
    PositionedOffset(u64),
    KnownAppendPosition(u64),
    UnknownAppendPosition,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CompletedMediaTransfer {
    bytes: u64,
    start: MediaTransferPosition,
}

impl CompletedMediaTransfer {
    pub const fn bytes(self) -> u64 {
        self.bytes
    }
    pub const fn start(self) -> MediaTransferPosition {
        self.start
    }

    pub(super) const fn new(bytes: u64, start: MediaTransferPosition) -> Self {
        Self { bytes, start }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PartialMediaTransfer {
    requested_bytes: u64,
    completed_bytes: u64,
    start: MediaTransferPosition,
    continuation_position: Option<u64>,
}

impl PartialMediaTransfer {
    pub(super) fn new(
        requested_bytes: u64,
        completed_bytes: u64,
        start: MediaTransferPosition,
        continuation_position: Option<u64>,
    ) -> Result<Self, MediaTransferShapeError> {
        if requested_bytes == 0 {
            return Err(MediaTransferShapeError::EmptyRequest);
        }
        if completed_bytes == 0 || completed_bytes >= requested_bytes {
            return Err(MediaTransferShapeError::NotAPartialTransfer);
        }
        if matches!(start, MediaTransferPosition::UnknownAppendPosition)
            && continuation_position.is_some()
        {
            return Err(MediaTransferShapeError::UnestablishedContinuation);
        }
        Ok(Self {
            requested_bytes,
            completed_bytes,
            start,
            continuation_position,
        })
    }

    pub const fn requested_bytes(self) -> u64 {
        self.requested_bytes
    }
    pub const fn completed_bytes(self) -> u64 {
        self.completed_bytes
    }
    pub const fn start(self) -> MediaTransferPosition {
        self.start
    }
    pub const fn continuation_position(self) -> Option<u64> {
        self.continuation_position
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MediaTransferShapeError {
    EmptyRequest,
    NotAPartialTransfer,
    UnestablishedContinuation,
    UnestablishedCompletedAppendPosition,
    ContinuationPositionOverflow,
    CompletedBeyondRequest,
}

/// Classifies one primitive transfer result without hiding short progress.
pub(super) fn classify_media_transfer(
    requested_bytes: u64,
    completed_bytes: u64,
    start: MediaTransferPosition,
) -> Result<MediaTransferProgress, MediaTransferShapeError> {
    if requested_bytes == 0 {
        return Err(MediaTransferShapeError::EmptyRequest);
    }
    if completed_bytes > requested_bytes {
        return Err(MediaTransferShapeError::CompletedBeyondRequest);
    }
    if completed_bytes == requested_bytes {
        validate_completed_position(start, completed_bytes)?;
        return Ok(MediaTransferProgress::Completed(
            CompletedMediaTransfer::new(completed_bytes, start),
        ));
    }
    if completed_bytes == 0 {
        return Ok(MediaTransferProgress::NoProgress);
    }
    let continuation_position = match start {
        MediaTransferPosition::PositionedOffset(offset)
        | MediaTransferPosition::KnownAppendPosition(offset) => Some(
            offset
                .checked_add(completed_bytes)
                .ok_or(MediaTransferShapeError::ContinuationPositionOverflow)?,
        ),
        MediaTransferPosition::UnknownAppendPosition => None,
    };
    PartialMediaTransfer::new(
        requested_bytes,
        completed_bytes,
        start,
        continuation_position,
    )
    .map(MediaTransferProgress::Partial)
}

fn validate_completed_position(
    start: MediaTransferPosition,
    completed_bytes: u64,
) -> Result<(), MediaTransferShapeError> {
    match start {
        MediaTransferPosition::PositionedOffset(offset)
        | MediaTransferPosition::KnownAppendPosition(offset) => offset
            .checked_add(completed_bytes)
            .map(|_| ())
            .ok_or(MediaTransferShapeError::ContinuationPositionOverflow),
        MediaTransferPosition::UnknownAppendPosition => {
            Err(MediaTransferShapeError::UnestablishedCompletedAppendPosition)
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MediaTransferProgress {
    NoProgress,
    Partial(PartialMediaTransfer),
    Completed(CompletedMediaTransfer),
}
