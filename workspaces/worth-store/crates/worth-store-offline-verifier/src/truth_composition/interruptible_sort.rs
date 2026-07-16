use std::cmp::Ordering;

pub(super) fn sort_by<T>(
    values: &mut [T],
    mut compare: impl FnMut(&T, &T) -> Ordering,
    reject_interruption: &mut impl FnMut() -> Result<(), crate::OfflineInspectionDenial>,
) -> Result<(), crate::OfflineInspectionDenial> {
    reject_interruption()?;
    for root in (0..values.len() / 2).rev() {
        sift_down(
            values,
            root,
            values.len(),
            &mut compare,
            reject_interruption,
        )?;
    }
    for end in (1..values.len()).rev() {
        reject_interruption()?;
        values.swap(0, end);
        sift_down(values, 0, end, &mut compare, reject_interruption)?;
    }
    Ok(())
}

fn sift_down<T>(
    values: &mut [T],
    mut root: usize,
    end: usize,
    compare: &mut impl FnMut(&T, &T) -> Ordering,
    reject_interruption: &mut impl FnMut() -> Result<(), crate::OfflineInspectionDenial>,
) -> Result<(), crate::OfflineInspectionDenial> {
    loop {
        reject_interruption()?;
        let left = root
            .checked_mul(2)
            .and_then(|index| index.checked_add(1))
            .unwrap_or(end);
        if left >= end {
            return Ok(());
        }
        let right = left + 1;
        let greater = if right < end && compare(&values[left], &values[right]).is_lt() {
            right
        } else {
            left
        };
        if !compare(&values[root], &values[greater]).is_lt() {
            return Ok(());
        }
        values.swap(root, greater);
        root = greater;
    }
}

#[cfg(test)]
mod tests {
    use super::sort_by;

    #[test]
    fn sort_is_ordered_and_interruptible_inside_heap_work() {
        let mut values = [9, 1, 7, 3, 8, 2, 6, 4, 5, 0];
        let mut checks = 0_u64;
        sort_by(&mut values, Ord::cmp, &mut || {
            checks += 1;
            Ok(())
        })
        .expect("sort");
        assert_eq!(values, [0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
        assert!(checks > values.len() as u64);

        let mut interrupted = [9, 1, 7, 3, 8, 2, 6, 4, 5, 0];
        let mut remaining = 3_u64;
        let denial = sort_by(&mut interrupted, Ord::cmp, &mut || {
            remaining = remaining.saturating_sub(1);
            if remaining == 0 {
                Err(crate::OfflineInspectionDenial::Cancelled)
            } else {
                Ok(())
            }
        })
        .expect_err("interruption must escape heap work");
        assert!(matches!(denial, crate::OfflineInspectionDenial::Cancelled));
    }
}
