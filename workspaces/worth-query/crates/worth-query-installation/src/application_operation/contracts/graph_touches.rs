use worth_foundational::facade::{CanonicalFieldPath, FieldKey};
use worth_query_declaration::facade::application_schema::{
    ApplicationOperationProgramTarget, ApplicationSchemaBindingIdentity,
};

use crate::application_schema::WorthQueryInstalledApplicationSchemaContractCatalog;
use crate::domain_operation::{
    WorthQueryOperationEntityTouchScope, WorthQueryOperationFieldTouchScope,
    WorthQueryOperationRelationTouchScope, WorthQueryOperationTouchContract,
    WorthQueryOperationTouchScope,
};

pub(crate) fn compile_graph_touches(
    binding: &ApplicationSchemaBindingIdentity,
    catalog: &WorthQueryInstalledApplicationSchemaContractCatalog,
    program: &[ApplicationOperationProgramTarget],
) -> Result<(WorthQueryOperationTouchContract, usize), ()> {
    let mut scopes = Vec::new();
    for target in program {
        let scope = match target {
            ApplicationOperationProgramTarget::Create { entity } => {
                WorthQueryOperationTouchScope::CreateEntity(
                    WorthQueryOperationEntityTouchScope::new(binding.clone(), entity.clone()),
                )
            }
            ApplicationOperationProgramTarget::Delete { entity } => {
                WorthQueryOperationTouchScope::DeleteEntity(
                    WorthQueryOperationEntityTouchScope::new(binding.clone(), entity.clone()),
                )
            }
            ApplicationOperationProgramTarget::Write {
                entity,
                aspect,
                field,
            } => {
                let installed = catalog.aspect(entity, aspect).ok_or(())?;
                let field = FieldKey::new(field.clone()).ok_or(())?;
                if !installed.contains_field(&field) {
                    return Err(());
                }
                WorthQueryOperationTouchScope::WriteField(WorthQueryOperationFieldTouchScope::new(
                    binding.clone(),
                    entity.clone(),
                    installed,
                    CanonicalFieldPath::single(field),
                ))
            }
            ApplicationOperationProgramTarget::Link { relation, from, to } => {
                WorthQueryOperationTouchScope::LinkRelation(
                    WorthQueryOperationRelationTouchScope::new(
                        binding.clone(),
                        relation.clone(),
                        from.clone(),
                        to.clone(),
                    ),
                )
            }
            ApplicationOperationProgramTarget::Unlink { relation, from, to } => {
                WorthQueryOperationTouchScope::UnlinkRelation(
                    WorthQueryOperationRelationTouchScope::new(
                        binding.clone(),
                        relation.clone(),
                        from.clone(),
                        to.clone(),
                    ),
                )
            }
            ApplicationOperationProgramTarget::Emit { .. } => continue,
        };
        scopes.push(scope);
    }
    scopes.sort_by(WorthQueryOperationTouchScope::canonical_order);
    scopes.dedup();
    let count = scopes.len();
    let contract = if scopes.is_empty() {
        WorthQueryOperationTouchContract::NotRequired
    } else {
        WorthQueryOperationTouchContract::Declared {
            graph_roles: vec!["primary".to_owned()],
            scopes,
        }
    };
    Ok((contract, count))
}
