pub(crate) struct BankReadProjectedBatch<Output> {
    output: Output,
    result_count: usize,
    truncated: bool,
}

impl<Output> BankReadProjectedBatch<Output> {
    pub(crate) const fn complete(output: Output, result_count: usize) -> Self {
        Self {
            output,
            result_count,
            truncated: false,
        }
    }

    pub(crate) const fn truncated(output: Output, result_count: usize) -> Self {
        Self {
            output,
            result_count,
            truncated: true,
        }
    }

    pub(crate) fn into_parts(self) -> (Output, usize, bool) {
        (self.output, self.result_count, self.truncated)
    }
}
