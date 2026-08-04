use bank_domain::{
    estate::{EmergencyAccessId, MandatoryReviewId},
    schema::{
        ApproveEstateEmergencyAccessOperation, BankSchema,
        CompleteEstateMandatoryReviewOperation, EmergencyAccess, EmergencyAccessExpiresAtField,
        EmergencyAccessIdentityField, EmergencyAccessIssuedAtField, EmergencyAccessReasonField,
        EmergencyAccessStatusField, EmergencyApprover, EmergencyGrant, EmergencyRequester,
        EmergencyReview, MandatoryReview, MandatoryReviewIdentityField, MandatoryReviewKindField,
        MandatoryReviewStatusField, ReviewEstate, ReviewPrincipal,
        RevokeEstateEmergencyAccessOperation,
    },
};
use worth_query_host::facade::{
    declaration::application_schema::{OperationReads, TypedApplicationReadableValue},
    primary_graph::{
        WorthQueryApplicationOperationInvariantProjectionReader,
        WorthQueryInvariantEntityIdentity, WorthQueryRequestedElevation,
    },
};

use super::BankEstateLifecycleProjectionDenial;

type ElevationIdentity = WorthQueryInvariantEntityIdentity<BankSchema, EmergencyAccess>;
type ReviewIdentity = WorthQueryInvariantEntityIdentity<BankSchema, MandatoryReview>;

pub(super) fn approval_lifecycle_identities(
    requested: &WorthQueryRequestedElevation,
) -> Result<(EmergencyAccessId, MandatoryReviewId), BankEstateLifecycleProjectionDenial> {
    let access = EmergencyAccessId::from_foundational_value(requested.elevation_identity())
        .ok_or(BankEstateLifecycleProjectionDenial::ReceiptIdentity(
            "emergency access",
        ))?;
    let review = MandatoryReviewId::from_foundational_value(requested.review_identity()).ok_or(
        BankEstateLifecycleProjectionDenial::ReceiptIdentity("mandatory review"),
    )?;
    Ok((access, review))
}

pub(super) fn seal_approval_lifecycle_facts(
    reader: &mut WorthQueryApplicationOperationInvariantProjectionReader<
        BankSchema,
        ApproveEstateEmergencyAccessOperation,
    >,
    access: EmergencyAccessId,
    review: MandatoryReviewId,
) -> Result<(), BankEstateLifecycleProjectionDenial> {
    seal_selected_lifecycle_facts(reader, access, review)
}

pub(super) fn seal_close_lifecycle_facts(
    reader: &mut WorthQueryApplicationOperationInvariantProjectionReader<
        BankSchema,
        RevokeEstateEmergencyAccessOperation,
    >,
    access: EmergencyAccessId,
) -> Result<(), BankEstateLifecycleProjectionDenial> {
    let elevation = reader.resolve_entity(EmergencyAccessIdentityField::reference(), access)?;
    let review_relations = reader.decision_relations_from(EmergencyReview::reference(), &elevation)?;
    let [review_relation] = review_relations.as_slice() else {
        return Err(BankEstateLifecycleProjectionDenial::RelationCardinality {
            relation: "EmergencyReview",
            expected: 1,
            observed: review_relations.len(),
        });
    };
    let review = review_relation.to().clone();
    seal_lifecycle_fields(reader, &elevation, &review)?;
    seal_remaining_lifecycle_relations(reader, &elevation, &review)
}

pub(super) fn seal_review_lifecycle_facts(
    reader: &mut WorthQueryApplicationOperationInvariantProjectionReader<
        BankSchema,
        CompleteEstateMandatoryReviewOperation,
    >,
    access: EmergencyAccessId,
    review: MandatoryReviewId,
) -> Result<(), BankEstateLifecycleProjectionDenial> {
    seal_selected_lifecycle_facts(reader, access, review)
}

fn seal_selected_lifecycle_facts<Operation>(
    reader: &mut WorthQueryApplicationOperationInvariantProjectionReader<BankSchema, Operation>,
    access: EmergencyAccessId,
    review: MandatoryReviewId,
) -> Result<(), BankEstateLifecycleProjectionDenial>
where
    EmergencyAccessIdentityField: OperationReads<Operation>,
    EmergencyAccessReasonField: OperationReads<Operation>,
    EmergencyAccessStatusField: OperationReads<Operation>,
    EmergencyAccessIssuedAtField: OperationReads<Operation>,
    EmergencyAccessExpiresAtField: OperationReads<Operation>,
    MandatoryReviewIdentityField: OperationReads<Operation>,
    MandatoryReviewKindField: OperationReads<Operation>,
    MandatoryReviewStatusField: OperationReads<Operation>,
    EmergencyRequester: OperationReads<Operation>,
    EmergencyApprover: OperationReads<Operation>,
    EmergencyGrant: OperationReads<Operation>,
    EmergencyReview: OperationReads<Operation>,
    ReviewEstate: OperationReads<Operation>,
    ReviewPrincipal: OperationReads<Operation>,
{
    let elevation = reader.resolve_entity(EmergencyAccessIdentityField::reference(), access)?;
    let review = reader.resolve_entity(MandatoryReviewIdentityField::reference(), review)?;
    seal_lifecycle_fields(reader, &elevation, &review)?;
    reader.decision_relations_from(EmergencyReview::reference(), &elevation)?;
    seal_remaining_lifecycle_relations(reader, &elevation, &review)
}

fn seal_lifecycle_fields<Operation>(
    reader: &mut WorthQueryApplicationOperationInvariantProjectionReader<BankSchema, Operation>,
    elevation: &ElevationIdentity,
    review: &ReviewIdentity,
) -> Result<(), BankEstateLifecycleProjectionDenial>
where
    EmergencyAccessIdentityField: OperationReads<Operation>,
    EmergencyAccessReasonField: OperationReads<Operation>,
    EmergencyAccessStatusField: OperationReads<Operation>,
    EmergencyAccessIssuedAtField: OperationReads<Operation>,
    EmergencyAccessExpiresAtField: OperationReads<Operation>,
    MandatoryReviewIdentityField: OperationReads<Operation>,
    MandatoryReviewKindField: OperationReads<Operation>,
    MandatoryReviewStatusField: OperationReads<Operation>,
{
    reader.require_decision_field(elevation, EmergencyAccessIdentityField::reference())?;
    reader.require_decision_field(elevation, EmergencyAccessReasonField::reference())?;
    reader.require_decision_field(elevation, EmergencyAccessStatusField::reference())?;
    reader.require_decision_field(elevation, EmergencyAccessIssuedAtField::reference())?;
    reader.require_decision_field(elevation, EmergencyAccessExpiresAtField::reference())?;
    reader.require_decision_field(review, MandatoryReviewIdentityField::reference())?;
    reader.require_decision_field(review, MandatoryReviewKindField::reference())?;
    reader.require_decision_field(review, MandatoryReviewStatusField::reference())?;
    Ok(())
}

fn seal_remaining_lifecycle_relations<Operation>(
    reader: &mut WorthQueryApplicationOperationInvariantProjectionReader<BankSchema, Operation>,
    elevation: &ElevationIdentity,
    review: &ReviewIdentity,
) -> Result<(), BankEstateLifecycleProjectionDenial>
where
    EmergencyRequester: OperationReads<Operation>,
    EmergencyApprover: OperationReads<Operation>,
    EmergencyGrant: OperationReads<Operation>,
    ReviewEstate: OperationReads<Operation>,
    ReviewPrincipal: OperationReads<Operation>,
{
    reader.decision_relations_to(EmergencyRequester::reference(), elevation)?;
    reader.decision_relations_to(EmergencyApprover::reference(), elevation)?;
    reader.decision_relations_from(EmergencyGrant::reference(), elevation)?;
    reader.decision_relations_from(ReviewEstate::reference(), review)?;
    reader.decision_relations_to(ReviewPrincipal::reference(), review)?;
    Ok(())
}
