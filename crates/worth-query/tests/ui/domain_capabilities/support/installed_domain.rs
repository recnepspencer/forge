use worth_query::facade::consumer_kit::{in_memory_test_runtime, WorthQueryTestBackendSchema};
use worth_query::facade::{domain, runtime};
use worth_foundational::facade::{
    AbsenceLaw, AspectContract, AspectContractRevision, AspectEvolutionPolicy, AspectIdentity,
    AspectKey, FieldDeclaration, FieldKey, FieldRequirement, ScalarAspectType, StructAspectShape,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DomainCapabilityFixture;

impl domain::WorthQueryDomainEntryMarker for DomainCapabilityFixture {
    fn domain_key(&self) -> &'static str {
        "WORTH.tests.domain-capabilities"
    }

    fn display_name(&self) -> &'static str {
        "DomainCapabilityFixture"
    }

    fn required_capability_families(&self) -> &'static [domain::WorthQueryCapabilityFamily] {
        &[]
    }
}

pub struct InstalledDomainCapabilityFixture {
    _workspace: runtime::WorthQueryWorkspace,
    contributions: domain::WorthQueryInstalledDomainContributionSurface,
}

impl InstalledDomainCapabilityFixture {
    pub fn contributions(&self) -> &domain::WorthQueryInstalledDomainContributionSurface {
        &self.contributions
    }
}

pub fn install(name: &str) -> InstalledDomainCapabilityFixture {
    let package = domain::WorthQueryDomainPackage::declare(
        DomainCapabilityFixture,
        domain::WorthQueryDomainIdentityDeclaration::new(
            domain::WorthQueryDomainIdentityNamespace::new("WORTH.tests").unwrap(),
            domain::WorthQueryDomainIdentityName::new("domain-capabilities").unwrap(),
            domain::WorthQueryDomainSemanticVersion::new(1, 0),
        ),
    )
    .permits_contribution(
        domain::WorthQueryDeclarationEntryContributionCategoryFamily::Admission,
    )
    .permits_contribution(
        domain::WorthQueryDeclarationEntryContributionCategoryFamily::SupportTraceability,
    )
    .permits_contribution(
        domain::WorthQueryDeclarationEntryContributionCategoryFamily::InvariantCapability,
    )
    .permits_contribution(
        domain::WorthQueryDeclarationEntryContributionCategoryFamily::WorkflowPreview,
    )
    .permits_contribution(
        domain::WorthQueryDeclarationEntryContributionCategoryFamily::ContinuityLineage,
    )
    .permits_contribution(
        domain::WorthQueryDeclarationEntryContributionCategoryFamily::ConsequenceAftermath,
    )
    .permits_contribution(
        domain::WorthQueryDeclarationEntryContributionCategoryFamily::ExplanationInspection,
    );
    let schema = WorthQueryTestBackendSchema::single_collection("Task")
        .aspect_contract(identity_contract())
        .unwrap()
        .aspect("identity.id", "identity.id")
        .unwrap();
    let workspace = in_memory_test_runtime()
        .with_schema(schema)
        .domain_package(package)
        .workspace(name)
        .unwrap();
    let handle = workspace.domain(DomainCapabilityFixture).unwrap();
    let contributions = handle.contributions_in(&workspace).unwrap();

    InstalledDomainCapabilityFixture {
        _workspace: workspace,
        contributions,
    }
}

fn identity_contract() -> AspectContract {
    let field = FieldDeclaration::new(
        FieldKey::new("id").unwrap(),
        ScalarAspectType::String,
        FieldRequirement::Required,
        AbsenceLaw::Required,
        AspectEvolutionPolicy::ExplicitBreakRequired,
    )
    .unwrap();
    AspectContract::struct_aspect(
        AspectKey::new("identity").unwrap(),
        AspectIdentity(0x5751_4301),
        AspectContractRevision(1),
        StructAspectShape::new([field]).unwrap(),
    )
}
