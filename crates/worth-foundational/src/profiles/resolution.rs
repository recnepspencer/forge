use super::FoundationalProfileSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum FoundationalProfileResolutionFamily {
    DiagnosticRichness,
    SupportPosture,
    CompatibilityPosture,
    AdmissionReadiness,
    RetentionDelivery,
    CertificationPosture,
    ExecutionObjective,
    ObservationActivation,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum FoundationalProfileResolutionRelation {
    Narrowing,
    ObjectiveSelection,
    ActivationSelection,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FoundationalProfileResolutionRecord {
    family: FoundationalProfileResolutionFamily,
    relation: FoundationalProfileResolutionRelation,
    /// Human-readable explanation only; family/relation and profile-derived
    /// validation carry all progression meaning and authority.
    reason: &'static str,
}

impl FoundationalProfileResolutionRecord {
    pub const fn new(
        family: FoundationalProfileResolutionFamily,
        relation: FoundationalProfileResolutionRelation,
        reason: &'static str,
    ) -> Self {
        Self {
            family,
            relation,
            reason,
        }
    }

    pub const fn family(&self) -> FoundationalProfileResolutionFamily {
        self.family
    }

    pub const fn relation(&self) -> FoundationalProfileResolutionRelation {
        self.relation
    }

    pub const fn reason(&self) -> &'static str {
        self.reason
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FoundationalProfileResolutionLedger {
    records: [Option<FoundationalProfileResolutionRecord>; 8],
    count: u8,
}

impl Default for FoundationalProfileResolutionLedger {
    fn default() -> Self {
        Self::empty()
    }
}

impl FoundationalProfileResolutionLedger {
    pub const fn empty() -> Self {
        Self {
            records: [None; 8],
            count: 0,
        }
    }

    pub const fn len(&self) -> usize {
        self.count as usize
    }

    pub const fn is_empty(&self) -> bool {
        self.count == 0
    }

    pub fn records(&self) -> impl Iterator<Item = FoundationalProfileResolutionRecord> + '_ {
        self.records.iter().flatten().copied()
    }

    pub fn get(
        &self,
        family: FoundationalProfileResolutionFamily,
    ) -> Option<FoundationalProfileResolutionRecord> {
        self.records.get(family as usize).and_then(|record| *record)
    }

    pub fn insert(
        &mut self,
        record: FoundationalProfileResolutionRecord,
    ) -> Result<(), FoundationalProfileResolutionLedgerDenial> {
        let slot = &mut self.records[record.family as usize];
        if slot.is_some() {
            return Err(FoundationalProfileResolutionLedgerDenial::DuplicateFamily(
                record.family,
            ));
        }
        *slot = Some(record);
        self.count += 1;
        Ok(())
    }

    pub(crate) fn replace_descriptive_reason(
        &mut self,
        family: FoundationalProfileResolutionFamily,
        reason: &'static str,
    ) {
        if let Some(record) = self.records[family as usize].as_mut() {
            *record =
                FoundationalProfileResolutionRecord::new(record.family, record.relation, reason);
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FoundationalProfileResolutionLedgerDenial {
    DuplicateFamily(FoundationalProfileResolutionFamily),
}

pub(crate) fn changed_resolution_families(
    stronger: FoundationalProfileSet,
    weaker: FoundationalProfileSet,
) -> FoundationalProfileResolutionLedger {
    let mut ledger = FoundationalProfileResolutionLedger::empty();
    insert_if_changed(
        &mut ledger,
        stronger.diagnostic_richness() != weaker.diagnostic_richness(),
        FoundationalProfileResolutionFamily::DiagnosticRichness,
        FoundationalProfileResolutionRelation::Narrowing,
        "diagnostic richness narrowed",
    );
    insert_if_changed(
        &mut ledger,
        stronger.support_posture() != weaker.support_posture(),
        FoundationalProfileResolutionFamily::SupportPosture,
        FoundationalProfileResolutionRelation::Narrowing,
        "support posture narrowed",
    );
    insert_if_changed(
        &mut ledger,
        stronger.compatibility_posture() != weaker.compatibility_posture(),
        FoundationalProfileResolutionFamily::CompatibilityPosture,
        FoundationalProfileResolutionRelation::Narrowing,
        "compatibility posture narrowed",
    );
    insert_if_changed(
        &mut ledger,
        stronger.admission_readiness() != weaker.admission_readiness(),
        FoundationalProfileResolutionFamily::AdmissionReadiness,
        FoundationalProfileResolutionRelation::Narrowing,
        "admission readiness narrowed",
    );
    insert_if_changed(
        &mut ledger,
        stronger.retention_delivery() != weaker.retention_delivery(),
        FoundationalProfileResolutionFamily::RetentionDelivery,
        FoundationalProfileResolutionRelation::Narrowing,
        "retention delivery narrowed",
    );
    insert_if_changed(
        &mut ledger,
        stronger.certification_posture() != weaker.certification_posture(),
        FoundationalProfileResolutionFamily::CertificationPosture,
        FoundationalProfileResolutionRelation::Narrowing,
        "certification posture narrowed",
    );
    insert_if_changed(
        &mut ledger,
        stronger.execution_objective() != weaker.execution_objective(),
        FoundationalProfileResolutionFamily::ExecutionObjective,
        FoundationalProfileResolutionRelation::ObjectiveSelection,
        "execution objective selected",
    );
    insert_if_changed(
        &mut ledger,
        stronger.observation_activation() != weaker.observation_activation(),
        FoundationalProfileResolutionFamily::ObservationActivation,
        FoundationalProfileResolutionRelation::ActivationSelection,
        "observation activation selected",
    );
    ledger
}

fn insert_if_changed(
    ledger: &mut FoundationalProfileResolutionLedger,
    changed: bool,
    family: FoundationalProfileResolutionFamily,
    relation: FoundationalProfileResolutionRelation,
    reason: &'static str,
) {
    if changed {
        ledger
            .insert(FoundationalProfileResolutionRecord::new(
                family, relation, reason,
            ))
            .expect("each profile family is inserted once");
    }
}
