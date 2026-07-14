#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct StoreSecurityScopeAdmissionCounterSnapshot {
    requests: u64,
    current_authority_checks: u64,
    physical_binding_checks: u64,
    key_scope_checks: u64,
    key_version_checks: u64,
    tenant_scope_checks: u64,
    authenticity_requirement_checks: u64,
    custody_posture_checks: u64,
    denials: u64,
    readmission_required: u64,
    replayed_admission_evidence: u64,
    wrong_physical_scopes: u64,
    wrong_key_scopes: u64,
    wrong_tenant_scopes: u64,
    missing_authenticity_requirements: u64,
    unsupported_authenticity_requirements: u64,
    missing_custody_postures: u64,
    stale_key_postures: u64,
    rebind_required_key_postures: u64,
    deferred_custody_postures: u64,
    unsupported_postures: u64,
    unavailable_postures: u64,
    witnesses_issued: u64,
}

impl StoreSecurityScopeAdmissionCounterSnapshot {
    pub const fn requests(self) -> u64 {
        self.requests
    }

    pub const fn current_authority_checks(self) -> u64 {
        self.current_authority_checks
    }

    pub const fn physical_binding_checks(self) -> u64 {
        self.physical_binding_checks
    }

    pub const fn key_scope_checks(self) -> u64 {
        self.key_scope_checks
    }

    pub const fn key_version_checks(self) -> u64 {
        self.key_version_checks
    }

    pub const fn tenant_scope_checks(self) -> u64 {
        self.tenant_scope_checks
    }

    pub const fn authenticity_requirement_checks(self) -> u64 {
        self.authenticity_requirement_checks
    }

    pub const fn custody_posture_checks(self) -> u64 {
        self.custody_posture_checks
    }

    pub const fn denials(self) -> u64 {
        self.denials
    }

    pub const fn readmission_required(self) -> u64 {
        self.readmission_required
    }

    pub const fn replayed_admission_evidence(self) -> u64 {
        self.replayed_admission_evidence
    }

    pub const fn wrong_physical_scopes(self) -> u64 {
        self.wrong_physical_scopes
    }

    pub const fn wrong_key_scopes(self) -> u64 {
        self.wrong_key_scopes
    }

    pub const fn wrong_tenant_scopes(self) -> u64 {
        self.wrong_tenant_scopes
    }

    pub const fn missing_authenticity_requirements(self) -> u64 {
        self.missing_authenticity_requirements
    }

    pub const fn unsupported_authenticity_requirements(self) -> u64 {
        self.unsupported_authenticity_requirements
    }

    pub const fn missing_custody_postures(self) -> u64 {
        self.missing_custody_postures
    }

    pub const fn stale_key_postures(self) -> u64 {
        self.stale_key_postures
    }

    pub const fn rebind_required_key_postures(self) -> u64 {
        self.rebind_required_key_postures
    }

    pub const fn deferred_custody_postures(self) -> u64 {
        self.deferred_custody_postures
    }

    pub const fn unsupported_postures(self) -> u64 {
        self.unsupported_postures
    }

    pub const fn unavailable_postures(self) -> u64 {
        self.unavailable_postures
    }

    pub const fn witnesses_issued(self) -> u64 {
        self.witnesses_issued
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub(crate) struct StoreSecurityScopeAdmissionCounters {
    snapshot: StoreSecurityScopeAdmissionCounterSnapshot,
}

impl StoreSecurityScopeAdmissionCounters {
    pub(crate) fn start_request() -> Self {
        let mut counters = Self::default();
        counters.snapshot.requests = 1;
        counters.snapshot.current_authority_checks = 1;
        counters
    }

    pub(crate) fn check_physical_binding(&mut self) {
        self.snapshot.physical_binding_checks += 1;
    }

    pub(crate) fn check_key_scope(&mut self) {
        self.snapshot.key_scope_checks += 1;
    }

    pub(crate) fn check_key_version(&mut self) {
        self.snapshot.key_version_checks += 1;
    }

    pub(crate) fn check_tenant_scope(&mut self) {
        self.snapshot.tenant_scope_checks += 1;
    }

    pub(crate) fn check_authenticity_requirement(&mut self) {
        self.snapshot.authenticity_requirement_checks += 1;
    }

    pub(crate) fn check_custody_posture(&mut self) {
        self.snapshot.custody_posture_checks += 1;
    }

    pub(crate) fn record_denial(&mut self) {
        self.snapshot.denials += 1;
    }

    pub(crate) fn record_readmission_required(&mut self) {
        self.snapshot.readmission_required += 1;
    }

    pub(crate) fn record_replayed_admission_evidence(&mut self) {
        self.snapshot.replayed_admission_evidence += 1;
    }

    pub(crate) fn record_wrong_physical_scope(&mut self) {
        self.snapshot.wrong_physical_scopes += 1;
    }

    pub(crate) fn record_wrong_key_scope(&mut self) {
        self.snapshot.wrong_key_scopes += 1;
    }

    pub(crate) fn record_wrong_tenant_scope(&mut self) {
        self.snapshot.wrong_tenant_scopes += 1;
    }

    pub(crate) fn record_missing_authenticity_requirement(&mut self) {
        self.snapshot.missing_authenticity_requirements += 1;
    }

    pub(crate) fn record_unsupported_authenticity_requirement(&mut self) {
        self.snapshot.unsupported_authenticity_requirements += 1;
    }

    pub(crate) fn record_missing_custody_posture(&mut self) {
        self.snapshot.missing_custody_postures += 1;
    }

    pub(crate) fn record_stale_key_posture(&mut self) {
        self.snapshot.stale_key_postures += 1;
    }

    pub(crate) fn record_rebind_required_key_posture(&mut self) {
        self.snapshot.rebind_required_key_postures += 1;
    }

    pub(crate) fn record_unsupported_posture(&mut self) {
        self.snapshot.unsupported_postures += 1;
    }

    pub(crate) fn record_unavailable_posture(&mut self) {
        self.snapshot.unavailable_postures += 1;
    }

    pub(crate) fn record_witnesses_issued(&mut self) {
        self.snapshot.witnesses_issued = 4;
    }

    pub(crate) const fn snapshot(self) -> StoreSecurityScopeAdmissionCounterSnapshot {
        self.snapshot
    }
}
