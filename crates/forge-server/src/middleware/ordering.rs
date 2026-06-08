use super::ForgeServerDenial;

pub(crate) fn select_primary_denial(
    left: ForgeServerDenial,
    right: ForgeServerDenial,
) -> ForgeServerDenial {
    if left.priority() <= right.priority() {
        left
    } else {
        right
    }
}
