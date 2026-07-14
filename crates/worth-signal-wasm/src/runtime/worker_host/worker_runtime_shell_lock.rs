use crate::runtime::core::RuntimeCore;

use super::WorkerRuntimeShellLock;

impl RuntimeCore {
    pub fn worker_runtime_shell_lock(&self) -> WorkerRuntimeShellLock {
        WorkerRuntimeShellLock::dedicated_worker_runtime_shell()
    }
}
