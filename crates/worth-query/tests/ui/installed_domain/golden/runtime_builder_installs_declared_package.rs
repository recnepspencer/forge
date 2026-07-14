use worth_query::facade::{domain, runtime};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct ConsumerDomain;

impl domain::WorthQueryDomainEntryMarker for ConsumerDomain {
    fn domain_key(&self) -> &'static str {
        "WORTH.consumer.example"
    }

    fn display_name(&self) -> &'static str {
        "ConsumerDomain"
    }

    fn required_capability_families(&self) -> &'static [domain::WorthQueryCapabilityFamily] {
        &[]
    }
}

fn package() -> domain::WorthQueryDomainPackage<ConsumerDomain> {
    domain::WorthQueryDomainPackage::declare(
        ConsumerDomain,
        domain::WorthQueryDomainIdentityDeclaration::new(
            domain::WorthQueryDomainIdentityNamespace::new("WORTH.consumer").unwrap(),
            domain::WorthQueryDomainIdentityName::new("example").unwrap(),
            domain::WorthQueryDomainSemanticVersion::new(1, 0),
        ),
    )
    .requires_capability(domain::WorthQueryCapabilityFamily::QueryRead)
    .requires_configuration(domain::WorthQueryConfigSectionFamily::Query)
}

fn install(
    builder: runtime::WorthQueryRuntimeBuilder,
) -> Result<runtime::WorthQueryRuntimeBuilder, domain::WorthQueryDomainPackageInstallationError> {
    builder.domain_package(package())
}

fn main() {
    let _ = install(runtime::WorthQueryRuntimeBuilder::new());
}
