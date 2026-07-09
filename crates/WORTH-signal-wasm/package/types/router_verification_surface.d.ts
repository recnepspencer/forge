declare const WORTHSignalRouteVerificationPackageBrand: unique symbol;
declare const WORTHSignalCanonicalVerificationPackageBrand: unique symbol;
declare const WORTHSignalNavigationIntentVerificationPackageBrand: unique symbol;
declare const WORTHSignalNavigationPlanVerificationPackageBrand: unique symbol;

export interface RouteReferenceVerificationPackage {
  readonly routeId: string;
  readonly routeSchemaDigest: string;
  readonly routeDeclarationDigest: string;
  readonly routeReferenceDigest: string;
  readonly [WORTHSignalRouteVerificationPackageBrand]: "routeReferenceVerificationPackage";
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
  readonly [WORTHSignalCanonicalVerificationPackageBrand]: "canonicalRouteVerificationPackage";
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
  readonly [WORTHSignalNavigationIntentVerificationPackageBrand]: "navigationIntentVerificationPackage";
}

export interface NavigationPlanVerificationPackage extends NavigationIntentVerificationPackage {
  readonly navigationPlanDigest: string;
  readonly navigationExplainabilityDigest: string;
  readonly navigationFreshnessDigest: string;
  readonly navigationContinuityAttributionDigest: string;
  readonly [WORTHSignalNavigationPlanVerificationPackageBrand]: "navigationPlanVerificationPackage";
}
