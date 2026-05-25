declare const forgeSignalRouteVerificationPackageBrand: unique symbol;
declare const forgeSignalCanonicalVerificationPackageBrand: unique symbol;
declare const forgeSignalNavigationIntentVerificationPackageBrand: unique symbol;
declare const forgeSignalNavigationPlanVerificationPackageBrand: unique symbol;

export interface RouteReferenceVerificationPackage {
  readonly routeId: string;
  readonly routeSchemaDigest: string;
  readonly routeDeclarationDigest: string;
  readonly routeReferenceDigest: string;
  readonly [forgeSignalRouteVerificationPackageBrand]: "routeReferenceVerificationPackage";
}

export interface CanonicalRouteVerificationPackage {
  readonly routeId: string;
  readonly routeSchemaDigest: string;
  readonly routeDeclarationDigest: string;
  readonly routeReferenceDigest: string;
  readonly canonicalUrlDigest: string;
  readonly equivalenceDigest: string;
  readonly searchDigest: string;
  readonly hashDigest: string;
  readonly [forgeSignalCanonicalVerificationPackageBrand]: "canonicalRouteVerificationPackage";
}

export interface NavigationIntentVerificationPackage {
  readonly routeId: string;
  readonly routeSchemaDigest: string;
  readonly routeDeclarationDigest: string;
  readonly routeReferenceDigest: string;
  readonly canonicalUrlDigest: string;
  readonly equivalenceDigest: string;
  readonly navigationIntentDigest: string;
  readonly navigationPolicyDigest: string;
  readonly navigationTransitionPolicyDigest: string;
  readonly navigationFreshnessPolicyDigest: string;
  readonly navigationHistoryEffectDigest: string;
  readonly navigationExecutionContractDigest: string;
  readonly [forgeSignalNavigationIntentVerificationPackageBrand]: "navigationIntentVerificationPackage";
}

export interface NavigationPlanVerificationPackage extends NavigationIntentVerificationPackage {
  readonly navigationPlanDigest: string;
  readonly navigationExplainabilityDigest: string;
  readonly navigationFreshnessDigest: string;
  readonly navigationContinuityAttributionDigest: string;
  readonly [forgeSignalNavigationPlanVerificationPackageBrand]: "navigationPlanVerificationPackage";
}
