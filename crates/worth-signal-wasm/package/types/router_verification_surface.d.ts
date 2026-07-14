declare const WorthSignalRouteVerificationPackageBrand: unique symbol;
declare const WorthSignalCanonicalVerificationPackageBrand: unique symbol;
declare const WorthSignalNavigationIntentVerificationPackageBrand: unique symbol;
declare const WorthSignalNavigationPlanVerificationPackageBrand: unique symbol;

export interface RouteReferenceVerificationPackage {
  readonly routeId: string;
  readonly routeSchemaDigest: string;
  readonly routeDeclarationDigest: string;
  readonly routeReferenceDigest: string;
  readonly [WorthSignalRouteVerificationPackageBrand]: "routeReferenceVerificationPackage";
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
  readonly [WorthSignalCanonicalVerificationPackageBrand]: "canonicalRouteVerificationPackage";
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
  readonly [WorthSignalNavigationIntentVerificationPackageBrand]: "navigationIntentVerificationPackage";
}

export interface NavigationPlanVerificationPackage extends NavigationIntentVerificationPackage {
  readonly navigationPlanDigest: string;
  readonly navigationExplainabilityDigest: string;
  readonly navigationFreshnessDigest: string;
  readonly navigationContinuityAttributionDigest: string;
  readonly [WorthSignalNavigationPlanVerificationPackageBrand]: "navigationPlanVerificationPackage";
}
