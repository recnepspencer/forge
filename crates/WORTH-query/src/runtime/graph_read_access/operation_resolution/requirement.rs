#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum WorthQueryGraphReadOperationCapabilityRequirementKind {
    RequiresAccessCapabilityRegistration,
}

impl WorthQueryGraphReadOperationCapabilityRequirementKind {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::RequiresAccessCapabilityRegistration => "requires_access_capability_registration",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct WorthQueryGraphReadOperationCapabilityRequirement {
    kind: WorthQueryGraphReadOperationCapabilityRequirementKind,
    operation_name: String,
    domain_owner: String,
    support_family: String,
    read_graph_digest: String,
    matched_relations: Vec<String>,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct WorthQueryGraphReadOperationCapabilityRequirementDeclaration {
    kind: WorthQueryGraphReadOperationCapabilityRequirementKind,
    operation_name: String,
    domain_owner: String,
    support_family: String,
}

impl WorthQueryGraphReadOperationCapabilityRequirementDeclaration {
    pub fn kind(&self) -> &WorthQueryGraphReadOperationCapabilityRequirementKind {
        &self.kind
    }

    pub fn operation_name(&self) -> &str {
        &self.operation_name
    }

    pub fn domain_owner(&self) -> &str {
        &self.domain_owner
    }

    pub fn support_family(&self) -> &str {
        &self.support_family
    }

    pub fn registration_required(
        operation_name: impl Into<String>,
        domain_owner: impl Into<String>,
        support_family: impl Into<String>,
    ) -> Self {
        Self {
            kind: WorthQueryGraphReadOperationCapabilityRequirementKind::RequiresAccessCapabilityRegistration,
            operation_name: operation_name.into(),
            domain_owner: domain_owner.into(),
            support_family: support_family.into(),
        }
    }

    pub(crate) fn digest_part(&self) -> String {
        format!(
            "operation_capability_requirement_declaration:{}:{}:{}:{}",
            self.kind.as_str(),
            self.domain_owner,
            self.operation_name,
            self.support_family
        )
    }

    pub(crate) fn resolve_for_read_graph(
        &self,
        read_graph_digest: impl Into<String>,
        mut matched_relations: Vec<String>,
    ) -> WorthQueryGraphReadOperationCapabilityRequirement {
        matched_relations.sort();
        matched_relations.dedup();
        WorthQueryGraphReadOperationCapabilityRequirement {
            kind: self.kind.clone(),
            operation_name: self.operation_name.clone(),
            domain_owner: self.domain_owner.clone(),
            support_family: self.support_family.clone(),
            read_graph_digest: read_graph_digest.into(),
            matched_relations,
        }
    }
}

impl WorthQueryGraphReadOperationCapabilityRequirement {
    pub fn kind(&self) -> &WorthQueryGraphReadOperationCapabilityRequirementKind {
        &self.kind
    }

    pub fn operation_name(&self) -> &str {
        &self.operation_name
    }

    pub fn domain_owner(&self) -> &str {
        &self.domain_owner
    }

    pub fn support_family(&self) -> &str {
        &self.support_family
    }

    pub fn read_graph_digest(&self) -> &str {
        &self.read_graph_digest
    }

    pub fn matched_relations(&self) -> &[String] {
        &self.matched_relations
    }

    pub(crate) fn digest_part(&self) -> String {
        format!(
            "operation_capability_requirement:{}:{}:{}:{}:{}:{}",
            self.kind.as_str(),
            self.domain_owner,
            self.operation_name,
            self.support_family,
            self.read_graph_digest,
            self.matched_relations.join(",")
        )
    }
}
