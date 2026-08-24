use std::collections::{BTreeMap, BTreeSet};

use worth_foundational::facade::{aspects, FieldKey};
use worth_query_declaration::facade::application_schema::{
    ApplicationOperationDecisionReadTarget, ApplicationSchemaBindingIdentity,
};

use crate::application_schema::WorthQueryInstalledApplicationSchemaContractCatalog;
use crate::domain_operation::{
    WorthQueryOperationApplicationProjectionScope, WorthQueryOperationEntityReadScope,
    WorthQueryOperationGraphAccess, WorthQueryOperationGraphParticipation,
    WorthQueryOperationGraphReadContract, WorthQueryOperationGraphReadRole,
    WorthQueryOperationGraphReadScope, WorthQueryOperationNativeProjectionContract,
    WorthQueryOperationRelationReadScope,
};

pub(crate) fn compile_graph_reads(
    binding: &ApplicationSchemaBindingIdentity,
    catalog: &WorthQueryInstalledApplicationSchemaContractCatalog,
    reads: &[ApplicationOperationDecisionReadTarget],
) -> Result<WorthQueryOperationGraphReadContract, ()> {
    let mut entities = BTreeSet::new();
    let mut projections = BTreeMap::<(String, String), BTreeSet<String>>::new();
    let mut relations = BTreeSet::new();
    for read in reads {
        match read {
            ApplicationOperationDecisionReadTarget::Entity { entity } => {
                entities.insert(entity.clone());
            }
            ApplicationOperationDecisionReadTarget::Field {
                entity,
                aspect,
                field,
            } => {
                projections
                    .entry((entity.clone(), aspect.clone()))
                    .or_default()
                    .insert(field.clone());
            }
            ApplicationOperationDecisionReadTarget::Relation { relation, from, to } => {
                relations.insert((relation.clone(), from.clone(), to.clone()));
            }
        }
    }
    let mut scopes = Vec::with_capacity(entities.len() + projections.len() + relations.len());
    scopes.extend(entities.into_iter().map(|entity| {
        WorthQueryOperationGraphReadScope::Entity(WorthQueryOperationEntityReadScope::new(
            binding.clone(),
            entity,
        ))
    }));
    for ((entity, aspect), fields) in projections {
        let installed = catalog.aspect(&entity, &aspect).ok_or(())?;
        let keys = fields
            .into_iter()
            .map(|field| FieldKey::new(field).ok_or(()))
            .collect::<Result<Vec<_>, _>>()?;
        if keys.iter().any(|field| !installed.contains_field(field)) {
            return Err(());
        }
        let mask = aspects()
            .projection_mask()
            .fields(keys.iter().map(FieldKey::as_str))
            .map_err(|_| ())?;
        let projection =
            WorthQueryOperationNativeProjectionContract::from_installed(installed, mask)
                .map_err(|_| ())?;
        scopes.push(WorthQueryOperationGraphReadScope::NativeProjection(
            WorthQueryOperationApplicationProjectionScope::new(
                binding.clone(),
                entity,
                installed.locus().aspect().clone(),
                projection,
            ),
        ));
    }
    scopes.extend(relations.into_iter().map(|(relation, from, to)| {
        WorthQueryOperationGraphReadScope::Relation(WorthQueryOperationRelationReadScope::new(
            binding.clone(),
            relation,
            from,
            to,
        ))
    }));
    Ok(WorthQueryOperationGraphReadContract::Declared {
        roles: vec![WorthQueryOperationGraphReadRole::new(
            "primary".to_owned(),
            WorthQueryOperationGraphParticipation::PrimaryLogicalGraph,
            WorthQueryOperationGraphAccess::Project,
            scopes,
        )],
    })
}
