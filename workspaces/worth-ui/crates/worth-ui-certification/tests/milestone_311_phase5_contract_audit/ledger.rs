const HEADER: &str = "id,production_claim,fixture_provenance,typed_result,mutation_control,\
structural_cost,teardown,evidence_command,status,evidence";

pub(super) fn validate(
    contract: &toml::Value,
    ledger: &str,
    require_closed: bool,
) -> Result<(), String> {
    let contract_status = contract["status"]
        .as_str()
        .ok_or_else(|| "Phase 5 contract has no status".to_owned())?;
    if !matches!(contract_status, "implementation" | "closed") {
        return Err(format!("unsupported Phase 5 status `{contract_status}`"));
    }
    let closed = require_closed || contract_status == "closed";
    let expected_ids = (1..=9)
        .map(|number| format!("VS-{number:02}"))
        .collect::<Vec<_>>();
    let scenarios = contract["scenario"]
        .as_array()
        .ok_or_else(|| "Phase 5 scenarios are not an array".to_owned())?;
    let contract_ids = scenarios
        .iter()
        .map(|scenario| required_text(scenario, "id"))
        .collect::<Result<Vec<_>, _>>()?;
    if contract_ids != expected_ids {
        return Err(
            "Phase 5 contract scenarios are not exactly ordered VS-01 through VS-09".into(),
        );
    }
    let contract_commands = scenarios
        .iter()
        .map(|scenario| required_text(scenario, "command"))
        .collect::<Result<Vec<_>, _>>()?;

    let mut lines = ledger.lines();
    if lines.next() != Some(HEADER) {
        return Err("Phase 5 ledger header drifted".to_owned());
    }
    let rows = lines.map(parse_csv_record).collect::<Result<Vec<_>, _>>()?;
    if rows.len() != expected_ids.len() {
        return Err(format!(
            "Phase 5 ledger has {} rows instead of {}",
            rows.len(),
            expected_ids.len()
        ));
    }
    for (index, fields) in rows.iter().enumerate() {
        validate_row(
            fields,
            &expected_ids[index],
            contract_commands[index],
            closed,
        )?;
    }
    Ok(())
}

fn validate_row(
    fields: &[String],
    expected_id: &str,
    expected_command: &str,
    closed: bool,
) -> Result<(), String> {
    if fields.len() != 10 {
        return Err(format!(
            "{expected_id} has {} fields instead of 10",
            fields.len()
        ));
    }
    if fields[0] != expected_id {
        return Err(format!(
            "ledger row expected {expected_id} but found {}",
            fields[0]
        ));
    }
    for (index, field) in fields.iter().enumerate().take(8).skip(1) {
        if field.trim().is_empty() {
            return Err(format!("{expected_id} has empty field {index}"));
        }
    }
    if fields[7] != expected_command {
        return Err(format!(
            "{expected_id} command does not match its Phase 5 contract owner"
        ));
    }
    match (closed, fields[8].as_str(), fields[9].trim().is_empty()) {
        (true, "PROVED", false) | (false, "PROVED", false) | (false, "OPEN", true) => Ok(()),
        (true, status, _) => Err(format!(
            "{expected_id} must be PROVED with nonempty evidence at closure, found `{status}`"
        )),
        (false, "OPEN", false) => Err(format!("{expected_id} is OPEN with closing evidence")),
        (false, "PROVED", true) => Err(format!("{expected_id} is PROVED without evidence")),
        (false, status, _) => Err(format!("{expected_id} has unsupported status `{status}`")),
    }
}

fn required_text<'a>(value: &'a toml::Value, field: &str) -> Result<&'a str, String> {
    value[field]
        .as_str()
        .filter(|text| !text.trim().is_empty())
        .ok_or_else(|| format!("Phase 5 scenario has no `{field}`"))
}

fn parse_csv_record(line: &str) -> Result<Vec<String>, String> {
    let mut fields = vec![String::new()];
    let mut quoted = false;
    let mut characters = line.chars().peekable();
    while let Some(character) = characters.next() {
        match character {
            '"' if quoted && characters.peek() == Some(&'"') => {
                fields
                    .last_mut()
                    .expect("CSV starts with one field")
                    .push('"');
                characters.next();
            }
            '"' => quoted = !quoted,
            ',' if !quoted => fields.push(String::new()),
            value => fields
                .last_mut()
                .expect("CSV starts with one field")
                .push(value),
        }
    }
    if quoted {
        return Err("ledger row has an unterminated quoted field".to_owned());
    }
    Ok(fields)
}
