import type { SignalValue } from "../model.js";
import type {
  FormDirtyState,
  FormPatchOperation,
  FormReadinessBlocker,
} from "./core.js";
import type {
  FormMessageArtifact,
  FormValidationArtifact,
  FormValidationReadView,
} from "./validation.js";

export type FormStepPosture =
  | "active"
  | "optional"
  | "skipped"
  | "blocked"
  | "removed"
  | "unavailable";

export type FormStepProgress =
  | "complete"
  | "changed"
  | "blocked"
  | "skipped"
  | "removed";

export interface FormStepDeclarationOptions {
  readonly group?: string;
  readonly order?: number;
  readonly optional?: boolean;
  readonly routeCoupled?: boolean;
  readonly density?: "compact" | "comfortable" | "spacious";
  readonly alignment?: "start" | "center" | "stretch";
  readonly responsive?: ReadonlyArray<string>;
  readonly dependencies?: ReadonlyArray<string>;
  readonly resolve?: (
    values: Record<string, SignalValue>,
    context: FormStepContext,
  ) => FormStepPosture | Partial<FormStepPostureArtifact> | true | null | undefined;
}

export interface FormStepDeclaration {
  readonly id: string;
  readonly fields: ReadonlyArray<string>;
}

export interface FormStepContext {
  readonly form: FormValidationReadView;
  readonly step: string;
  readonly fields: ReadonlyArray<string>;
  readonly dependencies: ReadonlyArray<string>;
}

export interface FormStepFactory {
  step(
    stepId: string,
    fields: ReadonlyArray<string>,
    options?: FormStepDeclarationOptions,
  ): FormStepDeclaration;
}

export type FormStepsBuilder =
  (factory: FormStepFactory) => Record<string, unknown>;

export interface FormStepPostureArtifact {
  readonly posture: FormStepPosture;
  readonly reason?: string;
}

export interface FormStepArtifact {
  readonly kind: "step";
  readonly id: string;
  readonly group: string | null;
  readonly order: number;
  readonly fields: ReadonlyArray<string>;
  readonly routeCoupled: boolean;
  readonly layout: {
    readonly density: "compact" | "comfortable" | "spacious";
    readonly alignment: "start" | "center" | "stretch";
    readonly responsive: ReadonlyArray<string>;
  };
  readonly posture: FormStepPosture;
  readonly reason?: string;
  readonly readiness: {
    readonly canEnter: boolean;
    readonly canComplete: boolean;
    readonly blockers: ReadonlyArray<FormReadinessBlocker>;
  };
  readonly dirty: {
    readonly isDirty: boolean;
    readonly fields: FormDirtyState["fields"];
  };
  readonly patch: {
    readonly empty: boolean;
    readonly operations: ReadonlyArray<FormPatchOperation>;
  };
  readonly validation: {
    readonly artifacts: ReadonlyArray<FormValidationArtifact>;
    readonly blockers: ReadonlyArray<FormReadinessBlocker>;
  };
  readonly messages: ReadonlyArray<FormMessageArtifact>;
  readonly progress: FormStepProgress;
}

export interface FormStepsReport {
  readonly artifacts: ReadonlyArray<FormStepArtifact>;
  readonly summary: {
    readonly total: number;
    readonly active: number;
    readonly optional: number;
    readonly skipped: number;
    readonly blocked: number;
    readonly removed: number;
    readonly unavailable: number;
    readonly complete: number;
    readonly changed: number;
  };
  readonly counters: {
    readonly costBasis: "derivedFullReportScan";
    readonly incrementalStatus: "notIncremental";
    readonly declarations: number;
    readonly routeCoupledDeclarations: number;
    readonly stepFieldMemberships: number;
    readonly dependencyReads: number;
    readonly readinessBlockers: number;
    readonly projectedPatchOperations: number;
    readonly projectedValidationArtifacts: number;
    readonly uniqueProjectedValidationArtifacts: number;
    readonly projectedMessages: number;
    readonly uniqueProjectedMessages: number;
  };
  readonly dependencyBreadth: ReadonlyArray<{
    readonly id: string;
    readonly fields: ReadonlyArray<string>;
    readonly dependencies: ReadonlyArray<string>;
    readonly routeCoupled: boolean;
    readonly layout: FormStepArtifact["layout"];
  }>;
}
