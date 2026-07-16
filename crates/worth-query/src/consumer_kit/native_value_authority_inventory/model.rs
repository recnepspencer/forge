#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum WorthQueryNativeValueAuthorityClass {
    ProofBearingCarrier,
    CoarseSemanticVocabulary,
    ContractCapabilityProjection,
    UnvalidatedNativeCarrier,
    IndependentCertificationOracle,
    ScalarOnlyBridge,
    MisleadingCarrier,
    RoleMarker,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum WorthQueryNativeValueDisposition {
    PreserveWithProof,
    ReplaceWithFoundationalValue,
    DeriveFromFoundationalContract,
    SealBehindContractValidation,
    PreserveAsIndependentOracle,
    PreserveScalarAndStruct,
    RealizeOrRename,
    PreserveRoleMarker,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryNativeValueAuthorityRow {
    symbol: &'static str,
    defining_path: &'static str,
    exporting_paths: &'static [&'static str],
    consumer_surfaces: &'static [&'static str],
    class: WorthQueryNativeValueAuthorityClass,
    disposition: WorthQueryNativeValueDisposition,
    closure_owner: &'static str,
}

impl WorthQueryNativeValueAuthorityRow {
    pub const fn new(
        symbol: &'static str,
        defining_path: &'static str,
        exporting_paths: &'static [&'static str],
        consumer_surfaces: &'static [&'static str],
        class: WorthQueryNativeValueAuthorityClass,
        disposition: WorthQueryNativeValueDisposition,
        closure_owner: &'static str,
    ) -> Self {
        Self {
            symbol,
            defining_path,
            exporting_paths,
            consumer_surfaces,
            class,
            disposition,
            closure_owner,
        }
    }

    pub fn symbol(&self) -> &'static str {
        self.symbol
    }

    pub fn defining_path(&self) -> &'static str {
        self.defining_path
    }

    pub fn exporting_paths(&self) -> &'static [&'static str] {
        self.exporting_paths
    }

    pub fn consumer_surfaces(&self) -> &'static [&'static str] {
        self.consumer_surfaces
    }

    pub fn class(&self) -> WorthQueryNativeValueAuthorityClass {
        self.class
    }

    pub fn disposition(&self) -> WorthQueryNativeValueDisposition {
        self.disposition
    }

    pub fn closure_owner(&self) -> &'static str {
        self.closure_owner
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryNativeValueSource {
    path: String,
    text: String,
}

impl WorthQueryNativeValueSource {
    pub fn new(path: impl Into<String>, text: impl Into<String>) -> Self {
        Self {
            path: path.into(),
            text: text.into(),
        }
    }

    pub fn path(&self) -> &str {
        &self.path
    }

    pub fn text(&self) -> &str {
        &self.text
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct WorthQueryNativeValueSourceSite {
    path: String,
    line: usize,
    symbol: String,
}

impl WorthQueryNativeValueSourceSite {
    pub fn new(path: impl Into<String>, line: usize, symbol: impl Into<String>) -> Self {
        Self {
            path: path.into(),
            line,
            symbol: symbol.into(),
        }
    }

    pub fn path(&self) -> &str {
        &self.path
    }

    #[cfg(test)]
    pub fn line(&self) -> usize {
        self.line
    }

    pub fn symbol(&self) -> &str {
        &self.symbol
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum WorthQueryNativeValueFindingKind {
    InvalidRustSource,
    UnclassifiedAuthority,
    MissingClassifiedAuthority,
    MissingFacadeExport,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct WorthQueryNativeValueFinding {
    kind: WorthQueryNativeValueFindingKind,
    site: WorthQueryNativeValueSourceSite,
}

impl WorthQueryNativeValueFinding {
    pub(crate) fn new(
        kind: WorthQueryNativeValueFindingKind,
        site: WorthQueryNativeValueSourceSite,
    ) -> Self {
        Self { kind, site }
    }

    #[cfg(test)]
    pub fn kind(&self) -> WorthQueryNativeValueFindingKind {
        self.kind
    }

    #[cfg(test)]
    pub fn site(&self) -> &WorthQueryNativeValueSourceSite {
        &self.site
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryNativeValueAuthorityAudit {
    observed_site_count: usize,
    findings: Vec<WorthQueryNativeValueFinding>,
}

impl WorthQueryNativeValueAuthorityAudit {
    pub(crate) fn new(
        observed_site_count: usize,
        findings: Vec<WorthQueryNativeValueFinding>,
    ) -> Self {
        Self {
            observed_site_count,
            findings,
        }
    }

    #[cfg(test)]
    pub fn observed_site_count(&self) -> usize {
        self.observed_site_count
    }

    pub fn findings(&self) -> &[WorthQueryNativeValueFinding] {
        &self.findings
    }

    #[cfg(test)]
    pub fn is_closed(&self) -> bool {
        self.findings.is_empty()
    }
}
