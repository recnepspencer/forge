#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum QueryGraphReadAccessCapabilityKind {
    Function,
    Type,
    AdmissionPosture,
    DenialKind,
    RequirementKind,
    ReceiptField,
    CostCounter,
    CapabilityGapPressure,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum QueryGraphReadAccessCapabilityAuthority {
    VocabularyOnly,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum QueryGraphReadAccessCapabilitySurface {
    RuntimeFacade,
    GraphReadAccessRuntime,
    ReadReceiptAccessor,
    AccessPlanningDocumentation,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct QueryGraphReadAccessCapabilityRow {
    kind: QueryGraphReadAccessCapabilityKind,
    query_label: &'static str,
    surface: QueryGraphReadAccessCapabilitySurface,
    authority: QueryGraphReadAccessCapabilityAuthority,
}

impl QueryGraphReadAccessCapabilityRow {
    pub(super) const fn from_query_owned_surface(
        kind: QueryGraphReadAccessCapabilityKind,
        query_label: &'static str,
        surface: QueryGraphReadAccessCapabilitySurface,
    ) -> Self {
        Self {
            kind,
            query_label,
            surface,
            authority: QueryGraphReadAccessCapabilityAuthority::VocabularyOnly,
        }
    }

    pub fn kind(&self) -> QueryGraphReadAccessCapabilityKind {
        self.kind
    }

    pub fn query_label(&self) -> &'static str {
        self.query_label
    }

    pub fn surface(&self) -> QueryGraphReadAccessCapabilitySurface {
        self.surface
    }

    pub fn authority(&self) -> QueryGraphReadAccessCapabilityAuthority {
        self.authority
    }

    pub fn claims_execution_authority(&self) -> bool {
        self.authority != QueryGraphReadAccessCapabilityAuthority::VocabularyOnly
    }
}
