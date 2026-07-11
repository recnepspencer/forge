mod tail;

pub use tail::{
    admit_replay_cursor, inspect_replay_tail_record, AdmittedReplayTailCursor,
    WalReplayTailCursorReport, WalReplayTailRecordReport,
};
