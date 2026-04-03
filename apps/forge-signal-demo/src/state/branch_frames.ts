import type { BranchId } from "../gear-scene/core/types";
import type { WorkerSnapshot } from "../gear-scene/worker/protocol";

export class BranchFrames {
  private frames = new Map<BranchId, ImageBitmap>();
  private reviewFrames = new Map<string, ImageBitmap>();

  update(
    snapshot: WorkerSnapshot,
    incomingFrames: Array<{ branchId: BranchId; bitmap: ImageBitmap }>,
    incomingReviewFrames: Array<{ id: string; bitmap: ImageBitmap }>,
  ) {
    const staleBitmaps: ImageBitmap[] = [];
    for (const frame of incomingFrames) {
      const previous = this.frames.get(frame.branchId);
      if (previous) staleBitmaps.push(previous);
      this.frames.set(frame.branchId, frame.bitmap);
    }
    for (const frame of incomingReviewFrames) {
      const previous = this.reviewFrames.get(frame.id);
      if (previous) staleBitmaps.push(previous);
      this.reviewFrames.set(frame.id, frame.bitmap);
    }

    const liveBranchIds = new Set(snapshot.branches.map((branch) => branch.id));
    for (const [branchId, bitmap] of this.frames.entries()) {
      if (!liveBranchIds.has(branchId)) {
        this.frames.delete(branchId);
        staleBitmaps.push(bitmap);
      }
    }

    const liveReviewFrameIds = new Set<string>();
    if (snapshot.mergeReview) {
      liveReviewFrameIds.add(snapshot.mergeReview.sourceFrameId);
      liveReviewFrameIds.add(snapshot.mergeReview.targetFrameId);
      liveReviewFrameIds.add(snapshot.mergeReview.mergedFrameId);
      for (const preview of snapshot.mergeReview.previews) {
        if (preview.frameId) {
          liveReviewFrameIds.add(preview.frameId);
        }
      }
    }
    for (const [frameId, bitmap] of this.reviewFrames.entries()) {
      if (!liveReviewFrameIds.has(frameId)) {
        this.reviewFrames.delete(frameId);
        staleBitmaps.push(bitmap);
      }
    }

    if (staleBitmaps.length > 0) {
      requestAnimationFrame(() => {
        for (const bitmap of staleBitmaps) {
          try {
            bitmap.close();
          } catch {
            // Ignore already-detached or already-closed bitmaps.
          }
        }
      });
    }
  }

  get(branchId: BranchId) {
    return this.frames.get(branchId) ?? null;
  }

  getReview(frameId: string) {
    return this.reviewFrames.get(frameId) ?? null;
  }
}
