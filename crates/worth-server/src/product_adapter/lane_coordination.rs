use std::{
    collections::HashMap,
    sync::{Arc, Mutex, Weak},
};

#[derive(Clone, Debug, Default)]
pub(super) struct WorthServerProductMutationLaneCoordinator {
    lanes: Arc<Mutex<HashMap<String, Weak<Mutex<()>>>>>,
}

impl WorthServerProductMutationLaneCoordinator {
    pub(super) fn coordinate<T>(&self, lane_identity: &str, operation: impl FnOnce() -> T) -> T {
        let lane = self.acquire_lane(lane_identity);
        let lane_guard = lane.lock().expect("product mutation lane lock");
        let result = operation();
        drop(lane_guard);
        self.release_unused_lane(lane_identity, &lane);
        result
    }

    fn acquire_lane(&self, lane_identity: &str) -> Arc<Mutex<()>> {
        let mut lanes = self.lanes.lock().expect("product mutation lane registry");
        if let Some(lane) = lanes.get(lane_identity).and_then(Weak::upgrade) {
            return lane;
        }
        let lane = Arc::new(Mutex::new(()));
        lanes.insert(lane_identity.to_string(), Arc::downgrade(&lane));
        lane
    }

    fn release_unused_lane(&self, lane_identity: &str, lane: &Arc<Mutex<()>>) {
        let mut lanes = self.lanes.lock().expect("product mutation lane registry");
        let remove = lanes
            .get(lane_identity)
            .and_then(Weak::upgrade)
            .is_some_and(|registered| {
                Arc::ptr_eq(&registered, lane) && Arc::strong_count(lane) == 2
            });
        if remove {
            lanes.remove(lane_identity);
        }
    }
}
