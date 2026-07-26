use super::*;

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
        let expected = self.validate_submitted_frame(&frame)?;
        let expectation = RetainedFrameExpectation::capture(expected, &frame);
        let next_frames = self.resident_frames.saturating_add(1);
        let next_bytes = self.resident_bytes.saturating_add(expectation.frame_bytes);
        if next_bytes > self.declaration.total_frame_bytes() {
            return Err(CandidateFrameWriteFailure::Contract(
                CandidateFrameContractViolation::FrameBytesExceedDeclaration,
            ));
        }

        let resident = self
            .residency
            .retain(frame)
            .map_err(CandidateFrameWriteFailure::Residency)?;
        if let Err(violation) = verify_retained_frame(resident.as_ref(), expectation) {
            resident
                .discard()
                .map_err(CandidateFrameWriteFailure::Residency)?;
            return Err(CandidateFrameWriteFailure::Contract(violation));
        }
        let physical = match store_write(resident.bytes()) {
            Ok(physical) => physical,
            Err(failure) => {
                if failure.effect_fate()
                    == crate::physical_runtime::PhysicalWorkEffectFate::ProvenNoEffect
                {
                    resident
                        .discard()
                        .map_err(CandidateFrameWriteFailure::Residency)?;
                }
                return Err(CandidateFrameWriteFailure::Effect(failure));
            }
        };
        let completion = resident
            .publish_clean(&physical)
            .map_err(CandidateFrameWriteFailure::Residency)?;
        if completion.frame_bytes() != expectation.frame_bytes {
            return Err(CandidateFrameWriteFailure::Contract(
                CandidateFrameContractViolation::FrameCompletionMismatch,
            ));
        }
        self.resident_frames = next_frames;
        self.resident_bytes = next_bytes;
        self.next_declaration += 1;
        Ok(completion)
    }

    fn validate_submitted_frame<EffectFailure>(
        &self,
        frame: &CandidateFrame,
    ) -> Result<CandidateFrameDeclaration, CandidateFrameWriteFailure<EffectFailure>> {
        if !coordinate_matches_role(frame.role(), frame.coordinate().artifact()) {
            return Err(CandidateFrameWriteFailure::Contract(
                CandidateFrameContractViolation::CoordinateRoleMismatch,
            ));
        }
        let Some(expected) = self.declaration.declaration(self.next_declaration) else {
            return Err(CandidateFrameWriteFailure::Contract(
                CandidateFrameContractViolation::FrameCountExceedsDeclaration,
            ));
        };
        if expected.role != frame.role()
            || expected.coordinate != frame.coordinate()
            || u64::from(expected.length) != frame.bytes().len() as u64
        {
            return Err(CandidateFrameWriteFailure::Contract(
                CandidateFrameContractViolation::UnexpectedFrame,
            ));
        }
        Ok(expected)
    }
}

#[derive(Clone, Copy)]
struct RetainedFrameExpectation {
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
