use crate::application_schema::ApplicationAbilityRef;

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum ApplicationQueryAuthorizationRequirement {
    Public,
    Ability {
        ability: String,
        scope_entity: String,
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
            ability: ability.name().to_owned(),
            scope_entity: ability.scope().to_owned(),
        }
    }

    pub fn ability(&self) -> Option<&str> {
        match self {
            Self::Public => None,
            Self::Ability { ability, .. } => Some(ability.as_str()),
        }
    }

    pub fn scope_entity(&self) -> Option<&str> {
        match self {
            Self::Public => None,
            Self::Ability { scope_entity, .. } => Some(scope_entity.as_str()),
        }
    }
}
