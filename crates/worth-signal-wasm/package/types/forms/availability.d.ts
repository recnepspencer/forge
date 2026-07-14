import type { SignalValue } from "../model.js";
import type { FormValidationReadView } from "./validation.js";
import type { FormHostReport } from "./host.js";

export type FormAvailabilityState =
  | "enabled"
  | "disabled"
  | "hidden"
  | "readonly"
  | "required"
  | "omitted"
  | "blocked"
  | "unavailable";

export type FormDraftAvailabilityPolicy =
  | "preserve"
  | "clear"
  | "freeze"
  | "omit";

export interface FormAvailabilityArtifact {
  readonly kind: "availability";
  readonly id: string;
  readonly scope: FormAvailabilityScope;
  readonly ownerId: string;
  readonly fields: ReadonlyArray<string>;
  readonly state: FormAvailabilityState;
  readonly draftPolicy: FormDraftAvailabilityPolicy;
  readonly dependencies: ReadonlyArray<string>;
  readonly reason?: string;
}

export type FormAvailabilityScope =
  | "field"
  | "action"
  | "control"
  | "group"
  | "section";

export interface FormAvailabilityContext {
  readonly form: FormValidationReadView;
  readonly scope: FormAvailabilityScope;
  readonly ownerId: string;
  readonly dependencies: ReadonlyArray<string>;
}

export interface FormAvailabilityFactory {
  field(
    fieldId: string,
    dependencies: ReadonlyArray<string>,
    resolver: (
      values: Record<string, SignalValue>,
      context: FormAvailabilityContext,
    ) => FormAvailabilityState | Partial<FormAvailabilityArtifact> | true | null | undefined,
    options?: { id?: string },
  ): unknown;
  action(
    actionId: string,
    dependencies: ReadonlyArray<string>,
    resolver: (
      values: Record<string, SignalValue>,
      context: FormAvailabilityContext,
    ) => FormAvailabilityState | Partial<FormAvailabilityArtifact> | true | null | undefined,
    options?: { id?: string },
  ): unknown;
  control(
    controlId: string,
    dependencies: ReadonlyArray<string>,
    resolver: (
      values: Record<string, SignalValue>,
      context: FormAvailabilityContext,
    ) => FormAvailabilityState | Partial<FormAvailabilityArtifact> | true | null | undefined,
    options?: { id?: string },
  ): unknown;
  group(
    groupId: string,
    fields: ReadonlyArray<string>,
    dependencies: ReadonlyArray<string>,
    resolver: (
      values: Record<string, SignalValue>,
      context: FormAvailabilityContext,
    ) => FormAvailabilityState | Partial<FormAvailabilityArtifact> | true | null | undefined,
    options?: { id?: string },
  ): unknown;
  section(
    sectionId: string,
    fields: ReadonlyArray<string>,
    dependencies: ReadonlyArray<string>,
    resolver: (
      values: Record<string, SignalValue>,
      context: FormAvailabilityContext,
    ) => FormAvailabilityState | Partial<FormAvailabilityArtifact> | true | null | undefined,
    options?: { id?: string },
  ): unknown;
}

export type FormAvailabilityBuilder =
  (factory: FormAvailabilityFactory) => Record<string, unknown>;

export interface FormAvailabilityReport {
  readonly artifacts: ReadonlyArray<FormAvailabilityArtifact>;
  readonly host: FormHostReport;
  readonly summary: Record<FormAvailabilityState, number> & {
    readonly byScope: Record<FormAvailabilityScope, number>;
  };
  readonly counters: {
    readonly costBasis: "derivedFullReportScan";
    readonly incrementalStatus: "notIncremental";
    readonly declarations: number;
    readonly dependencyReads: number;
    readonly fieldRegionMemberships: number;
    readonly blockingArtifacts: number;
    readonly scopeCounts: Record<FormAvailabilityScope, number>;
  };
  readonly dependencyBreadth: ReadonlyArray<{
    readonly id: string;
    readonly scope: FormAvailabilityScope;
    readonly ownerId: string;
    readonly fields: ReadonlyArray<string>;
    readonly dependencies: ReadonlyArray<string>;
  }>;
}
