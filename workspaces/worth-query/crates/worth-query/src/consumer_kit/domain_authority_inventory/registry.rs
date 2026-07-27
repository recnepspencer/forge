use super::{
    WorthQueryDomainAuthorityClass as Class, WorthQueryDomainAuthorityInventoryRow as Row,
};

const DOMAIN_IDENTITY: &str = "src/domain_installation/package_authority/identity.rs";
const DOMAIN_INVARIANTS: &str =
    "src/domain_installation/package_authority/package_definitions/invariant.rs";
const DOMAIN_OBLIGATIONS: &str =
    "src/domain_installation/package_authority/package_definitions/graph_obligation.rs";
const DOMAIN_OPERATIONS: &str =
    "src/domain_installation/package_authority/package_definitions/graph_read_operation.rs";
const DOMAIN_DECLARATION_FAMILIES: &str =
    "src/domain_installation/package_authority/package_definitions/declaration_family.rs";
const DOMAIN_PACKAGE: &str = "src/domain_installation/package_authority/package.rs";
const INSTALLED_HANDLE: &str = "src/domain_installation/installed_authority/handle.rs";
const BUILDER_DOMAIN_PACKAGES: &str = "src/runtime/builder/domain_packages.rs";
const BUILDER_GRAPH_PARTICIPATION: &str = "src/runtime/builder/graph_participation.rs";
const RUNTIME_DOMAIN_API: &str = "src/runtime/domain_installation_api.rs";
const WORKSPACE_DOMAIN_API: &str = "src/runtime/workspace_domain_installation.rs";
const DOMAIN_FACADE_EXPORTS: &str = "src/facade/exports_domain.rs";
const CONTRIBUTION_SURFACE: &str = "src/domain_capabilities/dx/common/root.rs";
const OPERATION_REGISTRY: &str = "src/runtime/graph_read_access/operation_resolution/registry.rs";

pub fn worth_query_domain_authority_inventory_rows() -> &'static [Row] {
    CORE_ROWS
}

const CORE_ROWS: &[Row] = &[
    package_type("WorthQueryDomainIdentityNamespace", DOMAIN_IDENTITY),
    package_type("WorthQueryDomainIdentityName", DOMAIN_IDENTITY),
    package_type("WorthQueryDomainSemanticVersion", DOMAIN_IDENTITY),
    package_type("WorthQueryDomainIdentityDeclaration", DOMAIN_IDENTITY),
    package_type("WorthQueryDomainInvariantDefinition", DOMAIN_INVARIANTS),
    package_type(
        "WorthQueryDomainGraphObligationDefinition",
        DOMAIN_OBLIGATIONS,
    ),
    package_type(
        "WorthQueryDomainGraphReadOperationDefinition",
        DOMAIN_OPERATIONS,
    ),
    package_type(
        "WorthQueryDomainDeclarationFamilyDefinition",
        DOMAIN_DECLARATION_FAMILIES,
    ),
    package_type("WorthQueryDomainPackage", DOMAIN_PACKAGE),
    package_input("WorthQueryDomainIdentityNamespace::new", DOMAIN_IDENTITY),
    package_input("WorthQueryDomainIdentityName::new", DOMAIN_IDENTITY),
    package_input("WorthQueryDomainSemanticVersion::new", DOMAIN_IDENTITY),
    package_input("WorthQueryDomainIdentityDeclaration::new", DOMAIN_IDENTITY),
    package_input(
        "WorthQueryDomainInvariantPredicate::requires_outgoing_relations",
        DOMAIN_INVARIANTS,
    ),
    package_input(
        "WorthQueryDomainInvariantDefinition::new",
        DOMAIN_INVARIANTS,
    ),
    package_input(
        "WorthQueryDomainGraphObligationDefinition::new",
        DOMAIN_OBLIGATIONS,
    ),
    package_input(
        "WorthQueryDomainGraphObligationDefinition::with_support_posture",
        DOMAIN_OBLIGATIONS,
    ),
    package_input(
        "WorthQueryDomainGraphReadOperationDefinition::new",
        DOMAIN_OPERATIONS,
    ),
    package_input(
        "WorthQueryDomainGraphReadOperationDefinition::accepts_relation",
        DOMAIN_OPERATIONS,
    ),
    package_input(
        "WorthQueryDomainGraphReadOperationDefinition::lowers_to",
        DOMAIN_OPERATIONS,
    ),
    package_input(
        "WorthQueryDomainGraphReadOperationDefinition::requires_support_family",
        DOMAIN_OPERATIONS,
    ),
    package_input(
        "WorthQueryDomainDeclarationFamilyDefinition::from_marker",
        DOMAIN_DECLARATION_FAMILIES,
    ),
    package_input("WorthQueryDomainPackage::declare", DOMAIN_PACKAGE),
    package_input(
        "WorthQueryDomainPackage::requires_capability",
        DOMAIN_PACKAGE,
    ),
    package_input(
        "WorthQueryDomainPackage::requires_configuration",
        DOMAIN_PACKAGE,
    ),
    package_input(
        "WorthQueryDomainPackage::requires_operating_posture",
        DOMAIN_PACKAGE,
    ),
    package_input("WorthQueryDomainPackage::invariant", DOMAIN_PACKAGE),
    package_input("WorthQueryDomainPackage::graph_obligation", DOMAIN_PACKAGE),
    package_input(
        "WorthQueryDomainPackage::graph_read_operation",
        DOMAIN_PACKAGE,
    ),
    package_input(
        "WorthQueryDomainPackage::declaration_family",
        DOMAIN_PACKAGE,
    ),
    package_input(
        "WorthQueryDomainPackage::declaration_families",
        DOMAIN_PACKAGE,
    ),
    package_input(
        "WorthQueryDomainPackage::permits_contribution",
        DOMAIN_PACKAGE,
    ),
    Row::new(
        "WorthQueryRuntimeBuilder::domain_package",
        BUILDER_DOMAIN_PACKAGES,
        None,
        Class::CanonicalInstallation,
        Class::CanonicalInstallation,
        "runtime-domain-installation-registry",
    ),
    provider_capability("WorthQueryRuntimeBuilder::session_graph_participation_provider"),
    provider_capability("WorthQueryRuntimeBuilder::decision_graph_participation_provider"),
    provider_capability("WorthQueryRuntimeBuilder::provisional_graph_participation_provider"),
    provider_capability("WorthQueryRuntimeBuilder::invariant_graph_participation_provider"),
    provider_capability("WorthQueryRuntimeBuilder::atomic_invariant_graph_participation_provider"),
    provider_capability(
        "WorthQueryRuntimeBuilder::convergent_invariant_graph_participation_provider",
    ),
    provider_capability(
        "WorthQueryRuntimeBuilder::atomic_convergent_invariant_graph_participation_provider",
    ),
    Row::new(
        "WorthQueryGraphReadOperationRegistry",
        OPERATION_REGISTRY,
        None,
        Class::DerivedIndex,
        Class::DerivedIndex,
        "installed-domain-execution-index",
    ),
    installed_handle_type("WorthQueryInstalledDomainHandle", INSTALLED_HANDLE),
    installed_handle_type(
        "WorthQueryInstalledDomainContributionSurface",
        CONTRIBUTION_SURFACE,
    ),
    installed_handle(
        "WorthQueryRuntime::domain",
        RUNTIME_DOMAIN_API,
        "runtime-domain-installation-registry",
    ),
    installed_handle(
        "WorthQueryWorkspace::domain",
        WORKSPACE_DOMAIN_API,
        "runtime-domain-installation-registry",
    ),
    installed_handle(
        "WorthQueryInstalledDomainHandle::contributions",
        INSTALLED_HANDLE,
        "installed-domain-handle",
    ),
    installed_handle(
        "WorthQueryInstalledDomainHandle::contributions_in",
        INSTALLED_HANDLE,
        "installed-domain-handle",
    ),
    installed_handle(
        "WorthQueryInstalledDomainHandle::authority_witness",
        INSTALLED_HANDLE,
        "installed-domain-handle",
    ),
    installed_handle(
        "WorthQueryInstalledDomainHandle::rebind_request",
        INSTALLED_HANDLE,
        "installed-domain-handle",
    ),
    installed_handle(
        "WorthQueryInstalledDomainHandle::graph_read_operation",
        INSTALLED_HANDLE,
        "installed-domain-handle",
    ),
    installed_handle(
        "WorthQueryInstalledDomainHandle::declarations",
        INSTALLED_HANDLE,
        "installed-domain-handle",
    ),
    installed_handle(
        "WorthQueryInstalledDomainHandle::declarations_in",
        INSTALLED_HANDLE,
        "installed-domain-handle",
    ),
    installed_handle(
        "WorthQueryInstalledDomainContributionSurface::intent_target",
        CONTRIBUTION_SURFACE,
        "installed-domain-handle",
    ),
    installed_handle(
        "WorthQueryInstalledDomainContributionSurface::for_intent",
        CONTRIBUTION_SURFACE,
        "installed-domain-handle",
    ),
    installed_handle(
        "WorthQueryInstalledDomainContributionSurface::for_intent_target",
        CONTRIBUTION_SURFACE,
        "installed-domain-handle",
    ),
    installed_handle(
        "WorthQueryInstalledDomainContributionSurface::admitted_plan_target",
        CONTRIBUTION_SURFACE,
        "installed-domain-handle",
    ),
    installed_handle(
        "WorthQueryInstalledDomainContributionSurface::for_admitted_intent_plan",
        CONTRIBUTION_SURFACE,
        "installed-domain-handle",
    ),
    installed_handle(
        "WorthQueryInstalledDomainContributionSurface::for_admitted_plan_target",
        CONTRIBUTION_SURFACE,
        "installed-domain-handle",
    ),
    installed_handle(
        "WorthQueryInstalledDomainContributionSurface::lower_runtime_target",
        CONTRIBUTION_SURFACE,
        "installed-domain-handle",
    ),
    installed_handle(
        "WorthQueryInstalledDomainContributionSurface::for_lower_runtime_boundary_envelope",
        CONTRIBUTION_SURFACE,
        "installed-domain-handle",
    ),
    installed_handle(
        "WorthQueryInstalledDomainContributionSurface::for_lower_runtime_target",
        CONTRIBUTION_SURFACE,
        "installed-domain-handle",
    ),
    installed_handle(
        "WorthQueryInstalledDomainContributionSurface::for_lower_runtime_boundary_source",
        CONTRIBUTION_SURFACE,
        "installed-domain-handle",
    ),
];

const fn package_type(symbol: &'static str, path: &'static str) -> Row {
    Row::new(
        symbol,
        path,
        Some(DOMAIN_FACADE_EXPORTS),
        Class::PackageInput,
        Class::PackageInput,
        "domain-package",
    )
}

const fn package_input(symbol: &'static str, path: &'static str) -> Row {
    Row::new(
        symbol,
        path,
        None,
        Class::PackageInput,
        Class::PackageInput,
        "domain-package",
    )
}

const fn installed_handle_type(symbol: &'static str, path: &'static str) -> Row {
    Row::new(
        symbol,
        path,
        Some(DOMAIN_FACADE_EXPORTS),
        Class::InstalledHandleCapability,
        Class::InstalledHandleCapability,
        "installed-domain-handle",
    )
}

const fn installed_handle(symbol: &'static str, path: &'static str, owner: &'static str) -> Row {
    Row::new(
        symbol,
        path,
        None,
        Class::InstalledHandleCapability,
        Class::InstalledHandleCapability,
        owner,
    )
}

const fn provider_capability(symbol: &'static str) -> Row {
    Row::new(
        symbol,
        BUILDER_GRAPH_PARTICIPATION,
        None,
        Class::CanonicalInstallation,
        Class::CanonicalInstallation,
        "runtime-graph-provider-registry",
    )
}
