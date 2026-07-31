use std::collections::BTreeMap;

use worth_query_installation::facade::{
    ApplicationAuthorizationPath, ApplicationAuthorizationPathEffect, ApplicationSchema,
    ApplicationSchemaMember, WorthQueryInstalledAbilityRequirement,
    WorthQueryInstalledApplicationSchema,
};
use worth_relational::facade::authorization::RelationalAuthorizationPathPlan;
use worth_runtime_bridge::facade::{
    BridgeAuthorizationCorrespondenceIdentity, BridgeAuthorizationInstallationRequest,
    BridgeAuthorizationPathContract, BridgeAuthorizationPathEffect, BridgeAuthorizationRuntime,
};

use super::super::schema_layout::WorthQueryPrimaryGraphLayout;
use super::lowering::lower_authorization_path;
use super::{authorization_denial, WorthQueryOperationAuthorizationDenial};

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
struct PolicyKey {
    ability: String,
    scope_entity: String,
    policy: String,
}

pub(super) struct WorthQueryInstalledAuthorizationPolicy {
    pub(super) correspondence: BridgeAuthorizationCorrespondenceIdentity,
    pub(super) bridge_paths: Vec<BridgeAuthorizationPathContract>,
    pub(super) relational_paths: Vec<RelationalAuthorizationPathPlan>,
    pub(super) principal_kind: worth_relational::facade::identity::KindId,
    pub(super) scope_kind: worth_relational::facade::identity::KindId,
}

pub(in crate::domain_computation::primary_graph) struct WorthQueryInstalledAuthorizationRegistry {
    bridge: BridgeAuthorizationRuntime,
    policies: BTreeMap<PolicyKey, WorthQueryInstalledAuthorizationPolicy>,
}

impl WorthQueryInstalledAuthorizationRegistry {
    pub(in crate::domain_computation::primary_graph) fn compile<Schema>(
        schema: &WorthQueryInstalledApplicationSchema<Schema>,
        layout: &WorthQueryPrimaryGraphLayout,
    ) -> Result<Self, WorthQueryOperationAuthorizationDenial>
    where
        Schema: ApplicationSchema,
    {
        let mut bridge = BridgeAuthorizationRuntime::new();
        let mut policies = BTreeMap::new();
        for member in schema.installed_declaration().members() {
            let ApplicationSchemaMember::AbilityPolicy {
                ability,
                scope_entity,
                policy,
                paths,
            } = member
            else {
                continue;
            };
            let key = PolicyKey {
                ability: ability.clone(),
                scope_entity: scope_entity.clone(),
                policy: policy.clone(),
            };
            let installed = compile_policy(schema, layout, &mut bridge, &key, paths)?;
            if policies.insert(key, installed).is_some() {
                return Err(authorization_denial(
                    policy,
                    "duplicate installed authorization policy",
                ));
            }
        }
        Ok(Self { bridge, policies })
    }

    pub(super) fn policy(
        &self,
        requirement: &WorthQueryInstalledAbilityRequirement,
    ) -> Result<&WorthQueryInstalledAuthorizationPolicy, WorthQueryOperationAuthorizationDenial>
    {
        let key = PolicyKey {
            ability: requirement.ability().to_string(),
            scope_entity: requirement.scope_entity().to_string(),
            policy: requirement.policy().to_string(),
        };
        self.policies
            .get(&key)
            .filter(|installed| {
                installed.bridge_paths.len() == requirement.policy_paths().len()
                    && installed
                        .bridge_paths
                        .iter()
                        .zip(requirement.policy_paths())
                        .all(|(compiled, installed_path)| {
                            compiled.identity() == installed_path.identity().bytes()
                                && compiled.effect()
                                    == lower_bridge_effect(installed_path.path().effect())
                        })
            })
            .ok_or_else(|| {
                authorization_denial(
                    requirement.policy(),
                    "operation requirement does not match installed policy",
                )
            })
    }

    pub(in crate::domain_computation::primary_graph) const fn bridge(
        &self,
    ) -> &BridgeAuthorizationRuntime {
        &self.bridge
    }
}

fn compile_policy<Schema>(
    schema: &WorthQueryInstalledApplicationSchema<Schema>,
    layout: &WorthQueryPrimaryGraphLayout,
    bridge: &mut BridgeAuthorizationRuntime,
    key: &PolicyKey,
    paths: &[ApplicationAuthorizationPath],
) -> Result<WorthQueryInstalledAuthorizationPolicy, WorthQueryOperationAuthorizationDenial>
where
    Schema: ApplicationSchema,
{
    let principal_entity = paths
        .first()
        .map(ApplicationAuthorizationPath::principal_entity)
        .ok_or_else(|| {
            authorization_denial(&key.policy, "installed authorization policy is empty")
        })?;
    let principal_kind = layout.entity_kind(principal_entity).ok_or_else(|| {
        authorization_denial(principal_entity, "policy principal kind is not installed")
    })?;
    let scope_kind = layout.entity_kind(&key.scope_entity).ok_or_else(|| {
        authorization_denial(&key.scope_entity, "policy scope kind is not installed")
    })?;
    let installed_paths = schema
        .installed_ability_requirement(&key.ability, &key.scope_entity)
        .ok_or_else(|| {
            authorization_denial(
                &key.policy,
                "installed authorization path identity is unavailable",
            )
        })?;
    let bridge_paths = installed_paths
        .policy_paths()
        .iter()
        .map(|installed_path| {
            BridgeAuthorizationPathContract::new(
                *installed_path.identity().bytes(),
                lower_bridge_effect(installed_path.path().effect()),
            )
        })
        .collect::<Vec<_>>();
    let relational_paths = paths
        .iter()
        .map(|path| lower_authorization_path(layout, path))
        .collect::<Result<Vec<_>, _>>()?;
    let correspondence = bridge
        .install(BridgeAuthorizationInstallationRequest::new(
            installed_paths.identity(),
            schema.binding_identity(),
            &key.ability,
            &key.scope_entity,
            &key.policy,
            bridge_paths.iter().copied(),
        ))
        .map_err(|denial| authorization_denial(denial.subject(), "Bridge rejected policy"))?;
    Ok(WorthQueryInstalledAuthorizationPolicy {
        correspondence,
        bridge_paths,
        relational_paths,
        principal_kind,
        scope_kind,
    })
}

const fn lower_bridge_effect(
    effect: ApplicationAuthorizationPathEffect,
) -> BridgeAuthorizationPathEffect {
    match effect {
        ApplicationAuthorizationPathEffect::Allow => BridgeAuthorizationPathEffect::Allow,
        ApplicationAuthorizationPathEffect::Deny => BridgeAuthorizationPathEffect::Deny,
    }
}
