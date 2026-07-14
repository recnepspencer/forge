use super::scales::FintechScale;

pub(super) fn book_for_instrument(scale: FintechScale, instrument_index: usize) -> usize {
    instrument_index % scale.books
}

pub(super) fn desk_for_book(scale: FintechScale, book_index: usize) -> usize {
    book_index % scale.desks
}
