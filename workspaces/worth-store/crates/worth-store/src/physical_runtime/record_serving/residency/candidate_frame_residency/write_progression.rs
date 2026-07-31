use super::*;

fn contract<EffectFailure>(
    violation: CandidateFrameContractViolation,
    posture: CandidateFrameFailurePosture,
) -> CandidateFrameWriteFailure<EffectFailure> {
    CandidateFrameWriteFailure::Contract { violation, posture }
}

fn residency<EffectFailure>(
    denial: RecordAppendDenial,
    posture: CandidateFrameFailurePosture,
) -> CandidateFrameWriteFailure<EffectFailure> {
    CandidateFrameWriteFailure::Residency { denial, posture }
}

pub(in crate::physical_runtime::record_serving) trait CandidateFrameEffectFailure {
    fn effect_fate(&self) -> crate::physical_runtime::PhysicalWorkEffectFate;
}

impl CandidateFrameEffectFailure for super::super::super::CanonicalRecordMutationFailure {
    fn effect_fate(&self) -> crate::physical_runtime::PhysicalWorkEffectFate {
        self.effect_fate()
    }
}

impl StoreCandidateFramePublicationSession<'_> {
    pub(in crate::physical_runtime::record_serving::residency) fn write_frame<EffectFailure>(
        &mut self,
        frame: CandidateFrame,
        store_write: &mut dyn FnMut(&[u8]) -> Result<CandidateFramePhysicalWrite, EffectFailure>,
    ) -> Result<CandidateFrameWriteCompletion, CandidateFrameWriteFailure<EffectFailure>>
    where
        EffectFailure: CandidateFrameEffectFailure,
    {
        let (resident, expectation) = self.retain_submitted_frame(frame)?;
        let physical = match store_write(resident.bytes()) {
            Ok(physical) => physical,
            Err(failure) => {
                if failure.effect_fate()
                    == crate::physical_runtime::PhysicalWorkEffectFate::ProvenNoEffect
                {
                    resident.discard().map_err(|denial| {
                        residency(denial, CandidateFrameFailurePosture::UnsettledBeforeEffect)
                    })?;
                }
                return Err(CandidateFrameWriteFailure::Effect(failure));
            }
        };
        let settlement = physical
            .settle_residency(
                resident.store_identity(),
                resident.coordinate(),
                resident.bytes(),
            )
            .map_err(|violation| {
                contract(violation, CandidateFrameFailurePosture::EffectPossible)
            })?;
        let completion = resident
            .publish_clean(settlement)
            .map_err(|denial| residency(denial, CandidateFrameFailurePosture::EffectPossible))?;
        self.complete_frame(expectation, &completion)?;
        Ok(completion)
    }

    pub(super) fn retain_submitted_frame<EffectFailure>(
        &mut self,
        frame: CandidateFrame,
    ) -> Result<
        (Box<dyn ResidentCandidateFrame>, RetainedFrameExpectation),
        CandidateFrameWriteFailure<EffectFailure>,
    > {
        let expected = self.validate_submitted_frame(&frame)?;
        let expectation = RetainedFrameExpectation::capture(expected, &frame);
        let next_bytes = self.resident_bytes.saturating_add(expectation.frame_bytes);
        if next_bytes > self.declaration.total_frame_bytes() {
            return Err(contract(
                CandidateFrameContractViolation::FrameBytesExceedDeclaration,
                CandidateFrameFailurePosture::ProvenNoEffect,
            ));
        }
        let resident = self
            .residency
            .retain(frame)
            .map_err(|denial| residency(denial, CandidateFrameFailurePosture::ProvenNoEffect))?;
        if let Err(violation) = verify_retained_frame(resident.as_ref(), expectation) {
            resident.discard().map_err(|denial| {
                residency(denial, CandidateFrameFailurePosture::UnsettledBeforeEffect)
            })?;
            return Err(contract(
                violation,
                CandidateFrameFailurePosture::ProvenNoEffect,
            ));
        }
        Ok((resident, expectation))
    }

    pub(super) fn complete_frame<EffectFailure>(
        &mut self,
        expectation: RetainedFrameExpectation,
        completion: &CandidateFrameWriteCompletion,
    ) -> Result<(), CandidateFrameWriteFailure<EffectFailure>> {
        if completion.frame_bytes() != expectation.frame_bytes {
            return Err(contract(
                CandidateFrameContractViolation::FrameCompletionMismatch,
                CandidateFrameFailurePosture::EffectPossible,
            ));
        }
        self.resident_frames = self.resident_frames.saturating_add(1);
        self.resident_bytes = self.resident_bytes.saturating_add(expectation.frame_bytes);
        self.next_declaration += 1;
        Ok(())
    }

    fn validate_submitted_frame<EffectFailure>(
        &self,
        frame: &CandidateFrame,
    ) -> Result<CandidateFrameDeclaration, CandidateFrameWriteFailure<EffectFailure>> {
        if !coordinate_matches_role(frame.role(), frame.coordinate().artifact()) {
            return Err(contract(
                CandidateFrameContractViolation::CoordinateRoleMismatch,
                CandidateFrameFailurePosture::ProvenNoEffect,
            ));
        }
        let Some(expected) = self.declaration.declaration(self.next_declaration) else {
            return Err(contract(
                CandidateFrameContractViolation::FrameCountExceedsDeclaration,
                CandidateFrameFailurePosture::ProvenNoEffect,
            ));
        };
        if expected.role != frame.role()
            || expected.coordinate != frame.coordinate()
            || u64::from(expected.length) != frame.bytes().len() as u64
        {
            return Err(contract(
                CandidateFrameContractViolation::UnexpectedFrame,
                CandidateFrameFailurePosture::ProvenNoEffect,
            ));
        }
        Ok(expected)
    }
}

#[derive(Clone, Copy)]
pub(super) struct RetainedFrameExpectation {
    declaration: CandidateFrameDeclaration,
    role: CandidateFrameRole,
    coordinate: CandidateFrameCoordinate,
    frame_bytes: u64,
    checksum: u32,
}

impl RetainedFrameExpectation {
    fn capture(declaration: CandidateFrameDeclaration, frame: &CandidateFrame) -> Self {
        Self {
            declaration,
            role: frame.role(),
            coordinate: frame.coordinate(),
            frame_bytes: frame.bytes().len() as u64,
            checksum: frame.checksum(),
        }
    }

    pub(super) const fn frame_bytes(self) -> u64 {
        self.frame_bytes
    }
}

fn verify_retained_frame(
    resident: &dyn ResidentCandidateFrame,
    expected: RetainedFrameExpectation,
) -> Result<(), CandidateFrameContractViolation> {
    if resident.bytes().len() as u64 != expected.frame_bytes
        || resident.role() != expected.role
        || resident.coordinate() != expected.coordinate
        || expected.declaration.role != expected.role
        || expected.declaration.coordinate != expected.coordinate
    {
        return Err(CandidateFrameContractViolation::RetainedFrameMismatch);
    }
    if worth_store_physical_format::durable_artifact_checksum(resident.bytes()) != expected.checksum
    {
        return Err(CandidateFrameContractViolation::RetainedFrameBytesChanged);
    }
    Ok(())
}
