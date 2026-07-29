use bank_domain::model::{AccountId, AccountJournalRevision, PaymentId};
use bank_domain::payments::{BusinessPayment, BusinessPaymentProjection};
use bank_domain::schema::*;
use worth_query_host::facade::domain::OperationReads;

use super::{
    AccountEntity, ApprovalEntity, BoundedProjectionState, PaymentEntity, ProjectionReader,
};
use crate::bank_projection::{missing_field, BankProjectionDenial};

pub(in crate::bank_projection) struct ProjectedPaymentAccounts {
    source: AccountEntity,
    source_id: AccountId,
    source_revision: AccountJournalRevision,
}

impl ProjectedPaymentAccounts {
    pub(in crate::bank_projection) const fn source_id(&self) -> AccountId {
        self.source_id
    }

    pub(in crate::bank_projection) const fn source(&self) -> &AccountEntity {
        &self.source
    }

    pub(in crate::bank_projection) const fn source_revision(&self) -> AccountJournalRevision {
        self.source_revision
    }
}

impl BoundedProjectionState {
    pub(in crate::bank_projection) fn project_admitted_payment<Operation>(
        &mut self,
        reader: &mut ProjectionReader<'_, '_, Operation>,
        payment: &PaymentEntity,
        id: PaymentId,
    ) -> Result<ProjectedPaymentAccounts, BankProjectionDenial>
    where
        PaymentIdentityField: OperationReads<Operation>,
        PaymentAmount: OperationReads<Operation>,
        PaymentStatusField: OperationReads<Operation>,
        PaymentSource: OperationReads<Operation>,
        PaymentDestination: OperationReads<Operation>,
        PaymentBusiness: OperationReads<Operation>,
        PaymentInitiator: OperationReads<Operation>,
        PaymentApproval: OperationReads<Operation>,
        ApprovalPrincipal: OperationReads<Operation>,
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
        self.project_payment(reader, payment, id)
    }

    fn project_payment<Operation>(
        &mut self,
        reader: &mut ProjectionReader<'_, '_, Operation>,
        payment: &PaymentEntity,
        id: PaymentId,
    ) -> Result<ProjectedPaymentAccounts, BankProjectionDenial>
    where
        PaymentIdentityField: OperationReads<Operation>,
        PaymentAmount: OperationReads<Operation>,
        PaymentStatusField: OperationReads<Operation>,
        PaymentSource: OperationReads<Operation>,
        PaymentDestination: OperationReads<Operation>,
        PaymentBusiness: OperationReads<Operation>,
        PaymentInitiator: OperationReads<Operation>,
        PaymentApproval: OperationReads<Operation>,
        ApprovalPrincipal: OperationReads<Operation>,
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
        self.validate_payment_identity(reader, payment, id)?;
        let source = exactly_one(
            &reader.decision_relations_from(PaymentSource::reference(), payment)?,
            "PaymentSource",
        )?
        .to()
        .clone();
        let destination = exactly_one(
            &reader.decision_relations_from(PaymentDestination::reference(), payment)?,
            "PaymentDestination",
        )?
        .to()
        .clone();
        let source_id = required_account_id(reader, &source)?;
        let destination_id = required_account_id(reader, &destination)?;
        let source_revision = self.project_account(reader, &source)?;
        self.project_account(reader, &destination)?;

        let business_relations =
            reader.decision_relations_from(PaymentBusiness::reference(), payment)?;
        let business = exactly_one(&business_relations, "PaymentBusiness")?.to();
        let business_id = missing_field(
            reader.decision_field(business, BusinessIdentityField::reference())?,
            "BusinessIdentityField",
        )?;
        self.project_business(reader, business_id)?;

        let initiator_relations =
            reader.decision_relations_to(PaymentInitiator::reference(), payment)?;
        let initiator = exactly_one(&initiator_relations, "PaymentInitiator")?.from();
        let initiator_id = missing_field(
            reader.decision_field(initiator, PrincipalIdentityField::reference())?,
            "PrincipalIdentityField",
        )?;
        self.project_principal(reader, initiator_id)?;
        let deciding_principal = self.project_payment_decision(reader, payment)?;

        let amount = missing_field(
            reader.decision_field(payment, PaymentAmount::reference())?,
            "PaymentAmount",
        )?;
        let status = missing_field(
            reader.decision_field(payment, PaymentStatusField::reference())?,
            "PaymentStatusField",
        )?;
        self.update_builder(|builder| {
            builder.projected_payment(BusinessPayment::from_projection(
                BusinessPaymentProjection {
                    id,
                    business: business_id,
                    source: source_id,
                    destination: destination_id,
                    initiator: initiator_id,
                    amount,
                    status,
                    deciding_principal,
                },
            ))
        });
        Ok(ProjectedPaymentAccounts {
            source,
            source_id,
            source_revision,
        })
    }

    fn validate_payment_identity<Operation>(
        &self,
        reader: &mut ProjectionReader<'_, '_, Operation>,
        payment: &PaymentEntity,
        expected: PaymentId,
    ) -> Result<(), BankProjectionDenial>
    where
        PaymentIdentityField: OperationReads<Operation>,
    {
        let actual = missing_field(
            reader.decision_field(payment, PaymentIdentityField::reference())?,
            "PaymentIdentityField",
        )?;
        if actual == expected {
            Ok(())
        } else {
            Err(BankProjectionDenial::AmbiguousRelation(
                "PaymentIdentityField",
            ))
        }
    }

    fn project_payment_decision<Operation>(
        &mut self,
        reader: &mut ProjectionReader<'_, '_, Operation>,
        payment: &PaymentEntity,
    ) -> Result<Option<bank_domain::model::BankPrincipalId>, BankProjectionDenial>
    where
        PaymentApproval: OperationReads<Operation>,
        ApprovalPrincipal: OperationReads<Operation>,
        PrincipalIdentityField: OperationReads<Operation>,
    {
        let approvals = reader.decision_relations_from(PaymentApproval::reference(), payment)?;
        let approval = match approvals.as_slice() {
            [] => return Ok(None),
            [approval] => approval.to(),
            _ => return Err(BankProjectionDenial::AmbiguousRelation("PaymentApproval")),
        };
        validate_approval_owner(reader, payment, approval)?;
        let principal_relations =
            reader.decision_relations_from(ApprovalPrincipal::reference(), approval)?;
        let principal = exactly_one(&principal_relations, "ApprovalPrincipal")?.to();
        let principal_id = missing_field(
            reader.decision_field(principal, PrincipalIdentityField::reference())?,
            "PrincipalIdentityField",
        )?;
        self.project_principal(reader, principal_id)?;
        Ok(Some(principal_id))
    }
}

fn validate_approval_owner<Operation>(
    reader: &mut ProjectionReader<'_, '_, Operation>,
    payment: &PaymentEntity,
    approval: &ApprovalEntity,
) -> Result<(), BankProjectionDenial>
where
    PaymentApproval: OperationReads<Operation>,
{
    let owners = reader.decision_relations_to(PaymentApproval::reference(), approval)?;
    if matches!(owners.as_slice(), [owner] if owner.from() == payment) {
        Ok(())
    } else {
        Err(BankProjectionDenial::AmbiguousRelation("PaymentApproval"))
    }
}

fn required_account_id<Operation>(
    reader: &mut ProjectionReader<'_, '_, Operation>,
    account: &AccountEntity,
) -> Result<AccountId, BankProjectionDenial>
where
    AccountIdentity: OperationReads<Operation>,
{
    missing_field(
        reader.decision_field(account, AccountIdentity::reference())?,
        "AccountIdentity",
    )
}

fn exactly_one<'row, Row>(
    rows: &'row [Row],
    relation: &'static str,
) -> Result<&'row Row, BankProjectionDenial> {
    match rows {
        [row] => Ok(row),
        [] => Err(BankProjectionDenial::MissingRelation(relation)),
        _ => Err(BankProjectionDenial::AmbiguousRelation(relation)),
    }
}
