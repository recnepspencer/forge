use std::collections::BTreeSet;

use bank_domain::model::{BankPrincipalId, PaymentId};
use bank_domain::reads::PaymentSummary;
use bank_domain::schema::*;
use worth_query_host::facade::domain::OperationReads;
use worth_query_host::facade::primary_graph::{
    WorthQueryApplicationOperationInvariantProjectionReader, WorthQueryInvariantEntityIdentity,
};

use crate::ordinary::BankReadProjectedBatch;
use crate::BankProjectionDenial;

type PaymentEntity = WorthQueryInvariantEntityIdentity<BankSchema, PaymentIntent>;

pub(crate) fn project_payment_read(
    reader: &mut WorthQueryApplicationOperationInvariantProjectionReader<
        '_,
        '_,
        BankSchema,
        ReadPaymentOperation,
    >,
    root: &PaymentEntity,
    expected: PaymentId,
) -> Result<BankReadProjectedBatch<PaymentSummary>, BankProjectionDenial> {
    let payment = project_summary(reader, root, expected)?;
    Ok(BankReadProjectedBatch::complete(payment, 1))
}

pub(crate) fn project_pending_payments_read(
    reader: &mut WorthQueryApplicationOperationInvariantProjectionReader<
        '_,
        '_,
        BankSchema,
        ReadPendingPaymentsOperation,
    >,
    root: &WorthQueryInvariantEntityIdentity<BankSchema, Principal>,
    principal: BankPrincipalId,
    maximum_results: usize,
) -> Result<BankReadProjectedBatch<Vec<PaymentSummary>>, BankProjectionDenial> {
    validate_principal(reader, root, principal)?;
    let mut pending = BTreeSet::new();
    for authorization in reader.relations_from(AccountAuthorizedUser::reference(), root)? {
        if reader.field(authorization.to(), AuthorizationRole::reference())
            != Some(bank_domain::model::CustomerRole::Approver)
        {
            continue;
        }
        let accounts =
            reader.relations_from(AuthorizationAccount::reference(), authorization.to())?;
        let account = exactly_one(&accounts, "AuthorizationAccount")?.to();
        for source in reader.relations_to(PaymentSource::reference(), account)? {
            if reader.field(source.from(), PaymentStatusField::reference())
                == Some(PaymentStatus::ApprovalRequired)
            {
                pending.insert(source.from().clone());
            }
        }
    }

    let truncated = pending.len() > maximum_results;
    let mut payments = Vec::with_capacity(pending.len().min(maximum_results));
    for payment in pending.into_iter().take(maximum_results) {
        let id = required(
            reader.field(&payment, PaymentIdentityField::reference()),
            "PaymentIdentityField",
        )?;
        payments.push(project_summary(reader, &payment, id)?);
    }
    payments.sort_by_key(|payment| payment.id());
    let count = payments.len();
    Ok(if truncated {
        BankReadProjectedBatch::truncated(payments, count)
    } else {
        BankReadProjectedBatch::complete(payments, count)
    })
}

fn project_summary<Operation>(
    reader: &mut WorthQueryApplicationOperationInvariantProjectionReader<
        '_,
        '_,
        BankSchema,
        Operation,
    >,
    payment: &PaymentEntity,
    expected: PaymentId,
) -> Result<PaymentSummary, BankProjectionDenial>
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
    BusinessIdentityField: OperationReads<Operation>,
    PrincipalIdentityField: OperationReads<Operation>,
{
    let actual = required(
        reader.field(payment, PaymentIdentityField::reference()),
        "PaymentIdentityField",
    )?;
    if actual != expected {
        return Err(BankProjectionDenial::AmbiguousRelation(
            "PaymentIdentityField",
        ));
    }
    let source_relations = reader.relations_from(PaymentSource::reference(), payment)?;
    let source = exactly_one(&source_relations, "PaymentSource")?.to();
    let destination_relations = reader.relations_from(PaymentDestination::reference(), payment)?;
    let destination = exactly_one(&destination_relations, "PaymentDestination")?.to();
    let business_relations = reader.relations_from(PaymentBusiness::reference(), payment)?;
    let business = exactly_one(&business_relations, "PaymentBusiness")?.to();
    let initiator_relations = reader.relations_to(PaymentInitiator::reference(), payment)?;
    let initiator = exactly_one(&initiator_relations, "PaymentInitiator")?.from();
    let approvals = reader.relations_from(PaymentApproval::reference(), payment)?;
    let deciding_principal = match approvals.as_slice() {
        [] => None,
        [approval] => {
            let owners = reader.relations_to(PaymentApproval::reference(), approval.to())?;
            if !matches!(owners.as_slice(), [owner] if owner.from() == payment) {
                return Err(BankProjectionDenial::AmbiguousRelation("PaymentApproval"));
            }
            let principals =
                reader.relations_from(ApprovalPrincipal::reference(), approval.to())?;
            let principal = exactly_one(&principals, "ApprovalPrincipal")?.to();
            Some(required(
                reader.field(principal, PrincipalIdentityField::reference()),
                "PrincipalIdentityField",
            )?)
        }
        _ => return Err(BankProjectionDenial::AmbiguousRelation("PaymentApproval")),
    };
    Ok(PaymentSummary::from_projection(
        actual,
        required(
            reader.field(business, BusinessIdentityField::reference()),
            "BusinessIdentityField",
        )?,
        required(
            reader.field(source, AccountIdentity::reference()),
            "AccountIdentity",
        )?,
        required(
            reader.field(destination, AccountIdentity::reference()),
            "AccountIdentity",
        )?,
        required(
            reader.field(initiator, PrincipalIdentityField::reference()),
            "PrincipalIdentityField",
        )?,
        required(
            reader.field(payment, PaymentAmount::reference()),
            "PaymentAmount",
        )?,
        required(
            reader.field(payment, PaymentStatusField::reference()),
            "PaymentStatusField",
        )?,
        deciding_principal,
    ))
}

fn validate_principal(
    reader: &mut WorthQueryApplicationOperationInvariantProjectionReader<
        '_,
        '_,
        BankSchema,
        ReadPendingPaymentsOperation,
    >,
    root: &WorthQueryInvariantEntityIdentity<BankSchema, Principal>,
    expected: BankPrincipalId,
) -> Result<(), BankProjectionDenial> {
    match reader.field(root, PrincipalIdentityField::reference()) {
        Some(actual) if actual == expected => Ok(()),
        Some(_) => Err(BankProjectionDenial::AmbiguousRelation(
            "PrincipalIdentityField",
        )),
        None => Err(BankProjectionDenial::MissingField("PrincipalIdentityField")),
    }
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

fn required<Value>(
    value: Option<Value>,
    field: &'static str,
) -> Result<Value, BankProjectionDenial> {
    value.ok_or(BankProjectionDenial::MissingField(field))
}
