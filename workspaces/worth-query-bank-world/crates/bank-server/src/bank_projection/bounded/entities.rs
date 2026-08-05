use bank_domain::model::{AccountId, BankPrincipalId, BusinessId, InstitutionId};
use bank_domain::schema::*;
use worth_query_host::facade::domain::OperationReads;

use super::{
    AccountEntity, BoundedProjectionState, BusinessEntity, InstitutionEntity, PrincipalEntity,
    ProjectionReader,
};
use crate::bank_projection::BankProjectionDenial;

impl BoundedProjectionState {
    pub(in crate::bank_projection) fn project_account_by_id<Operation>(
        &mut self,
        reader: &mut ProjectionReader<'_, '_, Operation>,
        id: AccountId,
    ) -> Result<AccountEntity, BankProjectionDenial>
    where
        AccountIdentity: OperationReads<Operation>,
        AccountingRevision: OperationReads<Operation>,
        InstitutionAccount: OperationReads<Operation>,
        InstitutionIdentityField: OperationReads<Operation>,
        Kind: OperationReads<Operation>,
        PersonalOwner: OperationReads<Operation>,
        BusinessAccount: OperationReads<Operation>,
        PrincipalIdentityField: OperationReads<Operation>,
        BusinessIdentityField: OperationReads<Operation>,
        Status: OperationReads<Operation>,
        AccountDisplayName: OperationReads<Operation>,
    {
        let account = reader.resolve_entity(AccountIdentity::reference(), id)?;
        self.project_account(reader, &account)?;
        Ok(account)
    }

    pub(in crate::bank_projection) fn project_principal<Operation>(
        &mut self,
        reader: &mut ProjectionReader<'_, '_, Operation>,
        id: BankPrincipalId,
    ) -> Result<PrincipalEntity, BankProjectionDenial>
    where
        PrincipalIdentityField: OperationReads<Operation>,
    {
        let principal = reader.resolve_entity(PrincipalIdentityField::reference(), id)?;
        validate_identity_field(
            reader.decision_field(&principal, PrincipalIdentityField::reference())?,
            id,
            "PrincipalIdentityField",
        )?;
        self.update_builder(|builder| builder.principal(id));
        Ok(principal)
    }

    pub(in crate::bank_projection) fn project_business<Operation>(
        &mut self,
        reader: &mut ProjectionReader<'_, '_, Operation>,
        id: BusinessId,
    ) -> Result<BusinessEntity, BankProjectionDenial>
    where
        BusinessIdentityField: OperationReads<Operation>,
    {
        let business = reader.resolve_entity(BusinessIdentityField::reference(), id)?;
        validate_identity_field(
            reader.decision_field(&business, BusinessIdentityField::reference())?,
            id,
            "BusinessIdentityField",
        )?;
        self.update_builder(|builder| builder.business(id));
        Ok(business)
    }

    pub(in crate::bank_projection) fn project_admitted_business<Operation>(
        &mut self,
        reader: &mut ProjectionReader<'_, '_, Operation>,
        business: &BusinessEntity,
        expected: BusinessId,
    ) -> Result<(), BankProjectionDenial>
    where
        BusinessIdentityField: OperationReads<Operation>,
    {
        validate_identity_field(
            reader.decision_field(business, BusinessIdentityField::reference())?,
            expected,
            "BusinessIdentityField",
        )?;
        self.update_builder(|builder| builder.business(expected));
        Ok(())
    }

    pub(in crate::bank_projection) fn project_admitted_institution<Operation>(
        &mut self,
        reader: &mut ProjectionReader<'_, '_, Operation>,
        institution: &InstitutionEntity,
        expected: InstitutionId,
    ) -> Result<(), BankProjectionDenial>
    where
        InstitutionIdentityField: OperationReads<Operation>,
    {
        validate_identity_field(
            reader.decision_field(institution, InstitutionIdentityField::reference())?,
            expected,
            "InstitutionIdentityField",
        )?;
        self.update_builder(|builder| builder.institution(expected));
        Ok(())
    }

    pub(in crate::bank_projection) fn project_primary_account<Operation>(
        &mut self,
        reader: &mut ProjectionReader<'_, '_, Operation>,
        principal: &PrincipalEntity,
    ) -> Result<Option<AccountEntity>, BankProjectionDenial>
    where
        PersonalOwner: OperationReads<Operation>,
        AccountIdentity: OperationReads<Operation>,
        AccountingRevision: OperationReads<Operation>,
        InstitutionAccount: OperationReads<Operation>,
        InstitutionIdentityField: OperationReads<Operation>,
        Kind: OperationReads<Operation>,
        BusinessAccount: OperationReads<Operation>,
        PrincipalIdentityField: OperationReads<Operation>,
        BusinessIdentityField: OperationReads<Operation>,
        Status: OperationReads<Operation>,
        AccountDisplayName: OperationReads<Operation>,
    {
        let relations = reader.decision_relations_from(PersonalOwner::reference(), principal)?;
        self.project_optional_related_account(reader, relations, "PersonalOwner")
    }

    pub(in crate::bank_projection) fn project_business_account<Operation>(
        &mut self,
        reader: &mut ProjectionReader<'_, '_, Operation>,
        business: &BusinessEntity,
    ) -> Result<Option<AccountEntity>, BankProjectionDenial>
    where
        BusinessAccount: OperationReads<Operation>,
        AccountIdentity: OperationReads<Operation>,
        AccountingRevision: OperationReads<Operation>,
        InstitutionAccount: OperationReads<Operation>,
        InstitutionIdentityField: OperationReads<Operation>,
        Kind: OperationReads<Operation>,
        PersonalOwner: OperationReads<Operation>,
        PrincipalIdentityField: OperationReads<Operation>,
        BusinessIdentityField: OperationReads<Operation>,
        Status: OperationReads<Operation>,
        AccountDisplayName: OperationReads<Operation>,
    {
        let relations = reader.decision_relations_from(BusinessAccount::reference(), business)?;
        self.project_optional_related_account(reader, relations, "BusinessAccount")
    }

    pub(in crate::bank_projection) fn project_institution_cash_account<Operation>(
        &mut self,
        reader: &mut ProjectionReader<'_, '_, Operation>,
        institution: &InstitutionEntity,
    ) -> Result<AccountEntity, BankProjectionDenial>
    where
        InstitutionCashAccount: OperationReads<Operation>,
        AccountIdentity: OperationReads<Operation>,
        AccountingRevision: OperationReads<Operation>,
        InstitutionAccount: OperationReads<Operation>,
        InstitutionIdentityField: OperationReads<Operation>,
        Kind: OperationReads<Operation>,
        PersonalOwner: OperationReads<Operation>,
        BusinessAccount: OperationReads<Operation>,
        PrincipalIdentityField: OperationReads<Operation>,
        BusinessIdentityField: OperationReads<Operation>,
        Status: OperationReads<Operation>,
        AccountDisplayName: OperationReads<Operation>,
    {
        let relations =
            reader.decision_relations_from(InstitutionCashAccount::reference(), institution)?;
        let [relation] = relations.as_slice() else {
            return Err(if relations.is_empty() {
                BankProjectionDenial::MissingRelation("InstitutionCashAccount")
            } else {
                BankProjectionDenial::AmbiguousRelation("InstitutionCashAccount")
            });
        };
        let account = relation.to().clone();
        self.project_account(reader, &account)?;
        Ok(account)
    }

    fn project_optional_related_account<Operation, Relation>(
        &mut self,
        reader: &mut ProjectionReader<'_, '_, Operation>,
        relations: Vec<
            worth_query_host::facade::primary_graph::WorthQueryInvariantRelation<
                BankSchema,
                Relation,
                impl Sized,
                Account,
            >,
        >,
        relation: &'static str,
    ) -> Result<Option<AccountEntity>, BankProjectionDenial>
    where
        AccountIdentity: OperationReads<Operation>,
        AccountingRevision: OperationReads<Operation>,
        InstitutionAccount: OperationReads<Operation>,
        InstitutionIdentityField: OperationReads<Operation>,
        Kind: OperationReads<Operation>,
        PersonalOwner: OperationReads<Operation>,
        BusinessAccount: OperationReads<Operation>,
        PrincipalIdentityField: OperationReads<Operation>,
        BusinessIdentityField: OperationReads<Operation>,
        Status: OperationReads<Operation>,
        AccountDisplayName: OperationReads<Operation>,
    {
        let account = match relations.as_slice() {
            [] => return Ok(None),
            [relation] => relation.to().clone(),
            _ => return Err(BankProjectionDenial::AmbiguousRelation(relation)),
        };
        self.project_account(reader, &account)?;
        Ok(Some(account))
    }
}

fn validate_identity_field<Value: Eq>(
    actual: Option<Value>,
    expected: Value,
    field: &'static str,
) -> Result<(), BankProjectionDenial> {
    match actual {
        Some(actual) if actual == expected => Ok(()),
        Some(_) => Err(BankProjectionDenial::AmbiguousRelation(field)),
        None => Err(BankProjectionDenial::MissingField(field)),
    }
}
