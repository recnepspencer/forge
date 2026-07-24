use worth_foundational::facade::{AspectKey, AspectValue, CanonicalF64};
use worth_query::facade::domain;

use super::contract::{
    candidate_content, candidate_id, candidate_layout, candidate_score, candidate_target,
    candidate_token, misaligned_candidate_layout,
};
use super::provider::ArtifactProbe;

const CANDIDATE_ROWS: usize = 32;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum NativeProviderMode {
    Standard,
    ShortChunks,
    Misaligned,
    SessionMismatch,
    ZeroProgress,
    Panic,
}

impl NativeProviderMode {
    pub(super) fn for_scenario(scenario: &str) -> Self {
        match scenario {
            "native-short-chunks" => Self::ShortChunks,
            "native-provider-alignment" => Self::Misaligned,
            "native-session-mismatch" => Self::SessionMismatch,
            "native-zero-progress" => Self::ZeroProgress,
            "native-provider-panic" => Self::Panic,
            _ => Self::Standard,
        }
    }
}

pub(super) struct CandidateNativeRows {
    probe: ArtifactProbe,
    mode: NativeProviderMode,
    ids: Vec<u64>,
    scores: Vec<CanonicalF64>,
    tokens: Vec<u64>,
    targets: Vec<u64>,
    contents: Vec<u64>,
}

impl CandidateNativeRows {
    pub(super) fn new(probe: ArtifactProbe, mode: NativeProviderMode) -> Self {
        let ids = (0..CANDIDATE_ROWS)
            .map(|row| 1_000 + row as u64)
            .collect::<Vec<_>>();
        let scores = (0..CANDIDATE_ROWS)
            .map(|row| CanonicalF64::from_f64(0.25 + row as f64 * 0.5))
            .collect::<Vec<_>>();
        let tokens = (0..CANDIDATE_ROWS)
            .map(|row| 0xA500 + row as u64)
            .collect::<Vec<_>>();
        let targets = (0..CANDIDATE_ROWS)
            .map(|row| 0xB600 + row as u64 * 3)
            .collect::<Vec<_>>();
        let contents = (0..CANDIDATE_ROWS)
            .map(|row| 0xC700 + row as u64 * 5)
            .collect::<Vec<_>>();
        Self {
            probe,
            mode,
            ids,
            scores,
            tokens,
            targets,
            contents,
        }
    }

    fn admit_session(
        &self,
        session: &domain::WorthQueryArtifactProviderAccessSession,
    ) -> Result<(), domain::WorthQueryArtifactProviderAccessDenial> {
        if self.mode == NativeProviderMode::SessionMismatch {
            return Err(domain::WorthQueryArtifactProviderAccessDenial::SessionMismatch);
        }
        if session.layout() != &candidate_layout() || session.generation() != 1 {
            return Err(domain::WorthQueryArtifactProviderAccessDenial::SessionMismatch);
        }
        Ok(())
    }

    fn admitted_range(
        &self,
        start_row: usize,
        max_rows: usize,
    ) -> Result<std::ops::Range<usize>, domain::WorthQueryArtifactProviderAccessDenial> {
        if start_row > self.ids.len() {
            return Err(domain::WorthQueryArtifactProviderAccessDenial::BoundsExceeded);
        }
        let available = self.ids.len() - start_row;
        let mode_limit = match self.mode {
            NativeProviderMode::ShortChunks => 3,
            NativeProviderMode::ZeroProgress => 0,
            _ => max_rows,
        };
        let width = available.min(max_rows).min(mode_limit);
        Ok(start_row..start_row + width)
    }

    fn source_bytes(&self, rows: usize, fields: &[AspectKey]) -> usize {
        fields.iter().fold(0, |bytes, field| {
            let width = if field == &candidate_id() {
                std::mem::size_of::<u64>()
            } else if field == &candidate_score() {
                std::mem::size_of::<CanonicalF64>()
            } else {
                0
            };
            bytes.saturating_add(rows.saturating_mul(width))
        })
    }

    fn projection_signature(&self, row: usize) -> u64 {
        self.tokens[row] ^ self.targets[row] ^ self.contents[row]
    }
}

impl domain::WorthQueryArtifactNativeAccessProvider for CandidateNativeRows {
    fn layout(&self) -> domain::WorthQueryArtifactNativeLayoutReference {
        if self.mode == NativeProviderMode::Misaligned {
            misaligned_candidate_layout()
        } else {
            candidate_layout()
        }
    }

    fn row_count(
        &self,
        session: &domain::WorthQueryArtifactProviderAccessSession,
    ) -> Result<usize, domain::WorthQueryArtifactProviderAccessDenial> {
        self.admit_session(session)?;
        self.probe.observe_native_row_count();
        Ok(self.ids.len())
    }

    fn borrow_rows<'a>(
        &'a self,
        session: &domain::WorthQueryArtifactProviderAccessSession,
        start_row: usize,
        max_rows: usize,
        fields: &[AspectKey],
    ) -> Result<
        domain::WorthQueryArtifactProviderBorrowedBatch<'a>,
        domain::WorthQueryArtifactProviderAccessDenial,
    > {
        self.admit_session(session)?;
        self.probe.observe_native_row_batch();
        if self.mode == NativeProviderMode::Panic {
            panic!("native provider row access panic");
        }
        let range = self.admitted_range(start_row, max_rows)?;
        let columns = fields
            .iter()
            .map(|field| {
                let values = if field == &candidate_id() {
                    domain::WorthQueryArtifactProviderFieldSlice::UInt64(&self.ids[range.clone()])
                } else if field == &candidate_score() {
                    domain::WorthQueryArtifactProviderFieldSlice::Float64(
                        &self.scores[range.clone()],
                    )
                } else {
                    return Err(domain::WorthQueryArtifactProviderAccessDenial::Unsupported);
                };
                Ok((field.clone(), values))
            })
            .collect::<Result<Vec<_>, _>>()?;
        Ok(domain::WorthQueryArtifactProviderBorrowedBatch::new(
            start_row,
            range.len(),
            self.source_bytes(range.len(), fields),
            columns,
        ))
    }

    fn borrow_field<'a>(
        &'a self,
        session: &domain::WorthQueryArtifactProviderAccessSession,
        start_row: usize,
        max_rows: usize,
        field: &AspectKey,
    ) -> Result<
        domain::WorthQueryArtifactProviderFieldSlice<'a>,
        domain::WorthQueryArtifactProviderAccessDenial,
    > {
        self.admit_session(session)?;
        self.probe.observe_native_field_slice();
        let range = self.admitted_range(start_row, max_rows)?;
        if field == &candidate_id() {
            Ok(domain::WorthQueryArtifactProviderFieldSlice::UInt64(
                &self.ids[range],
            ))
        } else if field == &candidate_score() {
            Ok(domain::WorthQueryArtifactProviderFieldSlice::Float64(
                &self.scores[range],
            ))
        } else {
            Err(domain::WorthQueryArtifactProviderAccessDenial::Unsupported)
        }
    }

    fn project_rows(
        &self,
        session: &domain::WorthQueryArtifactProviderAccessSession,
        projection_identity: &str,
        start_row: usize,
        max_rows: usize,
        sink: &mut domain::WorthQueryArtifactProjectionSink,
    ) -> Result<usize, domain::WorthQueryArtifactProviderAccessDenial> {
        self.admit_session(session)?;
        self.probe.observe_native_projection();
        let range = self.admitted_range(start_row, max_rows)?;
        for row in range.clone() {
            match projection_identity {
                "candidate-summary-v1" => sink.push_row([
                    AspectValue::UInt64(self.ids[row]),
                    AspectValue::Float64(self.scores[row]),
                ])?,
                "candidate-provenance-v1" => {
                    sink.push_row([AspectValue::UInt64(self.projection_signature(row))])?
                }
                _ => return Err(domain::WorthQueryArtifactProviderAccessDenial::Unsupported),
            }
        }
        Ok(range.len())
    }

    fn scalar<'a>(
        &'a self,
        session: &domain::WorthQueryArtifactProviderAccessSession,
        row: usize,
        field: &AspectKey,
    ) -> Result<
        domain::WorthQueryArtifactProviderValueView<'a>,
        domain::WorthQueryArtifactProviderAccessDenial,
    > {
        self.admit_session(session)?;
        self.probe.observe_native_scalar();
        if field == &candidate_id() {
            self.ids
                .get(row)
                .map(domain::WorthQueryArtifactProviderValueView::UInt64)
        } else if field == &candidate_score() {
            self.scores
                .get(row)
                .map(domain::WorthQueryArtifactProviderValueView::Float64)
        } else {
            return Err(domain::WorthQueryArtifactProviderAccessDenial::Unsupported);
        }
        .ok_or(domain::WorthQueryArtifactProviderAccessDenial::BoundsExceeded)
    }
}

pub(super) fn provider_native_source_fields() -> [AspectKey; 3] {
    [candidate_token(), candidate_target(), candidate_content()]
}
