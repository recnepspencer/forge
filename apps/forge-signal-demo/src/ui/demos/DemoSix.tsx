import { useCallback, useEffect, useMemo, useRef, useState } from "react";

import type { DemoMetadata } from "../../state/demoData";
import { DemoShell } from "../DemoShell";
import { useSignal } from "../Demos";
import { DemoSixGear } from "./DemoSixGear";
import {
  gearParamSpecs,
  initialGearParams,
  normalizeGearParams,
  type GearParamKey,
  type GearParams,
  type RuntimeTimelineBookmark,
} from "./demoSixTypes";
import "./DemoSix.css";

interface DemoSixProps {
  signals: SignalsFacade;
  demo: DemoMetadata;
  onNavigate: (path: string) => void;
}

interface RuntimeBranch {
  id: number;
  name: string;
  parent_branch_id: number | null;
  head_snapshot_id: number | null;
}

interface RawRuntimeBranch {
  id: number | bigint;
  name: string;
  parent_branch_id: number | bigint | null;
  head_snapshot_id: number | bigint | null;
}

interface SignalInputHandle {
  value(): number;
  set(value: number): void | Promise<void>;
}

interface SignalsTransaction {
  set(handle: SignalInputHandle, value: number): void;
}

interface SignalHistoryHandle {
  current_branch(): RawRuntimeBranch;
  branches(): readonly RawRuntimeBranch[];
  create_branch(name: string): RuntimeBranch | RawRuntimeBranch | Promise<RuntimeBranch | RawRuntimeBranch>;
  switch_branch(branchId: number): void | Promise<void>;
  branch_snapshot(branchId: number): unknown;
  branch_snapshot_id(branchId: number): number | bigint;
  restore_exact_branch_snapshot(branchId: number, snapshot: unknown): void | Promise<void>;
  plan_merge_branches(sourceBranchId: number, targetBranchId: number): MergePlanView | Promise<MergePlanView>;
  merge_branches_with_proof(sourceBranchId: number, targetBranchId: number): MergeResultProofView | Promise<MergeResultProofView>;
  branch_state_proof(branchId: number): { snapshotId: number | null; stateDigest: string };
  replay_for_branch(branchId: number): { frames: readonly ReplayFrameView[] };
  subscribe?(listener: () => void): () => void;
}

interface SignalsFacade {
  input(initial: number, options?: { debugName?: string }): SignalInputHandle;
  batch(callback: (tx: SignalsTransaction) => void): unknown;
  history(): SignalHistoryHandle;
}

interface MergePlanView {
  merge_kind: string;
  node_plan: readonly unknown[];
}

interface MergeResultProofView {
  result?: {
    merge_kind: string;
  };
  merge_kind?: string;
  proof?: {
    resultDigest?: string;
  };
}

interface ReplayFrameView {
  kind: string;
  snapshotId: number | null;
}

function numberId(value: number | bigint | null | undefined): number | null {
  if (value == null) return null;
  return Number(value);
}

function formatNumber(value: number) {
  return Number.isInteger(value) ? String(value) : value.toFixed(2);
}

function readBranchFromRaw(branch: RawRuntimeBranch): RuntimeBranch {
  return {
    id: Number(branch.id),
    name: branch.name,
    parent_branch_id: numberId(branch.parent_branch_id),
    head_snapshot_id: numberId(branch.head_snapshot_id),
  };
}

function readBranch(history: SignalHistoryHandle): RuntimeBranch {
  return readBranchFromRaw(history.current_branch());
}

function readBranches(history: SignalHistoryHandle): RuntimeBranch[] {
  return history.branches().map(readBranchFromRaw);
}

function createBookmark(
  history: SignalHistoryHandle,
  label: string,
  params: GearParams,
): RuntimeTimelineBookmark {
  const branch = readBranch(history);
  const snapshot = history.branch_snapshot(branch.id);
  const snapshotId = numberId(history.branch_snapshot_id(branch.id));
  return {
    id: `${branch.id}:${snapshotId ?? "none"}:${label}`,
    branchId: branch.id,
    branchName: branch.name,
    snapshotId,
    snapshot,
    label,
    params,
  };
}

export function DemoSix({ signals, demo, onNavigate }: DemoSixProps) {
  const history = useMemo(() => signals.history(), [signals]);
  const inputs = useMemo(() => ({
    innerRadius: signals.input(initialGearParams.innerRadius, { debugName: "gear.innerRadius" }),
    outerRadius: signals.input(initialGearParams.outerRadius, { debugName: "gear.outerRadius" }),
    thickness: signals.input(initialGearParams.thickness, { debugName: "gear.thickness" }),
    teeth: signals.input(initialGearParams.teeth, { debugName: "gear.teeth" }),
  }), [signals]);

  const runtimeParams = normalizeGearParams({
    innerRadius: useSignal<number>(signals, inputs.innerRadius),
    outerRadius: useSignal<number>(signals, inputs.outerRadius),
    thickness: useSignal<number>(signals, inputs.thickness),
    teeth: useSignal<number>(signals, inputs.teeth),
  });

  const [branches, setBranches] = useState<RuntimeBranch[]>(readBranches(history));
  const [activeBranch, setActiveBranch] = useState<RuntimeBranch | null>(readBranch(history));
  const [bookmarks, setBookmarks] = useState<RuntimeTimelineBookmark[]>([
    createBookmark(history, "initial gear", initialGearParams),
  ]);
  const [stagedParams, setStagedParams] = useState<GearParams | null>(null);
  const [lastPlan, setLastPlan] = useState<MergePlanView | null>(null);
  const [lastMerge, setLastMerge] = useState<MergeResultProofView | null>(null);
  const [lastEvent, setLastEvent] = useState("Runtime booted. Gear parameters are Forge input signals.");
  const debounceRef = useRef<number | null>(null);

  const displayParams = stagedParams ?? runtimeParams;

  const refreshHistoryView = useCallback(() => {
    setBranches(readBranches(history));
    setActiveBranch(readBranch(history));
  }, [history]);

  const readRuntimeParamsNow = useCallback((): GearParams => normalizeGearParams({
    innerRadius: inputs.innerRadius.value(),
    outerRadius: inputs.outerRadius.value(),
    thickness: inputs.thickness.value(),
    teeth: inputs.teeth.value(),
  }), [inputs]);

  const captureBookmark = useCallback((label: string, params: GearParams = runtimeParams) => {
    const bookmark = createBookmark(history, label, params);
    setBookmarks((current) => {
      if (current.some((item) => item.branchId === bookmark.branchId && item.snapshotId === bookmark.snapshotId)) {
        return current;
      }
      return [...current, bookmark];
    });
    refreshHistoryView();
  }, [history, refreshHistoryView, runtimeParams]);

  useEffect(() => {
    const unsubscribe = typeof history.subscribe === "function"
      ? history.subscribe(refreshHistoryView)
      : null;
    return () => {
      if (typeof unsubscribe === "function") unsubscribe();
    };
  }, [history, refreshHistoryView]);

  const commitParams = useCallback((params: GearParams, label: string) => {
    const next = normalizeGearParams(params);
    signals.batch((tx: SignalsTransaction) => {
      gearParamSpecs.forEach((spec) => {
        if (runtimeParams[spec.key] !== next[spec.key]) {
          tx.set(inputs[spec.key], next[spec.key]);
        }
      });
    });
    setStagedParams(null);
    captureBookmark(label, next);
    setLastEvent(`${label}: committed through signals.batch(...) on ${readBranch(history).name}.`);
  }, [captureBookmark, history, inputs, runtimeParams, signals]);

  const scheduleCommit = (key: GearParamKey, rawValue: number) => {
    const next = normalizeGearParams({
      ...displayParams,
      [key]: rawValue,
    });
    setStagedParams(next);
    setLastEvent(`${key} staged; Forge commit will happen after the debounce boundary.`);
    if (debounceRef.current != null) window.clearTimeout(debounceRef.current);
    debounceRef.current = window.setTimeout(() => {
      commitParams(next, `${key} -> ${formatNumber(next[key])}`);
      debounceRef.current = null;
    }, 320);
  };

  const flushStaged = () => {
    if (debounceRef.current != null) {
      window.clearTimeout(debounceRef.current);
      debounceRef.current = null;
    }
    if (stagedParams) {
      commitParams(stagedParams, "slider release");
    }
  };

  const createBranch = async () => {
    const branch = readBranchFromRaw(await history.create_branch(`gear-branch-${branches.length}`));
    await history.switch_branch(branch.id);
    refreshHistoryView();
    captureBookmark(`created ${branch.name}`);
    setLastEvent(`Created and checked out runtime branch ${branch.name}.`);
  };

  const switchBranch = async (branchId: number) => {
    await history.switch_branch(branchId);
    setStagedParams(null);
    refreshHistoryView();
    setLastEvent(`Checked out runtime branch ${branchId}.`);
  };

  const restoreBookmark = async (bookmark: RuntimeTimelineBookmark) => {
    await history.switch_branch(bookmark.branchId);
    await history.restore_exact_branch_snapshot(bookmark.branchId, bookmark.snapshot);
    setStagedParams(null);
    refreshHistoryView();
    setLastEvent(`Restored ${bookmark.branchName} to snapshot ${bookmark.snapshotId}.`);
  };

  const activeBookmarks = bookmarks.filter((bookmark) => bookmark.branchId === activeBranch?.id);
  const activeSnapshotId = activeBranch?.head_snapshot_id ?? null;
  const activeBookmarkIndex = activeBookmarks.findIndex((bookmark) => bookmark.snapshotId === activeSnapshotId);

  const restoreRelative = async (direction: -1 | 1) => {
    const next = activeBookmarks[activeBookmarkIndex + direction];
    if (next) await restoreBookmark(next);
  };

  const mergeIntoActive = async (sourceBranchId: number) => {
    if (!activeBranch || sourceBranchId === activeBranch.id) return;
    const plan = await history.plan_merge_branches(sourceBranchId, activeBranch.id);
    const result = await history.merge_branches_with_proof(sourceBranchId, activeBranch.id);
    setLastPlan(plan);
    setLastMerge(result);
    refreshHistoryView();
    captureBookmark(`merged branch ${sourceBranchId}`, readRuntimeParamsNow());
    setLastEvent(`Merged branch ${sourceBranchId} into ${activeBranch.name}; result proof retained by history().`);
  };

  const branchProof = activeBranch ? history.branch_state_proof(activeBranch.id) : null;
  const replay: { frames: readonly ReplayFrameView[] } = activeBranch
    ? history.replay_for_branch(activeBranch.id)
    : { frames: [] };
  const recentFrames: readonly ReplayFrameView[] = replay.frames.slice(-5);

  return (
    <DemoShell
      demo={demo}
      onNavigate={onNavigate}
      inspectorContent={
        <div className="demo-six-inspector">
          <div><span>[history.current_branch]</span> {activeBranch?.name ?? "unknown"} #{activeBranch?.id}</div>
          <div><span>[branch_state_proof]</span> snapshot={branchProof?.snapshotId ?? "none"} digest={branchProof?.stateDigest?.slice(0, 18) ?? "pending"}</div>
          <div><span>[merge.plan]</span> {lastPlan ? `${lastPlan.merge_kind} / nodes=${lastPlan.node_plan.length}` : "none yet"}</div>
          <div><span>[merge.result]</span> {lastMerge ? `${lastMerge.result?.merge_kind ?? lastMerge.merge_kind} / proof=${lastMerge.proof?.resultDigest?.slice(0, 18) ?? "retained"}` : "none yet"}</div>
          <div><span>[replay.tail]</span> {recentFrames.map((frame) => `${frame.kind}:${frame.snapshotId ?? "none"}`).join(" -> ") || "empty"}</div>
        </div>
      }
    >
      <div className="demo-six">
        <section className="demo-six-hero">
          <DemoSixGear params={displayParams} />
          <div className="demo-six-state">
            <p className="demo-six-kicker">Three.js geometry from Forge signal truth</p>
            <h3>{activeBranch?.name ?? "main"}</h3>
            <dl>
              {gearParamSpecs.map((spec) => (
                <div key={spec.key}>
                  <dt>{spec.label}</dt>
                  <dd>{formatNumber(displayParams[spec.key])}</dd>
                </div>
              ))}
            </dl>
            <p>{lastEvent}</p>
          </div>
        </section>

        <section className="demo-six-controls">
          {gearParamSpecs.map((spec) => (
            <label key={spec.key} className="demo-six-slider">
              <span>{spec.label}</span>
              <input
                type="range"
                min={spec.min}
                max={spec.max}
                step={spec.step}
                value={displayParams[spec.key]}
                onChange={(event) => scheduleCommit(spec.key, Number(event.target.value))}
                onPointerUp={flushStaged}
                onKeyUp={flushStaged}
              />
              <strong>{formatNumber(displayParams[spec.key])}</strong>
            </label>
          ))}
        </section>

        <section className="demo-six-actions">
          <button className="btn" onClick={createBranch}>Create branch</button>
          <button className="btn" disabled={activeBookmarkIndex <= 0} onClick={() => void restoreRelative(-1)}>Undo snapshot</button>
          <button className="btn" disabled={activeBookmarkIndex < 0 || activeBookmarkIndex >= activeBookmarks.length - 1} onClick={() => void restoreRelative(1)}>Redo snapshot</button>
        </section>

        <section className="demo-six-grid">
          <div>
            <h4>Runtime branches</h4>
            <div className="demo-six-branch-list">
              {branches.map((branch) => (
                <div key={branch.id} className={branch.id === activeBranch?.id ? "active" : ""}>
                  <button onClick={() => void switchBranch(branch.id)}>
                    {branch.name} <span>#{branch.id}</span>
                  </button>
                  {branch.id !== activeBranch?.id && (
                    <button onClick={() => void mergeIntoActive(branch.id)}>Merge into active</button>
                  )}
                </div>
              ))}
            </div>
          </div>

          <div>
            <h4>Git-style snapshot bookmarks</h4>
            <div className="demo-six-timeline">
              {bookmarks.map((bookmark) => (
                <button
                  key={bookmark.id}
                  className={bookmark.branchId === activeBranch?.id && bookmark.snapshotId === activeSnapshotId ? "active" : ""}
                  onClick={() => void restoreBookmark(bookmark)}
                >
                  <span>{bookmark.branchName}</span>
                  <strong>{bookmark.label}</strong>
                  <small>snapshot {bookmark.snapshotId ?? "none"}</small>
                </button>
              ))}
            </div>
          </div>
        </section>
      </div>
    </DemoShell>
  );
}
