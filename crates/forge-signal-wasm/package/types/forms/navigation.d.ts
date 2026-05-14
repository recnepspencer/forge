import type { FormReadinessBlocker } from "./core.js";
import type { FormStepActionCommand } from "./actions.js";

export interface FormNavigationTransitionArtifact {
  readonly kind: "navigationTransition";
  readonly artifactId: number;
  readonly observedAtMs: number;
  readonly action: string;
  readonly command: FormStepActionCommand;
  readonly stepId: string;
  readonly routeCoupled: boolean;
  readonly resultKind: "navigated" | "blocked";
  readonly fromStepId: string | null;
  readonly toStepId: string | null;
  readonly visitedStepIds: ReadonlyArray<string>;
  readonly skippedStepIds: ReadonlyArray<string>;
  readonly blockers: ReadonlyArray<FormReadinessBlocker>;
  readonly reason: string;
  readonly token: string | null;
  readonly navigationDigest: string;
}

export interface FormNavigationReport {
  readonly current: {
    readonly stepId: string | null;
    readonly visitedStepIds: ReadonlyArray<string>;
    readonly skippedStepIds: ReadonlyArray<string>;
  };
  readonly latest: FormNavigationTransitionArtifact | null;
  readonly history: ReadonlyArray<FormNavigationTransitionArtifact>;
  readonly summary: {
    readonly currentStepId: string | null;
    readonly localStepIds: ReadonlyArray<string>;
    readonly visitedStepIds: ReadonlyArray<string>;
    readonly skippedStepIds: ReadonlyArray<string>;
    readonly blockedTransitions: number;
  };
  readonly counters: {
    readonly costBasis: "controllerLocalStepNavigationHistoryScan";
    readonly incrementalStatus: "notIncremental";
    readonly localSteps: number;
    readonly visitedSteps: number;
    readonly skippedSteps: number;
    readonly transitions: number;
    readonly blockedTransitions: number;
  };
  readonly digest: string;
}
