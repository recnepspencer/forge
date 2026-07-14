import type { SignalValue } from "../model.js";
import type { FormAvailabilityContext } from "./availability.js";

export type FormAdmissionCapability =
  | "edit"
  | "patch"
  | "submit"
  | "action"
  | "approval"
  | "signature"
  | "review"
  | "reason";

export type FormAdmissionPosture =
  | "admitted"
  | "denied"
  | "blocked"
  | "unavailable"
  | "requiresApproval"
  | "requiresSignature"
  | "requiresReview"
  | "requiresReason";

export interface FormAdmissionArtifact {
  readonly kind: "admission";
  readonly id: string;
  readonly scope: "field" | "action";
  readonly ownerId: string;
  readonly capability: FormAdmissionCapability;
  readonly posture: FormAdmissionPosture;
  readonly dependencies: ReadonlyArray<string>;
  readonly actorDigest?: string;
  readonly policyDigest?: string;
  readonly currentActorDigest?: string;
  readonly currentPolicyDigest?: string;
  readonly binding?: FormAdmissionBindingEvidence;
  readonly stale?: {
    readonly isStale: boolean;
    readonly reasons: ReadonlyArray<string>;
  };
  readonly reason?: string;
}

export interface FormAdmissionContext extends FormAvailabilityContext {
  readonly capability: FormAdmissionCapability;
  readonly binding: FormCurrentAdmissionBinding;
}

export interface FormCurrentAdmissionBinding {
  readonly sourceDigest: string;
  readonly patchDigest: string;
  readonly schemaDigest: string;
  readonly bindingDigest: string;
}

export interface FormAdmissionBindingEvidence {
  readonly expected: {
    readonly actorDigest: string;
    readonly policyDigest: string;
    readonly sourceDigest: string | null;
    readonly patchDigest: string | null;
    readonly schemaDigest: string | null;
  };
  readonly current: {
    readonly actorDigest: string;
    readonly policyDigest: string;
    readonly sourceDigest: string | null;
    readonly patchDigest: string | null;
    readonly schemaDigest: string | null;
  };
  readonly bindingDigest: string | null;
}

export interface FormAdmissionFactory {
  field(
    fieldId: string,
    capability: FormAdmissionCapability,
    dependencies: ReadonlyArray<string>,
    resolver: (
      values: Record<string, SignalValue>,
      context: FormAdmissionContext,
    ) => FormAdmissionPosture | Partial<FormAdmissionArtifact> | true | null | undefined,
    options?: { id?: string },
  ): unknown;
  action(
    actionId: string,
    capability: FormAdmissionCapability,
    dependencies: ReadonlyArray<string>,
    resolver: (
      values: Record<string, SignalValue>,
      context: FormAdmissionContext,
    ) => FormAdmissionPosture | Partial<FormAdmissionArtifact> | true | null | undefined,
    options?: { id?: string },
  ): unknown;
}

export type FormAdmissionBuilder =
  (factory: FormAdmissionFactory) => Record<string, unknown>;

export interface FormAdmissionReport {
  readonly artifacts: ReadonlyArray<FormAdmissionArtifact>;
  readonly summary: Record<FormAdmissionPosture, number>;
  readonly counters: {
    readonly costBasis: "derivedFullReportScan";
    readonly incrementalStatus: "notIncremental";
    readonly declarations: number;
    readonly dependencyReads: number;
    readonly fieldScopes: number;
    readonly actionScopes: number;
    readonly regulatedArtifacts: number;
    readonly staleRegulatedArtifacts: number;
  };
  readonly dependencyBreadth: ReadonlyArray<{
    readonly id: string;
    readonly scope: "field" | "action";
    readonly ownerId: string;
    readonly capability: FormAdmissionCapability;
    readonly dependencies: ReadonlyArray<string>;
  }>;
}
