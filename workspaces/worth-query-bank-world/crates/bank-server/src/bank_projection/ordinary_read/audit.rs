use bank_domain::model::InstitutionId;
use bank_domain::reads::AccountActivityItem;
use bank_domain::schema::*;
use worth_query_host::facade::primary_graph::{
    WorthQueryApplicationOperationInvariantProjectionReader, WorthQueryInvariantEntityIdentity,
};

use super::account_activity::account_postings;
use crate::ordinary::BankReadProjectedBatch;
use crate::BankProjectionDenial;

pub(crate) fn project_institution_audit_read(
    reader: &mut WorthQueryApplicationOperationInvariantProjectionReader<
        '_,
        '_,
        BankSchema,
        AuditInstitutionActivityOperation,
    >,
    root: &WorthQueryInvariantEntityIdentity<BankSchema, Institution>,
    expected: InstitutionId,
    maximum_results: usize,
) -> Result<BankReadProjectedBatch<Vec<AccountActivityItem>>, BankProjectionDenial> {
    match reader.field(root, InstitutionIdentityField::reference()) {
        Some(actual) if actual == expected => {}
        Some(_) => {
            return Err(BankProjectionDenial::AmbiguousRelation(
                "InstitutionIdentityField",
            ))
        }
        None => {
            return Err(BankProjectionDenial::MissingField(
                "InstitutionIdentityField",
            ))
        }
    }
    let accounts = reader.relations_from(InstitutionAccount::reference(), root)?;
    let mut activity = Vec::new();
    for relation in accounts {
        activity.extend(account_postings(reader, relation.to())?.1);
    }
    activity.sort_by_key(|item| (item.journal(), item.account()));
    let truncated = activity.len() > maximum_results;
    activity.truncate(maximum_results);
    let count = activity.len();
    Ok(if truncated {
        BankReadProjectedBatch::truncated(activity, count)
    } else {
        BankReadProjectedBatch::complete(activity, count)
    })
}
