use std::collections::HashMap;

use crate::model::{OutputId, TxOutput};

#[derive(Debug)]
pub struct UtxoLedger {
    utxos: HashMap<OutputId, TxOutput>,
    next_output_id: u64,
}

impl UtxoLedger {
    pub fn new() -> Self {
        Self {
            utxos: HashMap::new(),
            next_output_id: 0,
        }
    }

    pub fn create_genesis_output(
        &mut self,
        owner: String,
        amount: u64,
    ) -> OutputId {
        let output_id = OutputId::new(self.next_output_id);

        let output = TxOutput {
            owner,
            amount,
        };

        self.utxos.insert(output_id, output);
        self.next_output_id += 1;

        output_id
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creates_empty_ledger() {
        let ledger = UtxoLedger::new();

        assert_eq!(ledger.utxos.len(), 0);
        assert_eq!(ledger.next_output_id, 0);
    }

    #[test]
    fn creates_genesis_output() {
        let mut ledger = UtxoLedger::new();

        let output_id =
            ledger.create_genesis_output(String::from("Alice"), 100);

        assert_eq!(output_id.value(), 0);
        assert_eq!(ledger.utxos.len(), 1);
        assert_eq!(ledger.next_output_id, 1);
    }
}
