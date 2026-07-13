#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum WorthQueryDeclarativeCapabilityFamily {
    Read,
    Live,
    Inspection,
    GeneralDeclaration,
}

impl WorthQueryDeclarativeCapabilityFamily {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Read => "read",
            Self::Live => "live",
            Self::Inspection => "inspection",
            Self::GeneralDeclaration => "general-declaration",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum WorthQueryDeclarativePhaseResponsibility {
    Declare,
    Refine,
    Admit,
    Plan,
    Execute,
    AssembleOutcome,
    Inspect,
}

impl WorthQueryDeclarativePhaseResponsibility {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Declare => "declare",
            Self::Refine => "refine",
            Self::Admit => "admit",
            Self::Plan => "plan",
            Self::Execute => "execute",
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
        Self {
            source_path,
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
