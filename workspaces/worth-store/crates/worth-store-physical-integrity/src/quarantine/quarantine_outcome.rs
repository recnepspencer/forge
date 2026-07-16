use crate::{QuarantineRecord, QuarantineSealDenial};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct QuarantineSealCounterSnapshot {
    inspected_findings: u64,
    sealed_records: u64,
}

impl QuarantineSealCounterSnapshot {
    const fn sealed() -> Self {
        Self {
            inspected_findings: 1,
            sealed_records: 1,
        }
    }

    const fn denied() -> Self {
        Self {
            inspected_findings: 1,
            sealed_records: 0,
        }
    }

    pub const fn inspected_findings(self) -> u64 {
        self.inspected_findings
    }
    pub const fn sealed_records(self) -> u64 {
        self.sealed_records
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum QuarantineSealCase {
    Sealed(Box<QuarantineRecord>),
    Denied(QuarantineSealDenial),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QuarantineSealOutcome {
    case: QuarantineSealCase,
    counters: QuarantineSealCounterSnapshot,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QuarantineSealOutcomeView<'a> {
    Sealed(&'a QuarantineRecord),
    Denied(&'a QuarantineSealDenial),
}

impl QuarantineSealOutcome {
    pub(crate) fn sealed(record: QuarantineRecord) -> Self {
        Self {
            case: QuarantineSealCase::Sealed(Box::new(record)),
            counters: QuarantineSealCounterSnapshot::sealed(),
        }
    }

    pub(crate) fn denied(denial: QuarantineSealDenial) -> Self {
        Self {
            case: QuarantineSealCase::Denied(denial),
            counters: QuarantineSealCounterSnapshot::denied(),
        }
    }

    pub const fn view(&self) -> QuarantineSealOutcomeView<'_> {
        match &self.case {
            QuarantineSealCase::Sealed(record) => QuarantineSealOutcomeView::Sealed(record),
            QuarantineSealCase::Denied(denial) => QuarantineSealOutcomeView::Denied(denial),
        }
    }

    pub const fn counters(&self) -> QuarantineSealCounterSnapshot {
        self.counters
    }
    pub const fn is_err(&self) -> bool {
        matches!(self.case, QuarantineSealCase::Denied(_))
    }

    pub fn into_result(self) -> Result<QuarantineRecord, QuarantineSealDenial> {
        match self.case {
            QuarantineSealCase::Sealed(record) => Ok(*record),
            QuarantineSealCase::Denied(denial) => Err(denial),
        }
    }

    pub fn unwrap(self) -> QuarantineRecord {
        self.into_result().unwrap()
    }
    pub fn expect(self, message: &str) -> QuarantineRecord {
        self.into_result().expect(message)
    }
    pub fn unwrap_err(self) -> QuarantineSealDenial {
        self.into_result().unwrap_err()
    }
    pub fn expect_err(self, message: &str) -> QuarantineSealDenial {
        self.into_result().expect_err(message)
    }
}
