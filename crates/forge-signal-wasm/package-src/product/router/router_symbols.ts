const ROUTE_DECLARATION = Symbol("forge.router.declaration");
const ROUTE_BREADCRUMB_DECLARATION = Symbol("forge.router.breadcrumb-declaration");
const ROUTE_BREADCRUMB_PARENT_DECLARATION = Symbol("forge.router.breadcrumb-parent-declaration");
const ROUTE_BREADCRUMB_ENTRY_DECLARATION = Symbol("forge.router.breadcrumb-entry-declaration");
const ROUTE_BREADCRUMB_TRAIL_DECLARATION = Symbol("forge.router.breadcrumb-trail-declaration");
const ROUTE_CARRIED_BREADCRUMBS = Symbol("forge.router.carried-breadcrumbs");
const ROUTE_RESTORED_BREADCRUMBS = Symbol("forge.router.restored-breadcrumbs");
const ROUTE_RESOURCE_DECLARATION = Symbol("forge.router.resource-declaration");
const ROUTE_PREREQUISITE_DECLARATION = Symbol("forge.router.prerequisite-declaration");
const ROUTE_ADMISSION_SOURCE_DECLARATION = Symbol("forge.router.admission-source-declaration");
const ROUTE_RECOVERY_DECLARATION = Symbol("forge.router.recovery-declaration");
const ROUTE_FORMS_AUTHORITY_DECLARATION = Symbol("forge.router.forms-authority-declaration");
const ROUTE_FORMS_AUTHORITY = Symbol("forge.router.forms-authority");
const ROUTE_LAYOUT_DECLARATION = Symbol("forge.router.layout-declaration");
const ROUTE_LAYOUT_REFERENCE = Symbol("forge.router.layout-reference");
const ROUTE_REFERENCE = Symbol("forge.router.reference");
const ROUTE_LOCATION = Symbol("forge.router.location");
const ROUTE_PROJECTED_CAPABILITY = Symbol("forge.router.projected-capability");
const ROUTE_LAYOUT_PLACEMENT = Symbol("forge.router.layout-placement");
const ROUTE_OUTLET_CONTRACT = Symbol("forge.router.outlet-contract");
const ROUTE_PROJECTED_CANDIDATE = Symbol("forge.router.projected-candidate");
const ROUTE_PREFETCH_ARTIFACT = Symbol("forge.router.prefetch-artifact");
const ROUTE_SPECULATIVE_BRANCH_PLAN = Symbol("forge.router.speculative-branch-plan");
const ROUTE_ADMISSION_PLAN = Symbol("forge.router.admission-plan");
const ROUTE_ADMITTED_CAPABILITY = Symbol("forge.router.admitted-capability");
const ROUTE_OUTCOME = Symbol("forge.router.outcome");
const ROUTE_TRANSITION_ARTIFACT = Symbol("forge.router.transition-artifact");
const ROUTE_TREE_ROOT = Symbol("forge.router.tree-root");
const RAW_LOCATION_AUTHORITY = Symbol("forge.router.raw-location-authority");
const CANONICAL_URL_AUTHORITY = Symbol("forge.router.canonical-url-authority");

export {
  CANONICAL_URL_AUTHORITY,
  RAW_LOCATION_AUTHORITY,
  ROUTE_ADMISSION_PLAN,
  ROUTE_ADMISSION_SOURCE_DECLARATION,
  ROUTE_ADMITTED_CAPABILITY,
  ROUTE_BREADCRUMB_DECLARATION,
  ROUTE_BREADCRUMB_ENTRY_DECLARATION,
  ROUTE_BREADCRUMB_PARENT_DECLARATION,
  ROUTE_BREADCRUMB_TRAIL_DECLARATION,
  ROUTE_CARRIED_BREADCRUMBS,
  ROUTE_RESTORED_BREADCRUMBS,
  ROUTE_DECLARATION,
  ROUTE_RESOURCE_DECLARATION,
  ROUTE_FORMS_AUTHORITY,
  ROUTE_FORMS_AUTHORITY_DECLARATION,
  ROUTE_LAYOUT_DECLARATION,
  ROUTE_LAYOUT_PLACEMENT,
  ROUTE_LAYOUT_REFERENCE,
  ROUTE_LOCATION,
  ROUTE_OUTCOME,
  ROUTE_PREFETCH_ARTIFACT,
  ROUTE_PREREQUISITE_DECLARATION,
  ROUTE_RECOVERY_DECLARATION,
  ROUTE_OUTLET_CONTRACT,
  ROUTE_PROJECTED_CANDIDATE,
  ROUTE_PROJECTED_CAPABILITY,
  ROUTE_REFERENCE,
  ROUTE_SPECULATIVE_BRANCH_PLAN,
  ROUTE_TRANSITION_ARTIFACT,
  ROUTE_TREE_ROOT,
};
