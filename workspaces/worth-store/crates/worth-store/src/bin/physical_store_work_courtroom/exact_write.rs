use worth_foundational::{
    aspects, performance, AspectMask, AspectValue, ContractValidationInput,
    FoundationalPerformanceAccessPatternPosture, FoundationalPerformanceBoundary,
    FoundationalPerformanceBreadthLocalityPosture, FoundationalPerformanceBudgetKind,
    FoundationalPerformanceEvidenceStrength, FoundationalPerformanceExecutionTemperature,
    FoundationalPerformanceFallbackDebtPosture, FoundationalPerformanceFreshnessRetentionPosture,
    FoundationalPerformanceWorkClass, InternedString, MutationMask, ProjectionMask,
    ScalarAspectType,
};
use worth_proof::TransitionOutcome;
use worth_store_aspect_native::{
    StoreAspectAuthorityInput, StoreAspectBoundaryFact, StoreAspectContractAdmission,
    StoreAspectIdentity, StoreAspectPatchAuthorityInput, StoreAspectPatchBoundaryFact,
    StorePhysicalBoundaryWitness,
};
use worth_store_io_scheduler::foreground_reservation::{
    BandwidthToken, CacheResidencyHint, DirtyPageBudget, ForegroundLaneDeclaration,
    ForegroundLatencyEnvelope, ForegroundResourceBudget, QueueSlot, WorkerPermit, WriteBackWindow,
};
use worth_store_physical_backend::ArtifactRangeWriteDurabilityRequirement;
use worth_store_physical_format::{RecordArtifactFile, RecordFrameCoordinate};
use worth_store_security::{
    StoreAuthenticityRequirement, StoreCustodyPosture, StoreKeyScope, StoreKeyVersionPosture,
    StoreSecurityScopeAdmissionExpectation, StoreSecurityScopeAdmissionRequest, StoreTenantScope,
};

use worth_store::physical_runtime::{
    PhysicalExecutorCommand, PhysicalMutationWorkRequest, PhysicalSchedulerDemand,
    PhysicalWorkProfileDeclaration, PhysicalWorkReadiness, PhysicalWorkScope,
    PhysicalWorkSemanticBasis, ServingPhysicalRuntime,
};

const BOOTSTRAP_MAGIC: &[u8; 8] = b"WRC5FRM\0";

struct ExactWriteProfileAdmission {
    witness: StorePhysicalBoundaryWitness,
    contract: worth_foundational::AspectContract,
    identity: StoreAspectIdentity,
    admission: StoreAspectContractAdmission,
    security: worth_store_security::StoreAuthorityBoundSecurityScopeReceipt,
}

pub(super) fn profile() -> Result<PhysicalWorkProfileDeclaration, String> {
    let admitted = admit_profile()?;
    PhysicalWorkProfileDeclaration::new(admitted.security, [admitted.admission])
        .map_err(|denial| format!("courtroom physical-work profile denied: {denial:?}"))
}

pub(super) fn bind() -> Result<(PhysicalWorkProfileDeclaration, PhysicalMutationWorkRequest), String>
{
    let admitted = admit_profile()?;
    let basis = mutation_basis(
        &admitted.contract,
        admitted.identity,
        admitted.witness,
        admitted.admission.clone(),
    )?;
    let profile = PhysicalWorkProfileDeclaration::new(admitted.security, [admitted.admission])
        .map_err(|denial| format!("courtroom physical-work profile denied: {denial:?}"))?;
    let coordinate = RecordFrameCoordinate::new(RecordArtifactFile::BootstrapCatalog, 0, 8)
        .ok_or_else(|| "courtroom exact-write coordinate was denied".to_owned())?;
    let request = PhysicalMutationWorkRequest::exact_write(
        PhysicalWorkScope::one(coordinate),
        basis,
        admitted.security,
        ArtifactRangeWriteDurabilityRequirement::BufferedWrite,
    )
    .map_err(|denial| format!("courtroom exact-write request denied: {denial:?}"))?;
    Ok((profile, request))
}

fn admit_profile() -> Result<ExactWriteProfileAdmission, String> {
    let witness = physical_witness()?;
    let key = aspects()
        .vocabulary()
        .key("store.physical.courtroom.exact-write")
        .map_err(|denial| format!("courtroom exact-write aspect key was denied: {denial:?}"))?;
    let contract = aspects()
        .contract()
        .for_key(key.clone())
        .identified_by(aspects().vocabulary().identity(1_541))
        .at_revision(aspects().vocabulary().revision(1))
        .scalar(ScalarAspectType::String);
    let identity = StoreAspectIdentity::from_aspect_key(key);
    let admission = StoreAspectContractAdmission::new(identity.clone(), contract.clone(), witness)
        .map_err(|denial| format!("courtroom aspect admission denied: {denial:?}"))?
        .admit_projection_mask(AspectMask::<ProjectionMask>::whole_aspect())
        .map_err(|denial| format!("courtroom projection admission denied: {denial:?}"))?
        .admit_mutation_mask(AspectMask::<MutationMask>::whole_aspect())
        .map_err(|denial| format!("courtroom mutation admission denied: {denial:?}"))?;
    let authority = boundary_fact(&contract, identity.clone(), witness)?;
    let security = security_scope(authority)?;
    Ok(ExactWriteProfileAdmission {
        witness,
        contract,
        identity,
        admission,
        security,
    })
}

pub(super) fn prepare_command(
    serving: &ServingPhysicalRuntime,
    request: PhysicalMutationWorkRequest,
) -> Result<PhysicalExecutorCommand, String> {
    let receipt = match serving
        .physical_mutation_submission()
        .submit(request)
        .into_raw()
    {
        TransitionOutcome::Success(receipt) => receipt,
        outcome => {
            return Err(format!(
                "courtroom exact-write submission denied: {outcome:?}"
            ))
        }
    };
    let admitted = serving
        .admit_physical_work(receipt)
        .map_err(|denial| format!("courtroom exact-write admission denied: {denial:?}"))?;
    let ready = match serving
        .request_physical_work(admitted)
        .map_err(|denial| format!("courtroom exact-write request denied: {denial:?}"))?
    {
        PhysicalWorkReadiness::Ready(ready) => ready,
        PhysicalWorkReadiness::Blocked(blocked) => {
            return Err(format!(
                "courtroom exact write unexpectedly blocked: {:?}",
                blocked.condition()
            ))
        }
    };
    let (reservation, backend) = serving
        .reserve_physical_scheduler_foreground(write_lane())
        .map_err(|denial| format!("courtroom write reservation denied: {denial:?}"))?;
    let demand = PhysicalSchedulerDemand::foreground(ready, reservation, None)
        .map_err(|denial| format!("courtroom write demand denied: {denial:?}"))?;
    let requested_budget = demand.queue_work().requested_budget();
    let admitted = serving
        .admit_physical_scheduler_demand(demand, &backend, policy(requested_budget))
        .map_err(|denial| format!("courtroom scheduler admission denied: {denial:?}"))?;
    PhysicalExecutorCommand::exact_write(admitted, BOOTSTRAP_MAGIC.as_slice())
        .map_err(|denial| format!("courtroom exact-write command denied: {denial:?}"))
}

fn physical_witness() -> Result<StorePhysicalBoundaryWitness, String> {
    let authority =
        worth_store_contracts::StorePhysicalAuthorityWitness::for_aspect_native_boundary(
            worth_store_contracts::ROADMAP_2_ASPECT_NATIVE_GATE_SCOPE,
        )
        .map_err(|denial| format!("courtroom physical authority was denied: {denial:?}"))?;
    StorePhysicalBoundaryWitness::from_physical_authority(authority)
        .map_err(|denial| format!("courtroom physical witness denied: {denial:?}"))
}

fn boundary_fact(
    contract: &worth_foundational::AspectContract,
    identity: StoreAspectIdentity,
    witness: StorePhysicalBoundaryWitness,
) -> Result<StoreAspectBoundaryFact, String> {
    let value = validated_value(contract, "courtroom-exact-write-authority")?;
    let state = match aspects().authoritative_state().admit([value]) {
        TransitionOutcome::Success(state) => state,
        outcome => return Err(format!("courtroom authority state denied: {outcome:?}")),
    };
    StoreAspectBoundaryFact::from_admitted_state(
        identity,
        StoreAspectAuthorityInput::new(state, witness),
    )
    .map_err(|denial| format!("courtroom authority fact denied: {denial:?}"))
}

fn mutation_basis(
    contract: &worth_foundational::AspectContract,
    identity: StoreAspectIdentity,
    witness: StorePhysicalBoundaryWitness,
    admission: StoreAspectContractAdmission,
) -> Result<PhysicalWorkSemanticBasis, String> {
    let patch = match aspects()
        .patch()
        .whole_aspect()
        .set(validated_value(contract, "courtroom-exact-write-mutation")?)
        .finish()
    {
        TransitionOutcome::Success(patch) => patch,
        outcome => return Err(format!("courtroom mutation patch denied: {outcome:?}")),
    };
    let fact = StoreAspectPatchBoundaryFact::from_authoritative_patch(
        identity,
        StoreAspectPatchAuthorityInput::new(patch, witness),
    )
    .map_err(|denial| format!("courtroom mutation fact denied: {denial:?}"))?;
    PhysicalWorkSemanticBasis::mutation(fact, admission)
        .map_err(|denial| format!("courtroom mutation basis denied: {denial:?}"))
}

fn validated_value(
    contract: &worth_foundational::AspectContract,
    value: &'static str,
) -> Result<worth_foundational::ContractValidatedAspectArtifact, String> {
    match aspects()
        .validate()
        .against(contract)
        .value(ContractValidationInput::from(AspectValue::String(
            InternedString::from(value),
        ))) {
        TransitionOutcome::Success(value) => Ok(value),
        outcome => Err(format!("courtroom aspect value denied: {outcome:?}")),
    }
}

fn security_scope(
    authority: StoreAspectBoundaryFact,
) -> Result<worth_store_security::StoreAuthorityBoundSecurityScopeReceipt, String> {
    let current = worth_store_authority::require_current_store_authority(authority);
    let authenticity = StoreAuthenticityRequirement::not_required();
    let expectation = StoreSecurityScopeAdmissionExpectation::new(
        StoreKeyScope::StoreManagedRoot,
        StoreTenantScope::StoreInternal,
        authenticity,
        StoreCustodyPosture::InternalStoreCustody,
    );
    let request = StoreSecurityScopeAdmissionRequest::new(
        &current,
        StoreKeyScope::StoreManagedRoot,
        StoreKeyVersionPosture::Current,
        StoreTenantScope::StoreInternal,
        authenticity,
        StoreCustodyPosture::InternalStoreCustody,
        expectation,
    );
    match worth_store_security::admit_store_security_scope(request) {
        TransitionOutcome::Success(scope) => Ok(scope.authority_bound_receipt()),
        outcome => Err(format!("courtroom security scope denied: {outcome:?}")),
    }
}

fn write_lane() -> ForegroundLaneDeclaration {
    ForegroundLaneDeclaration::ordinary_page_write()
        .with_latency_envelope(ForegroundLatencyEnvelope::bounded_interference(
            "hostile-exact-write",
            1,
        ))
        .with_budget(
            ForegroundResourceBudget::new()
                .with_queue_slots(QueueSlot::new(1).unwrap())
                .with_bandwidth(BandwidthToken::bytes(8).unwrap())
                .with_write_back(WriteBackWindow::pages(1).unwrap())
                .with_dirty_pages(DirtyPageBudget::pages(1).unwrap())
                .with_worker_permits(WorkerPermit::new(1).unwrap())
                .with_cache_residency(CacheResidencyHint::frames(1).unwrap()),
        )
}

fn policy(
    budget: worth_store_io_scheduler::BackgroundResourceBudget,
) -> worth_foundational::FoundationalPolicyAdmissionReceipt {
    let claim = performance()
        .claim()
        .policy_admission()
        .boundary(FoundationalPerformanceBoundary::AuthoritativeExecution)
        .evidence_strength(FoundationalPerformanceEvidenceStrength::RuntimePolicyAdmission)
        .breadth_locality(FoundationalPerformanceBreadthLocalityPosture::DeltaBound)
        .access_pattern(FoundationalPerformanceAccessPatternPosture::PointLookup)
        .execution_temperature(FoundationalPerformanceExecutionTemperature::HotPath)
        .freshness_retention(FoundationalPerformanceFreshnessRetentionPosture::ExactBasisCurrent)
        .fallback_debt(FoundationalPerformanceFallbackDebtPosture::Verified)
        .include_work(FoundationalPerformanceWorkClass::AuthoritativeMutation)
        .exclude_work(FoundationalPerformanceWorkClass::SupportReportAssembly)
        .finish()
        .unwrap();
    let receipt = performance().policy_admission_receipt(claim);
    [
        (
            FoundationalPerformanceBudgetKind::Breadth,
            budget.queue_slots() + budget.worker_permits(),
        ),
        (
            FoundationalPerformanceBudgetKind::Density,
            budget.bandwidth_tokens() + budget.cache_residency_hints(),
        ),
        (
            FoundationalPerformanceBudgetKind::Locality,
            budget.read_ahead_window() + budget.write_back_window() + budget.reclaim_permits(),
        ),
    ]
    .into_iter()
    .filter(|(_, units)| *units != 0)
    .fold(receipt, |receipt, (kind, units)| {
        receipt.budget_decision(kind, units as u32, units as u32)
    })
    .finish()
    .unwrap()
}
