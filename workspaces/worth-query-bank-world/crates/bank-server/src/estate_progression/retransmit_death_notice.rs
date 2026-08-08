use bank_domain::{
    estate::{
        DeathNoticeId, DeathNoticeStatus, EstateAction, EstateCaseId,
        EstateDeathNotificationRequest,
    },
    model::BankPrincipalId,
    schema::{
        BankSchema, DeathNoticeIdentityField, DeathNoticeStatusField, DeathNoticeSubject,
        EstateCase, EstateDeathNotice, EstateDeathNotificationEffect, EstateDeceased,
        PrincipalIdentityField, RetransmitDeathNoticeEstateCapability,
        RetransmitDeathNoticeEstateOperation,
    },
};
use worth_query_host::facade::{
    admission::authenticated_principal::WorthQueryRequestScope,
    declaration::application_schema::TypedMutationPreconditions,
    primary_graph::{
        WorthQueryAdmittedApplicationOperation, WorthQueryApplicationEffectProgram,
        WorthQueryApplicationIdempotencyBinding,
        WorthQueryApplicationOperationInvariantProjectionReader, WorthQueryInvariantEntityIdentity,
    },
};

use super::{notify_death::BankDeathNotificationProjectionDenial, BankEstateProgressionDenial};
use crate::{BankAuthenticatedPrincipal, BankIdentityRuntime, BankMutationCommitOutcome};

type AdmittedRetransmitOperation = WorthQueryAdmittedApplicationOperation<
    BankSchema,
    RetransmitDeathNoticeEstateOperation,
    EstateAction,
    EstateCase,
>;
type RetransmitEffectProgram = WorthQueryApplicationEffectProgram<
    BankSchema,
    RetransmitDeathNoticeEstateOperation,
    EstateAction,
    EstateCase,
>;

impl BankIdentityRuntime {
    /// Retransmit the death-notice rail for a notice already requested locally.
    ///
    /// Writes no domain fields: the co-committed dispatch outbox is the sole
    /// local anchor (R8.25 / R8.55 O2).
    pub fn retransmit_estate_death_notice(
        &self,
        principal: &BankAuthenticatedPrincipal,
        action: EstateAction,
        idempotency: WorthQueryApplicationIdempotencyBinding,
        request: &WorthQueryRequestScope,
    ) -> Result<BankMutationCommitOutcome, BankEstateProgressionDenial> {
        let command = retransmit_command(action)?;
        let admission = self.admit_retransmit_operation(principal, action, request)?;
        if let Some(outcome) =
            super::idempotency::resolve_admitted_idempotency(self, &admission, idempotency)?
        {
            return Ok(outcome);
        }
        let program = self.materialize_retransmit_effect(admission, command)?;
        Ok(self
            .application_runtime()
            .compare_and_commit_application(program, idempotency)
            .into())
    }

    fn admit_retransmit_operation(
        &self,
        principal: &BankAuthenticatedPrincipal,
        action: EstateAction,
        request: &WorthQueryRequestScope,
    ) -> Result<AdmittedRetransmitOperation, BankEstateProgressionDenial> {
        let capability = self
            .application_runtime()
            .installed_schema()
            .capability(
                RetransmitDeathNoticeEstateCapability::reference(),
                RetransmitDeathNoticeEstateOperation::reference(),
            )
            .map_err(BankEstateProgressionDenial::CapabilityInstallation)?;
        let access = self
            .application_runtime()
            .admit_capability_access(principal.query(), &capability, action, request)
            .map_err(BankEstateProgressionDenial::Authorization)?;
        let operation = self
            .application_runtime()
            .installed_schema()
            .installed_operation(RetransmitDeathNoticeEstateOperation::reference())
            .map_err(BankEstateProgressionDenial::OperationInstallation)?;
        self.application_runtime()
            .authorize_capability_operation(
                access,
                &operation,
                TypedMutationPreconditions::<
                    BankSchema,
                    RetransmitDeathNoticeEstateOperation,
                    EstateCase,
                >::default(),
            )
            .map_err(BankEstateProgressionDenial::Authorization)
    }

    fn materialize_retransmit_effect(
        &self,
        admission: AdmittedRetransmitOperation,
        command: RetransmitCommand,
    ) -> Result<RetransmitEffectProgram, BankEstateProgressionDenial> {
        let projected = self
            .invariant_projection()
            .project_admitted_operation(&admission, |reader, estate| {
                project_retransmit(reader, estate, command)
            })
            .map_err(BankEstateProgressionDenial::Projection)?;
        let (projection_result, projection, _) = projected.into_parts();
        projection_result.map_err(BankEstateProgressionDenial::DeathNotificationProjection)?;
        let reads = self
            .application_runtime()
            .begin_projected_application_read_attempt(admission, projection)
            .map_err(BankEstateProgressionDenial::Attempt)?;
        let mut effects = reads
            .complete_projected_dependencies()
            .map_err(BankEstateProgressionDenial::Attempt)?
            .begin_effect_program();
        effects
            .emit_external(
                EstateDeathNotificationEffect::reference(),
                EstateDeathNotificationRequest::new(
                    command.estate,
                    command.notice,
                    command.subject,
                ),
            )
            .map_err(BankEstateProgressionDenial::Attempt)?;
        effects
            .finish()
            .map_err(BankEstateProgressionDenial::Attempt)
    }
}

#[derive(Clone, Copy)]
struct RetransmitCommand {
    estate: EstateCaseId,
    notice: DeathNoticeId,
    subject: BankPrincipalId,
}

fn retransmit_command(
    action: EstateAction,
) -> Result<RetransmitCommand, BankEstateProgressionDenial> {
    match action {
        EstateAction::RetransmitDeathNotice {
            estate,
            notice,
            subject,
        } => Ok(RetransmitCommand {
            estate,
            notice,
            subject,
        }),
        _ => Err(BankEstateProgressionDenial::CommandInput(
            "RetransmitDeathNoticeEstateOperation",
        )),
    }
}

fn project_retransmit(
    reader: &mut WorthQueryApplicationOperationInvariantProjectionReader<
        BankSchema,
        RetransmitDeathNoticeEstateOperation,
    >,
    estate: &WorthQueryInvariantEntityIdentity<BankSchema, EstateCase>,
    command: RetransmitCommand,
) -> Result<(), BankDeathNotificationProjectionDenial> {
    let notice = exact_notice(reader, estate, command.notice)?;
    let status = reader
        .decision_field(&notice, DeathNoticeStatusField::reference())?
        .ok_or(BankDeathNotificationProjectionDenial::MissingNoticeStatus)?;
    if status != DeathNoticeStatus::NotificationRequested {
        return Err(BankDeathNotificationProjectionDenial::NoticeNotReported(
            status,
        ));
    }
    require_notice_subject(reader, &notice, command.subject)?;
    require_estate_subject(reader, estate, command.subject)
}

fn exact_notice(
    reader: &mut WorthQueryApplicationOperationInvariantProjectionReader<
        BankSchema,
        RetransmitDeathNoticeEstateOperation,
    >,
    estate: &WorthQueryInvariantEntityIdentity<BankSchema, EstateCase>,
    expected: DeathNoticeId,
) -> Result<
    WorthQueryInvariantEntityIdentity<BankSchema, bank_domain::schema::DeathNotice>,
    BankDeathNotificationProjectionDenial,
> {
    let relations = reader.decision_relations_from(EstateDeathNotice::reference(), estate)?;
    let [relation] = relations.as_slice() else {
        return Err(relation_cardinality("EstateDeathNotice", relations.len()));
    };
    let notice = relation.to().clone();
    let observed = reader
        .decision_field(&notice, DeathNoticeIdentityField::reference())?
        .ok_or(BankDeathNotificationProjectionDenial::MissingNoticeIdentity)?;
    if observed != expected {
        return Err(BankDeathNotificationProjectionDenial::NoticeMismatch { expected, observed });
    }
    Ok(notice)
}

fn require_notice_subject(
    reader: &mut WorthQueryApplicationOperationInvariantProjectionReader<
        BankSchema,
        RetransmitDeathNoticeEstateOperation,
    >,
    notice: &WorthQueryInvariantEntityIdentity<BankSchema, bank_domain::schema::DeathNotice>,
    expected: BankPrincipalId,
) -> Result<(), BankDeathNotificationProjectionDenial> {
    let relations = reader.decision_relations_from(DeathNoticeSubject::reference(), notice)?;
    let [relation] = relations.as_slice() else {
        return Err(relation_cardinality("DeathNoticeSubject", relations.len()));
    };
    let observed = reader
        .decision_field(relation.to(), PrincipalIdentityField::reference())?
        .ok_or(BankDeathNotificationProjectionDenial::MissingSubjectIdentity("death notice"))?;
    if observed != expected {
        return Err(
            BankDeathNotificationProjectionDenial::NoticeSubjectMismatch { expected, observed },
        );
    }
    Ok(())
}

fn require_estate_subject(
    reader: &mut WorthQueryApplicationOperationInvariantProjectionReader<
        BankSchema,
        RetransmitDeathNoticeEstateOperation,
    >,
    estate: &WorthQueryInvariantEntityIdentity<BankSchema, EstateCase>,
    expected: BankPrincipalId,
) -> Result<(), BankDeathNotificationProjectionDenial> {
    let relations = reader.decision_relations_from(EstateDeceased::reference(), estate)?;
    let [relation] = relations.as_slice() else {
        return Err(relation_cardinality("EstateDeceased", relations.len()));
    };
    let observed = reader
        .decision_field(relation.to(), PrincipalIdentityField::reference())?
        .ok_or(BankDeathNotificationProjectionDenial::MissingSubjectIdentity("estate"))?;
    if observed != expected {
        return Err(
            BankDeathNotificationProjectionDenial::EstateSubjectMismatch { expected, observed },
        );
    }
    Ok(())
}

fn relation_cardinality(
    relation: &'static str,
    observed: usize,
) -> BankDeathNotificationProjectionDenial {
    BankDeathNotificationProjectionDenial::RelationCardinality {
        relation,
        expected: 1,
        observed,
    }
}
