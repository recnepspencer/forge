use super::{require_non_empty, HadwigerResearchDeclarationShapeError};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LowerBoundTilingIterationDeclaration {
    packet_id: String,
    session_digest: String,
    evidence_basis: Vec<String>,
    required_checker_lanes: Vec<String>,
    reactivation_obligations: Vec<String>,
}

impl LowerBoundTilingIterationDeclaration {
    pub fn new(
        packet_id: impl Into<String>,
        session_digest: impl Into<String>,
        evidence_basis: impl Into<String>,
        required_checker_lanes: impl Into<String>,
        reactivation_obligations: impl Into<String>,
    ) -> Self {
        Self::try_new(
            packet_id,
            session_digest,
            [evidence_basis],
            [required_checker_lanes],
            [reactivation_obligations],
        )
        .expect("iteration declaration identity fields must be non-empty")
    }

    pub fn try_new<E, L, R, EV, LV, RV>(
        packet_id: impl Into<String>,
        session_digest: impl Into<String>,
        evidence_basis: E,
        required_checker_lanes: L,
        reactivation_obligations: R,
    ) -> Result<Self, HadwigerResearchDeclarationShapeError>
    where
        E: IntoIterator<Item = EV>,
        L: IntoIterator<Item = LV>,
        R: IntoIterator<Item = RV>,
        EV: Into<String>,
        LV: Into<String>,
        RV: Into<String>,
    {
        Ok(Self {
            packet_id: require_non_empty(packet_id, "packet_id")?,
            session_digest: require_non_empty(session_digest, "session_digest")?,
            evidence_basis: require_non_empty_list(evidence_basis, "evidence_basis")?,
            required_checker_lanes: require_non_empty_list(
                required_checker_lanes,
                "required_checker_lanes",
            )?,
            reactivation_obligations: require_non_empty_list(
                reactivation_obligations,
                "reactivation_obligations",
            )?,
        })
    }

    pub(crate) fn packet_id(&self) -> &str {
        &self.packet_id
    }

    pub(crate) fn session_digest(&self) -> &str {
        &self.session_digest
    }

    pub(crate) fn required_checker_lanes(&self) -> &[String] {
        &self.required_checker_lanes
    }

    pub(crate) fn evidence_basis(&self) -> &[String] {
        &self.evidence_basis
    }

    pub(crate) fn reactivation_obligations(&self) -> &[String] {
        &self.reactivation_obligations
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UpperBoundTilingIterationDeclaration {
    packet_id: String,
    session_digest: String,
    evidence_basis: Vec<String>,
    required_checker_lanes: Vec<String>,
    reactivation_obligations: Vec<String>,
}

impl UpperBoundTilingIterationDeclaration {
    pub fn new(
        packet_id: impl Into<String>,
        session_digest: impl Into<String>,
        evidence_basis: impl Into<String>,
        required_checker_lanes: impl Into<String>,
        reactivation_obligations: impl Into<String>,
    ) -> Self {
        Self::try_new(
            packet_id,
            session_digest,
            [evidence_basis],
            [required_checker_lanes],
            [reactivation_obligations],
        )
        .expect("iteration declaration identity fields must be non-empty")
    }

    pub fn try_new<E, L, R, EV, LV, RV>(
        packet_id: impl Into<String>,
        session_digest: impl Into<String>,
        evidence_basis: E,
        required_checker_lanes: L,
        reactivation_obligations: R,
    ) -> Result<Self, HadwigerResearchDeclarationShapeError>
    where
        E: IntoIterator<Item = EV>,
        L: IntoIterator<Item = LV>,
        R: IntoIterator<Item = RV>,
        EV: Into<String>,
        LV: Into<String>,
        RV: Into<String>,
    {
        Ok(Self {
            packet_id: require_non_empty(packet_id, "packet_id")?,
            session_digest: require_non_empty(session_digest, "session_digest")?,
            evidence_basis: require_non_empty_list(evidence_basis, "evidence_basis")?,
            required_checker_lanes: require_non_empty_list(
                required_checker_lanes,
                "required_checker_lanes",
            )?,
            reactivation_obligations: require_non_empty_list(
                reactivation_obligations,
                "reactivation_obligations",
            )?,
        })
    }

    pub(crate) fn packet_id(&self) -> &str {
        &self.packet_id
    }

    pub(crate) fn session_digest(&self) -> &str {
        &self.session_digest
    }

    pub(crate) fn required_checker_lanes(&self) -> &[String] {
        &self.required_checker_lanes
    }

    pub(crate) fn evidence_basis(&self) -> &[String] {
        &self.evidence_basis
    }

    pub(crate) fn reactivation_obligations(&self) -> &[String] {
        &self.reactivation_obligations
    }
}

fn require_non_empty_list<I, T>(
    values: I,
    field: &'static str,
) -> Result<Vec<String>, HadwigerResearchDeclarationShapeError>
where
    I: IntoIterator<Item = T>,
    T: Into<String>,
{
    let mut retained = Vec::new();
    for value in values {
        retained.push(require_non_empty(value, field)?);
    }
    if retained.is_empty() {
        return Err(HadwigerResearchDeclarationShapeError::EmptyIdentityField { field });
    }
    retained.sort();
    retained.dedup();
    Ok(retained)
}
