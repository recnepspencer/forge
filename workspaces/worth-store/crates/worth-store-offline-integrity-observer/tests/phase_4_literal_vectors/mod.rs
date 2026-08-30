mod checkpoint_records;
mod durable_frames;
mod oracle;
mod page_frames;
mod physical_work_obligation;
mod wal_frame;

#[test]
fn physical_work_literal_is_independently_consumable() {
    physical_work_obligation::verify();
}

#[test]
fn durable_frame_family_literals_are_independently_consumable() {
    durable_frames::verify();
}

#[test]
fn every_declared_page_size_literal_is_independently_consumable() {
    page_frames::verify();
}

#[test]
fn wal_literal_is_independently_consumable() {
    wal_frame::verify();
}

#[test]
fn every_checkpoint_record_kind_is_independently_consumable() {
    checkpoint_records::verify();
}
