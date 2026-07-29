use std::collections::BTreeMap;

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct UiAuthoredDeclarationLookup {
    identities_by_provenance: BTreeMap<u64, Box<[Box<str>]>>,
    component_capabilities_by_provenance: BTreeMap<u64, Box<[Box<str>]>>,
    theme_token_declarations: BTreeMap<Box<str>, Box<str>>,
}

impl UiAuthoredDeclarationLookup {
    pub(crate) fn from_entries<'capability>(
        entries: impl IntoIterator<
            Item = (
                u64,
                String,
                Option<&'capability str>,
                Option<&'capability str>,
            ),
        >,
    ) -> Self {
        let mut identities_by_provenance = BTreeMap::<u64, Vec<Box<str>>>::new();
        let mut component_capabilities_by_provenance = BTreeMap::<u64, Vec<Box<str>>>::new();
        let mut theme_token_declarations = BTreeMap::new();
        for (provenance, identity, component, theme_token) in entries {
            identities_by_provenance
                .entry(provenance)
                .or_default()
                .push(identity.clone().into());
            if let Some(component) = component {
                component_capabilities_by_provenance
                    .entry(provenance)
                    .or_default()
                    .push(component.into());
            }
            if let Some(theme_token) = theme_token {
                theme_token_declarations.insert(theme_token.into(), identity.into());
            }
        }
        Self {
            identities_by_provenance: identities_by_provenance
                .into_iter()
                .map(|(provenance, identities)| (provenance, identities.into_boxed_slice()))
                .collect(),
            component_capabilities_by_provenance: component_capabilities_by_provenance
                .into_iter()
                .map(|(provenance, identities)| (provenance, identities.into_boxed_slice()))
                .collect(),
            theme_token_declarations,
        }
    }

    pub(crate) fn unique_identity(&self, provenance: u64) -> Option<&str> {
        match self.identities_by_provenance.get(&provenance)?.as_ref() {
            [identity] => Some(identity),
            _ => None,
        }
    }

    pub(crate) fn unique_component_capability_identity(&self, provenance: u64) -> Option<&str> {
        match self
            .component_capabilities_by_provenance
            .get(&provenance)?
            .as_ref()
        {
            [identity] => Some(identity),
            _ => None,
        }
    }

    pub(crate) fn theme_token_declaration_identity(
        &self,
        capability_identity: &str,
    ) -> Option<&str> {
        self.theme_token_declarations
            .get(capability_identity)
            .map(Box::as_ref)
    }
}
