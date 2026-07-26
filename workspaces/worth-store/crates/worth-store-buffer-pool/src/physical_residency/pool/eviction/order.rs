use super::super::*;

impl PoolState {
    pub(in crate::physical_residency::pool) fn append_evictable(
        &mut self,
        coordinate: RecordFrameCoordinate,
    ) {
        let prior_tail = self.evictable_tail;
        let entry = self
            .frames
            .get_mut(&coordinate)
            .expect("an evictable frame remains resident");
        entry.older_evictable = prior_tail;
        entry.newer_evictable = None;
        if let Some(tail) = prior_tail {
            self.frames
                .get_mut(&tail)
                .expect("eviction tail remains resident")
                .newer_evictable = Some(coordinate);
        } else {
            self.evictable_head = Some(coordinate);
        }
        self.evictable_tail = Some(coordinate);
    }

    pub(in crate::physical_residency::pool) fn detach_evictable(
        &mut self,
        coordinate: RecordFrameCoordinate,
    ) {
        let Some(entry) = self.frames.get(&coordinate) else {
            return;
        };
        let older = entry.older_evictable;
        let newer = entry.newer_evictable;
        if older.is_none() && newer.is_none() && self.evictable_head != Some(coordinate) {
            return;
        }
        if let Some(older) = older {
            self.frames
                .get_mut(&older)
                .expect("older eviction neighbor remains resident")
                .newer_evictable = newer;
        } else {
            self.evictable_head = newer;
        }
        if let Some(newer) = newer {
            self.frames
                .get_mut(&newer)
                .expect("newer eviction neighbor remains resident")
                .older_evictable = older;
        } else {
            self.evictable_tail = older;
        }
        let entry = self
            .frames
            .get_mut(&coordinate)
            .expect("detached eviction entry remains resident");
        entry.older_evictable = None;
        entry.newer_evictable = None;
    }
}
