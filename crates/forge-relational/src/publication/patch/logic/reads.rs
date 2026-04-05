use crate::logic::runtime::RelationalRuntime;
use crate::publication::patch::data::{
    PatchStreamBatch, PatchStreamReadError, PatchStreamReadErrorClass, PatchStreamRequest,
};

pub(crate) fn read_patch_stream(
    runtime: &RelationalRuntime,
    request: PatchStreamRequest,
) -> Result<PatchStreamBatch, PatchStreamReadError> {
    if request.max_commits == 0 {
        return Err(PatchStreamReadError {
            class: PatchStreamReadErrorClass::InvalidBatchSize,
            detail: "patch stream request must ask for at least one commit".to_string(),
        });
    }

    let latest_position = runtime.history().latest_patch_stream_position();
    let latest_commit_id = runtime
        .publication
        .latest_bundle
        .as_ref()
        .map(|bundle| bundle.commit.commit_id)
        .or_else(|| {
            runtime
                .history()
                .latest_commit()
                .map(|commit| commit.commit_id)
        });

    if let Some(after_position) = request.after_position {
        if !runtime
            .history()
            .contains_patch_stream_position(after_position)
        {
            return Err(PatchStreamReadError {
                class: PatchStreamReadErrorClass::UnknownResumePosition,
                detail: format!("unknown patch stream resume position {}", after_position.0),
            });
        }
    }

    let patches = runtime
        .history()
        .patches_after(request.after_position, request.max_commits);

    Ok(PatchStreamBatch {
        resumed_after: request.after_position,
        next_position: patches.last().map(|patch| patch.position),
        latest_position,
        latest_commit_id,
        patches,
    })
}
