#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum ForgeQueryMutationTargetClass {
    Collection,
    Entity,
}

impl ForgeQueryMutationTargetClass {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Collection => "collection",
            Self::Entity => "entity",
        }
    }
}

impl std::fmt::Display for ForgeQueryMutationTargetClass {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryMutationTargetDescriptor {
    target_class: ForgeQueryMutationTargetClass,
    collection: Option<String>,
    entity_identity: Option<String>,
}

impl ForgeQueryMutationTargetDescriptor {
    pub(in crate::runtime) fn new(
        target_class: ForgeQueryMutationTargetClass,
        collection: Option<String>,
        entity_identity: Option<String>,
    ) -> Self {
        Self {
            target_class,
            collection,
            entity_identity,
        }
    }

    pub fn target_class(&self) -> ForgeQueryMutationTargetClass {
        self.target_class
    }

    pub fn collection(&self) -> Option<&str> {
        self.collection.as_deref()
    }

    pub fn entity_identity(&self) -> Option<&str> {
        self.entity_identity.as_deref()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryMutationTargetEvidence {
    declared: ForgeQueryMutationTargetDescriptor,
    resolved: ForgeQueryMutationTargetDescriptor,
}

impl ForgeQueryMutationTargetEvidence {
    pub(in crate::runtime) fn new(
        declared: ForgeQueryMutationTargetDescriptor,
        resolved: ForgeQueryMutationTargetDescriptor,
    ) -> Self {
        Self { declared, resolved }
    }

    pub fn declared(&self) -> &ForgeQueryMutationTargetDescriptor {
        &self.declared
    }

    pub fn resolved(&self) -> &ForgeQueryMutationTargetDescriptor {
        &self.resolved
    }

    #[cfg(test)]
    pub(crate) fn test_only(
        target_class: ForgeQueryMutationTargetClass,
        collection: Option<&str>,
        entity_identity: Option<&str>,
    ) -> Self {
        let descriptor = ForgeQueryMutationTargetDescriptor {
            target_class,
            collection: collection.map(str::to_string),
            entity_identity: entity_identity.map(str::to_string),
        };
        Self {
            declared: descriptor.clone(),
            resolved: descriptor,
        }
    }
}
