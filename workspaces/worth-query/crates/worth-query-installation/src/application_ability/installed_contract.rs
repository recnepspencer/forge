use std::marker::PhantomData;

use sha2::{Digest, Sha256};
use worth_query_declaration::facade::application_schema::{
    ApplicationAuthorizationPath, ApplicationSchema, ApplicationSchemaBindingIdentity,
    ApplicationSchemaMember,
};

use crate::application_schema::WorthQueryInstalledApplicationSchema;
use crate::installed_index::WorthQueryInstalledPackageAuthority;

/// Opaque installation authority for one schema-declared scoped ability.
///
/// The value is neither cloneable nor serializable. Its authority identity
/// includes the private package-installation nonce, so copied names and scope
/// labels cannot recreate it.
pub struct WorthQueryInstalledAbility<Schema, Ability, Scope> {
    binding_identity: ApplicationSchemaBindingIdentity,
    owner: String,
    schema_name: String,
    ability: String,
    scope_entity: String,
    policy: Option<String>,
    policy_paths: Vec<ApplicationAuthorizationPath>,
    authority_identity: String,
    _marker: PhantomData<fn() -> (Schema, Ability, Scope)>,
}

impl<Schema, Ability, Scope> WorthQueryInstalledAbility<Schema, Ability, Scope> {
    pub(crate) fn from_installed_schema(
        schema: &WorthQueryInstalledApplicationSchema<Schema>,
        ability: &str,
        scope_entity: &str,
    ) -> Self
    where
        Schema: ApplicationSchema,
    {
        let installed_policy = schema
            .installed_declaration()
            .members()
            .iter()
            .find_map(|member| match member {
                ApplicationSchemaMember::AbilityPolicy {
                    ability: candidate,
                    scope_entity: candidate_scope,
                    policy,
                    paths,
                } if candidate == ability && candidate_scope == scope_entity => {
                    Some((policy.clone(), paths.clone()))
                }
                _ => None,
            });
        let (policy, policy_paths) = installed_policy
            .map(|(policy, paths)| (Some(policy), paths))
            .unwrap_or_else(|| (None, Vec::new()));
        let binding_identity = schema.binding_identity();
        let authority_identity = authority_identity(
            &schema.package_authority.authority_nonce,
            &binding_identity,
            ability,
            scope_entity,
            policy.as_deref(),
        );
        Self {
            binding_identity,
            owner: schema.owner().to_string(),
            schema_name: schema.schema_name().to_string(),
            ability: ability.to_string(),
            scope_entity: scope_entity.to_string(),
            policy,
            policy_paths,
            authority_identity,
            _marker: PhantomData,
        }
    }

    pub fn binding_identity(&self) -> &ApplicationSchemaBindingIdentity {
        &self.binding_identity
    }

    pub fn owner(&self) -> &str {
        &self.owner
    }

    pub fn schema_name(&self) -> &str {
        &self.schema_name
    }

    pub fn ability(&self) -> &str {
        &self.ability
    }

    pub fn scope_entity(&self) -> &str {
        &self.scope_entity
    }

    pub fn policy(&self) -> Option<&str> {
        self.policy.as_deref()
    }

    pub fn policy_paths(&self) -> &[ApplicationAuthorizationPath] {
        &self.policy_paths
    }

    pub fn authority_identity(&self) -> &str {
        &self.authority_identity
    }

    pub(crate) fn meaning_matches(&self, members: &[ApplicationSchemaMember]) -> bool {
        let ability_matches = members.iter().any(|member| {
            matches!(
                member,
                ApplicationSchemaMember::Ability {
                    ability,
                    scope_entity,
                } if ability == &self.ability && scope_entity == &self.scope_entity
            )
        });
        let policy_matches = self.policy.as_ref().is_none_or(|installed_policy| {
            members.iter().any(|member| {
                matches!(
                    member,
                    ApplicationSchemaMember::AbilityPolicy {
                        ability,
                        scope_entity,
                        policy,
                        paths,
                    } if ability == &self.ability
                        && scope_entity == &self.scope_entity
                        && policy == installed_policy
                        && paths == &self.policy_paths
                )
            })
        });
        ability_matches && policy_matches
    }

    pub(crate) fn authority_matches(&self, package: &WorthQueryInstalledPackageAuthority) -> bool {
        self.authority_identity
            == authority_identity(
                &package.authority_nonce,
                &self.binding_identity,
                &self.ability,
                &self.scope_entity,
                self.policy.as_deref(),
            )
    }
}

impl<Schema, Ability, Scope> std::fmt::Debug
    for WorthQueryInstalledAbility<Schema, Ability, Scope>
{
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("WorthQueryInstalledAbility")
            .field("ability", &self.ability)
            .field("scope_entity", &self.scope_entity)
            .field("policy", &self.policy())
            .field("binding_identity", &self.binding_identity)
            .finish_non_exhaustive()
    }
}

fn authority_identity(
    nonce: &[u8; 32],
    identity: &ApplicationSchemaBindingIdentity,
    ability: &str,
    scope_entity: &str,
    policy: Option<&str>,
) -> String {
    let mut hash = Sha256::new();
    hash.update(b"worth-query-installed-ability-v2");
    hash.update(nonce);
    for value in [
        identity.package_identity(),
        identity.schema_identity().as_str(),
        ability,
        scope_entity,
        policy.unwrap_or("unbound-policy"),
    ] {
        hash.update(value.len().to_le_bytes());
        hash.update(value.as_bytes());
    }
    format!("{:x}", hash.finalize())
}
