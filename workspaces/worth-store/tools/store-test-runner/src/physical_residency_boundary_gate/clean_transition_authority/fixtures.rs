pub(super) fn honest_ownership() -> &'static str {
    r#"
pub struct PhysicalResidencyPoolOwner { pool: PhysicalResidencyPool }
pub struct CandidateFrameCleanAuthority { owner: Arc<PoolInner> }
pub struct FrameWritebackCleanAuthority { owner: Arc<PoolInner> }
pub fn open(store: Store, limits: Limits) -> Result<Self, Denial> {
    let pool = PhysicalResidencyPool::open(store, limits)?;
    let candidate_clean = CandidateFrameCleanAuthority { owner: Arc::clone(&pool.inner) };
    let writeback_clean = FrameWritebackCleanAuthority { owner: Arc::clone(&pool.inner) };
}
pub fn into_parts(self) { (self.pool, self.candidate_clean, self.writeback_clean) }
impl CandidateFrameCleanAuthority {
    fn authorizes(&self, owner: &Arc<PoolInner>) -> bool { Arc::ptr_eq(&self.owner, owner) }
}
impl FrameWritebackCleanAuthority {
    fn authorizes(&self, owner: &Arc<PoolInner>) -> bool { Arc::ptr_eq(&self.owner, owner) }
}
"#
}

pub(super) fn honest_dirty() -> &'static str {
    r#"
pub fn complete_candidate_publication(
    self,
    authority: &CandidateFrameCleanAuthority,
) -> Result<Lease, Denial> {
    let lease = self.lease.as_ref().unwrap();
    if !authority.authorizes(&lease.owner) {
        return Err(CandidateCleanAuthorityMismatch);
    }
}
"#
}

pub(super) fn honest_writeback() -> &'static str {
    r#"
pub fn complete_writeback(
    self,
    authority: &FrameWritebackCleanAuthority,
) -> Result<(), Denial> {
    if !authority.authorizes(&self.owner) {
        return Err(WritebackCleanAuthorityMismatch);
    }
}
"#
}

pub(super) fn honest_frame_ports() -> &'static str {
    r#"
struct RecordFramePorts {
    writeback_clean: Arc<FrameWritebackCleanAuthority>,
}
fn bounded(store: Store, limits: Limits) {
    let (pool, candidate_clean, writeback_clean) =
        PhysicalResidencyPoolOwner::open(store, limits)?.into_parts();
}
"#
}

pub(super) fn honest_candidate_publisher() -> &'static str {
    r#"
struct BoundedResidentCandidateFrame {
    candidate_clean: Arc<CandidateFrameCleanAuthority>,
}
fn publish_clean(self, settlement: Settlement) {
    let _settlement = settlement.settlement();
    self.resident.complete_candidate_publication(&self.candidate_clean);
}
"#
}

pub(super) fn honest_writeback_completion() -> &'static str {
    r#"
fn publish_clean(self, authority: &FrameWritebackCleanAuthority) {
    if !receipt_matches_claim(&self.claim, &self.receipt) { return Err(()); }
    self.claim.complete_writeback(authority);
}
"#
}

pub(super) fn honest_writeback_execution() -> &'static str {
    r#"
fn execute(self) {
    if settled_success {
        completion.publish_clean(self.frame_ports.writeback_clean_authority());
    }
}
"#
}
