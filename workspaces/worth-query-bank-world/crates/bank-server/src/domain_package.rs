use bank_domain::schema::BankSchema;
use worth_query_host::facade::declaration::application_schema::ApplicationSchemaDeclarationDenial;
use worth_query_host::facade::domain::{
    WorthQueryPortableDomainIdentity, WorthQueryPortableDomainPackage,
};

pub(crate) fn bank_domain_package(
) -> Result<WorthQueryPortableDomainPackage, ApplicationSchemaDeclarationDenial> {
    let declaration = BankSchema::declaration()?;
    Ok(
        WorthQueryPortableDomainPackage::new(WorthQueryPortableDomainIdentity::new(
            "WORTH.bank",
            1,
            0,
        ))
        .application_schema(declaration),
    )
}
