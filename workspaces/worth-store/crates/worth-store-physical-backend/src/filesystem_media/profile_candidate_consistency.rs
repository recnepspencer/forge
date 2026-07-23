pub(super) fn require_material_agreement<T: Eq>(
    first: T,
    remaining: impl IntoIterator<Item = T>,
) -> Result<T, std::io::ErrorKind> {
    for candidate in remaining {
        if candidate != first {
            return Err(std::io::ErrorKind::InvalidData);
        }
    }
    Ok(first)
}

#[cfg(test)]
mod tests {
    use super::require_material_agreement;

    #[test]
    fn conflicting_matching_volume_candidates_fail_closed() {
        let first = ("ntfs", 4096_u64, false, false);
        let conflict = ("ntfs", 65_536_u64, false, false);
        assert_eq!(
            require_material_agreement(first, [conflict]),
            Err(std::io::ErrorKind::InvalidData)
        );
    }
}
