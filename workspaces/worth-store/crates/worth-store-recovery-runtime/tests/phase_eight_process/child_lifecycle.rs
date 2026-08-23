use std::process::{Child, ExitStatus};
use std::time::{Duration, Instant};

pub(super) struct ProcessChildGuard {
    child: Option<Child>,
    terminated: bool,
}

impl ProcessChildGuard {
    pub(super) fn new(child: Child) -> Self {
        Self {
            child: Some(child),
            terminated: false,
        }
    }

    pub(super) fn id(&self) -> u32 {
        self.child.as_ref().expect("guarded child is present").id()
    }

    pub(super) fn child_mut(&mut self) -> &mut Child {
        self.child.as_mut().expect("guarded child is present")
    }

    pub(super) fn kill_and_wait(&mut self) -> Result<ExitStatus, String> {
        self.terminate_and_reap("kill guarded Phase 8 child")
    }

    pub(super) fn wait_with_output_within(
        mut self,
        timeout: Duration,
    ) -> Result<std::process::Output, String> {
        let deadline = Instant::now() + timeout;
        loop {
            let poll = self
                .child
                .as_mut()
                .expect("guarded child is present")
                .try_wait();
            match poll {
                Ok(Some(_)) => {
                    self.terminated = true;
                    let child = self.child.take().expect("guarded child is present");
                    let output = child
                        .wait_with_output()
                        .map_err(|error| format!("collect guarded Phase 8 child output: {error}"));
                    return output;
                }
                Ok(None) if Instant::now() < deadline => {
                    std::thread::sleep(Duration::from_millis(20));
                }
                Ok(None) => {
                    let timeout_error =
                        format!("Phase 8 child exceeded bounded wait of {:?}", timeout);
                    return match self.terminate_and_reap("terminate timed-out Phase 8 child") {
                        Ok(_) => Err(timeout_error),
                        Err(cleanup) => Err(format!("{timeout_error}; cleanup failed: {cleanup}")),
                    };
                }
                Err(error) => {
                    let poll_error = format!("poll guarded Phase 8 child: {error}");
                    return match self.terminate_and_reap("terminate unpollable Phase 8 child") {
                        Ok(_) => Err(poll_error),
                        Err(cleanup) => Err(format!("{poll_error}; cleanup failed: {cleanup}")),
                    };
                }
            }
        }
    }

    fn terminate_and_reap(&mut self, label: &str) -> Result<ExitStatus, String> {
        let child = self.child.as_mut().expect("guarded child is present");
        let kill_error = child.kill().err().map(|error| format!("{label}: {error}"));
        let wait_result = child
            .wait()
            .map_err(|error| format!("wait for guarded Phase 8 child: {error}"));
        match wait_result {
            Ok(status) => {
                self.terminated = true;
                match kill_error {
                    Some(error) => Err(format!(
                        "{error}; child was nevertheless reaped as {status}"
                    )),
                    None => Ok(status),
                }
            }
            Err(wait_error) => match kill_error {
                Some(kill_error) => Err(format!("{kill_error}; {wait_error}")),
                None => Err(wait_error),
            },
        }
    }
}

impl Drop for ProcessChildGuard {
    fn drop(&mut self) {
        if self.terminated {
            return;
        }
        if let Some(child) = self.child.as_mut() {
            if child.try_wait().ok().flatten().is_none() {
                if let Err(error) = self.terminate_and_reap("emergency child termination") {
                    eprintln!("Phase 8 emergency child cleanup failed: {error}");
                }
            }
        }
    }
}
