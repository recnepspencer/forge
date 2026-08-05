#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum WorthQueryDomainInstallationGrammarStage {
    DeclarePackage,
    ValidatePackage,
    AdmitPackage,
    InstallIntoRuntime,
    ObtainRuntimeAffineHandle,
    DeclareCapability,
    ExecuteOrInspect,
}

impl WorthQueryDomainInstallationGrammarStage {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::DeclarePackage => "declare-package",
            Self::ValidatePackage => "validate-package",
            Self::AdmitPackage => "admit-package",
            Self::InstallIntoRuntime => "install-into-runtime",
            Self::ObtainRuntimeAffineHandle => "obtain-runtime-affine-handle",
            Self::DeclareCapability => "declare-capability",
            Self::ExecuteOrInspect => "execute-or-inspect",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryDomainInstallationGrammar {
    stages: &'static [WorthQueryDomainInstallationGrammarStage],
    package_fields: &'static [&'static str],
    transcript_owners: &'static [&'static str],
}

impl WorthQueryDomainInstallationGrammar {
    pub fn stages(&self) -> &'static [WorthQueryDomainInstallationGrammarStage] {
        self.stages
    }

    pub fn package_fields(&self) -> &'static [&'static str] {
        self.package_fields
    }

    pub fn transcript_owners(&self) -> &'static [&'static str] {
        self.transcript_owners
    }
}

const STAGES: &[WorthQueryDomainInstallationGrammarStage] = &[
    WorthQueryDomainInstallationGrammarStage::DeclarePackage,
    WorthQueryDomainInstallationGrammarStage::ValidatePackage,
    WorthQueryDomainInstallationGrammarStage::AdmitPackage,
    WorthQueryDomainInstallationGrammarStage::InstallIntoRuntime,
    WorthQueryDomainInstallationGrammarStage::ObtainRuntimeAffineHandle,
    WorthQueryDomainInstallationGrammarStage::DeclareCapability,
    WorthQueryDomainInstallationGrammarStage::ExecuteOrInspect,
];

const PACKAGE_FIELDS: &[&str] = &[
    "schema-version",
    "typed-domain-identity",
    "required-capabilities",
    "required-configuration",
    "operating-requirements",
    "invariant-definitions",
    "graph-read-operations",
    "declaration-families",
    "contribution-policy",
];

const TRANSCRIPT_OWNERS: &[&str] = &[
    "domain-package",
    "package-validation",
    "package-admission",
    "runtime-installation",
    "installed-domain-handle",
    "ordinary-domain-capability",
    "ordinary-outcome-and-inspection",
];

pub fn worth_query_domain_installation_grammar() -> WorthQueryDomainInstallationGrammar {
    WorthQueryDomainInstallationGrammar {
        stages: STAGES,
        package_fields: PACKAGE_FIELDS,
        transcript_owners: TRANSCRIPT_OWNERS,
    }
}
