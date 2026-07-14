import { createRouteDeclaration } from "./router_declaration.js";
import { createHashNamespace, createSearchNamespace } from "./router_fields.js";
import { isRouteLocation } from "./router_location.js";
import { createRoutePrerequisiteDeclaration } from "./projection/admission/router_admission_declaration.js";
import { createRouteAdmissionSourceNamespace } from "./projection/admission/router_admission_source_declaration.js";
import { createRouteFormsAuthorityDeclaration } from "./projection/admission/router_forms_authority_declaration.js";
import {
  createRouteBreadcrumbDeclaration,
  createRouteBreadcrumbEntryDeclaration,
  createRouteBreadcrumbParentDeclaration,
  createRouteBreadcrumbTrailDeclaration,
} from "./projection/breadcrumb/router_breadcrumb_declaration.js";
import {
  createCarriedBreadcrumbsArtifact,
  createRestoredBreadcrumbsArtifact,
} from "./projection/breadcrumb/router_breadcrumb_provenance.js";
import { createBrowserAuthorityCoherenceNamespace } from "./projection/ingress/router_browser_authority_coherence.js";
import { createBrowserHistoryNamespace } from "./projection/ingress/router_browser_history_ingress.js";
import { createHydrationNamespace } from "./projection/ingress/router_hydration_handoff.js";
import { createBrowserHistoryStory } from "./projection/ingress/router_browser_history_story.js";
import { createRouteRestoreBoundary } from "./projection/ingress/router_restore_boundary.js";
import { createWarmupIngressNamespace } from "./projection/ingress/router_warmup_ingress.js";
import { createBrowserHistoryWritebackNamespace } from "./projection/ingress/router_browser_history_writeback.js";
import { defineRoutes } from "./projection/router_definition.js";
import { createRouteLayoutDeclaration } from "./projection/router_layout_declaration.js";
import { createRouteResourceDeclaration } from "./projection/router_resource_declaration.js";
import { createRouteRecoveryDeclaration } from "./projection/recovery/router_recovery_declaration.js";
import {
  createCanonicalUrlAuthority,
  createRawLocationAuthority,
  isCanonicalUrlAuthority,
  isRawLocationAuthority,
} from "./url_authority/router_url_authority.js";

function createRouterNamespace(scopeId = null) {
  const browserHistory = createBrowserHistoryNamespace();
  const createStory = (initialReport) => createBrowserHistoryStory(initialReport);
  const carryBreadcrumbs = (trail) => createCarriedBreadcrumbsArtifact(trail);
  return Object.freeze({
    search: createSearchNamespace(),
    hash: createHashNamespace(),
    browserHistory: Object.freeze({
      ...browserHistory,
      story(initialReport) {
        return createStory(initialReport);
      },
      coherence: createBrowserAuthorityCoherenceNamespace(),
      writeback: createBrowserHistoryWritebackNamespace(),
    }),
    hydration: createHydrationNamespace(),
    warmup: createWarmupIngressNamespace(),
    host: createRouteAdmissionSourceNamespace("hostCapability"),
    resource: createRouteAdmissionSourceNamespace("resourceTruth"),
    graph: createRouteAdmissionSourceNamespace("graphTruth"),
    raw(href, options = {}) {
      return createRawLocationAuthority(href, options);
    },
    canonical(href, options = {}) {
      return createCanonicalUrlAuthority(href, options);
    },
    route(route, options = {}) {
      return createRouteDeclaration(route, options);
    },
    resourceLine(resourceFamily, options = {}) {
      return createRouteResourceDeclaration(resourceFamily, options);
    },
    forms(surfaceId, options = {}) {
      return createRouteFormsAuthorityDeclaration(surfaceId, options);
    },
    breadcrumb(options) {
      return createRouteBreadcrumbDeclaration(options);
    },
    breadcrumbEntry(options) {
      return createRouteBreadcrumbEntryDeclaration(options);
    },
    breadcrumbParent(options) {
      return createRouteBreadcrumbParentDeclaration(options);
    },
    breadcrumbTrail(entries) {
      return createRouteBreadcrumbTrailDeclaration(entries);
    },
    carryBreadcrumbs(trail) {
      return carryBreadcrumbs(trail);
    },
    restoreBreadcrumbs(trail) {
      return createRestoredBreadcrumbsArtifact(trail);
    },
    restoreBoundary(snapshotEnvelopeArtifact) {
      return createRouteRestoreBoundary(snapshotEnvelopeArtifact);
    },
    prerequisite(name, evaluate) {
      return createRoutePrerequisiteDeclaration(name, evaluate);
    },
    recovery(name, evaluate) {
      return createRouteRecoveryDeclaration(name, evaluate);
    },
    layout(routeOrDeclaration, optionsOrChildren, maybeChildren) {
      return createRouteLayoutDeclaration(
        routeOrDeclaration,
        optionsOrChildren,
        maybeChildren,
      );
    },
    define(definitions) {
      return defineRoutes(definitions, scopeId, {
        browserHistory: Object.freeze({
          ...browserHistory,
          story: createStory,
        }),
        carryBreadcrumbs,
      });
    },
    isRouteLocation(value) {
      return isRouteLocation(value);
    },
    isRawLocationAuthority(value) {
      return isRawLocationAuthority(value);
    },
    isCanonicalUrlAuthority(value) {
      return isCanonicalUrlAuthority(value);
    },
  });
}

export { createRouterNamespace };
