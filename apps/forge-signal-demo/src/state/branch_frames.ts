import type { BranchId } from "../gear-scene/core/types";
import type { WorkerSnapshot } from "../gear-scene/worker/protocol";

export class BranchFrames {
  private frames = new Map<BranchId, ImageBitmap>();

  update(snapshot: WorkerSnapshot, incomingFrames: Array<{ branchId: BranchId; bitmap: ImageBitmap }>) {
    const staleBitmaps: ImageBitmap[] = [];
    for (const frame of incomingFrames) {
      const previous = this.frames.get(frame.branchId);
      if (previous) staleBitmaps.push(previous);
      this.frames.set(frame.branchId, frame.bitmap);
    }

    const liveBranchIds = new Set(snapshot.branches.map((branch) => branch.id));
    for (const [branchId, bitmap] of this.frames.entries()) {
      if (!liveBranchIds.has(branchId)) {
        this.frames.delete(branchId);
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
}
