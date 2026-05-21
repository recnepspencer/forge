use std::borrow::Borrow;

use crate::construction::certification::corpus::compound::PrimitiveConstructionCompoundRow;

pub(crate) fn derive_specialized_rows<Row, Error, Rows>(
    rows: Rows,
    include: impl Fn(&PrimitiveConstructionCompoundRow) -> bool,
    mut build: impl FnMut(&PrimitiveConstructionCompoundRow) -> Result<Row, Error>,
) -> Result<Vec<Row>, Error>
where
    Rows: IntoIterator,
    Rows::Item: std::borrow::Borrow<PrimitiveConstructionCompoundRow>,
{
    let mut derived = Vec::new();
    for row in rows {
        let row = row.borrow();
        if include(row) {
            derived.push(build(row)?);
        }
    }
    Ok(derived)
}

pub(crate) fn require_specialized_row_field<T, Error>(
    scenario_id: &str,
    field_name: &str,
    value: Option<T>,
    invalid: impl FnOnce(String) -> Error,
) -> Result<T, Error> {
    value.ok_or_else(|| {
        invalid(format!(
            "compound specialized row '{}' is missing {}",
            scenario_id, field_name
        ))
    })
}
