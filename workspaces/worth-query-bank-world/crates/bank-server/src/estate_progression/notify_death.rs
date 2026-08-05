use bank_domain::{
    estate::{
        DeathNoticeId, DeathNoticeStatus, EstateAction, EstateCaseId,
        EstateDeathNotificationRequest,
    },
    model::BankPrincipalId,
    schema::{
        BankSchema, DeathNoticeIdentityField, DeathNoticeStatusField, DeathNoticeSubject,
        EstateCase, EstateDeathNotice, EstateDeathNotificationEffect, EstateDeceased,
        NotifyDeathEstateCapability, NotifyDeathEstateOperation, PrincipalIdentityField,
    },
};
use worth_query_host::facade::{
    admission::authenticated_principal::WorthQueryRequestScope,
    declaration::application_schema::TypedMutationPreconditions,
    primary_graph::{
        WorthQueryAdmittedApplicationOperation, WorthQueryApplicationEffectProgram,
        WorthQueryApplicationIdempotencyBinding,
        WorthQueryApplicationOperationInvariantProjectionReader, WorthQueryEntityResolutionDenial,
        WorthQueryInvariantDecisionPlanDenial, WorthQueryInvariantEntityIdentity,
        WorthQueryInvariantProjectionTraversalDenial,
    },
};

use super::BankEstateProgressionDenial;
use crate::{BankAuthenticatedPrincipal, BankIdentityRuntime, BankMutationCommitOutcome};

type AdmittedNotificationOperation = WorthQueryAdmittedApplicationOperation<
    BankSchema,
    NotifyDeathEstateOperation,
    EstateAction,
    EstateCase,
>;
type NotificationEffectProgram = WorthQueryApplicationEffectProgram<
    BankSchema,
    NotifyDeathEstateOperation,
    EstateAction,
    EstateCase,
>;

#[derive(Debug)]
pub enum BankDeathNotificationProjectionDenial {
    RelationCardinality {
        relation: &'static str,
        expected: usize,
        observed: usize,
    },
    MissingNoticeIdentity,
    NoticeMismatch {
        expected: DeathNoticeId,
        observed: DeathNoticeId,
    },
    MissingNoticeStatus,
    NoticeNotReported(DeathNoticeStatus),
    MissingSubjectIdentity(&'static str),
    NoticeSubjectMismatch {
        expected: BankPrincipalId,
        observed: BankPrincipalId,
    },
    EstateSubjectMismatch {
        expected: BankPrincipalId,
        observed: BankPrincipalId,
    },
    EntityResolution(WorthQueryEntityResolutionDenial),
    DecisionPlan(WorthQueryInvariantDecisionPlanDenial),
    Traversal(WorthQueryInvariantProjectionTraversalDenial),
}

impl BankIdentityRuntime {
    pub fn notify_estate_death(
        &self,
        principal: &BankAuthenticatedPrincipal,
        action: EstateAction,
        idempotency: WorthQueryApplicationIdempotencyBinding,
        request: &WorthQueryRequestScope,
    ) -> Result<BankMutationCommitOutcome, BankEstateProgressionDenial> {
        let command = notification_command(action)?;
        let admission = self.admit_notification_operation(principal, action, request)?;
        if let Some(outcome) =
            super::idempotency::resolve_admitted_idempotency(self, &admission, idempotency)?
        {
            return Ok(outcome);
        }
        let program = self.materialize_notification_effect(admission, command)?;
        Ok(self
            .application_runtime()
            .compare_and_commit_application(program, idempotency)
            .into())
    }

    fn admit_notification_operation(
        &self,
        principal: &BankAuthenticatedPrincipal,
        action: EstateAction,
        request: &WorthQueryRequestScope,
    ) -> Result<AdmittedNotificationOperation, BankEstateProgressionDenial> {
        let capability = self
            .application_runtime()
            .installed_schema()
            .capability(
                NotifyDeathEstateCapability::reference(),
                NotifyDeathEstateOperation::reference(),
            )
            .map_err(BankEstateProgressionDenial::CapabilityInstallation)?;
        let access = self
            .application_runtime()
            .admit_capability_access(principal.query(), &capability, action, request)
            .map_err(BankEstateProgressionDenial::Authorization)?;
        let operation = self
            .application_runtime()
            .installed_schema()
            .installed_operation(NotifyDeathEstateOperation::reference())
            .map_err(BankEstateProgressionDenial::OperationInstallation)?;
        self.application_runtime()
            .authorize_capability_operation(
                access,
                &operation,
                TypedMutationPreconditions::<
                    BankSchema,
                    NotifyDeathEstateOperation,
                    EstateCase,
                >::default(),
            )
            .map_err(BankEstateProgressionDenial::Authorization)
    }

    fn materialize_notification_effect(
        &self,
        admission: AdmittedNotificationOperation,
        command: NotificationCommand,
    ) -> Result<NotificationEffectProgram, BankEstateProgressionDenial> {
        let projected = self
            .invariant_projection()
            .project_admitted_operation(&admission, |reader, estate| {
                project_notification(reader, estate, command)
            })
            .map_err(BankEstateProgressionDenial::Projection)?;
        let (projection_result, projection, _) = projected.into_parts();
        projection_result.map_err(BankEstateProgressionDenial::DeathNotificationProjection)?;
        let reads = self
            .application_runtime()
            .begin_projected_application_read_attempt(admission, projection)
            .map_err(BankEstateProgressionDenial::Attempt)?;
        let notice = reads
            .resolve_entity(DeathNoticeIdentityField::reference(), command.notice)
            .map_err(BankEstateProgressionDenial::Attempt)?;
        let mut effects = reads
            .complete_projected_dependencies()
            .map_err(BankEstateProgressionDenial::Attempt)?
            .begin_effect_program();
        let notice = effects
            .existing_entity(&notice)
            .map_err(BankEstateProgressionDenial::Attempt)?;
        effects
            .write_field(
                &notice,
                DeathNoticeStatusField::reference(),
                DeathNoticeStatus::NotificationRequested,
            )
            .map_err(BankEstateProgressionDenial::Attempt)?;
        effects
            .emit(
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
struct NotificationCommand {
    estate: EstateCaseId,
    notice: DeathNoticeId,
    subject: BankPrincipalId,
}

fn notification_command(
    action: EstateAction,
) -> Result<NotificationCommand, BankEstateProgressionDenial> {
    match action {
        EstateAction::NotifyDeath {
            estate,
            notice,
            subject,
        } => Ok(NotificationCommand {
            estate,
            notice,
            subject,
        }),
        _ => Err(BankEstateProgressionDenial::CommandInput(
            "NotifyDeathEstateOperation",
        )),
    }
}

fn project_notification(
    reader: &mut WorthQueryApplicationOperationInvariantProjectionReader<
        BankSchema,
        NotifyDeathEstateOperation,
    >,
    estate: &WorthQueryInvariantEntityIdentity<BankSchema, EstateCase>,
    command: NotificationCommand,
) -> Result<(), BankDeathNotificationProjectionDenial> {
    let notice = exact_notice(reader, estate, command.notice)?;
    let status = reader
        .decision_field(&notice, DeathNoticeStatusField::reference())?
        .ok_or(BankDeathNotificationProjectionDenial::MissingNoticeStatus)?;
    if status != DeathNoticeStatus::Reported {
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
        NotifyDeathEstateOperation,
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
        NotifyDeathEstateOperation,
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
        NotifyDeathEstateOperation,
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

impl From<WorthQueryEntityResolutionDenial> for BankDeathNotificationProjectionDenial {
    fn from(denial: WorthQueryEntityResolutionDenial) -> Self {
        Self::EntityResolution(denial)
    }
}

impl From<WorthQueryInvariantDecisionPlanDenial> for BankDeathNotificationProjectionDenial {
    fn from(denial: WorthQueryInvariantDecisionPlanDenial) -> Self {
        Self::DecisionPlan(denial)
    }
}

impl From<WorthQueryInvariantProjectionTraversalDenial> for BankDeathNotificationProjectionDenial {
    fn from(denial: WorthQueryInvariantProjectionTraversalDenial) -> Self {
        Self::Traversal(denial)
    }
}

impl std::fmt::Display for BankDeathNotificationProjectionDenial {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::RelationCardinality {
                relation,
                expected,
                observed,
            } => write!(
                formatter,
                "{relation} expected {expected} target, observed {observed}"
            ),
            Self::MissingNoticeIdentity => write!(formatter, "death notice has no identity"),
            Self::NoticeMismatch { expected, observed } => write!(
                formatter,
                "estate notice {observed:?} does not match command notice {expected:?}"
            ),
            Self::MissingNoticeStatus => write!(formatter, "death notice has no status"),
            Self::NoticeNotReported(status) => {
                write!(formatter, "death notice is not reportable: {status:?}")
            }
            Self::MissingSubjectIdentity(owner) => {
                write!(formatter, "{owner} subject has no identity")
            }
            Self::NoticeSubjectMismatch { expected, observed } => write!(
                formatter,
                "notice subject {observed:?} does not match command subject {expected:?}"
            ),
            Self::EstateSubjectMismatch { expected, observed } => write!(
                formatter,
                "estate subject {observed:?} does not match command subject {expected:?}"
            ),
            Self::EntityResolution(denial) => denial.fmt(formatter),
            Self::DecisionPlan(denial) => denial.fmt(formatter),
            Self::Traversal(denial) => denial.fmt(formatter),
        }
    }
}

impl std::error::Error for BankDeathNotificationProjectionDenial {}
