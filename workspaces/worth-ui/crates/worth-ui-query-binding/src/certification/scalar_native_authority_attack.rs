use worth_foundational::facade::InternedString;
use worth_query::facade::installed::operation::{
    WorthQueryNativeAccessCounters, WorthQueryNativeAccessDenialKind,
};

use crate::application_binding::{
    WorthUiScalarTextConsumptionOutcome, WorthUiScalarTextExecutionOutcome,
    WorthUiScalarTextPublicationOutcome, WorthUiScalarTextSettlementOutcome,
    WorthUiSettledScalarTextProjection,
};
use crate::WorthUiQueryWorkspaceExt;

use super::scalar_native_authority_projection::{
    project_scalar_native_key, WorthUiScalarNativeKeyReport,
};

#[derive(Debug, Eq, PartialEq)]
pub struct WorthUiScalarNativeAuthorityAttackReport {
    owner_key: WorthUiScalarNativeKeyReport,
    foreign_key: WorthUiScalarNativeKeyReport,
    owner_value: String,
    owner_counters: WorthQueryNativeAccessCounters,
    foreign_denial: WorthQueryNativeAccessDenialKind,
    foreign_counters: WorthQueryNativeAccessCounters,
}

impl WorthUiScalarNativeAuthorityAttackReport {
    pub fn owner_key(&self) -> &WorthUiScalarNativeKeyReport {
        &self.owner_key
    }

    pub fn foreign_key(&self) -> &WorthUiScalarNativeKeyReport {
        &self.foreign_key
    }

    pub fn owner_value(&self) -> &str {
        &self.owner_value
    }

    pub fn owner_counters(&self) -> WorthQueryNativeAccessCounters {
        self.owner_counters
    }

    pub fn foreign_denial(&self) -> WorthQueryNativeAccessDenialKind {
        self.foreign_denial
    }

    pub fn foreign_counters(&self) -> WorthQueryNativeAccessCounters {
        self.foreign_counters
    }
}

pub fn certify_scalar_native_authority_attack(
    value: &str,
) -> WorthUiScalarNativeAuthorityAttackReport {
    let owner = settled_world(value);
    let foreign = settled_world(value);
    let owner_key = project_scalar_native_key(&owner);
    let foreign_key = project_scalar_native_key(&foreign);
    let owner_access = owner
        .certification_native_value(owner.certification_native_key())
        .expect("owner-issued native key must admit");
    let owner_counters = owner_access.counters();
    let owner_value = raw_text(owner_access.fact());
    let foreign_denial = owner
        .certification_native_value(foreign.certification_native_key())
        .expect_err("equal-printable foreign native key must deny");

    WorthUiScalarNativeAuthorityAttackReport {
        owner_key,
        foreign_key,
        owner_value,
        owner_counters,
        foreign_denial: foreign_denial.kind(),
        foreign_counters: foreign_denial.counters(),
    }
}

fn settled_world(value: &str) -> WorthUiSettledScalarTextProjection {
    let mut workspace = super::scalar_projection_workspace(true);
    super::insert_projection_status(&mut workspace, "platform.pulse.status", value);
    let installed = workspace.worth_ui().expect("Worth UI domain installed");
    let operation = installed.scalar_text_operation_reference();
    let prepared = operation
        .enter_attempt(&workspace)
        .expect("exact scalar operating world")
        .prepare_consumer(&crate::UiProjectionFieldRequirement::query_text_status())
        .unwrap_or_else(|_| panic!("exact scalar consumer must prepare"));
    let executed = match prepared.execute(&mut workspace) {
        WorthUiScalarTextExecutionOutcome::Executed(executed) => *executed,
        _ => panic!("scalar authority fixture must execute"),
    };
    let published = match executed.publish() {
        WorthUiScalarTextPublicationOutcome::Published(published) => *published,
        _ => panic!("scalar authority fixture must publish"),
    };
    let consumed = match published.consume() {
        WorthUiScalarTextConsumptionOutcome::Consumed(consumed) => *consumed,
        _ => panic!("scalar authority fixture must consume"),
    };
    match consumed.settle() {
        WorthUiScalarTextSettlementOutcome::Settled(settled) => *settled,
        _ => panic!("scalar authority fixture must settle"),
    }
}

fn raw_text(fact: &worth_query::facade::foundation::ConsumedFieldValueFact) -> String {
    match fact.as_interned_string() {
        Ok(InternedString::Raw(value)) => value.clone(),
        other => panic!("scalar authority fixture expected raw text, got {other:?}"),
    }
}
