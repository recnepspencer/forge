use std::fs::{File, OpenOptions};

use fs4::FileExt;

use super::PhysicalOperationalControlStore;
use crate::operational_control::ControlMediaFault;

impl PhysicalOperationalControlStore {
    pub(super) fn with_shared_journal<T>(
        &self,
        inspect: impl FnOnce(&mut File) -> Result<T, ControlMediaFault>,
    ) -> Result<T, ControlMediaFault> {
        let mut file = OpenOptions::new().read(true).open(self.location.path())?;
        FileExt::lock_shared(&file)?;
        let result = self
            .verify_open_journal(&file)
            .and_then(|()| inspect(&mut file))
            .and_then(|value| self.verify_open_journal(&file).map(|()| value));
        let unlock = FileExt::unlock(&file);
        match (result, unlock) {
            (Ok(value), Ok(())) => Ok(value),
            (Err(error), _) => Err(error),
            (Ok(_), Err(error)) => Err(error.into()),
        }
    }
}
