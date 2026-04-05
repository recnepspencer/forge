use crate::facade::publication::{PatchStreamPosition, PatchStreamRequest, RelationalPatchRecord};
use crate::facade::runtime::RelationalRuntime;

#[allow(dead_code)]
pub(crate) fn collect_patch_stream_from_head(
    runtime: &RelationalRuntime,
    window_size: usize,
) -> Vec<RelationalPatchRecord> {
    collect_patch_stream_after(runtime, None, window_size)
}

#[allow(dead_code)]
pub(crate) fn collect_patch_stream_after(
    runtime: &RelationalRuntime,
    after_position: Option<PatchStreamPosition>,
    window_size: usize,
) -> Vec<RelationalPatchRecord> {
    let mut collected = Vec::new();
    let mut after = after_position;

    loop {
        let batch = runtime
            .publication()
            .read_patch_stream(PatchStreamRequest {
                after_position: after,
                max_commits: window_size,
            })
            .unwrap();

        if batch.patches.is_empty() {
            break;
        }

        after = batch.patches.last().map(|patch| patch.position);
        collected.extend(batch.patches);
    }

    collected
}
