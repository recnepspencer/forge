use std::sync::Arc;

use crate::mounting::{
    UiMountedCollectionRowIdentity, UiMountedCollectionTextChange,
    UiMountedCollectionTextDirective, UiMountedCollectionTextRow,
};

pub(super) fn project_collection(
    fact: &worth_ui_query_binding::UiCollectionProjectionFactReceipt,
) -> Result<(UiMountedCollectionTextDirective, Arc<str>), super::super::UiRebindPlanningDenial> {
    use worth_ui_query_binding::{UiPresentProjection, UiProjectionAvailability};

    match fact.availability() {
        UiProjectionAvailability::Unavailable(receipt) => Ok((
            UiMountedCollectionTextDirective::Clear,
            super::unavailable_label(receipt.kind()),
        )),
        UiProjectionAvailability::Present(UiPresentProjection::Current(value))
        | UiProjectionAvailability::Present(UiPresentProjection::RetainedStale { value, .. }) => {
            let directive = match fact.delivery() {
                worth_ui_query_binding::UiCollectionProjectionDelivery::Snapshot => {
                    UiMountedCollectionTextDirective::Replace(project_rows(value.rows())?)
                }
                worth_ui_query_binding::UiCollectionProjectionDelivery::Patch => {
                    UiMountedCollectionTextDirective::Patch(project_changes(fact, value.rows())?)
                }
            };
            Ok((directive, present_label(fact.availability(), value)))
        }
        UiProjectionAvailability::Stopped(receipt) => Ok((
            UiMountedCollectionTextDirective::Preserve,
            super::stopped_label(receipt.kind()),
        )),
    }
}

fn project_rows(
    rows: &[worth_ui_query_binding::UiCollectionProjectionTextRow],
) -> Result<Box<[UiMountedCollectionTextRow]>, super::super::UiRebindPlanningDenial> {
    rows.iter()
        .map(project_row)
        .collect::<Result<Vec<_>, _>>()
        .map(Vec::into_boxed_slice)
}

fn project_row(
    row: &worth_ui_query_binding::UiCollectionProjectionTextRow,
) -> Result<UiMountedCollectionTextRow, super::super::UiRebindPlanningDenial> {
    if row.selected_values().len() > usize::from(u16::MAX) {
        return Err(
            super::super::UiRebindPlanningDenial::InvalidCollectionProjectionContent(
                super::super::UiCollectionProjectionContentDenial::SelectedFieldCapacityExceeded,
            ),
        );
    }
    Ok(UiMountedCollectionTextRow::new(
        UiMountedCollectionRowIdentity::from_query(row.row()),
        row.selected_values()
            .iter()
            .map(|value| Arc::from(value.as_str()))
            .collect::<Vec<_>>(),
    ))
}

fn project_changes(
    fact: &worth_ui_query_binding::UiCollectionProjectionFactReceipt,
    rows: &[worth_ui_query_binding::UiCollectionProjectionTextRow],
) -> Result<Box<[UiMountedCollectionTextChange]>, super::super::UiRebindPlanningDenial> {
    let mut changed_rows = Vec::with_capacity(rows.len());
    for row in rows {
        if changed_rows.iter().any(
            |existing: &&worth_ui_query_binding::UiCollectionProjectionTextRow| {
                existing.row().query_identity() == row.row().query_identity()
            },
        ) {
            return Err(invalid(
                super::super::UiCollectionProjectionContentDenial::DuplicateChangedRow,
            ));
        }
        changed_rows.push(row);
    }
    let mut changes = Vec::with_capacity(fact.changes().len());
    for change in fact.changes() {
        changes.push(project_change(change, &mut changed_rows)?);
    }
    if !changed_rows.is_empty() {
        return Err(invalid(
            super::super::UiCollectionProjectionContentDenial::UnusedChangedRow,
        ));
    }
    Ok(changes.into_boxed_slice())
}

fn project_change(
    change: &worth_ui_query_binding::UiCollectionProjectionChange,
    rows: &mut Vec<&worth_ui_query_binding::UiCollectionProjectionTextRow>,
) -> Result<UiMountedCollectionTextChange, super::super::UiRebindPlanningDenial> {
    use worth_ui_query_binding::UiCollectionProjectionChange as Change;

    Ok(match change {
        Change::Insert { row, at } => UiMountedCollectionTextChange::Insert {
            row: take_row(rows, row)?,
            at: *at,
        },
        Change::Remove { row, from } => UiMountedCollectionTextChange::Remove {
            identity: UiMountedCollectionRowIdentity::from_query(row),
            from: *from,
        },
        Change::Move { row, from, to } => UiMountedCollectionTextChange::Move {
            identity: UiMountedCollectionRowIdentity::from_query(row),
            from: *from,
            to: *to,
        },
        Change::Regroup { row, .. } => UiMountedCollectionTextChange::Regroup {
            identity: UiMountedCollectionRowIdentity::from_query(row),
        },
        Change::Update { row } => UiMountedCollectionTextChange::Update(take_row(rows, row)?),
        Change::WindowShift => UiMountedCollectionTextChange::WindowShift,
        Change::ResetRequired { .. } => {
            return Err(invalid(
                super::super::UiCollectionProjectionContentDenial::ResetReachedContentPlanning,
            ));
        }
    })
}

fn take_row(
    rows: &mut Vec<&worth_ui_query_binding::UiCollectionProjectionTextRow>,
    reference: &worth_ui_query_binding::UiCollectionProjectionRowReference,
) -> Result<UiMountedCollectionTextRow, super::super::UiRebindPlanningDenial> {
    rows.iter()
        .position(|row| row.row().query_identity() == reference.query_identity())
        .map(|index| rows.remove(index))
        .ok_or_else(|| {
            invalid(super::super::UiCollectionProjectionContentDenial::MissingChangedRow)
        })
        .and_then(project_row)
}

fn present_label(
    availability: &worth_ui_query_binding::UiProjectionAvailability<
        worth_ui_query_binding::UiCollectionProjectionValue,
    >,
    value: &worth_ui_query_binding::UiCollectionProjectionValue,
) -> Arc<str> {
    use worth_ui_query_binding::{
        UiCollectionCompleteness, UiPresentProjection, UiProjectionAvailability,
    };

    let currency = match availability {
        UiProjectionAvailability::Present(UiPresentProjection::Current(_)) => "CURRENT",
        UiProjectionAvailability::Present(UiPresentProjection::RetainedStale {
            activity, ..
        }) => return super::retained_label(activity.kind()),
        UiProjectionAvailability::Unavailable(_) | UiProjectionAvailability::Stopped(_) => {
            unreachable!("present collection posture requires a present availability")
        }
    };
    let completeness = match value.completeness() {
        UiCollectionCompleteness::Complete => "COMPLETE",
        UiCollectionCompleteness::Partial => "PARTIAL",
    };
    if value.continuation().is_some() {
        Arc::from(format!("{currency} · {completeness} · CONTINUATION"))
    } else {
        Arc::from(format!("{currency} · {completeness}"))
    }
}

fn invalid(
    denial: super::super::UiCollectionProjectionContentDenial,
) -> super::super::UiRebindPlanningDenial {
    super::super::UiRebindPlanningDenial::InvalidCollectionProjectionContent(denial)
}
