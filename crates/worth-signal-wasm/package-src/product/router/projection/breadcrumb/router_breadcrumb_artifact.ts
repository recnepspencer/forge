import { createCanonicalDigest } from "../../url_authority/router_verification_packages.js";
import {
  createRouteBreadcrumbProvenanceArtifact,
} from "./router_breadcrumb_entry_provenance.js";

function createRouteBreadcrumbEntry(routeCapability, breadcrumbDeclaration) {
  if (breadcrumbDeclaration === null) {
    return null;
  }
  return createBreadcrumbEntryArtifact(
    routeCapability,
    breadcrumbDeclaration,
    "resolved",
    "routeDeclaration",
  );
}

function createRouteBreadcrumbTrail(routeCapability, breadcrumbDeclaration) {
  const options =
    arguments.length > 2 && arguments[2] !== undefined ? arguments[2] : {};
  if (breadcrumbDeclaration === null) {
    return null;
  }
  const ownEntry = createRouteBreadcrumbEntry(routeCapability, breadcrumbDeclaration);
  const parentTrail = resolveParentTrail(
    routeCapability,
    breadcrumbDeclaration.parent,
    options,
  );
  return createRouteBreadcrumbTrailArtifact([
    ...(parentTrail?.entries ?? []),
    ownEntry,
  ]);
}

function createHistoryFallbackBreadcrumbTrail(historyEntries) {
  return createRouteBreadcrumbTrailArtifact(
    historyEntries.map((entry) => {
      const verification = Object.freeze({
        breadcrumbEntryDigest: createCanonicalDigest("route-breadcrumb-entry", {
          crumbId: `history:${entry.routeId}`,
          routeId: entry.routeId,
          href: entry.href,
          label: entry.routeId,
          status: "fallback",
          sourceKind: "historyFallback",
          targetKind: "routeHref",
          targetHref: entry.href,
        }),
      });
      return Object.freeze({
        kind: "routeBreadcrumbEntry",
        crumbId: `history:${entry.routeId}`,
        routeId: entry.routeId,
        href: entry.href,
        label: entry.routeId,
        status: "fallback",
        sourceKind: "historyFallback",
        targetKind: "routeHref",
        targetHref: entry.href,
        restoreBoundary() {
          return entry.restoreBoundary?.() ?? null;
        },
        restore(history) {
          return entry.restore(history);
        },
        replay(history) {
          return entry.replay(history);
        },
        provenance() {
          return createRouteBreadcrumbProvenanceArtifact(this);
        },
        verification() {
          return verification;
        },
      });
    }),
  );
}

function resolveParentTrail(routeCapability, parentDeclaration, options) {
  if (parentDeclaration === null) {
    return null;
  }
  if (parentDeclaration.recompute !== null) {
    const recomputed = parentDeclaration.recompute(createBreadcrumbContext(routeCapability));
    const trail = normalizeParentResult(routeCapability, recomputed, "recomputed");
    if (trail !== null) {
      return trail;
    }
  }
  if (
    parentDeclaration.carry === true
    && options.restoredBreadcrumbs !== null
    && options.restoredBreadcrumbs !== undefined
  ) {
    return createRestoredBreadcrumbTrail(options.restoredBreadcrumbs.entries);
  }
  if (parentDeclaration.carry === true && options.carriedBreadcrumbs !== null && options.carriedBreadcrumbs !== undefined) {
    return createCarriedBreadcrumbTrail(options.carriedBreadcrumbs.entries);
  }
  if (parentDeclaration.fallback !== null) {
    return normalizeParentResult(routeCapability, parentDeclaration.fallback, "fallback");
  }
  return null;
}

function normalizeParentResult(routeCapability, result, sourceKind) {
  if (result === null || result === undefined) {
    return null;
  }
  if (result.entries !== undefined && Array.isArray(result.entries)) {
    return createRouteBreadcrumbTrailArtifact(
      result.entries.map((entry) => createBreadcrumbEntryArtifact(
        routeCapability,
        entry,
        sourceKind === "recomputed" ? "recomputed" : "fallback",
        sourceKind,
      )),
    );
  }
  if (result.id !== undefined) {
    return createRouteBreadcrumbTrailArtifact([
      createBreadcrumbEntryArtifact(
        routeCapability,
        result,
        sourceKind === "recomputed" ? "recomputed" : "fallback",
        sourceKind,
      ),
    ]);
  }
  throw new TypeError(
    "route breadcrumb parent recompute/fallback must return breadcrumbEntry(...), breadcrumbTrail(...), or null",
  );
}

function createBreadcrumbEntryArtifact(routeCapability, breadcrumbDeclaration, status, sourceKind) {
  const label = resolveLabel(routeCapability, breadcrumbDeclaration.label);
  const target = resolveTarget(routeCapability, breadcrumbDeclaration.target, status === "resolved");
  const verification = Object.freeze({
    breadcrumbEntryDigest: createCanonicalDigest("route-breadcrumb-entry", {
      crumbId: breadcrumbDeclaration.id,
      routeId: routeCapability.routeId,
      href: routeCapability.href,
      label,
      status,
      sourceKind,
      targetKind: target.targetKind,
      targetHref: target.targetHref,
    }),
  });
  return Object.freeze({
    kind: "routeBreadcrumbEntry",
    crumbId: breadcrumbDeclaration.id,
    routeId: routeCapability.routeId,
    href: routeCapability.href,
    label,
    status,
    sourceKind,
    targetKind: target.targetKind,
    targetHref: target.targetHref,
    provenance() {
      return createRouteBreadcrumbProvenanceArtifact(this);
    },
    verification() {
      return verification;
    },
  });
}

function createCarriedBreadcrumbTrail(entries) {
  return createRouteBreadcrumbTrailArtifact(entries.map((entry) => {
    const verification = Object.freeze({
      breadcrumbEntryDigest: createCanonicalDigest("route-breadcrumb-entry", {
        crumbId: entry.crumbId,
        routeId: entry.routeId,
        href: entry.href,
        label: entry.label,
        status: "carried",
        sourceKind: "carriedProvenance",
        targetKind: entry.targetKind,
        targetHref: entry.targetHref,
        carriedFromDigest: entry.verification().breadcrumbEntryDigest,
      }),
    });
    return Object.freeze({
      kind: "routeBreadcrumbEntry",
      crumbId: entry.crumbId,
      routeId: entry.routeId,
      href: entry.href,
      label: entry.label,
      status: "carried",
      sourceKind: "carriedProvenance",
      targetKind: entry.targetKind,
      targetHref: entry.targetHref,
      restoreBoundary() {
        return typeof entry.restoreBoundary === "function"
          ? entry.restoreBoundary()
          : null;
      },
      restore(history) {
        if (typeof entry.restore === "function") {
          return entry.restore(history);
        }
        throw new TypeError(
          "browserHistoryBreadcrumbEntry.restore(...) requires restore-backed breadcrumb provenance",
        );
      },
      replay(history) {
        if (typeof entry.replay === "function") {
          return entry.replay(history);
        }
        throw new TypeError(
          "browserHistoryBreadcrumbEntry.replay(...) requires replay-backed breadcrumb provenance",
        );
      },
      provenance() {
        return createRouteBreadcrumbProvenanceArtifact(this);
      },
      verification() {
        return verification;
      },
    });
  }));
}

function createRestoredBreadcrumbTrail(entries) {
  return createRouteBreadcrumbTrailArtifact(entries.map((entry) => {
    const verification = Object.freeze({
      breadcrumbEntryDigest: createCanonicalDigest("route-breadcrumb-entry", {
        crumbId: entry.crumbId,
        routeId: entry.routeId,
        href: entry.href,
        label: entry.label,
        status: "restored",
        sourceKind: "restoredProvenance",
        targetKind: entry.targetKind,
        targetHref: entry.targetHref,
        restoredFromDigest: entry.verification().breadcrumbEntryDigest,
        restoreBoundaryDigest: entry.restoreBoundary().verification().routeRestoreBoundaryDigest,
      }),
    });
    return Object.freeze({
      kind: "routeBreadcrumbEntry",
      crumbId: entry.crumbId,
      routeId: entry.routeId,
      href: entry.href,
      label: entry.label,
      status: "restored",
      sourceKind: "restoredProvenance",
      targetKind: entry.targetKind,
      targetHref: entry.targetHref,
      restoreBoundary() {
        return entry.restoreBoundary();
      },
      restore(history) {
        return entry.restore(history);
      },
      replay(history) {
        if (typeof entry.replay === "function") {
          return entry.replay(history);
        }
        throw new TypeError(
          "browserHistoryBreadcrumbEntry.replay(...) requires replay-backed breadcrumb provenance",
        );
      },
      provenance() {
        return createRouteBreadcrumbProvenanceArtifact(this);
      },
      verification() {
        return verification;
      },
    });
  }));
}

function createRouteBreadcrumbTrailArtifact(entries) {
  const frozenEntries = Object.freeze(entries.slice());
  const verification = Object.freeze({
    breadcrumbTrailDigest: createCanonicalDigest("route-breadcrumb-trail", {
      entryDigests: frozenEntries.map((entry) => entry.verification().breadcrumbEntryDigest),
    }),
  });
  return Object.freeze({
    kind: "routeBreadcrumbTrail",
    entries: frozenEntries,
    verification() {
      return verification;
    },
  });
}

function createBreadcrumbContext(routeCapability) {
  return Object.freeze({
    routeId: routeCapability.routeId,
    href: routeCapability.href,
    params: routeCapability.params,
    search: routeCapability.search,
    hash: routeCapability.hash,
    descriptor() {
      return routeCapability.descriptor();
    },
    canonical() {
      return routeCapability.canonical();
    },
  });
}

function resolveLabel(routeCapability, labelSource) {
  const label = typeof labelSource === "function"
    ? labelSource(createBreadcrumbContext(routeCapability))
    : labelSource;
  if (typeof label !== "string" || label.trim().length === 0) {
    throw new TypeError("route breadcrumb label must resolve to a non-empty string");
  }
  return label;
}

function resolveTarget(routeCapability, targetSource, defaultToRouteHref) {
  const resolved = typeof targetSource === "function"
    ? targetSource(createBreadcrumbContext(routeCapability))
    : targetSource;
  if (resolved === null || resolved === undefined) {
    if (defaultToRouteHref) {
      return Object.freeze({
        targetKind: "routeHref",
        targetHref: routeCapability.href,
      });
    }
    return Object.freeze({
      targetKind: "none",
      targetHref: null,
    });
  }
  if (typeof resolved === "string") {
    return Object.freeze({
      targetKind: isExternalHref(resolved) ? "externalHref" : "routeHref",
      targetHref: resolved,
    });
  }
  if (typeof resolved === "object" && typeof resolved.href === "string") {
    return Object.freeze({
      targetKind: "routeHref",
      targetHref: resolved.href,
    });
  }
  throw new TypeError(
    "route breadcrumb target must resolve to a string href, route artifact, or null",
  );
}

function isExternalHref(value) {
  return /^[a-z]+:\/\//i.test(value);
}

export {
  createHistoryFallbackBreadcrumbTrail,
  createRouteBreadcrumbEntry,
  createRouteBreadcrumbTrail,
};
