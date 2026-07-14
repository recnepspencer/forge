import type { RuntimeTimelineBookmark } from "./demos/demoSixTypes";

interface HistoryTimelineGraphProps {
  bookmarks: readonly RuntimeTimelineBookmark[];
  activeSnapshotId: number | null;
  activeBranchId: number | null;
  onRestore: (bookmark: RuntimeTimelineBookmark) => void;
}

export function HistoryTimelineGraph({
  bookmarks,
  activeSnapshotId,
  activeBranchId,
  onRestore,
}: HistoryTimelineGraphProps) {
  const branches = [...new Map(bookmarks.map((bookmark) => [bookmark.branchId, bookmark.branchName])).entries()];
  const laneFor = (branchId: number) => Math.max(0, branches.findIndex(([id]) => id === branchId));
  const stepX = 118;
  const laneHeight = 76;
  const startX = 44;
  const startY = 34;
  const width = Math.max(320, bookmarks.length * stepX + 88);
  const height = Math.max(120, branches.length * laneHeight + 58);
  const byId = new Map(bookmarks.map((bookmark, index) => [bookmark.id, { bookmark, index }]));

  return (
    <div className="history-timeline" style={{ minWidth: width }}>
      <svg width={width} height={height} viewBox={`0 0 ${width} ${height}`} aria-hidden="true">
        {branches.map(([branchId]) => {
          const y = startY + laneFor(branchId) * laneHeight;
          return <line key={branchId} x1={0} y1={y} x2={width} y2={y} className="history-timeline-lane" />;
        })}
        {bookmarks.flatMap((bookmark, index) => {
          const x = startX + index * stepX;
          const y = startY + laneFor(bookmark.branchId) * laneHeight;
          return bookmark.parentIds.map((parentId) => {
            const parent = byId.get(parentId);
            if (!parent) return null;
            const px = startX + parent.index * stepX;
            const py = startY + laneFor(parent.bookmark.branchId) * laneHeight;
            const midX = (px + x) * 0.5;
            return (
              <path
                key={`${bookmark.id}:${parentId}:edge`}
                d={`M ${px} ${py} C ${midX} ${py}, ${midX} ${y}, ${x} ${y}`}
                className={bookmark.branchId === parent.bookmark.branchId ? "history-timeline-edge" : "history-timeline-edge branch"}
              />
            );
          });
        })}
      </svg>
      <div className="history-timeline-nodes">
        {bookmarks.map((bookmark, index) => {
          const x = startX + index * stepX;
          const y = startY + laneFor(bookmark.branchId) * laneHeight;
          const active = bookmark.branchId === activeBranchId && bookmark.snapshotId === activeSnapshotId;
          return (
            <button
              key={bookmark.id}
              type="button"
              className={`history-timeline-node ${active ? "active" : ""}`}
              style={{ left: x, top: y }}
              onClick={() => onRestore(bookmark)}
            >
              <span />
              <strong>{bookmark.label}</strong>
              <small>{bookmark.branchName}</small>
            </button>
          );
        })}
      </div>
    </div>
  );
}
