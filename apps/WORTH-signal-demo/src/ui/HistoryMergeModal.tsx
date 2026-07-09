import { useMemo, useState } from "react";
import { createPortal } from "react-dom";

import { gearParamSpecs, type GearParamKey, type RuntimeTimelineBookmark } from "./demos/demoSixTypes";
import type {
  MergePlanView,
  MergeResultProofView,
  RuntimeBranch,
  SignalHistoryHandle,
} from "./historyRuntimeTypes";

interface FormFieldHandle {
  value(): string;
  set(value: string): unknown;
}

interface StepFormHandle {
  fields: Record<string, FormFieldHandle>;
  steps(): {
    summary: { total: number; complete: number; changed: number; blocked: number };
    artifacts: ReadonlyArray<{ id: string; progress: string; readiness: { canComplete: boolean } }>;
  };
  navigation(): { current: { stepId: string | null }; summary: { currentStepId: string | null } };
  executeAction(actionId: string): { resultKind: string; reason?: string };
}

interface SignalsFacade {
  form(declaration: unknown): unknown;
}

interface MergeConflict {
  key: GearParamKey;
  label: string;
  base: number;
  source: number;
  target: number;
}

interface MergeBranchesModalProps {
  open: boolean;
  signals: SignalsFacade;
  history: SignalHistoryHandle;
  branches: readonly RuntimeBranch[];
  activeBranch: RuntimeBranch;
  bookmarks: readonly RuntimeTimelineBookmark[];
  onClose: () => void;
  onMerged: (
    sourceBranchId: number,
    targetBranchId: number,
    plan: MergePlanView,
    result: MergeResultProofView,
  ) => void;
  onUnavailable: (message: string) => void;
}

function branchHeadBookmark(
  branchId: number,
  branches: readonly RuntimeBranch[],
  bookmarks: readonly RuntimeTimelineBookmark[],
) {
  const branch = branches.find((entry) => entry.id === branchId);
  return bookmarks.find((bookmark) =>
    bookmark.branchId === branchId && bookmark.snapshotId === branch?.head_snapshot_id
  ) ?? [...bookmarks].reverse().find((bookmark) => bookmark.branchId === branchId) ?? null;
}

function ancestors(bookmark: RuntimeTimelineBookmark | null, byId: Map<string, RuntimeTimelineBookmark>) {
  const seen = new Set<string>();
  const visit = (current: RuntimeTimelineBookmark | null) => {
    if (!current || seen.has(current.id)) return;
    seen.add(current.id);
    current.parentIds.forEach((parentId) => visit(byId.get(parentId) ?? null));
  };
  visit(bookmark);
  return seen;
}

function commonBase(
  source: RuntimeTimelineBookmark | null,
  target: RuntimeTimelineBookmark | null,
  bookmarks: readonly RuntimeTimelineBookmark[],
) {
  const byId = new Map(bookmarks.map((bookmark) => [bookmark.id, bookmark]));
  const sourceAncestors = ancestors(source, byId);
  return [...bookmarks].reverse().find((bookmark) =>
    sourceAncestors.has(bookmark.id) && ancestors(target, byId).has(bookmark.id)
  ) ?? bookmarks[0] ?? null;
}

function semanticConflicts(
  source: RuntimeTimelineBookmark | null,
  target: RuntimeTimelineBookmark | null,
  base: RuntimeTimelineBookmark | null,
): MergeConflict[] {
  if (!source || !target || !base) return [];
  return gearParamSpecs.flatMap((spec) => {
    const sourceChanged = source.params[spec.key] !== base.params[spec.key];
    const targetChanged = target.params[spec.key] !== base.params[spec.key];
    const diverged = source.params[spec.key] !== target.params[spec.key];
    return sourceChanged && targetChanged && diverged
      ? [{
          key: spec.key,
          label: spec.label,
          base: base.params[spec.key],
          source: source.params[spec.key],
          target: target.params[spec.key],
        }]
      : [];
  });
}

function createConflictForm(signals: SignalsFacade, conflicts: readonly MergeConflict[]) {
  const source = Object.fromEntries(conflicts.map((conflict) => [conflict.key, "source"]));
  return signals.form({
    source,
    fields: ({ field }: { field: (fieldId: string) => unknown }) =>
      Object.fromEntries(conflicts.map((conflict) => [conflict.key, field(conflict.key)])),
    steps: ({ step }: { step: (stepId: string, fields: readonly string[], options?: unknown) => unknown }) =>
      Object.fromEntries(conflicts.map((conflict, index) => [
        conflict.key,
        step(conflict.key, [conflict.key], { order: index + 1, group: "merge-conflicts" }),
      ])),
    actions: ({ step }: { step: (actionId: string, stepId: string, command: string) => unknown }) =>
      Object.fromEntries(conflicts.flatMap((conflict) => [
        [`jump-${conflict.key}`, step(`jump-${conflict.key}`, conflict.key, "jump")],
        [`next-${conflict.key}`, step(`next-${conflict.key}`, conflict.key, "next")],
      ])),
  }) as StepFormHandle;
}

export function HistoryMergeModal({
  open,
  signals,
  history,
  branches,
  activeBranch,
  bookmarks,
  onClose,
  onMerged,
  onUnavailable,
}: MergeBranchesModalProps) {
  const selectable = branches.filter((branch) => branch.id !== activeBranch.id);
  const [sourceBranchId, setSourceBranchId] = useState<number | null>(selectable[0]?.id ?? null);
  const [targetBranchId, setTargetBranchId] = useState(activeBranch.id);
  const [phase, setPhase] = useState<"pick" | "conflicts">("pick");
  const [busy, setBusy] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [, setFormRevision] = useState(0);
  const sourceHead = branchHeadBookmark(sourceBranchId ?? -1, branches, bookmarks);
  const targetHead = branchHeadBookmark(targetBranchId, branches, bookmarks);
  const base = commonBase(sourceHead, targetHead, bookmarks);
  const conflicts = useMemo(
    () => semanticConflicts(sourceHead, targetHead, base),
    [base, sourceHead, targetHead],
  );
  const conflictForm = useMemo(
    () => conflicts.length > 0 ? createConflictForm(signals, conflicts) : null,
    [conflicts, signals],
  );
  if (!open) return null;

  const executeMerge = async () => {
    if (sourceBranchId == null || sourceBranchId === targetBranchId) return;
    setBusy(true);
    setError(null);
    try {
      const plan = typeof history.plan_merge_policy_preview === "function"
        ? await history.plan_merge_policy_preview({
            source_branch_id: sourceBranchId,
            target_branch_id: targetBranchId,
            conflict_policy_name: "signal.conflict.reject-shared-state",
          })
        : await history.plan_merge_branches(sourceBranchId, targetBranchId);
      if (conflicts.length > 0 && phase !== "conflicts") {
        setPhase("conflicts");
        setBusy(false);
        return;
      }
      const choices = conflicts.map((conflict) => conflictForm?.fields[conflict.key]?.value() ?? "source");
      if (choices.some((choice) => choice === "target")) {
        onUnavailable("WORTH Forms captured mixed conflict choices, but this runtime only exposes source-resolution or reject-shared-state merge execution.");
        setBusy(false);
        return;
      }
      const result = await history.merge_branches_with_proof(sourceBranchId, targetBranchId);
      onMerged(sourceBranchId, targetBranchId, plan, result);
      onClose();
    } catch (mergeError) {
      setError(mergeError instanceof Error ? mergeError.message : String(mergeError));
    } finally {
      setBusy(false);
    }
  };

  return createPortal(
    <div className="history-merge-layer" role="presentation">
      <button className="history-merge-backdrop" type="button" aria-label="Close merge dialog" onClick={onClose} />
      <div className="history-merge-modal" role="dialog" aria-modal="true" aria-label="Merge branches">
        <div className="history-merge-header">
          <p className="history-kicker">Runtime branch merge</p>
          <h3>{phase === "pick" ? "Choose two branches" : "Resolve conflicts with WORTH Forms"}</h3>
        </div>

        {phase === "pick" && (
          <div className="history-merge-picker">
            <BranchPicker title="Source branch" branches={selectable} selectedId={sourceBranchId} onSelect={setSourceBranchId} />
            <BranchPicker title="Target branch" branches={branches} selectedId={targetBranchId} onSelect={setTargetBranchId} />
          </div>
        )}

        {phase === "conflicts" && conflictForm && (
          <div className="history-conflict-form">
            <div className="history-conflict-progress">
              <span>{conflictForm.navigation().summary.currentStepId ?? conflicts[0]?.key}</span>
              <span>{conflictForm.steps().summary.total} steps from form.steps()</span>
            </div>
            {conflicts.map((conflict) => (
              <div className="history-conflict-step" key={conflict.key}>
                <div>
                  <strong>{conflict.label}</strong>
                  <span>base {conflict.base} / source {conflict.source} / target {conflict.target}</span>
                </div>
                <div className="history-conflict-choice">
                  <button
                    type="button"
                    className={conflictForm.fields[conflict.key]?.value() === "source" ? "active" : ""}
                    onClick={() => {
                      conflictForm.fields[conflict.key]?.set("source");
                      conflictForm.executeAction(`jump-${conflict.key}`);
                      setFormRevision((revision) => revision + 1);
                    }}
                  >
                    Prefer source gear
                  </button>
                  <button
                    type="button"
                    className={conflictForm.fields[conflict.key]?.value() === "target" ? "active" : ""}
                    onClick={() => {
                      conflictForm.fields[conflict.key]?.set("target");
                      conflictForm.executeAction(`jump-${conflict.key}`);
                      setFormRevision((revision) => revision + 1);
                    }}
                  >
                    Prefer target gear
                  </button>
                </div>
              </div>
            ))}
            <code>form.steps(): {conflictForm.steps().summary.changed} changed / {conflictForm.steps().summary.blocked} blocked</code>
          </div>
        )}

        {error && <div className="history-error">{error}</div>}
        <div className="history-merge-actions">
          {phase === "conflicts" && <button type="button" onClick={() => setPhase("pick")}>Back</button>}
          <button type="button" onClick={onClose}>Cancel</button>
          <button type="button" disabled={busy || sourceBranchId == null || sourceBranchId === targetBranchId} onClick={() => void executeMerge()}>
            {busy ? "Merging..." : conflicts.length > 0 && phase === "pick" ? "Review conflicts" : "Merge branches"}
          </button>
        </div>
      </div>
    </div>,
    document.body,
  );
}

function BranchPicker({
  title,
  branches,
  selectedId,
  onSelect,
}: {
  title: string;
  branches: readonly RuntimeBranch[];
  selectedId: number | null;
  onSelect: (branchId: number) => void;
}) {
  return (
    <div className="history-branch-picker">
      <h4>{title}</h4>
      {branches.map((branch) => (
        <button
          key={branch.id}
          type="button"
          className={branch.id === selectedId ? "active" : ""}
          onClick={() => onSelect(branch.id)}
        >
          <span>{branch.name}</span>
          <small>snapshot {branch.head_snapshot_id ?? "none"}</small>
        </button>
      ))}
    </div>
  );
}
