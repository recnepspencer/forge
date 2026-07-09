export type NavigationIntentKind =
  | "push"
  | "replace"
  | "canonicalize"
  | "softRefresh"
  | "sameRouteMutation"
  | "breadcrumbReturn"
  | "restoreBack";

export type NavigationContinuityPolicy =
  | "refresh-immediately"
  | "preserve-visible-while-pending"
  | "preserve-visible-until-explicit-refresh";

export type NavigationProjectionRefreshPolicy =
  | "immediate"
  | "after-admission"
  | "explicit";

export type NavigationArtifactPolicy =
  | "minimal"
  | "diagnostics";

export type NavigationCommitPolicy =
  | "directCommit"
  | "speculativeBranch";

export type NavigationRedirectPolicy =
  | "followRedirect"
  | "surfaceRedirect";

export type NavigationDeployment =
  | "workerFirst"
  | "mainThreadCompatibility";

export interface NavigationPolicy {
  continuity?: NavigationContinuityPolicy;
  projectionRefresh?: NavigationProjectionRefreshPolicy;
  artifactPolicy?: NavigationArtifactPolicy;
  commit?: NavigationCommitPolicy;
  redirect?: NavigationRedirectPolicy;
  deployment?: NavigationDeployment;
}

export interface NavigationIntentOptions {
  kind?: NavigationIntentKind;
  policy?: NavigationPolicy;
}
