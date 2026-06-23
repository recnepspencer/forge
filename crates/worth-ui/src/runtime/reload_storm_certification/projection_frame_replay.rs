use crate::runtime::{
    WorthUiProjectionRebindBatchDigest, WorthUiProjectionRebindBatchReceipt,
    WorthUiReloadProjectionBreadthCertification,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorthUiProjectionFrameReplayDigest(u64);

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiProjectionFrameReplayCertification {
    projection_frame_replay_digest: WorthUiProjectionFrameReplayDigest,
    projection_frame_count: usize,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiProjectionFrameReplayDenial {
    OriginalBreadthCertificationMismatch,
    ReplayedBreadthCertificationMismatch,
    ProjectionFrameCountMismatch,
    ProjectionFrameDigestMismatch,
}

impl WorthUiProjectionFrameReplayCertification {
    pub fn certify(
        original_breadth: &WorthUiReloadProjectionBreadthCertification,
        original_batch: &WorthUiProjectionRebindBatchReceipt,
        replayed_breadth: &WorthUiReloadProjectionBreadthCertification,
        replayed_batch: &WorthUiProjectionRebindBatchReceipt,
    ) -> Result<Self, WorthUiProjectionFrameReplayDenial> {
        reject_uncertified_original_batch(original_breadth, original_batch)?;
        reject_uncertified_replayed_batch(replayed_breadth, replayed_batch)?;
        if original_batch.rows().len() != replayed_batch.rows().len() {
            return Err(WorthUiProjectionFrameReplayDenial::ProjectionFrameCountMismatch);
        }
        let original_digest = WorthUiProjectionFrameReplayDigest::from_batch(original_batch);
        let replayed_digest = WorthUiProjectionFrameReplayDigest::from_batch(replayed_batch);
        if original_digest != replayed_digest {
            return Err(WorthUiProjectionFrameReplayDenial::ProjectionFrameDigestMismatch);
        }
        Ok(Self {
            projection_frame_replay_digest: original_digest,
            projection_frame_count: original_batch.rows().len(),
        })
    }

    pub fn projection_frame_replay_digest(&self) -> WorthUiProjectionFrameReplayDigest {
        self.projection_frame_replay_digest
    }

    pub fn projection_frame_count(&self) -> usize {
        self.projection_frame_count
    }
}

impl WorthUiProjectionFrameReplayDigest {
    fn from_batch(batch: &WorthUiProjectionRebindBatchReceipt) -> Self {
        let rows = batch.rows().iter().map(|row| {
            format!(
                "row:{}|{:?}|{:?}|{}|{}",
                row.projection_identity().as_str(),
                row.projection_family(),
                row.status(),
                row.previous_frame_digest(),
                row.rebound_frame_digest()
            )
        });
        Self(super::digest::fold_texts(rows))
    }

    pub fn raw(self) -> u64 {
        self.0
    }
}

fn reject_uncertified_original_batch(
    breadth: &WorthUiReloadProjectionBreadthCertification,
    batch: &WorthUiProjectionRebindBatchReceipt,
) -> Result<(), WorthUiProjectionFrameReplayDenial> {
    if breadth.projection_rebind_batch_digest()
        != WorthUiProjectionRebindBatchDigest::from_batch(batch)
    {
        return Err(WorthUiProjectionFrameReplayDenial::OriginalBreadthCertificationMismatch);
    }
    Ok(())
}

fn reject_uncertified_replayed_batch(
    breadth: &WorthUiReloadProjectionBreadthCertification,
    batch: &WorthUiProjectionRebindBatchReceipt,
) -> Result<(), WorthUiProjectionFrameReplayDenial> {
    if breadth.projection_rebind_batch_digest()
        != WorthUiProjectionRebindBatchDigest::from_batch(batch)
    {
        return Err(WorthUiProjectionFrameReplayDenial::ReplayedBreadthCertificationMismatch);
    }
    Ok(())
}
