#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum WorthQueryDeclarativeCapabilityFamily {
    Read,
    Aggregate,
    Live,
    Historical,
    Comparison,
    Preview,
    Mutation,
    Workflow,
    Inspection,
    DomainExtension,
    GeneralDeclaration,
}

impl WorthQueryDeclarativeCapabilityFamily {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Read => "read",
            Self::Aggregate => "aggregate",
            Self::Live => "live",
            Self::Historical => "historical",
            Self::Comparison => "comparison",
            Self::Preview => "preview",
            Self::Mutation => "mutation",
            Self::Workflow => "workflow",
            Self::Inspection => "inspection",
            Self::DomainExtension => "domain-extension",
            Self::GeneralDeclaration => "general-declaration",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum WorthQueryDeclarativePhaseResponsibility {
    Declare,
    Refine,
    Canonicalize,
    Bind,
    Validate,
    Admit,
    Plan,
    Lower,
    Execute,
    Maintain,
    Dispose,
    AssembleOutcome,
    Inspect,
}

impl WorthQueryDeclarativePhaseResponsibility {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Declare => "declare",
            Self::Refine => "refine",
            Self::Canonicalize => "canonicalize",
            Self::Bind => "bind",
            Self::Validate => "validate",
            Self::Admit => "admit",
            Self::Plan => "plan",
            Self::Lower => "lower",
            Self::Execute => "execute",
            Self::Maintain => "maintain",
            Self::Dispose => "dispose",
            Self::AssembleOutcome => "assemble-outcome",
            Self::Inspect => "inspect",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum WorthQueryDeclarativeSurfaceClass {
    OrdinaryDeclaration,
    OrdinaryOutcome,
    SealedReturnedProof,
    ExtensionContract,
    TerminalRepresentation,
    Diagnostics,
    Certification,
    Compatibility,
    InternalMechanism,
}

impl WorthQueryDeclarativeSurfaceClass {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::OrdinaryDeclaration => "ordinary-declaration",
            Self::OrdinaryOutcome => "ordinary-outcome",
            Self::SealedReturnedProof => "sealed-returned-proof",
            Self::ExtensionContract => "extension-contract",
            Self::TerminalRepresentation => "terminal-representation",
            Self::Diagnostics => "diagnostics",
            Self::Certification => "certification",
            Self::Compatibility => "compatibility",
            Self::InternalMechanism => "internal-mechanism",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorthQueryDeclarativeSurfaceRow {
    source_path: &'static str,
    owner: Option<&'static str>,
    function_name: &'static str,
    capability_family: WorthQueryDeclarativeCapabilityFamily,
    phase_responsibility: WorthQueryDeclarativePhaseResponsibility,
    current_class: WorthQueryDeclarativeSurfaceClass,
    target_class: WorthQueryDeclarativeSurfaceClass,
    expected_consumer: &'static str,
    replacement: &'static str,
}

impl WorthQueryDeclarativeSurfaceRow {
    #[allow(clippy::too_many_arguments)]
    pub(crate) const fn new(
        source_path: &'static str,
        function_name: &'static str,
        capability_family: WorthQueryDeclarativeCapabilityFamily,
        phase_responsibility: WorthQueryDeclarativePhaseResponsibility,
        current_class: WorthQueryDeclarativeSurfaceClass,
        target_class: WorthQueryDeclarativeSurfaceClass,
        expected_consumer: &'static str,
        replacement: &'static str,
    ) -> Self {
        Self::with_owner(
            source_path,
            None,
            function_name,
            capability_family,
            phase_responsibility,
            current_class,
            target_class,
            expected_consumer,
            replacement,
        )
    }

    #[allow(clippy::too_many_arguments)]
    pub(crate) const fn method(
        source_path: &'static str,
        owner: &'static str,
        function_name: &'static str,
        capability_family: WorthQueryDeclarativeCapabilityFamily,
        phase_responsibility: WorthQueryDeclarativePhaseResponsibility,
        current_class: WorthQueryDeclarativeSurfaceClass,
        target_class: WorthQueryDeclarativeSurfaceClass,
        expected_consumer: &'static str,
        replacement: &'static str,
    ) -> Self {
        Self::with_owner(
            source_path,
            Some(owner),
            function_name,
            capability_family,
            phase_responsibility,
            current_class,
            target_class,
            expected_consumer,
            replacement,
        )
    }

    #[allow(clippy::too_many_arguments)]
    const fn with_owner(
        source_path: &'static str,
        owner: Option<&'static str>,
        function_name: &'static str,
        capability_family: WorthQueryDeclarativeCapabilityFamily,
        phase_responsibility: WorthQueryDeclarativePhaseResponsibility,
        current_class: WorthQueryDeclarativeSurfaceClass,
        target_class: WorthQueryDeclarativeSurfaceClass,
        expected_consumer: &'static str,
        replacement: &'static str,
    ) -> Self {
        Self {
            source_path,
            owner,
            function_name,
            capability_family,
            phase_responsibility,
            current_class,
            target_class,
            expected_consumer,
            replacement,
        }
    }

    pub fn source_path(&self) -> &'static str {
        self.source_path
    }

    pub fn function_name(&self) -> &'static str {
        self.function_name
    }

    pub fn owner(&self) -> Option<&'static str> {
        self.owner
    }

    pub fn capability_family(&self) -> WorthQueryDeclarativeCapabilityFamily {
        self.capability_family
    }

    pub fn phase_responsibility(&self) -> WorthQueryDeclarativePhaseResponsibility {
        self.phase_responsibility
    }

    pub fn current_class(&self) -> WorthQueryDeclarativeSurfaceClass {
        self.current_class
    }

    pub fn target_class(&self) -> WorthQueryDeclarativeSurfaceClass {
        self.target_class
    }

    pub fn expected_consumer(&self) -> &'static str {
        self.expected_consumer
    }

    pub fn replacement(&self) -> &'static str {
        self.replacement
    }
}
