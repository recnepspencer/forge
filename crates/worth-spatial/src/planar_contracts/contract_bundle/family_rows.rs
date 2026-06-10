use super::basis::PlanarContractBundleValidationBasis;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum PlanarContractBundleFamily {
    Admission,
    TopologyContractCompleteness,
    Precision,
    LocalFrame,
    ProjectionConsumption,
    PredicateAuthority,
    PredicateCertificateConsumption,
    SegmentContact,
    PolygonWinding,
    SignedArea,
    CoplanarOverlap,
}

impl PlanarContractBundleFamily {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Admission => "admission",
            Self::TopologyContractCompleteness => "topology-contract-completeness",
            Self::Precision => "precision",
            Self::LocalFrame => "local-frame",
            Self::ProjectionConsumption => "projection-consumption",
            Self::PredicateAuthority => "predicate-authority",
            Self::PredicateCertificateConsumption => "predicate-certificate-consumption",
            Self::SegmentContact => "segment-contact",
            Self::PolygonWinding => "polygon-winding",
            Self::SignedArea => "signed-area",
            Self::CoplanarOverlap => "coplanar-overlap",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarContractBundleFamilyRow {
    family: PlanarContractBundleFamily,
    receipt_count: usize,
    retained_fact_digests: Vec<String>,
    declaration_digests: Vec<String>,
    envelope_digests: Vec<String>,
}

impl PlanarContractBundleFamilyRow {
    pub(crate) fn new(
        family: PlanarContractBundleFamily,
        retained_fact_digests: Vec<String>,
        declaration_digests: Vec<String>,
        envelope_digests: Vec<String>,
    ) -> Self {
        let mut row = Self {
            family,
            receipt_count: retained_fact_digests.len(),
            retained_fact_digests,
            declaration_digests,
            envelope_digests,
        };
        row.retained_fact_digests.sort();
        row.declaration_digests.sort();
        row.envelope_digests.sort();
        row
    }

    pub fn family(&self) -> PlanarContractBundleFamily {
        self.family
    }

    pub fn receipt_count(&self) -> usize {
        self.receipt_count
    }

    pub fn retained_fact_digests(&self) -> &[String] {
        &self.retained_fact_digests
    }

    pub fn declaration_digests(&self) -> &[String] {
        &self.declaration_digests
    }

    pub fn envelope_digests(&self) -> &[String] {
        &self.envelope_digests
    }
}

pub(crate) fn build_family_rows(
    basis: &PlanarContractBundleValidationBasis,
) -> Vec<PlanarContractBundleFamilyRow> {
    let mut rows = vec![
        admission_row(basis),
        single_fact_row(
            PlanarContractBundleFamily::TopologyContractCompleteness,
            basis.topology_contract_receipt().fact_digest(),
            basis.topology_contract_receipt().declaration_digest(),
            basis.topology_contract_receipt().envelope_digest(),
        ),
        single_fact_row(
            PlanarContractBundleFamily::Precision,
            basis.precision_receipt().fact_digest(),
            basis.precision_receipt().declaration_digest(),
            basis.precision_receipt().envelope_digest(),
        ),
        single_fact_row(
            PlanarContractBundleFamily::LocalFrame,
            basis.local_frame_receipt().fact_digest(),
            basis.local_frame_receipt().declaration_digest(),
            basis.local_frame_receipt().envelope_digest(),
        ),
        receipt_vec_row(
            PlanarContractBundleFamily::ProjectionConsumption,
            basis.projection_receipts().iter().map(|receipt| {
                (
                    receipt.fact_digest(),
                    receipt.declaration_digest(),
                    receipt.envelope_digest(),
                )
            }),
        ),
        receipt_vec_row(
            PlanarContractBundleFamily::PredicateAuthority,
            basis.predicate_receipts().iter().map(|receipt| {
                (
                    receipt.fact_digest(),
                    receipt.declaration_digest(),
                    receipt.envelope_digest(),
                )
            }),
        ),
        receipt_vec_row(
            PlanarContractBundleFamily::SegmentContact,
            basis.segment_receipts().iter().map(|receipt| {
                (
                    receipt.fact_digest(),
                    receipt.declaration_digest(),
                    receipt.envelope_digest(),
                )
            }),
        ),
        single_fact_row(
            PlanarContractBundleFamily::PolygonWinding,
            basis.winding_receipt().fact_digest(),
            basis.winding_receipt().declaration_digest(),
            basis.winding_receipt().envelope_digest(),
        ),
        single_fact_row(
            PlanarContractBundleFamily::SignedArea,
            basis.signed_area_receipt().fact_digest(),
            basis.signed_area_receipt().declaration_digest(),
            basis.signed_area_receipt().envelope_digest(),
        ),
        single_fact_row(
            PlanarContractBundleFamily::CoplanarOverlap,
            basis.overlap_receipt().fact_digest(),
            basis.overlap_receipt().declaration_digest(),
            basis.overlap_receipt().envelope_digest(),
        ),
        single_fact_row(
            PlanarContractBundleFamily::PredicateCertificateConsumption,
            basis.predicate_consumption_receipt().fact_digest(),
            basis.predicate_consumption_receipt().declaration_digest(),
            basis.predicate_consumption_receipt().envelope_digest(),
        ),
    ];
    rows.sort_by_key(|row| row.family());
    rows
}

fn admission_row(basis: &PlanarContractBundleValidationBasis) -> PlanarContractBundleFamilyRow {
    PlanarContractBundleFamilyRow::new(
        PlanarContractBundleFamily::Admission,
        vec![basis.admission_receipt().row_digest().to_string()],
        vec![basis.admission_receipt().matrix_digest().to_string()],
        Vec::new(),
    )
}

fn single_fact_row(
    family: PlanarContractBundleFamily,
    fact_digest: &str,
    declaration_digest: &str,
    envelope_digest: &str,
) -> PlanarContractBundleFamilyRow {
    PlanarContractBundleFamilyRow::new(
        family,
        vec![fact_digest.to_string()],
        vec![declaration_digest.to_string()],
        vec![envelope_digest.to_string()],
    )
}

fn receipt_vec_row<'a, I>(
    family: PlanarContractBundleFamily,
    receipts: I,
) -> PlanarContractBundleFamilyRow
where
    I: IntoIterator<Item = (&'a str, &'a str, &'a str)>,
{
    let receipts = receipts.into_iter().collect::<Vec<_>>();
    PlanarContractBundleFamilyRow::new(
        family,
        receipts
            .iter()
            .map(|(fact_digest, _, _)| (*fact_digest).to_string())
            .collect(),
        receipts
            .iter()
            .map(|(_, declaration_digest, _)| (*declaration_digest).to_string())
            .collect(),
        receipts
            .iter()
            .map(|(_, _, envelope_digest)| (*envelope_digest).to_string())
            .collect(),
    )
}
