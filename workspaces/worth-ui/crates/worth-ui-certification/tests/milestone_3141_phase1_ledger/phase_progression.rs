use std::collections::BTreeMap;

use super::Row;

pub(super) fn validate(rows: &BTreeMap<String, Row>) -> Result<(), String> {
    for phase in 2_u8..=4 {
        reject_predecessor_bypass(rows, phase)?;
    }
    require_phase_gate(rows, "3", "P3-PREDECESSOR-01")?;
    require_phase_gate(rows, "4", "P4-TEXT-PROFILE-01")?;
    require_close_last(rows, "3", "P3-CLOSE-01")
}

pub(super) fn validate_closure(
    rows: &BTreeMap<String, Row>,
    through_phase: u8,
) -> Result<(), String> {
    let open = rows.values().find(|row| {
        row["phase"]
            .parse::<u8>()
            .is_ok_and(|phase| phase <= through_phase)
            && (row["result"] != "PROVED" || row["final_source"] != "true")
    });
    match open {
        Some(row) => Err(format!(
            "{} remains open for Phase {through_phase} closure",
            row["requirement"]
        )),
        None => Ok(()),
    }
}

fn reject_predecessor_bypass(rows: &BTreeMap<String, Row>, phase: u8) -> Result<(), String> {
    let current_proved = rows.values().any(|row| {
        row["phase"]
            .parse::<u8>()
            .is_ok_and(|observed| observed == phase)
            && row["result"] == "PROVED"
    });
    let predecessor_open = rows.values().any(|row| {
        row["phase"]
            .parse::<u8>()
            .is_ok_and(|predecessor| predecessor < phase)
            && row["result"] != "PROVED"
    });
    if current_proved && predecessor_open {
        return Err(format!(
            "Phase {phase} proof cannot precede predecessor closure"
        ));
    }
    Ok(())
}

fn require_phase_gate(rows: &BTreeMap<String, Row>, phase: &str, gate: &str) -> Result<(), String> {
    let advanced = rows.iter().any(|(requirement, row)| {
        row["phase"] == phase && requirement != gate && row["result"] == "PROVED"
    });
    if advanced && rows[gate]["result"] != "PROVED" {
        return Err(format!(
            "{gate} must close before other Phase {phase} proof"
        ));
    }
    Ok(())
}

fn require_close_last(
    rows: &BTreeMap<String, Row>,
    phase: &str,
    close: &str,
) -> Result<(), String> {
    if rows[close]["result"] != "PROVED" {
        return Ok(());
    }
    let sibling_open = rows.iter().any(|(requirement, row)| {
        row["phase"] == phase && requirement != close && row["result"] != "PROVED"
    });
    (!sibling_open)
        .then_some(())
        .ok_or_else(|| format!("{close} cannot precede its phase requirements"))
}
