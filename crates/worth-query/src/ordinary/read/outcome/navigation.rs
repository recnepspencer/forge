use super::{WorthQueryReadCompletion, WorthQueryReadStop};

#[derive(Debug)]
pub enum WorthQueryReadOutcome {
    Completed(WorthQueryReadCompletion),
    Stopped(WorthQueryReadStop),
}

impl WorthQueryReadOutcome {
    pub fn completed(&self) -> Option<&WorthQueryReadCompletion> {
        match self {
            Self::Completed(result) => Some(result),
            Self::Stopped(_) => None,
        }
    }

    pub fn stop(&self) -> Option<&WorthQueryReadStop> {
        match self {
            Self::Completed(_) => None,
            Self::Stopped(stop) => Some(stop),
        }
    }

    pub fn into_result(self) -> Result<WorthQueryReadCompletion, WorthQueryReadStop> {
        match self {
            Self::Completed(result) => Ok(result),
            Self::Stopped(stop) => Err(stop),
        }
    }
}
