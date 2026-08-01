use crate::mounting::{UiMountedCollectionTextChange, UiMountedCollectionTextRow};

use super::super::super::UiMountedProjectionDenial;

pub(super) fn apply(
    predecessor: &[UiMountedCollectionTextRow],
    changes: &[UiMountedCollectionTextChange],
) -> Result<Vec<UiMountedCollectionTextRow>, UiMountedProjectionDenial> {
    let mut rows = predecessor.to_vec();
    for change in changes {
        apply_one(&mut rows, change)?;
    }
    Ok(rows)
}

fn apply_one(
    rows: &mut Vec<UiMountedCollectionTextRow>,
    change: &UiMountedCollectionTextChange,
) -> Result<(), UiMountedProjectionDenial> {
    use UiMountedCollectionTextChange as Change;

    match change {
        Change::Insert { row, at } => insert(rows, row, *at),
        Change::Remove { identity, from } => remove(rows, identity, *from),
        Change::Move { identity, from, to } => move_row(rows, identity, *from, *to),
        Change::Update(row) => update(rows, row),
        Change::WindowShift => Ok(()),
    }
}

fn insert(
    rows: &mut Vec<UiMountedCollectionTextRow>,
    row: &UiMountedCollectionTextRow,
    at: usize,
) -> Result<(), UiMountedProjectionDenial> {
    if at > rows.len() || find(rows, &row.identity()).is_some() {
        return Err(UiMountedProjectionDenial::InvalidSemanticCollectionPatch);
    }
    rows.insert(at, row.clone());
    Ok(())
}

fn remove(
    rows: &mut Vec<UiMountedCollectionTextRow>,
    identity: &worth_ui_query_binding::UiQueryEvidenceReference,
    from: usize,
) -> Result<(), UiMountedProjectionDenial> {
    if find(rows, identity) != Some(from) {
        return Err(UiMountedProjectionDenial::InvalidSemanticCollectionPatch);
    }
    rows.remove(from);
    Ok(())
}

fn move_row(
    rows: &mut Vec<UiMountedCollectionTextRow>,
    identity: &worth_ui_query_binding::UiQueryEvidenceReference,
    from: usize,
    to: usize,
) -> Result<(), UiMountedProjectionDenial> {
    if find(rows, identity) != Some(from) || to >= rows.len() {
        return Err(UiMountedProjectionDenial::InvalidSemanticCollectionPatch);
    }
    let row = rows.remove(from);
    rows.insert(to, row);
    Ok(())
}

fn update(
    rows: &mut [UiMountedCollectionTextRow],
    replacement: &UiMountedCollectionTextRow,
) -> Result<(), UiMountedProjectionDenial> {
    let Some(index) = find(rows, &replacement.identity()) else {
        return Err(UiMountedProjectionDenial::InvalidSemanticCollectionPatch);
    };
    rows[index] = replacement.clone();
    Ok(())
}

fn find(
    rows: &[UiMountedCollectionTextRow],
    identity: &worth_ui_query_binding::UiQueryEvidenceReference,
) -> Option<usize> {
    rows.iter().position(|row| row.identity() == *identity)
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use super::*;

    #[test]
    fn keyed_patch_preserves_identity_across_update_move_and_remove() {
        let [a, b, c] = identities(["a", "b", "c"]);
        let predecessor = [row(a, "Alpha"), row(b, "Bravo"), row(c, "Charlie")];
        let changes = [
            UiMountedCollectionTextChange::Update(row(b, "Bravo updated")),
            UiMountedCollectionTextChange::Move {
                identity: c,
                from: 2,
                to: 0,
            },
            UiMountedCollectionTextChange::Remove {
                identity: a,
                from: 1,
            },
        ];

        let applied = apply(&predecessor, &changes).expect("the exact keyed patch applies");

        assert_eq!(
            applied
                .iter()
                .map(|row| (row.identity(), row.selected_values()[0].as_ref()))
                .collect::<Vec<_>>(),
            [(c, "Charlie"), (b, "Bravo updated")]
        );
    }

    #[test]
    fn positional_twin_with_wrong_identity_is_rejected() {
        let [a, b] = identities(["a", "b"]);
        let predecessor = [row(a, "Alpha"), row(b, "Bravo")];
        let changes = [UiMountedCollectionTextChange::Remove {
            identity: b,
            from: 0,
        }];

        assert_eq!(
            apply(&predecessor, &changes),
            Err(UiMountedProjectionDenial::InvalidSemanticCollectionPatch)
        );
    }

    fn row(
        identity: worth_ui_query_binding::UiQueryEvidenceReference,
        value: &str,
    ) -> UiMountedCollectionTextRow {
        UiMountedCollectionTextRow::new(identity, [Arc::from(value)])
    }

    fn identities<const N: usize>(
        seeds: [&str; N],
    ) -> [worth_ui_query_binding::UiQueryEvidenceReference; N] {
        worth_ui_query_binding::certification::query_evidence_references(seeds)
    }
}
