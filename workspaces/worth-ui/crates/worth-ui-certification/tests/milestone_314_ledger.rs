const HEADER: &str = "id,production_claim,fixture_provenance,typed_result,mutation_control,\
structural_cost,teardown,evidence_command,status,evidence";

pub(super) fn validate_phase_1(contract: &toml::Value, ledger: &str) -> Result<(), String> {
    validate_at_phase(contract, ledger, 1)
}

pub(super) fn validate_at_phase(
    contract: &toml::Value,
    ledger: &str,
    current_phase: i64,
) -> Result<(), String> {
    require_contract_identity(contract)?;
    let proof_rows = contract["proof_row"]
        .as_array()
        .ok_or_else(|| "proof_row is not an array".to_owned())?;
    let expected_ids = (1..=13)
        .map(|number| format!("IA-{number:02}"))
        .collect::<Vec<_>>();
    if proof_rows.len() != expected_ids.len() {
        return Err("contract does not contain exactly thirteen proof rows".to_owned());
    }

    let manifest_rows = proof_rows
        .iter()
        .zip(&expected_ids)
        .map(|(row, expected)| validate_manifest_row(row, expected))
        .collect::<Result<Vec<_>, _>>()?;
    let rows = parse_ledger(ledger)?;
    validate_rows(&rows, &expected_ids, &manifest_rows, current_phase)
}

fn require_contract_identity(contract: &toml::Value) -> Result<(), String> {
    let exact = [
        ("schema", "worth-ui.milestone-3.14.phase-1-contract.v1"),
        ("milestone", "3.14"),
        ("status", "closed"),
    ];
    for (field, expected) in exact {
        if contract[field].as_str() != Some(expected) {
            return Err(format!("contract `{field}` is not `{expected}`"));
        }
    }
    if contract["phase"].as_integer() != Some(1) {
        return Err("contract phase is not 1".to_owned());
    }
    if contract["evidence_topology"]["new_test_target"].as_bool() != Some(false)
        || contract["evidence_topology"]["new_compile_island"].as_bool() != Some(false)
        || contract["evidence_topology"]["compile_sessions"].as_integer() != Some(2)
    {
        return Err("Phase 1 changed the established evidence topology".to_owned());
    }
    Ok(())
}

fn validate_manifest_row<'a>(
    row: &'a toml::Value,
    expected_id: &str,
) -> Result<(&'a str, i64), String> {
    if required_text(row, "id")? != expected_id {
        return Err(format!("expected contract row {expected_id}"));
    }
    let phase = row["closure_phase"]
        .as_integer()
        .ok_or_else(|| format!("{expected_id} has no closure phase"))?;
    let expected_phase = match expected_id {
        "IA-02" | "IA-03" => 2,
        "IA-04" | "IA-05" | "IA-06" => 3,
        "IA-07" | "IA-08" | "IA-09" => 4,
        "IA-01" | "IA-10" | "IA-11" | "IA-12" | "IA-13" => 5,
        _ => return Err(format!("unsupported proof row {expected_id}")),
    };
    if phase != expected_phase {
        return Err(format!("{expected_id} closure phase drifted"));
    }
    Ok((required_text(row, "command")?, phase))
}

fn validate_rows(
    rows: &[Vec<String>],
    expected_ids: &[String],
    manifest_rows: &[(&str, i64)],
    current_phase: i64,
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
            return Err(format!("{expected_id} has {} fields", row.len()));
        }
        if &row[0] != expected_id {
            return Err(format!("expected {expected_id}, found {}", row[0]));
        }
        for (field, value) in row.iter().enumerate().take(8).skip(1) {
            if value.trim().is_empty() {
                return Err(format!("{expected_id} has empty field {field}"));
            }
        }
        let (command, closure_phase) = manifest_rows[index];
        if row[7] != command {
            return Err(format!("{expected_id} evidence command drifted"));
        }
        match row[8].as_str() {
            "OPEN" if !row[9].trim().is_empty() => {
                return Err(format!("{expected_id} has evidence while still OPEN"));
            }
            "OPEN" => {}
            "PROVED" if closure_phase > current_phase => {
                return Err(format!("{expected_id} closed before its owning phase"));
            }
            "PROVED" if row[9].trim().len() < 80 => {
                return Err(format!("{expected_id} has insubstantial proof evidence"));
            }
            "PROVED" => {}
            status => return Err(format!("{expected_id} has invalid status {status}")),
        }
    }
    Ok(())
}

pub(super) fn parse_ledger(ledger: &str) -> Result<Vec<Vec<String>>, String> {
    let mut lines = ledger.lines();
    if lines.next() != Some(HEADER) {
        return Err("ledger header drifted".to_owned());
    }
    lines.map(parse_csv_record).collect()
}

pub(super) fn render_ledger(rows: &[Vec<String>]) -> String {
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
        return Err("ledger row has an unterminated quote".to_owned());
    }
    Ok(fields)
}

fn quote(field: &str) -> String {
    format!("\"{}\"", field.replace('"', "\"\""))
}

fn required_text<'a>(value: &'a toml::Value, field: &str) -> Result<&'a str, String> {
    value[field]
        .as_str()
        .filter(|text| !text.trim().is_empty())
        .ok_or_else(|| format!("contract row has no `{field}`"))
}
