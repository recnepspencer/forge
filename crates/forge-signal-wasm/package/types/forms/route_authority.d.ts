import type { RouteFormsAuthorityArtifact } from "../router_admission_surface.js";

export interface FormRouteAuthorityArtifact {
  readonly kind: "routeAuthorityUpdate";
  readonly artifactId: number;
  readonly observedAtMs: number;
  readonly source: "report" | "clear";
  readonly routeId: string | null;
  readonly href: string | null;
  readonly scopeKind: "route" | null;
  readonly surfaceId: string | null;
  readonly continuity: RouteFormsAuthorityArtifact["continuity"] | null;
  readonly reason: string;
  readonly handoff: FormRouteAuthorityHandoffArtifact;
  readonly draftContinuity: FormRouteAuthorityDraftContinuityArtifact;
  readonly continuityApplied:
    | "preservedDraft"
    | "frozeDraft"
    | "discardedDraft"
    | "maintainedAuthority"
    | "deferredDraft"
    | "clearedAuthority";
  readonly transitionKind:
    | "initialAuthority"
    | "authorityChanged"
    | "authorityRefreshed"
    | "authorityCleared"
    | "alreadyCleared";
  readonly previousAuthorityDigest: string | null;
  readonly previousDraftDigest: string | null;
  readonly nextDraftDigest: string | null;
  readonly verificationDigest: string | null;
  readonly routeAuthorityDigest: string;
}

export interface FormRouteAuthorityHandoffArtifact {
  readonly kind: "routeAuthorityHandoff";
  readonly routeId: string | null;
  readonly href: string | null;
  readonly scopeKind: "route" | null;
  readonly surfaceId: string | null;
  readonly posture: RouteFormsAuthorityArtifact["continuity"] | "cleared";
  readonly draftDisposition: FormRouteAuthorityArtifact["continuityApplied"];
  readonly routeCoupledBehavior: "admitted" | "deferred" | "cleared";
  readonly transitionKind: FormRouteAuthorityArtifact["transitionKind"];
  readonly reason: string;
}

export interface FormRouteAuthorityDraftContinuityArtifact {
  readonly kind: "routeAuthorityDraftContinuity";
  readonly routeId: string | null;
  readonly href: string | null;
  readonly surfaceId: string | null;
  readonly posture: FormRouteAuthorityArtifact["continuityApplied"];
  readonly authorityChange: FormRouteAuthorityArtifact["transitionKind"];
  readonly draftChanged: boolean;
  readonly draftResolution:
    | "preservedValue"
    | "preservedFrozenValue"
    | "replacedFromSource"
    | "awaitingAdmittedTruth"
    | "authorityCleared";
  readonly previousAuthorityDigest: string | null;
  readonly previousDraftDigest: string | null;
  readonly nextDraftDigest: string | null;
  readonly reason: string;
}

export interface FormRouteAuthorityReport {
  readonly current: FormRouteAuthorityArtifact | null;
  readonly history: ReadonlyArray<FormRouteAuthorityArtifact>;
  readonly summary: {
    readonly routeId: string | null;
    readonly href: string | null;
    readonly surfaceId: string | null;
    readonly continuity: RouteFormsAuthorityArtifact["continuity"] | null;
    readonly handoff: FormRouteAuthorityHandoffArtifact | null;
    readonly draftContinuity: FormRouteAuthorityDraftContinuityArtifact | null;
    readonly authorityAvailable: boolean;
    readonly continuityApplied: FormRouteAuthorityArtifact["continuityApplied"] | null;
    readonly transitionKind: FormRouteAuthorityArtifact["transitionKind"] | null;
    readonly previousAuthorityDigest: string | null;
    readonly previousDraftDigest: string | null;
    readonly nextDraftDigest: string | null;
  };
  readonly counters: {
    readonly costBasis: "routeAuthorityHistoryScan";
    readonly incrementalStatus: "notIncremental";
    readonly updates: number;
    readonly clearUpdates: number;
    readonly initialReports: number;
    readonly changedReports: number;
    readonly refreshedReports: number;
    readonly clearedTransitions: number;
    readonly redundantClears: number;
    readonly preservedDraftUpdates: number;
    readonly frozenDraftUpdates: number;
    readonly discardedDraftUpdates: number;
    readonly deferredDraftUpdates: number;
    readonly preserveUpdates: number;
    readonly freezeUpdates: number;
    readonly discardUpdates: number;
    readonly deferUpdates: number;
  };
  readonly digest: string;
}
