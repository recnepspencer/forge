use crate::runtime::{
    WorthQueryGraphObligationExecutionBudget, WorthQueryGraphObligationKind,
    WorthQueryGraphObligationRegistration, WorthQueryGraphObligationSupportLane,
    WorthQueryGraphObligationSupportMatrix, WorthQueryGraphObligationSupportStatus,
};

use super::error::{
    WorthQueryGraphObligationConsumerKitError, WorthQueryGraphObligationConsumerKitErrorKind,
};
use super::kit_digest;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryGraphObligationSupportPin {
    rows: Vec<WorthQueryGraphObligationSupportPinRow>,
    pin_digest: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct WorthQueryGraphObligationSupportPinRow {
    obligation_kind: WorthQueryGraphObligationKind,
    support_lane: WorthQueryGraphObligationSupportLane,
    expected_status: WorthQueryGraphObligationSupportStatus,
    expected_budget_digest: Option<String>,
    row_digest: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryGraphObligationSupportPinFinding {
    obligation_kind: WorthQueryGraphObligationKind,
    support_lane: WorthQueryGraphObligationSupportLane,
    expected_status: WorthQueryGraphObligationSupportStatus,
    observed_status: Option<WorthQueryGraphObligationSupportStatus>,
    expected_budget_digest: Option<String>,
    observed_budget_digest: Option<String>,
}

impl WorthQueryGraphObligationSupportPin {
    pub fn supported(
        rows: impl IntoIterator<
            Item = (
                WorthQueryGraphObligationKind,
                WorthQueryGraphObligationSupportLane,
            ),
        >,
    ) -> Self {
        Self::new(rows.into_iter().map(|(kind, lane)| {
            (
                kind,
                lane,
                WorthQueryGraphObligationSupportStatus::Supported,
            )
        }))
    }

    pub fn new(
        rows: impl IntoIterator<
            Item = (
                WorthQueryGraphObligationKind,
                WorthQueryGraphObligationSupportLane,
                WorthQueryGraphObligationSupportStatus,
            ),
        >,
    ) -> Self {
        Self::from_rows(
            rows.into_iter()
                .map(|(obligation_kind, support_lane, expected_status)| {
                    WorthQueryGraphObligationSupportPinRow::new(
                        obligation_kind,
                        support_lane,
                        expected_status,
                        None,
                    )
                }),
        )
    }

    pub fn new_with_budget(
        rows: impl IntoIterator<
            Item = (
                WorthQueryGraphObligationKind,
                WorthQueryGraphObligationSupportLane,
                WorthQueryGraphObligationSupportStatus,
                WorthQueryGraphObligationExecutionBudget,
            ),
        >,
    ) -> Self {
        Self::from_rows(rows.into_iter().map(
            |(obligation_kind, support_lane, expected_status, expected_budget)| {
                WorthQueryGraphObligationSupportPinRow::new(
                    obligation_kind,
                    support_lane,
                    expected_status,
                    Some(expected_budget.budget_digest().to_string()),
                )
            },
        ))
    }

    pub fn supported_with_budget(
        rows: impl IntoIterator<
            Item = (
                WorthQueryGraphObligationKind,
                WorthQueryGraphObligationSupportLane,
                WorthQueryGraphObligationExecutionBudget,
            ),
        >,
    ) -> Self {
        Self::new_with_budget(rows.into_iter().map(|(kind, lane, budget)| {
            (
                kind,
                lane,
                WorthQueryGraphObligationSupportStatus::Supported,
                budget,
            )
        }))
    }

    fn from_rows(rows: impl IntoIterator<Item = WorthQueryGraphObligationSupportPinRow>) -> Self {
        let mut rows = rows.into_iter().collect::<Vec<_>>();
        rows.sort_by(|left, right| left.row_digest.cmp(&right.row_digest));
        let pin_digest = kit_digest(
            "graph-obligation-support-pin",
            rows.iter().map(|row| row.row_digest.as_str()),
        );
        Self { rows, pin_digest }
    }

    pub fn evaluate_for_registrations(
        &self,
        matrix: &WorthQueryGraphObligationSupportMatrix,
        registrations: &[WorthQueryGraphObligationRegistration],
    ) -> Result<(), WorthQueryGraphObligationConsumerKitError> {
        let findings = self.findings_for_registrations(matrix, registrations);
        if findings.is_empty() {
            Ok(())
        } else {
            Err(WorthQueryGraphObligationConsumerKitError::new(
                WorthQueryGraphObligationConsumerKitErrorKind::SupportPinDrift,
                format!(
                    "graph obligation support pin drifted on {} rows",
                    findings.len()
                ),
            ))
        }
    }

    pub fn evaluate(
        &self,
        matrix: &WorthQueryGraphObligationSupportMatrix,
    ) -> Result<(), WorthQueryGraphObligationConsumerKitError> {
        let findings = self.findings(matrix);
        if findings.is_empty() {
            Ok(())
        } else {
            Err(WorthQueryGraphObligationConsumerKitError::new(
                WorthQueryGraphObligationConsumerKitErrorKind::SupportPinDrift,
                format!(
                    "graph obligation support pin drifted on {} rows",
                    findings.len()
                ),
            ))
        }
    }

    pub fn findings(
        &self,
        matrix: &WorthQueryGraphObligationSupportMatrix,
    ) -> Vec<WorthQueryGraphObligationSupportPinFinding> {
        self.findings_for_registrations(matrix, &[])
    }

    pub fn findings_for_registrations(
        &self,
        matrix: &WorthQueryGraphObligationSupportMatrix,
        registrations: &[WorthQueryGraphObligationRegistration],
    ) -> Vec<WorthQueryGraphObligationSupportPinFinding> {
        self.rows
            .iter()
            .flat_map(|pin| {
                let observed = matrix
                    .rows_for_kind(pin.obligation_kind)
                    .find(|row| row.support_lane() == pin.support_lane)
                    .map(|row| row.status());
                let mut findings = Vec::new();
                if observed != Some(pin.expected_status) {
                    findings.push(pin.finding(observed, None));
                }
                if let Some(expected_budget_digest) = &pin.expected_budget_digest {
                    let observed_budget_digests = registrations
                        .iter()
                        .filter(|registration| registration.kind() == pin.obligation_kind)
                        .filter(|registration| {
                            registration.support_posture().lane() == pin.support_lane
                        })
                        .map(|registration| registration.execution_budget().budget_digest())
                        .collect::<Vec<_>>();
                    if observed_budget_digests.is_empty() {
                        findings.push(pin.finding(observed, None));
                    } else {
                        findings.extend(
                            observed_budget_digests
                                .into_iter()
                                .filter(|observed_budget_digest| {
                                    *observed_budget_digest != expected_budget_digest
                                })
                                .map(|observed_budget_digest| {
                                    pin.finding(observed, Some(observed_budget_digest.to_string()))
                                }),
                        );
                    }
                }
                findings
            })
            .collect()
    }

    pub fn row_count(&self) -> usize {
        self.rows.len()
    }

    pub fn pin_digest(&self) -> &str {
        &self.pin_digest
    }
}

impl WorthQueryGraphObligationSupportPinFinding {
    pub fn obligation_kind(&self) -> WorthQueryGraphObligationKind {
        self.obligation_kind
    }

    pub fn support_lane(&self) -> WorthQueryGraphObligationSupportLane {
        self.support_lane
    }

    pub fn expected_status(&self) -> WorthQueryGraphObligationSupportStatus {
        self.expected_status
    }

    pub fn observed_status(&self) -> Option<WorthQueryGraphObligationSupportStatus> {
        self.observed_status
    }

    pub fn expected_budget_digest(&self) -> Option<&str> {
        self.expected_budget_digest.as_deref()
    }

    pub fn observed_budget_digest(&self) -> Option<&str> {
        self.observed_budget_digest.as_deref()
    }
}

impl WorthQueryGraphObligationSupportPinRow {
    fn new(
        obligation_kind: WorthQueryGraphObligationKind,
        support_lane: WorthQueryGraphObligationSupportLane,
        expected_status: WorthQueryGraphObligationSupportStatus,
        expected_budget_digest: Option<String>,
    ) -> Self {
        let row_digest = kit_digest(
            "graph-obligation-support-pin-row",
            [
                obligation_kind.as_str(),
                support_lane.as_str(),
                expected_status.as_str(),
                expected_budget_digest.as_deref().unwrap_or("<unbudgeted>"),
            ],
        );
        Self {
            obligation_kind,
            support_lane,
            expected_status,
            expected_budget_digest,
            row_digest,
        }
    }

    fn finding(
        &self,
        observed_status: Option<WorthQueryGraphObligationSupportStatus>,
        observed_budget_digest: Option<String>,
    ) -> WorthQueryGraphObligationSupportPinFinding {
        WorthQueryGraphObligationSupportPinFinding {
            obligation_kind: self.obligation_kind,
            support_lane: self.support_lane,
            expected_status: self.expected_status,
            observed_status,
            expected_budget_digest: self.expected_budget_digest.clone(),
            observed_budget_digest,
        }
    }
}
