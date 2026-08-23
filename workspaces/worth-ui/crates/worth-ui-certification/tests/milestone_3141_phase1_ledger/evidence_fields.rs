use std::collections::BTreeMap;

pub(super) fn validate_cost(value: &str) -> Result<(), String> {
    let fields = named_numeric_fields(value)?;
    (!fields.is_empty())
        .then_some(())
        .ok_or_else(|| "empty cost evidence".to_owned())
}

pub(super) fn named_numeric_fields(value: &str) -> Result<BTreeMap<&str, u64>, String> {
    let mut fields = BTreeMap::new();
    for field in value.split(';') {
        let Some((name, amount)) = field.split_once('=') else {
            return Err("cost evidence must be named numeric counters".to_owned());
        };
        let amount = amount
            .parse::<u64>()
            .map_err(|_| "invalid cost counter".to_owned())?;
        if name.is_empty() || fields.insert(name, amount).is_some() {
            return Err("invalid cost counter".to_owned());
        }
    }
    Ok(fields)
}

pub(super) fn validate_mutation_control(value: &str, expected_family: &str) -> Result<(), String> {
    let fields = value
        .split(';')
        .map(|field| {
            field
                .split_once('=')
                .ok_or_else(|| "mutation control must use named fields".to_owned())
        })
        .collect::<Result<BTreeMap<_, _>, _>>()?;
    if fields.get("family") != Some(&expected_family)
        || fields.get("case").is_none_or(|case| case.is_empty())
    {
        return Err("proved evidence has the wrong mutation family".to_owned());
    }
    Ok(())
}
