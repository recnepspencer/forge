use super::WorthServerDenial;

pub(crate) fn select_primary_denial(
    left: WorthServerDenial,
    right: WorthServerDenial,
) -> WorthServerDenial {
    if left.priority() <= right.priority() {
        left
    } else {
        right
    }
}
