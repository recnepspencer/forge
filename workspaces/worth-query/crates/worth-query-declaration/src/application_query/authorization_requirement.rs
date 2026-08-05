use crate::application_schema::ApplicationAbilityRef;

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum ApplicationQueryAuthorizationRequirement {
    Public,
    Ability {
        ability: &'static str,
        scope_entity: &'static str,
    },
}

impl ApplicationQueryAuthorizationRequirement {
    pub const fn public() -> Self {
        Self::Public
    }

    pub(crate) fn for_ability<Schema, Ability, Scope>(
        ability: ApplicationAbilityRef<Schema, Ability, Scope>,
    ) -> Self {
        Self::Ability {
            ability: ability.name(),
            scope_entity: ability.scope(),
        }
    }

    pub const fn ability(&self) -> Option<&'static str> {
        match self {
            Self::Public => None,
            Self::Ability { ability, .. } => Some(ability),
        }
    }

    pub const fn scope_entity(&self) -> Option<&'static str> {
        match self {
            Self::Public => None,
            Self::Ability { scope_entity, .. } => Some(scope_entity),
        }
    }
}
