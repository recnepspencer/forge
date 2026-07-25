use std::sync::{Arc, Barrier};

use super::*;

#[test]
fn disjoint_candidate_sessions_and_reads_progress_independently() {
    let identity = store(16);
    let pool = PhysicalResidencyPool::open(identity, limits(256, 3, 2, 64, 6)).unwrap();
    let stable_key = PhysicalFrameKey::new(identity, coordinate(1, 16));
    drop(pool.load(stable_key, |bytes| fill(bytes, 3)).unwrap());
    let first_key = PhysicalFrameKey::new(identity, coordinate(2, 32));
    let second_key = PhysicalFrameKey::new(identity, coordinate(3, 32));
    let mut first = pool.reserve_candidate_frames(&[first_key]).unwrap();
    let mut second = pool.reserve_candidate_frames(&[second_key]).unwrap();
    let first_dirty = first
        .reserve_next(first_key)
        .unwrap()
        .admit(vec![4; 32])
        .unwrap();
    let second_dirty = second
        .reserve_next(second_key)
        .unwrap()
        .admit(vec![5; 32])
        .unwrap();
    let stable = pool
        .load(stable_key, |_| -> Result<(), ()> {
            panic!("an unrelated resident read must not refault")
        })
        .unwrap();
    assert_eq!(&*stable, &[3; 16]);
    drop(stable);
    drop(first_dirty.publish_clean_for_pool_test().unwrap());
    drop(second_dirty.publish_clean_for_pool_test().unwrap());
    drop((first, second));
    assert!(!pool.close().requires_inspection());
}

#[test]
fn unrelated_faults_do_not_hold_the_metadata_lock_during_io() {
    let identity = store(6);
    let pool = Arc::new(PhysicalResidencyPool::open(identity, limits(2048, 2, 1, 64, 4)).unwrap());
    let barrier = Arc::new(Barrier::new(2));
    let mut workers = Vec::new();
    for block in 1..=2 {
        let pool = Arc::clone(&pool);
        let barrier = Arc::clone(&barrier);
        workers.push(std::thread::spawn(move || {
            let key = PhysicalFrameKey::new(identity, coordinate(block, 32));
            pool.load(key, |target| {
                barrier.wait();
                target.fill(block as u8);
                Ok::<_, ()>(())
            })
            .unwrap()
        }));
    }
    let leases: Vec<_> = workers
        .into_iter()
        .map(|worker| worker.join().unwrap())
        .collect();
    assert_eq!(leases.len(), 2);
    assert_eq!(pool.counters().source_loads(), 2);
}
