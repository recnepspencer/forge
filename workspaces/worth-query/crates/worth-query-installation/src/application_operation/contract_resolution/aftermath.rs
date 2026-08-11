//! Resolves one operation's zero-or-one aftermath contract.

use worth_query_declaration::facade::application_aftermath::PortableApplicationAftermathContract;
use worth_query_declaration::facade::application_schema::ApplicationSchemaMember;

use super::WorthQueryOperationContractCardinalityDenial;

pub(crate) fn operation_aftermath(
    members: &[ApplicationSchemaMember],
    operation: &str,
) -> Result<
    Option<PortableApplicationAftermathContract>,
    WorthQueryOperationContractCardinalityDenial,
> {
    let mut resolved = None;
    for member in members {
        let ApplicationSchemaMember::OperationAftermath {
            operation: installed,
            contract,
        } = member
        else {
            continue;
        };
        if installed != operation {
            continue;
        }
        if resolved.is_some() {
            return Err(WorthQueryOperationContractCardinalityDenial::AmbiguousAftermath);
        }
        resolved = Some(contract.clone());
    }
    Ok(resolved)
}
