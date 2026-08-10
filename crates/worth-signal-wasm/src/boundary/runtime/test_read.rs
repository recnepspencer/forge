use super::super::types::SignalRuntime;

#[cfg(test)]
impl SignalRuntime {
    pub(in crate::boundary) fn read_for_test(
        &self,
        id: &str,
    ) -> Result<crate::expression::model::SignalValue, crate::boundary::errors::WorthSignalJsError>
    {
        let value = self.core.borrow_mut().read_value(id)?;
        {
            let mut core = self.core.borrow_mut();
            core.note_compatibility_read(1);
            core.note_compatibility_signal_serialization(id, &value);
        }
        Ok(value)
    }
}
