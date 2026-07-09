import { useCallback, useEffect, useRef, useState } from "react";
import { createPortal } from "react-dom";
import { createSignals } from "worth-signal-wasm";

import { DemoSixGear } from "./demos/DemoSixGear";
import { HistoryMergeModal } from "./HistoryMergeModal";
import { HistoryTimelineGraph } from "./HistoryTimelineGraph";
import {
  gearParamSpecs,
  initialGearParams,
  normalizeGearParams,
  type GearParamKey,
  type GearParams,
  type RuntimeTimelineBookmark,
} from "./demos/demoSixTypes";
import {
  createBookmark,
  formatNumber,
  numberId,
  readBranch,
  readBranches,
  readBranchFromRaw,
  readParams,
  type GearRuntime,
  type MergePlanView,
  type MergeResultProofView,
  type RuntimeBranch,
  type SignalsFacade,
} from "./historyRuntimeTypes";
import { useSignal } from "./Demos";
import "./historySection.css";

interface HistorySectionProps {
  onNavigate: (path: string) => void;
}

function HistoryWorkbench({ runtime }: { runtime: GearRuntime }) {
  const { signals, history, inputs } = runtime;
  const runtimeParams = normalizeGearParams({
    innerRadius: useSignal<number>(signals, inputs.innerRadius),
    outerRadius: useSignal<number>(signals, inputs.outerRadius),
    thickness: useSignal<number>(signals, inputs.thickness),
    teeth: useSignal<number>(signals, inputs.teeth),
  });
  const [branches, setBranches] = useState<RuntimeBranch[]>(readBranches(history));
  const [activeBranch, setActiveBranch] = useState<RuntimeBranch>(readBranch(history));
  const [bookmarks, setBookmarks] = useState<RuntimeTimelineBookmark[]>([
    createBookmark(history, "init", initialGearParams),
  ]);
  const [staged, setStaged] = useState<GearParams | null>(null);
  const [lastPlan, setLastPlan] = useState<MergePlanView | null>(null);
  const [lastMerge, setLastMerge] = useState<MergeResultProofView | null>(null);
  const [event, setEvent] = useState("Runtime branch graph is live.");
  const [drawerOpen, setDrawerOpen] = useState(false);
  const [mergeOpen, setMergeOpen] = useState(false);
  const debounceRef = useRef<number | null>(null);
  const bookmarksRef = useRef(bookmarks);
  const display = staged ?? runtimeParams;

  const refresh = useCallback(() => {
    setBranches(readBranches(history));
    setActiveBranch(readBranch(history));
  }, [history]);

  const activeBookmarkId = () => {
    const branch = readBranch(history);
    const snapshotId = numberId(history.branch_snapshot_id(branch.id));
    return bookmarksRef.current.find((bookmark) =>
      bookmark.branchId === branch.id && bookmark.snapshotId === snapshotId
    )?.id ?? null;
  };

  const branchHeadBookmarkId = (branchId: number) => {
    const branch = readBranches(history).find((entry) => entry.id === branchId);
    return bookmarksRef.current.find((bookmark) =>
      bookmark.branchId === branchId && bookmark.snapshotId === branch?.head_snapshot_id
    )?.id ?? null;
  };

  const remember = (label: string, params = readParams(inputs), parentIds: readonly string[] = []) => {
    const bookmark = createBookmark(history, label, params, parentIds);
    setBookmarks((current) => current.some((item) => item.branchId === bookmark.branchId && item.snapshotId === bookmark.snapshotId)
      ? current
      : (() => {
          const next = [...current, bookmark];
          bookmarksRef.current = next;
          return next;
        })());
    refresh();
  };

  useEffect(() => {
    if (!drawerOpen) return undefined;
    const previousOverflow = document.body.style.overflow;
    const previousPaddingRight = document.body.style.paddingRight;
    const scrollbarWidth = window.innerWidth - document.documentElement.clientWidth;
    document.body.style.overflow = "hidden";
    if (scrollbarWidth > 0) {
      document.body.style.paddingRight = `${scrollbarWidth}px`;
    }
    return () => {
      document.body.style.overflow = previousOverflow;
      document.body.style.paddingRight = previousPaddingRight;
    };
  }, [drawerOpen]);

  const commit = (params: GearParams, label: string) => {
    const next = normalizeGearParams(params);
    const parentId = activeBookmarkId();
    signals.batch((tx) => {
      gearParamSpecs.forEach((spec) => {
        if (runtimeParams[spec.key] !== next[spec.key]) tx.set(inputs[spec.key], next[spec.key]);
      });
    });
    setStaged(null);
    remember(label, next, parentId ? [parentId] : []);
    setEvent(`${label} committed on ${readBranch(history).name}.`);
  };

  const schedule = (key: GearParamKey, value: number) => {
    const next = normalizeGearParams({ ...display, [key]: value });
    setStaged(next);
    if (debounceRef.current != null) window.clearTimeout(debounceRef.current);
    debounceRef.current = window.setTimeout(() => {
      commit(next, `${key} ${formatNumber(next[key])}`);
      debounceRef.current = null;
    }, 300);
  };

  const createBranch = async () => {
    const parentId = activeBookmarkId();
    const parentBranch = readBranch(history);
    const branch = readBranchFromRaw(await history.create_branch(`branch-${branches.length}`));
    refresh();
    await history.switch_branch(branch.id);
    remember(`fork ${branch.name}`, readParams(inputs), parentId ? [parentId] : []);
    await history.switch_branch(parentBranch.id);
    refresh();
    setEvent(`${branch.name} created from ${parentBranch.name}; toggle its tab when you want to edit it.`);
  };

  const restore = async (bookmark: RuntimeTimelineBookmark) => {
    await history.switch_branch(bookmark.branchId);
    await history.restore_exact_branch_snapshot(bookmark.branchId, bookmark.snapshot);
    setStaged(null);
    refresh();
    setEvent(`Restored ${bookmark.branchName} to snapshot ${bookmark.snapshotId}.`);
  };

  const applyMergeResult = (
    sourceBranchId: number,
    targetBranchId: number,
    plan: MergePlanView,
    result: MergeResultProofView,
  ) => {
    const targetParentId = branchHeadBookmarkId(targetBranchId);
    const sourceParentId = branchHeadBookmarkId(sourceBranchId);
    setLastPlan(plan);
    setLastMerge(result);
    refresh();
    remember(`merge ${sourceBranchId}`, readParams(inputs), [targetParentId, sourceParentId].filter((id): id is string => Boolean(id)));
    setEvent(`Merged branch ${sourceBranchId} into branch ${targetBranchId}; result proof retained by history().`);
  };

  const proof = history.branch_state_proof(activeBranch.id);

  const controls = (closeButton: boolean) => (
    <>
      <p className="history-kicker">WORTH runtime state</p>
      <h3>{activeBranch.name}</h3>
      {gearParamSpecs.map((spec) => (
        <label key={spec.key}>
          <span>{spec.label}</span>
          <input
            type="range"
            min={spec.min}
            max={spec.max}
            step={spec.step}
            value={display[spec.key]}
            onChange={(change) => schedule(spec.key, Number(change.target.value))}
          />
          <strong>{formatNumber(display[spec.key])}</strong>
        </label>
      ))}
      {closeButton && (
        <button className="history-drawer-close" type="button" onClick={() => setDrawerOpen(false)}>
          Close controls
        </button>
      )}
    </>
  );

  return (
    <div className="history-workbench">
      <div className="history-stage-grid">
        <div className="history-gear-stack">
          <div className="history-tabs" aria-label="Runtime branches">
            {branches.map((branch) => (
              <button
                key={branch.id}
                type="button"
                className={branch.id === activeBranch.id ? "active" : ""}
                onClick={() => {
                  void Promise.resolve(history.switch_branch(branch.id)).then(refresh);
                }}
              >
                {branch.name}
              </button>
            ))}
            <button type="button" onClick={createBranch}>+ branch</button>
            <button type="button" onClick={() => setMergeOpen(true)}>Merge</button>
          </div>
          <DemoSixGear params={display} />
          <button className="history-drawer-toggle" type="button" onClick={() => setDrawerOpen(true)}>
            Tune gear parameters
          </button>
          <div className="history-timeline-shell history-timeline-shell-under-gear">
            <HistoryTimelineGraph
              bookmarks={bookmarks}
              activeBranchId={activeBranch.id}
              activeSnapshotId={activeBranch.head_snapshot_id}
              onRestore={(bookmark) => void restore(bookmark)}
            />
          </div>
        </div>
        <div className="history-controls" aria-label="Gear parameter controls">
          {controls(false)}
        </div>
      </div>
      {drawerOpen && createPortal(
        <div className="history-modal-layer" role="presentation">
          <button
            className="history-modal-backdrop"
            type="button"
            aria-label="Close gear controls"
            onClick={() => setDrawerOpen(false)}
          />
          <div className="history-modal-controls" role="dialog" aria-modal="true" aria-label="Gear parameter controls">
            {controls(true)}
          </div>
        </div>,
        document.body,
      )}
      <HistoryMergeModal
        open={mergeOpen}
        signals={signals}
        history={history}
        branches={branches}
        activeBranch={activeBranch}
        bookmarks={bookmarks}
        onClose={() => setMergeOpen(false)}
        onMerged={applyMergeResult}
        onUnavailable={setEvent}
      />

      <div className="history-proof-row">
        <code>branch_state_proof: {proof.stateDigest.slice(0, 18)} / snapshot {proof.snapshotId ?? "none"}</code>
        <code>merge_plan: {lastPlan ? `${lastPlan.merge_kind}, nodes=${lastPlan.node_plan.length}` : "not run"}</code>
        <code>merge_result: {lastMerge ? `${lastMerge.result?.merge_kind ?? lastMerge.merge_kind}, proof=${lastMerge.proof?.resultDigest?.slice(0, 18) ?? "retained"}` : "not run"}</code>
      </div>

      <div className="history-event">{event}</div>
    </div>
  );
}

export function HistorySection({ onNavigate }: HistorySectionProps) {
  const [runtime, setRuntime] = useState<GearRuntime | null>(null);
  const [bootError, setBootError] = useState<string | null>(null);

  useEffect(() => {
    let disposed = false;
    createSignals({ deployment: "mainThreadCompatibility" })
      .then((signals) => {
        if (disposed) return;
        const WORTHSignals = signals as unknown as SignalsFacade;
        setRuntime({
          signals: WORTHSignals,
          history: WORTHSignals.history(),
          inputs: {
            innerRadius: WORTHSignals.input(initialGearParams.innerRadius, { debugName: "landing.gear.innerRadius" }),
            outerRadius: WORTHSignals.input(initialGearParams.outerRadius, { debugName: "landing.gear.outerRadius" }),
            thickness: WORTHSignals.input(initialGearParams.thickness, { debugName: "landing.gear.thickness" }),
            teeth: WORTHSignals.input(initialGearParams.teeth, { debugName: "landing.gear.teeth" }),
          },
        });
      })
      .catch((error: unknown) => setBootError(error instanceof Error ? error.message : String(error)));
    return () => {
      disposed = true;
    };
  }, []);

  return (
    <div className="xai-section-band accent-history history-section">
      <div className="xai-section-heading">
        <span className="xai-section-eyebrow">06 / History</span>
        <h2>Time, undo, and branching are first-class.</h2>
        <p>
          Replay, undo, forks, and merge-aware surfaces can live above the same
          runtime when history is modeled as retained truth instead of ad hoc
          snapshots. This gear is just Three.js; the timeline is WORTH
          <code> history()</code>.
        </p>
      </div>
      {bootError && <div className="history-error">{bootError}</div>}
      {!runtime && !bootError && <div className="history-loading">Booting WORTH runtime...</div>}
      {runtime && <HistoryWorkbench runtime={runtime} />}
      <div className="xai-section-actions">
        <button className="xai-button xai-button-secondary" type="button" onClick={() => onNavigate("#/docs/resources/branch-native-effects")}>
          Read branch docs
        </button>
      </div>
    </div>
  );
}
