export interface RouteRequestIdentity {
  activeTarget: string;
  deviationGranted: boolean;
  effectiveRevision: string;
  navigationNonce: number;
  role: string;
}

export interface RoutePresentation<Report, PageLine> {
  pageLine: PageLine | null;
  report: Report;
  requestKey: string;
}

export function routeRequestKey(identity: RouteRequestIdentity): string {
  return JSON.stringify([
    identity.activeTarget,
    identity.role,
    identity.effectiveRevision,
    identity.deviationGranted,
    identity.navigationNonce,
  ]);
}

export function currentRoutePresentation<Report, PageLine>(
  presentation: RoutePresentation<Report, PageLine> | null,
  requestKey: string,
): RoutePresentation<Report, PageLine> | null {
  return presentation?.requestKey === requestKey ? presentation : null;
}
