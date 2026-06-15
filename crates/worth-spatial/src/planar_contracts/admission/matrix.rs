use worth_primitives::{truth_digest_parts, TruthDigestScope};

use super::classification::classify_planar_admission;
use super::premetaboss_rows::{premetaboss_admission_rows, PlanarPremetabossAdmissionRow};
use super::{
    PlanarAdmissionClass, PlanarAdmissionFamily, PlanarAdmissionReason, PlanarQueryPosture,
    PlanarRuntimeConcern,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarAdmissionRow {
    family: PlanarAdmissionFamily,
    concern: PlanarRuntimeConcern,
    class: PlanarAdmissionClass,
    query_posture: PlanarQueryPosture,
    reason: PlanarAdmissionReason,
    row_digest: String,
}

impl PlanarAdmissionRow {
    fn new(family: PlanarAdmissionFamily, concern: PlanarRuntimeConcern) -> Self {
        let (class, query_posture, reason) = classify_planar_admission(family, concern);
        let mut parts = vec![
            format!("family:{}", family.as_str()),
            format!("concern:{}", concern.as_str()),
            format!("class:{}", class.as_str()),
            format!("reason:{}", reason.as_str()),
        ];
        parts.extend(query_posture.digest_parts());
        let row_digest = hash_parts(&parts);
        Self {
            family,
            concern,
            class,
            query_posture,
            reason,
            row_digest,
        }
    }

    pub fn family(&self) -> PlanarAdmissionFamily {
        self.family
    }

    pub fn concern(&self) -> PlanarRuntimeConcern {
        self.concern
    }

    pub fn class(&self) -> PlanarAdmissionClass {
        self.class
    }

    pub fn query_posture(&self) -> &PlanarQueryPosture {
        &self.query_posture
    }

    pub fn reason(&self) -> PlanarAdmissionReason {
        self.reason
    }

    pub fn rationale(&self) -> &'static str {
        self.reason.as_str()
    }

    pub fn row_digest(&self) -> &str {
        &self.row_digest
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarAdmissionMatrix {
    rows: Vec<PlanarAdmissionRow>,
    premetaboss_rows: Vec<PlanarPremetabossAdmissionRow>,
    matrix_digest: String,
}

impl PlanarAdmissionMatrix {
    pub fn rows(&self) -> &[PlanarAdmissionRow] {
        &self.rows
    }

    pub fn matrix_digest(&self) -> &str {
        &self.matrix_digest
    }

    pub fn premetaboss_rows(&self) -> &[PlanarPremetabossAdmissionRow] {
        &self.premetaboss_rows
    }

    pub fn row(
        &self,
        family: PlanarAdmissionFamily,
        concern: PlanarRuntimeConcern,
    ) -> Option<&PlanarAdmissionRow> {
        self.rows
            .iter()
            .find(|row| row.family() == family && row.concern() == concern)
    }

    pub fn admit(
        &self,
        family: PlanarAdmissionFamily,
        concern: PlanarRuntimeConcern,
    ) -> Option<PlanarAdmissionReceipt> {
        let row = self.row(family, concern)?;
        row.class()
            .admits_runtime()
            .then(|| PlanarAdmissionReceipt {
                family,
                concern,
                class: row.class(),
                row_digest: row.row_digest().to_string(),
                matrix_digest: self.matrix_digest().to_string(),
            })
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarAdmissionReceipt {
    family: PlanarAdmissionFamily,
    concern: PlanarRuntimeConcern,
    class: PlanarAdmissionClass,
    row_digest: String,
    matrix_digest: String,
}

impl PlanarAdmissionReceipt {
    pub fn family(&self) -> PlanarAdmissionFamily {
        self.family
    }

    pub fn concern(&self) -> PlanarRuntimeConcern {
        self.concern
    }

    pub fn class(&self) -> PlanarAdmissionClass {
        self.class
    }

    pub fn row_digest(&self) -> &str {
        &self.row_digest
    }

    pub fn matrix_digest(&self) -> &str {
        &self.matrix_digest
    }
}

pub fn planar_admission_matrix() -> PlanarAdmissionMatrix {
    let rows = PlanarAdmissionFamily::all()
        .into_iter()
        .flat_map(|family| {
            PlanarRuntimeConcern::all()
                .into_iter()
                .map(move |concern| PlanarAdmissionRow::new(family, concern))
        })
        .collect::<Vec<_>>();
    let matrix_digest = hash_parts(
        &rows
            .iter()
            .map(|row| row.row_digest().to_string())
            .collect::<Vec<_>>(),
    );
    let mut matrix = PlanarAdmissionMatrix {
        rows,
        premetaboss_rows: Vec::new(),
        matrix_digest,
    };
    matrix.premetaboss_rows = premetaboss_admission_rows(&matrix);
    matrix
}

pub fn admit_planar_contract_family(
    family: PlanarAdmissionFamily,
    concern: PlanarRuntimeConcern,
) -> Option<PlanarAdmissionReceipt> {
    planar_admission_matrix().admit(family, concern)
}

fn hash_parts(parts: &[String]) -> String {
    truth_digest_parts(TruthDigestScope::ArtifactIdentity, parts)
}
