import { useEffect, useRef, useState } from "react";
import type { FC } from "react";

import {
  createGearScenario,
  type GearConflictSelection,
} from "../local-truth-gear/gear_scenario";
import type {
  GearBranchRole,
  GearConflictChoice,
  GearHistoryNode,
  GearScenarioView,
  GearSignalProjectionView,
} from "../local-truth-gear/gear_scenario_view";
import type {
  GearDesignAspect,
  GearTruth,
} from "../local-truth-gear/gear_truth";
import { DemoSixGear } from "./demos/DemoSixGear";
import { GearAspectExplainer } from "./demos/GearAspectExplainer";
import { GearHistoryGraph } from "./demos/GearHistoryGraph";
import "./compositionSection.css";
import "./gearConflictReview.css";

interface CompositionSectionProps {
  onNavigate: (path: string) => void;
}

type GearScenario = Awaited<ReturnType<typeof createGearScenario>>;

export const CompositionSection: FC<CompositionSectionProps> = () => {
  const [scenario, setScenario] = useState<GearScenario | null>(null);
  const [view, setView] = useState<GearScenarioView | null>(null);
  const [conflictChoices, setConflictChoices] = useState<Record<string, GearConflictChoice>>({});
  const [busy, setBusy] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const actionTail = useRef<Promise<void>>(Promise.resolve());
  const pendingActions = useRef(0);

  useEffect(() => {
    let active = true;
    let created: GearScenario | null = null;
    void createGearScenario()
      .then(async (next) => {
        created = next;
        const initialView = await next.readView();
        if (!active) return next.terminate();
        setScenario(next);
        setView(initialView);
      })
      .catch((cause) => {
        if (active) setError(errorMessage(cause));
      });
    return () => {
      active = false;
      void created?.terminate();
    };
  }, []);

  const enqueue = (operation: () => Promise<GearScenarioView>) => {
    pendingActions.current += 1;
    setBusy(true);
    setError(null);
    const action = actionTail.current.then(operation);
    actionTail.current = action.then(
      (nextView) => {
        setView(nextView);
      },
      (cause) => setError(errorMessage(cause)),
    ).finally(() => {
      pendingActions.current -= 1;
      if (pendingActions.current === 0) setBusy(false);
    });
  };

  const commitBranchAspect = (
    role: GearBranchRole,
    aspect: GearDesignAspect,
    value: number,
  ) => {
    if (!scenario || !view?.activeDesignBranchId || view.phase !== "editing") return;
    enqueue(() => scenario.commitBranchPatch(role, { [aspect]: value }));
  };

  const forkBranches = () => {
    if (!scenario || view?.historySelection) return;
    setConflictChoices({});
    enqueue(() => scenario.forkDesignBranch());
  };

  const runMergeAction = () => {
    if (!scenario || !view?.activeDesignBranchId) return;
    if (view.phase === "review") {
      const selections: GearConflictSelection[] = view.conflicts.map(({ id }) => ({
        conflictId: id,
        choice: conflictChoices[id],
      }));
      enqueue(async () => {
        const nextView = await scenario.resolveMerge(selections);
        setConflictChoices({});
        return nextView;
      });
      return;
    }
    setConflictChoices({});
    enqueue(() => scenario.mergeBranches());
  };

  const unresolvedConflictCount = view?.conflicts.filter(({ id }) => !conflictChoices[id]).length ?? 0;
  const canEditBranches = Boolean(view?.activeDesignBranchId)
    && view?.phase === "editing"
    && !view.historySelection;
  const designBranchName = view?.designBranchName ?? "Design branch";

  const selectHistoryNode = (node: GearHistoryNode) => {
    if (!scenario) return;
    enqueue(() => scenario.selectHistoryCommit(node.branchId, node.id));
  };

  return (
    <div className="xai-section-band accent-composition composition-section">
      <div className="xai-section-heading gear-section-heading">
        <span className="xai-section-eyebrow">06 / Aspect composition</span>
        <h2>Merge aspects, not objects.</h2>
        <p>
          Fork the gear and move both branches. Worth compares every aspect to the basis you
          forked from — only a true collision needs you.
        </p>
      </div>

      {error ? <div className="gear-workspace-error">Runtime error: {error}</div> : null}
      {!view || !scenario ? (
        <div className="gear-workspace-loading">Preparing the gear workspace...</div>
      ) : (
        <>
          <section className="gear-workspace">
            <div className="gear-workspace-copy">
              <span>{workspacePhaseLabel(view)}</span>
              <h3>{view.headline}</h3>
            </div>

            <div className="gear-branch-comparison">
              <GearBranchEditor
                disabled={!canEditBranches || busy}
                label="Main"
                onCommit={(aspect, value) => commitBranchAspect("main", aspect, value)}
                roleDescription={branchRoleDescription(view, "main")}
                tone="main"
                values={view.main}
              />
              {view.activeDesignBranchId || view.phase === "merged" ? (
                <GearBranchEditor
                  disabled={!canEditBranches || busy}
                  label={designBranchName}
                  onCommit={(aspect, value) => commitBranchAspect("design", aspect, value)}
                  roleDescription={branchRoleDescription(view, "design")}
                  tone="design"
                  values={view.design}
                />
              ) : (
                <div className="gear-workspace-visual empty">
                  <span className="gear-workspace-projection branch">Design branch</span>
                  <div className="gear-branch-empty">
                    <strong>+</strong>
                    <span>Fork Main to get a second writable branch.</span>
                  </div>
                </div>
              )}
            </div>

            <SignalProjectionStrip projection={view.signalProjection} />

            {view.phase === "review" ? (
              <div className="gear-conflict-review">
                {view.conflicts.map((conflict) => (
                  <div className="gear-conflict-row" key={conflict.id}>
                    <div>
                      <span>Conflicting aspect</span>
                      <code>{conflict.aspectId}</code>
                    </div>
                    <button
                      className={conflictChoices[conflict.id] === "main" ? "selected" : ""}
                      onClick={() => setConflictChoices((current) => ({ ...current, [conflict.id]: "main" }))}
                      type="button"
                    >
                      <span>Keep Main</span>
                      <strong>{formatConflictValue(conflict.mainValue)}</strong>
                    </button>
                    <button
                      className={conflictChoices[conflict.id] === "design" ? "selected" : ""}
                      onClick={() => setConflictChoices((current) => ({ ...current, [conflict.id]: "design" }))}
                      type="button"
                    >
                      <span>Use Design</span>
                      <strong>{formatConflictValue(conflict.designValue)}</strong>
                    </button>
                  </div>
                ))}
              </div>
            ) : null}

            <div className="gear-workspace-actions">
              {!view.activeDesignBranchId ? (
                <button
                  className="gear-branch-button"
                  disabled={busy || Boolean(view.historySelection)}
                  onClick={forkBranches}
                  type="button"
                >
                  {view.historySelection
                    ? "Select a live head to resume"
                    : view.phase === "merged" ? "Fork again from Main" : "Fork the design branch"}
                </button>
              ) : (
                <button
                  className="gear-merge-button"
                  disabled={busy || Boolean(view.historySelection) || (view.phase === "review" && unresolvedConflictCount > 0)}
                  onClick={runMergeAction}
                  type="button"
                >
                  {mergeButtonLabel(view, busy, unresolvedConflictCount)}
                </button>
              )}
            </div>
          </section>

          <GearHistoryGraph
            busy={busy}
            nodes={view.history}
            onSelect={selectHistoryNode}
            selection={view.historySelection}
          />

          <GearAspectExplainer />
        </>
      )}
    </div>
  );
};

function GearBranchEditor({
  disabled,
  label,
  onCommit,
  roleDescription,
  tone,
  values,
}: {
  disabled: boolean;
  label: string;
  onCommit: (aspect: GearDesignAspect, value: number) => void;
  roleDescription: string;
  tone: "main" | "design";
  values: GearTruth;
}) {
  return (
    <div className={`gear-branch-editor ${tone}`}>
      <div className={`gear-workspace-visual ${tone}`}>
        <DemoSixGear params={values} />
        <span className={`gear-workspace-projection ${tone}`}>{label}</span>
        <small>{roleDescription}</small>
      </div>
      <div className="gear-branch-controls" aria-label={`${label} aspect controls`}>
        <GearSlider branchLabel={label} disabled={disabled} format={(value) => `${value.toFixed(2)} in`} label="Thickness" max={1.2} min={0.2} onCommit={(value) => onCommit("thickness", value)} step={0.02} value={values.thickness} />
        <GearSlider branchLabel={label} disabled={disabled} format={(value) => `${Math.round(value)}`} label="Gear count" max={36} min={10} onCommit={(value) => onCommit("teeth", value)} step={1} value={values.teeth} />
        <GearSlider branchLabel={label} disabled={disabled} format={(value) => `${value.toFixed(2)} in`} label="Hole size" max={1.1} min={0.3} onCommit={(value) => onCommit("innerRadius", value)} step={0.02} value={values.innerRadius} />
      </div>
    </div>
  );
}

/**
 * The slider thumb previews the value you are choosing; nothing is written
 * until you release. Release commits exactly one aspect to Local Truth, and
 * the gear re-renders only from the committed view.
 */
function GearSlider({
  branchLabel,
  disabled,
  format,
  label,
  max,
  min,
  onCommit,
  step,
  value,
}: {
  branchLabel: string;
  disabled: boolean;
  format: (value: number) => string;
  label: string;
  max: number;
  min: number;
  onCommit: (value: number) => void;
  step: number;
  value: number;
}) {
  const [pending, setPending] = useState<number | null>(null);
  const committedValue = useRef(value);

  useEffect(() => {
    if (committedValue.current !== value) {
      committedValue.current = value;
      setPending(null);
    }
  }, [value]);

  const shown = pending ?? value;
  const commitPending = () => {
    if (pending !== null && pending !== value) onCommit(pending);
  };

  return (
    <label className="gear-slider">
      <span>
        <strong>{label}</strong>
        <output className={pending !== null && pending !== value ? "is-pending" : ""}>{format(shown)}</output>
      </span>
      <input
        aria-label={`${branchLabel} ${label}`}
        disabled={disabled}
        max={max}
        min={min}
        onBlur={commitPending}
        onChange={(event) => setPending(Number(event.currentTarget.value))}
        onKeyUp={commitPending}
        onPointerUp={commitPending}
        step={step}
        type="range"
        value={shown}
      />
    </label>
  );
}

function SignalProjectionStrip({
  projection,
}: {
  projection: GearSignalProjectionView | null;
}) {
  if (!projection) return null;
  return (
    <div className="gear-signal-projection">
      <div>
        <span>Signal projection · {projection.branchName} · native branch #{projection.signalBranchId}</span>
        <strong>basis {projection.basisDigest.slice(0, 16)}…</strong>
      </div>
      <code>truth.derivation(branch).binding — this digest advances with every commit</code>
    </div>
  );
}

function mergeButtonLabel(view: GearScenarioView, busy: boolean, unresolvedCount: number) {
  if (busy) return "Committing…";
  if (view.historySelection) return "Select a live head to resume";
  if (view.phase !== "review") return "Merge branches";
  if (unresolvedCount > 0) return `Choose ${unresolvedCount} conflicting aspect${unresolvedCount === 1 ? "" : "s"}`;
  return "Commit the merge";
}

function workspacePhaseLabel(view: GearScenarioView) {
  if (view.historySelection) return "Sealed snapshot";
  if (view.phase === "ready") return "One branch";
  if (view.phase === "editing") return "Two live branches";
  if (view.phase === "review") return "Aspect review";
  return "Merged";
}

function branchRoleDescription(
  view: GearScenarioView,
  role: GearBranchRole,
) {
  if (view.historySelection?.lane === role) return "Historical snapshot";
  if (view.historySelection) return "Live head";
  if (view.phase === "merged") return role === "main" ? "Merged result" : "Reviewed source";
  return role === "main" ? "Merge target" : "Merge source";
}

function formatConflictValue(value: unknown) {
  return typeof value === "number" ? Number(value.toFixed(2)).toString() : String(value);
}

function errorMessage(cause: unknown) {
  return cause instanceof Error ? cause.message : String(cause);
}
