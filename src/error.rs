use crate::model::OutputId;
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LedgerError {
    InputNotFound(OutputId),
    DuplicateInput(OutputId),
    UnauthorizedSpend(OutputId),
    ValueMismatch {
        input_total: u64,
        output_total: u64,
    },
    ZeroValueOutput,
    ArithmeticOverflow,
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stores_missing_input_id() {
        let error = LedgerError::InputNotFound(OutputId::new(9));

        assert_eq!(
            error,
            LedgerError::InputNotFound(OutputId::new(9))
        );
    }
}
