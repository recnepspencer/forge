use std::collections::BTreeSet;

const HEADER: &str = "id,production_claim,fixture_provenance,typed_result,mutation_control,\
structural_cost,teardown,evidence_command,status,evidence";

pub(super) fn validate_phase_1(contract: &toml::Value, ledger: &str) -> Result<(), String> {
    let expected = (1..=11)
        .map(|number| format!("P1-{number:02}"))
        .collect::<Vec<_>>();
    let gates = contract["phase_gate"]
        .as_array()
        .ok_or_else(|| "Phase 1 gates are not an array".to_owned())?;
    let gate_ids = gates
        .iter()
        .map(|gate| required_text(gate, "id").map(str::to_owned))
        .collect::<Result<Vec<_>, _>>()?;
    if gate_ids != expected {
        return Err("Phase 1 contract gates are not exactly P1-01 through P1-11".to_owned());
    }
    let commands = gates
        .iter()
        .map(|gate| required_text(gate, "command"))
        .collect::<Result<Vec<_>, _>>()?;
    let closed = contract["status"].as_str() == Some("closed");
    let rows = parse_ledger(ledger)?;
    validate_rows(&rows, &expected, Some(&commands), closed)
}

pub(super) fn validate_phase_5(
    contract: &toml::Value,
    ledger: &str,
    require_closed: bool,
) -> Result<(), String> {
    let scenarios = contract["scenario"]
        .as_array()
        .ok_or_else(|| "Phase 5 scenarios are not an array".to_owned())?;
    let expected = std::iter::once("RB-01".to_owned())
        .chain((1..=12).map(|number| format!("TT-{number:02}")))
        .collect::<Vec<_>>();
    let ids = scenarios
        .iter()
        .map(|scenario| required_text(scenario, "id").map(str::to_owned))
        .collect::<Result<Vec<_>, _>>()?;
    if ids != expected {
        return Err("Phase 5 scenarios are not exactly RB-01 then TT-01 through TT-12".to_owned());
    }
    let owners = scenarios
        .iter()
        .map(|scenario| required_text(scenario, "owner"))
        .collect::<Result<BTreeSet<_>, _>>()?;
    if owners.len() != expected.len() {
        return Err("Phase 5 evidence owners are not unique".to_owned());
    }
    let commands = scenarios
        .iter()
        .map(|scenario| required_text(scenario, "command"))
        .collect::<Result<Vec<_>, _>>()?;
    if scenarios
        .iter()
        .any(|scenario| scenario["minimum_evidence_count"].as_integer() != Some(1))
    {
        return Err("Phase 5 scenarios must require at least one evidence result".to_owned());
    }
    let closed = require_closed || contract["status"].as_str() == Some("closed");
    let rows = parse_ledger(ledger)?;
    validate_rows(&rows, &expected, Some(&commands), closed)
}

pub(super) fn prove_all(ledger: &str) -> Result<String, String> {
    let mut rows = parse_ledger(ledger)?;
    for row in &mut rows {
        row[8] = "PROVED".to_owned();
        row[9] = format!(
            "source=final-source; command={}; result=passed evidence_count=1; owner={}",
            row[7], row[0]
        );
    }
    Ok(render_ledger(&rows))
}

pub(super) fn hostile_mutations(closed: &str) -> Vec<(&'static str, String)> {
    let rows = parse_ledger(closed).expect("closed mutation source parses");
    let mut mutations = Vec::new();

    let mut reopened = rows.clone();
    reopened[0][8] = "OPEN".to_owned();
    reopened[0][9].clear();
    mutations.push(("reopened", render_ledger(&reopened)));

    let mut reordered = rows.clone();
    reordered.swap(0, 1);
    mutations.push(("reordered", render_ledger(&reordered)));

    let mut missing = rows.clone();
    missing.remove(1);
    mutations.push(("missing", render_ledger(&missing)));

    let mut duplicated = rows.clone();
    duplicated[1] = duplicated[0].clone();
    mutations.push(("duplicated", render_ledger(&duplicated)));

    let mut command_drift = rows.clone();
    command_drift[0][7].push_str(" --ignored");
    mutations.push(("command drift", render_ledger(&command_drift)));

    let mut fabricated = rows;
    fabricated[0][9] = "all good".to_owned();
    mutations.push(("fabricated evidence", render_ledger(&fabricated)));

    let mut zero_count = parse_ledger(closed).expect("closed mutation source parses");
    zero_count[0][9] = zero_count[0][9].replace("evidence_count=1", "evidence_count=0");
    mutations.push(("zero evidence count", render_ledger(&zero_count)));

    mutations
}

fn validate_rows(
    rows: &[Vec<String>],
    expected_ids: &[String],
    commands: Option<&[&str]>,
    closed: bool,
) -> Result<(), String> {
    if rows.len() != expected_ids.len() {
        return Err(format!(
            "ledger has {} rows instead of {}",
            rows.len(),
            expected_ids.len()
        ));
    }
    for (index, row) in rows.iter().enumerate() {
        let expected_id = &expected_ids[index];
        if row.len() != 10 {
            return Err(format!(
                "{expected_id} has {} fields instead of 10",
                row.len()
            ));
        }
        if &row[0] != expected_id {
            return Err(format!("expected {expected_id}, found {}", row[0]));
        }
        for (field, value) in row.iter().enumerate().take(8).skip(1) {
            if value.trim().is_empty() {
                return Err(format!("{expected_id} has empty field {field}"));
            }
        }
        if let Some(commands) = commands {
            if row[7] != commands[index] {
                return Err(format!("{expected_id} command drifted from its manifest"));
            }
        }
        validate_status(expected_id, &row[8], &row[9], closed)?;
    }
    Ok(())
}

fn validate_status(id: &str, status: &str, evidence: &str, closed: bool) -> Result<(), String> {
    match (closed, status, evidence.trim().is_empty()) {
        (false, "OPEN", true) => Ok(()),
        (false, "PROVED", false) | (true, "PROVED", false) => {
            validate_structured_evidence(id, evidence)
        }
        (true, _, _) => Err(format!("{id} is not PROVED with evidence at closure")),
        (false, "OPEN", false) => Err(format!("{id} is OPEN with closing evidence")),
        (false, "PROVED", true) => Err(format!("{id} is PROVED without evidence")),
        (false, other, _) => Err(format!("{id} has unsupported status `{other}`")),
    }
}

fn validate_structured_evidence(id: &str, evidence: &str) -> Result<(), String> {
    for field in ["source=", "; command=", "; result=", "; owner="] {
        if !evidence.contains(field) {
            return Err(format!("{id} evidence omits `{field}`"));
        }
    }
    let count = evidence
        .split_whitespace()
        .find_map(|part| part.strip_prefix("evidence_count="))
        .and_then(|value| value.trim_end_matches(';').parse::<usize>().ok())
        .ok_or_else(|| format!("{id} evidence has no parseable evidence_count"))?;
    if count == 0 {
        return Err(format!("{id} evidence count is zero"));
    }
    Ok(())
}

fn parse_ledger(ledger: &str) -> Result<Vec<Vec<String>>, String> {
    let mut lines = ledger.lines();
    if lines.next() != Some(HEADER) {
        return Err("ledger header drifted".to_owned());
    }
    lines.map(parse_csv_record).collect()
}

fn parse_csv_record(line: &str) -> Result<Vec<String>, String> {
    let mut fields = vec![String::new()];
    let mut quoted = false;
    let mut characters = line.chars().peekable();
    while let Some(character) = characters.next() {
        match character {
            '"' if quoted && characters.peek() == Some(&'"') => {
                fields.last_mut().expect("one CSV field").push('"');
                characters.next();
            }
            '"' => quoted = !quoted,
            ',' if !quoted => fields.push(String::new()),
            value => fields.last_mut().expect("one CSV field").push(value),
        }
    }
    if quoted {
        return Err("ledger row has an unterminated quoted field".to_owned());
    }
    Ok(fields)
}

fn render_ledger(rows: &[Vec<String>]) -> String {
    let records = rows
        .iter()
        .map(|row| {
            row.iter()
                .map(|field| quote(field))
                .collect::<Vec<_>>()
                .join(",")
        })
        .collect::<Vec<_>>()
        .join("\n");
    format!("{HEADER}\n{records}\n")
}

fn quote(field: &str) -> String {
    format!("\"{}\"", field.replace('"', "\"\""))
}

fn required_text<'a>(value: &'a toml::Value, field: &str) -> Result<&'a str, String> {
    value[field]
        .as_str()
        .filter(|text| !text.trim().is_empty())
        .ok_or_else(|| format!("Phase 5 scenario has no `{field}`"))
}
