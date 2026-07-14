use super::{
    WorthQueryCapabilityFamily, WorthQueryDomainEntryMarker, WorthQueryDomainOperatingContext,
    WorthQueryInstalledDomainDeclarationContext,
};
use crate::consumer_kit::{in_memory_test_runtime, WorthQueryTestBackendSchema};
use crate::domain_installation::{
    WorthQueryDomainDeclarationFamilyDefinition, WorthQueryDomainIdentityDeclaration,
    WorthQueryDomainIdentityName, WorthQueryDomainIdentityNamespace, WorthQueryDomainPackage,
    WorthQueryDomainSemanticVersion,
};

/// Builds test declaration authority through the same package/install/lookup path
/// available to external consumers.
pub(crate) fn installed_declaration_context<D, C>(
    marker: D,
    context: C,
    declaration_families: impl IntoIterator<Item = WorthQueryDomainDeclarationFamilyDefinition>,
) -> WorthQueryInstalledDomainDeclarationContext<D, C>
where
    D: WorthQueryDomainEntryMarker + 'static,
    C: WorthQueryDomainOperatingContext<D>,
{
    let (_, context) = installed_declaration_workspace(marker, context, declaration_families);
    context
}

pub(crate) fn installed_declaration_workspace<D, C>(
    marker: D,
    context: C,
    declaration_families: impl IntoIterator<Item = WorthQueryDomainDeclarationFamilyDefinition>,
) -> (
    crate::runtime::WorthQueryWorkspace,
    WorthQueryInstalledDomainDeclarationContext<D, C>,
)
where
    D: WorthQueryDomainEntryMarker + 'static,
    C: WorthQueryDomainOperatingContext<D>,
{
    let (namespace, name) = marker
        .domain_key()
        .rsplit_once('.')
        .expect("test domain keys must contain a namespace and name");
    let identity = WorthQueryDomainIdentityDeclaration::new(
        WorthQueryDomainIdentityNamespace::new(namespace)
            .expect("test domain namespace must be valid"),
        WorthQueryDomainIdentityName::new(name).expect("test domain name must be valid"),
        WorthQueryDomainSemanticVersion::new(1, 0),
    );
    let mut package = WorthQueryDomainPackage::declare(marker, identity);
    for family in merged_capabilities(&marker, &context) {
        package = package.requires_capability(family);
    }
    for section in context.required_config_sections() {
        package = package.requires_configuration(*section);
    }
    for requirement in context.required_operating_requirements() {
        package = package.requires_operating_posture(*requirement);
    }
    let mut families = declaration_families.into_iter().collect::<Vec<_>>();
    families.sort_by(|left, right| left.family_key().cmp(right.family_key()));
    families.dedup_by(|left, right| left.family_key() == right.family_key());
    package = package.declaration_families(families);

    let schema = WorthQueryTestBackendSchema::single_collection("TestEntity")
        .aspect("identity.id", "identity.id")
        .expect("test declaration schema should be valid");
    let workspace = in_memory_test_runtime()
        .with_schema(schema)
        .domain_package(package)
        .workspace(format!("{}.workspace", marker.domain_key()))
        .expect("test domain package should install");
    let context = workspace
        .domain(marker)
        .expect("installed test domain should be available")
        .declarations_in(&workspace, context)
        .expect("installed declaration context should admit");
    (workspace, context)
}

fn merged_capabilities<D, C>(marker: &D, context: &C) -> Vec<WorthQueryCapabilityFamily>
where
    D: WorthQueryDomainEntryMarker,
    C: WorthQueryDomainOperatingContext<D>,
{
    let mut capabilities = marker.required_capability_families().to_vec();
    capabilities.extend_from_slice(context.required_capability_families());
    capabilities.sort();
    capabilities.dedup();
    capabilities
}

pub(crate) fn family<D, F>() -> WorthQueryDomainDeclarationFamilyDefinition
where
    D: WorthQueryDomainEntryMarker,
    F: super::WorthQueryDeclarationFamilyMarker<D>,
{
    WorthQueryDomainDeclarationFamilyDefinition::from_marker::<D, F>(1)
        .expect("test declaration family identity must be valid")
}
