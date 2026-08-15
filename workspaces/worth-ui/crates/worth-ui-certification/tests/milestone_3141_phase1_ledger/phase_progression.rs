use std::collections::BTreeMap;

use super::Row;

pub(super) fn validate(rows: &BTreeMap<String, Row>) -> Result<(), String> {
    for phase in 2_u8..=5 {
        reject_predecessor_bypass(rows, phase)?;
    }
    require_phase_gate(rows, "3", "P3-PREDECESSOR-01", &[])?;
    require_phase_gate(rows, "4", "P4-PREDECESSOR-01", &[])?;
    require_phase_gate(rows, "4", "P4-TEXT-PROFILE-01", &["P4-PREDECESSOR-01"])?;
    if rows.contains_key("P5-PREDECESSOR-01") {
        require_phase_gate(rows, "5", "P5-PREDECESSOR-01", &[])?;
    }
    require_close_last(rows, "3", "P3-CLOSE-01")?;
    require_close_last(rows, "4", "P4-CLOSE-01")?;
    require_close_last(rows, "5", "P5-CLOSE-01")
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
            && (row["result"] != "PROVED" || row["final_source"] != "true")
    });
    if current_proved && predecessor_open {
        return Err(format!(
            "Phase {phase} proof cannot precede predecessor closure"
        ));
    }
    Ok(())
}

fn require_phase_gate(
    rows: &BTreeMap<String, Row>,
    phase: &str,
    gate: &str,
    prior_gates: &[&str],
) -> Result<(), String> {
    let advanced = rows.iter().any(|(requirement, row)| {
        row["phase"] == phase
            && requirement != gate
            && !prior_gates.contains(&requirement.as_str())
            && row["result"] == "PROVED"
    });
    if advanced && (rows[gate]["result"] != "PROVED" || rows[gate]["final_source"] != "true") {
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
    let Some(close_row) = rows.get(close) else {
        return Ok(());
    };
    if close_row["result"] != "PROVED" {
        return Ok(());
    }
    let sibling_open = rows.iter().any(|(requirement, row)| {
        row["phase"] == phase
            && requirement != close
            && (row["result"] != "PROVED" || row["final_source"] != "true")
    });
    (!sibling_open)
        .then_some(())
        .ok_or_else(|| format!("{close} cannot precede its phase requirements"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn phase_four_gates_are_ordered_and_require_final_source() {
        let mut rows = baseline();
        prove(&mut rows, "P4-TEXT-PROFILE-01", true);
        assert!(validate(&rows).unwrap_err().contains("P4-PREDECESSOR-01"));

        prove(&mut rows, "P4-PREDECESSOR-01", false);
        assert!(validate(&rows).unwrap_err().contains("P4-PREDECESSOR-01"));

        prove(&mut rows, "P4-PREDECESSOR-01", true);
        open(&mut rows, "P4-TEXT-PROFILE-01");
        prove(&mut rows, "P4-OTHER-01", true);
        assert!(validate(&rows).unwrap_err().contains("P4-TEXT-PROFILE-01"));

        prove(&mut rows, "P4-TEXT-PROFILE-01", false);
        assert!(validate(&rows).unwrap_err().contains("P4-TEXT-PROFILE-01"));
        prove(&mut rows, "P4-TEXT-PROFILE-01", true);
        assert!(validate(&rows).is_ok());
    }

    #[test]
    fn phase_four_close_cannot_precede_an_open_sibling() {
        let mut rows = baseline();
        prove(&mut rows, "P4-PREDECESSOR-01", true);
        prove(&mut rows, "P4-TEXT-PROFILE-01", true);
        prove(&mut rows, "P4-CLOSE-01", true);
        assert!(validate(&rows).unwrap_err().contains("P4-CLOSE-01"));
        prove(&mut rows, "P4-OTHER-01", true);
        assert!(validate(&rows).is_ok());
    }

    fn baseline() -> BTreeMap<String, Row> {
        BTreeMap::from([
            ("P1-ONE".to_owned(), row("1", "PROVED", "true")),
            ("P2-ONE".to_owned(), row("2", "PROVED", "true")),
            ("P3-PREDECESSOR-01".to_owned(), row("3", "PROVED", "true")),
            ("P3-CLOSE-01".to_owned(), row("3", "PROVED", "true")),
            ("P4-PREDECESSOR-01".to_owned(), row("4", "OPEN", "false")),
            ("P4-TEXT-PROFILE-01".to_owned(), row("4", "OPEN", "false")),
            ("P4-OTHER-01".to_owned(), row("4", "OPEN", "false")),
            ("P4-CLOSE-01".to_owned(), row("4", "OPEN", "false")),
        ])
    }

    fn row(phase: &str, result: &str, final_source: &str) -> Row {
        BTreeMap::from([
            ("phase".to_owned(), phase.to_owned()),
            ("result".to_owned(), result.to_owned()),
            ("final_source".to_owned(), final_source.to_owned()),
        ])
    }

    fn prove(rows: &mut BTreeMap<String, Row>, requirement: &str, final_source: bool) {
        rows.get_mut(requirement)
            .unwrap()
            .insert("result".to_owned(), "PROVED".to_owned());
        rows.get_mut(requirement)
            .unwrap()
            .insert("final_source".to_owned(), final_source.to_string());
    }

    fn open(rows: &mut BTreeMap<String, Row>, requirement: &str) {
        rows.get_mut(requirement)
            .unwrap()
            .insert("result".to_owned(), "OPEN".to_owned());
        rows.get_mut(requirement)
            .unwrap()
            .insert("final_source".to_owned(), "false".to_owned());
    }
}
