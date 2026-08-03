use super::super::{authority_trace, inventory, public_api};
use super::{parse_ledger, read_repository_document, LedgerRow, LEDGER, REQUIRED_FAMILIES};

pub(super) fn validate_current_accounting(rows: &[LedgerRow]) -> Result<(), String> {
    validate_claims(rows, &CurrentLedgerAccounting::read(rows)?)
}

#[test]
fn current_ledger_accounting_matches_every_authoritative_owner() {
    let rows = current_rows();
    validate_current_accounting(&rows).unwrap_or_else(|denial| panic!("{denial}"));
}

#[test]
fn current_ledger_accounting_rejects_each_independent_fact_drift() {
    let rows = current_rows();
    let accounting = CurrentLedgerAccounting::read(&rows).expect("read current accounting");
    for (guarantee, current, stale) in accounting.controlled_fact_drifts() {
        let mut stale_rows = rows.clone();
        replace_evidence_fact(&mut stale_rows, guarantee, &current, &stale);
        assert!(
            validate_current_accounting(&stale_rows).is_err(),
            "MUTANT_PREDICATE:phase-ten-ledger-current-accounting-drift-accepted: {guarantee} accepted {stale}"
        );
    }
}

fn validate_claims(rows: &[LedgerRow], accounting: &CurrentLedgerAccounting) -> Result<(), String> {
    for claim in accounting.claims() {
        let row = rows
            .iter()
            .find(|row| row.id == claim.guarantee)
            .ok_or_else(|| {
                format!(
                    "C.7 ledger omits accounting guarantee `{}`",
                    claim.guarantee
                )
            })?;
        if !row.current_evidence.contains(&claim.clause) {
            return Err(format!(
                "C.7 ledger guarantee `{}` has stale current accounting; expected `{}`",
                claim.guarantee, claim.clause
            ));
        }
    }
    Ok(())
}

fn current_rows() -> Vec<LedgerRow> {
    let document = read_repository_document(LEDGER).expect("read C.7 closure ledger");
    parse_ledger(&document).expect("parse C.7 closure ledger")
}

fn replace_evidence_fact(rows: &mut [LedgerRow], guarantee: &str, current: &str, stale: &str) {
    let evidence = &mut rows
        .iter_mut()
        .find(|row| row.id == guarantee)
        .unwrap_or_else(|| panic!("missing controlled accounting guarantee `{guarantee}`"))
        .current_evidence;
    assert_eq!(
        evidence.matches(current).count(),
        1,
        "controlled accounting fact `{current}`"
    );
    *evidence = evidence.replacen(current, stale, 1);
}

struct CurrentLedgerAccounting {
    guarantee_rows: usize,
    required_families: usize,
    authority_lanes: usize,
    api: public_api::PublicApiAccounting,
    removal: inventory::RemovalLedgerAccounting,
}

impl CurrentLedgerAccounting {
    fn read(rows: &[LedgerRow]) -> Result<Self, String> {
        Ok(Self {
            guarantee_rows: rows.len(),
            required_families: REQUIRED_FAMILIES.len(),
            authority_lanes: authority_trace::current_authority_lane_count(),
            api: public_api::current_accounting(),
            removal: inventory::current_accounting()?,
        })
    }

    fn claims(&self) -> [AccountingClaim; 4] {
        [
            AccountingClaim::new(
                "C7-AUTHORITY-01",
                format!(
                    "[current-accounting live-consumers={} absent-paths={} authority-lanes={}]",
                    self.removal.live_consumers, self.removal.absent_paths, self.authority_lanes
                ),
            ),
            AccountingClaim::new(
                "C7-API-01",
                format!(
                    "[current-accounting locked-surfaces={} phase-ten-surfaces={}]",
                    self.api.locked_surfaces, self.api.phase_ten_surfaces
                ),
            ),
            AccountingClaim::new(
                "C7-LEDGER-01",
                format!(
                    "[current-accounting guarantee-rows={} required-families={}]",
                    self.guarantee_rows, self.required_families
                ),
            ),
            AccountingClaim::new(
                "C7-CLEANUP-01",
                format!(
                    "[current-accounting removal-rows={} live-consumers={} absent-paths={} deleted-paths={} completed-moves={}]",
                    self.removal.rows,
                    self.removal.live_consumers,
                    self.removal.absent_paths,
                    self.removal.deleted_paths,
                    self.removal.completed_moves
                ),
            ),
        ]
    }

    fn controlled_fact_drifts(&self) -> Vec<(&'static str, String, String)> {
        [
            (
                "C7-AUTHORITY-01",
                "live-consumers",
                self.removal.live_consumers,
            ),
            ("C7-AUTHORITY-01", "absent-paths", self.removal.absent_paths),
            ("C7-AUTHORITY-01", "authority-lanes", self.authority_lanes),
            ("C7-API-01", "locked-surfaces", self.api.locked_surfaces),
            (
                "C7-API-01",
                "phase-ten-surfaces",
                self.api.phase_ten_surfaces,
            ),
            ("C7-LEDGER-01", "guarantee-rows", self.guarantee_rows),
            ("C7-LEDGER-01", "required-families", self.required_families),
            ("C7-CLEANUP-01", "removal-rows", self.removal.rows),
            (
                "C7-CLEANUP-01",
                "live-consumers",
                self.removal.live_consumers,
            ),
            ("C7-CLEANUP-01", "absent-paths", self.removal.absent_paths),
            ("C7-CLEANUP-01", "deleted-paths", self.removal.deleted_paths),
            (
                "C7-CLEANUP-01",
                "completed-moves",
                self.removal.completed_moves,
            ),
        ]
        .into_iter()
        .map(|(guarantee, name, value)| {
            assert!(
                value > 0,
                "controlled accounting fact `{name}` must be nonzero"
            );
            (
                guarantee,
                format!("{name}={value}"),
                format!("{name}={}", value - 1),
            )
        })
        .collect()
    }
}

struct AccountingClaim {
    guarantee: &'static str,
    clause: String,
}

impl AccountingClaim {
    fn new(guarantee: &'static str, clause: String) -> Self {
        Self { guarantee, clause }
    }
}
