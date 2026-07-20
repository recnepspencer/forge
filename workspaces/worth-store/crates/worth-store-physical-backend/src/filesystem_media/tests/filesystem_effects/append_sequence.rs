use super::super::super::*;
use super::fixture::{created, owner, staged_path};

#[test]
fn append_contention_assigns_disjoint_exact_ranges_inside_one_owner() {
    let (_root, owner) = owner();
    let path = staged_path(&owner, 2);
    let handle = created(owner.create_new(&path));
    let alias = match owner.open_existing_for_mutation(&path).into_result() {
        NamespaceFileOpenResult::Opened { handle, .. } => handle,
        other => panic!("open same-file alias: {other:?}"),
    };
    let blocks = [b"aaaaaaaa", b"bbbbbbbb", b"cccccccc", b"dddddddd"];

    let mut starts = std::thread::scope(|scope| {
        let joins = blocks
            .iter()
            .enumerate()
            .map(|(index, block)| {
                let target = if index % 2 == 0 { &handle } else { &alias };
                scope.spawn(move || target.append(AppendRequest::new(*block)))
            })
            .collect::<Vec<_>>();
        joins
            .into_iter()
            .map(|join| match join.join().expect("append thread").result() {
                MediaOperationResult::Completed(CompletedMediaEffect::AppendCompleted(
                    transfer,
                )) => match transfer.start() {
                    MediaTransferPosition::KnownAppendPosition(start) => start,
                    other => panic!("append start was not established: {other:?}"),
                },
                other => panic!("append did not complete: {other:?}"),
            })
            .collect::<Vec<_>>()
    });
    starts.sort_unstable();
    assert_eq!(starts, [0, 8, 16, 24]);
    assert!(matches!(
        handle.metadata().result(),
        MediaMetadataResult::Observed(metadata)
            if metadata.file_type() == MediaFileType::RegularFile
                && metadata.logical_length() == 32
    ));
}

#[test]
fn append_and_positioned_extension_are_one_serializable_file_sequence() {
    let (_root, owner) = owner();
    let path = staged_path(&owner, 3);
    let handle = created(owner.create_new(&path));
    let start = std::sync::Barrier::new(2);
    let (append, positioned) = std::thread::scope(|scope| {
        let append = scope.spawn(|| {
            start.wait();
            handle.append(AppendRequest::new(b"aaaaaaaa"))
        });
        let positioned = scope.spawn(|| {
            start.wait();
            handle.positioned_write(PositionedWriteRequest::new(0, b"pppppppp"))
        });
        (
            append.join().expect("append thread"),
            positioned.join().expect("positioned thread"),
        )
    });
    assert_eq!(
        positioned.effect_status(),
        MediaEffectStatus::CompletedEffect
    );
    let append_start = match append.result() {
        MediaOperationResult::Completed(CompletedMediaEffect::AppendCompleted(transfer)) => {
            match transfer.start() {
                MediaTransferPosition::KnownAppendPosition(start) => start,
                other => panic!("append range unavailable: {other:?}"),
            }
        }
        other => panic!("append failed: {other:?}"),
    };
    let mut bytes = [0_u8; 16];
    let observed = handle.positioned_read(PositionedReadRequest::new(0, &mut bytes));
    match append_start {
        0 => {
            assert!(matches!(
                observed.result(),
                PositionedReadResult::Failed(failure)
                    if matches!(
                        failure.kind(),
                        MediaOperationFailureKind::PartialTransfer(transfer)
                            if transfer.completed_bytes() == 8
                    )
            ));
            assert_eq!(&bytes[..8], b"pppppppp");
        }
        8 => {
            assert!(matches!(
                observed.result(),
                PositionedReadResult::Transferred(transfer) if transfer.bytes() == 16
            ));
            assert_eq!(&bytes, b"ppppppppaaaaaaaa");
        }
        other => panic!("non-serializable append start {other}"),
    }
}
