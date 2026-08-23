use super::super::documents::split_csv;

const HEADER: &str = "scope,surface,source_owner,disposition,destination_owner,phase";

pub(super) fn parse_inventory(document: &str) -> Result<Vec<ApiRow>, String> {
    let mut lines = document.lines();
    if lines.next() != Some(HEADER) {
        return Err("C.8 API inventory has an invalid schema".into());
    }
    lines
        .filter(|line| !line.trim().is_empty())
        .enumerate()
        .map(|(index, line)| {
            let columns = split_csv(line, 6)
                .map_err(|error| format!("C.8 API row {}: {error}", index + 2))?;
            Ok(ApiRow {
                scope: columns[0].to_owned(),
                surface: columns[1].to_owned(),
                source_owner: columns[2].to_owned(),
                disposition: columns[3].to_owned(),
                destination_owner: columns[4].to_owned(),
                phase: columns[5].to_owned(),
            })
        })
        .collect()
}

pub(super) struct ApiRow {
    pub(super) scope: String,
    pub(super) surface: String,
    pub(super) source_owner: String,
    pub(super) disposition: String,
    pub(super) destination_owner: String,
    pub(super) phase: String,
}
