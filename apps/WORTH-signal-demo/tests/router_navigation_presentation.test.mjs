import assert from "node:assert/strict";
import test from "node:test";

import {
  currentRoutePresentation,
  routeRequestKey,
} from "../src/ui/routerSectionPresentation.ts";

const request = {
  activeTarget: "/line/overview",
  deviationGranted: false,
  effectiveRevision: "B",
  navigationNonce: 0,
  role: "operator",
};

test("a route value is visible only to the exact request that produced it", () => {
  const requestKey = routeRequestKey(request);
  const presentation = {
    pageLine: { value: "overview" },
    report: { outcome: "admitted" },
    requestKey,
  };

  assert.equal(currentRoutePresentation(presentation, requestKey), presentation);

  for (const changedRequest of [
    { ...request, activeTarget: "/batches/B-2214/record" },
    { ...request, role: "qa" },
    { ...request, effectiveRevision: "C" },
    { ...request, deviationGranted: true },
    { ...request, navigationNonce: 1 },
  ]) {
    assert.equal(
      currentRoutePresentation(presentation, routeRequestKey(changedRequest)),
      null,
    );
  }
});
