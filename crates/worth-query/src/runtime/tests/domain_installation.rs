use super::support::*;
use crate::application::{
    WorthQueryApplicationFacade, WorthQueryCapabilityFamily, WorthQueryConfigSectionFamily,
    WorthQueryDeclarationEntryContributionCategoryFamily,
};
use crate::authoring::{
    AspectFieldSelector, DetailQueryBuilder, DetailResultShapeBuilder, RelationName,
    WorthQueryGraphReadDomainOperationDeclaration,
};
use crate::domain_capabilities::{
    admit_eligible_domain_capability_contribution,
    evaluate_requested_domain_capability_contribution, materialize_canonical_admission_artifact,
    prepare_admitted_domain_capability_contribution_for_materialization,
    WorthQueryAdmissionContributionAuthoring, WorthQueryDomainCapabilityProgressionDenialKind,
};
use crate::domain_installation::{
    WorthQueryDomainGraphReadOperationDefinition, WorthQueryDomainHandleDenialKind,
    WorthQueryDomainIdentityDeclaration, WorthQueryDomainIdentityName,
    WorthQueryDomainIdentityNamespace, WorthQueryDomainInstallationDenialKind,
    WorthQueryDomainPackage, WorthQueryDomainRebindDenialKind, WorthQueryDomainRebindNextAction,
    WorthQueryDomainSemanticVersion,
};
use crate::runtime::{
    WorthQueryIntentDeclaration, WorthQueryIntentInput, WorthQueryReadBuilder,
    WorthQueryReadFamily, WorthQueryRuntime,
};
use crate::schema_view::{QuerySchemaView, SchemaFieldKind, SchemaFieldView, SchemaRelationView};

mod authority;
mod journey;
mod rebind;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct InstalledDomain;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct OtherInstalledDomain;

fn identity<D>() -> WorthQueryDomainIdentityDeclaration<D> {
    identity_version(0)
}

fn identity_version<D>(minor: u32) -> WorthQueryDomainIdentityDeclaration<D> {
    WorthQueryDomainIdentityDeclaration::new(
        WorthQueryDomainIdentityNamespace::new("WORTH.tests").unwrap(),
        WorthQueryDomainIdentityName::new("installed-domain").unwrap(),
        WorthQueryDomainSemanticVersion::new(1, minor),
    )
}

fn package<D>(marker: D) -> WorthQueryDomainPackage<D> {
    WorthQueryDomainPackage::declare(marker, identity())
        .requires_capability(WorthQueryCapabilityFamily::QueryRead)
        .requires_configuration(WorthQueryConfigSectionFamily::Query)
        .graph_read_operation(
            WorthQueryDomainGraphReadOperationDefinition::new(
                WorthQueryDomainIdentityName::new("neighbors").unwrap(),
                1,
            )
            .accepts_relation(RelationName::new("manager").unwrap()),
        )
        .permits_contribution(WorthQueryDeclarationEntryContributionCategoryFamily::Admission)
}

fn admitted<D>(marker: D) -> crate::domain_installation::WorthQueryAdmittedDomainPackage<D> {
    package(marker)
        .validate()
        .unwrap()
        .admit(&WorthQueryApplicationFacade::runtime_backed_default())
        .unwrap()
}

fn installed_runtime() -> WorthQueryRuntime {
    complete_backend_from_parts_builder()
        .domain_package(admitted(InstalledDomain))
        .unwrap()
        .build_backend_from_parts()
        .build()
        .unwrap()
}

fn changed_package_runtime() -> WorthQueryRuntime {
    let changed = WorthQueryDomainPackage::declare(InstalledDomain, identity_version(1))
        .requires_capability(WorthQueryCapabilityFamily::QueryRead)
        .requires_configuration(WorthQueryConfigSectionFamily::Query)
        .permits_contribution(WorthQueryDeclarationEntryContributionCategoryFamily::Admission)
        .validate()
        .unwrap()
        .admit(&WorthQueryApplicationFacade::runtime_backed_default())
        .unwrap();
    complete_backend_from_parts_builder()
        .domain_package(changed)
        .unwrap()
        .build_backend_from_parts()
        .build()
        .unwrap()
}

#[test]
fn equivalent_packages_mint_semantically_equal_but_runtime_affine_handles() {
    let left = installed_runtime();
    let right = installed_runtime();
    let left_handle = left.domain(InstalledDomain).unwrap();
    let right_handle = right.domain(InstalledDomain).unwrap();

    assert_eq!(
        left_handle.package_identity(),
        right_handle.package_identity()
    );
    assert_ne!(
        left_handle.installation_identity(),
        right_handle.installation_identity()
    );
    assert_eq!(
        right
            .validate_installed_domain_handle(&left_handle)
            .unwrap_err()
            .kind(),
        WorthQueryDomainHandleDenialKind::ForeignRuntime
    );
    left.validate_installed_domain_handle(&left_handle).unwrap();
}

#[test]
fn duplicate_semantic_package_denies_before_a_runtime_can_be_published() {
    let builder = complete_backend_from_parts_builder()
        .domain_package(admitted(InstalledDomain))
        .unwrap();
    let denial = builder
        .domain_package(admitted(OtherInstalledDomain))
        .err()
        .expect("equivalent package identity must deny atomically");
    assert_eq!(
        denial.kind(),
        WorthQueryDomainInstallationDenialKind::DuplicatePackageIdentity
    );
}

#[test]
fn ordinary_explanation_and_admission_resolve_installed_operation_by_index() {
    let runtime = installed_runtime();
    let family = installed_operation_family();
    let admission = runtime
        .admit_graph_read_access_for_family(&family)
        .expect("installed operation must resolve without an injected registry");
    assert!(admission.is_admitted());
    let workspace = runtime.workspace("installed-domain-test").unwrap();
    workspace
        .explain_graph_read_access_shape(&family)
        .expect("ordinary explanation must use runtime-installed operations");

    let counters = workspace.runtime.domain_installation_lookup_counters();
    assert_eq!(counters.indexed_operation_lookups(), 2);
    assert_eq!(counters.package_content_scans(), 0);
}

#[test]
fn handle_lookup_is_constant_width_and_installation_is_self_describing() {
    let runtime = installed_runtime();
    let receipt = runtime
        .domain_installation_receipt(InstalledDomain)
        .unwrap();
    assert_eq!(receipt.construction_counters().package_lowerings(), 1);
    assert_eq!(
        receipt
            .construction_counters()
            .graph_read_operation_index_entries(),
        1
    );
    assert!(receipt.warnings().is_empty());
    assert_eq!(receipt.domain_owner(), "WORTH.tests.installed-domain");
    assert_eq!(receipt.definition_counts().graph_read_operations(), 1);

    let rebuild = runtime.verify_domain_execution_index_rebuild();
    assert!(rebuild.is_equivalent());
    assert_eq!(rebuild.operation_count(), 1);

    runtime.domain(InstalledDomain).unwrap();
    runtime.domain(InstalledDomain).unwrap();
    let counters = runtime.domain_installation_lookup_counters();
    assert_eq!(counters.handle_lookups(), 2);
    assert_eq!(counters.package_content_scans(), 0);
}

fn installed_operation_family() -> WorthQueryReadFamily {
    let operation = WorthQueryGraphReadDomainOperationDeclaration::new(
        "neighbors",
        1,
        "WORTH.tests.installed-domain",
    )
    .unwrap()
    .admit_relation_reference("manager")
    .unwrap();
    let graph = WorthQueryReadBuilder::standalone()
        .local_detail(
            "user",
            schema(),
            |query: DetailQueryBuilder| {
                query
                    .project(AspectFieldSelector::new("identity", "id").unwrap())
                    .domain_graph_operation(operation)
            },
            |shape: DetailResultShapeBuilder| {
                shape.field(
                    crate::authoring::AuthoredResultShapeField::new("identity", "id", "id")
                        .unwrap(),
                )
            },
        )
        .unwrap();
    WorthQueryReadFamily::new_kernel_only("installed-neighbors", graph)
}

fn schema() -> QuerySchemaView {
    QuerySchemaView::new(
        "installed-domain-schema",
        [SchemaFieldView::new(
            crate::authoring::AspectName::new("identity").unwrap(),
            crate::authoring::FieldName::new("id").unwrap(),
            SchemaFieldKind::String,
        )],
        [SchemaRelationView::new(
            RelationName::new("manager").unwrap(),
            1,
        )],
    )
}
