#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct OutputId(u64);

impl OutputId {
    pub fn new(value: u64) -> Self {
        Self(value)
    }

    pub fn value(&self) -> u64 {
        self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MockAuthorization {
    ClaimedOwner(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TxInput {
    pub output_id: OutputId,
    pub authorization: MockAuthorization,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TxOutput {
    pub owner: String,
    pub amount: u64,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Transaction {
    pub inputs: Vec<TxInput>,
    pub outputs: Vec<TxOutput>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creates_and_reads_output_id() {
        let output_id = OutputId::new(42);

        assert_eq!(output_id.value(), 42);
    }

    #[test]
    fn creates_claimed_owner_authorization() {
        let authorization = MockAuthorization::ClaimedOwner(String::from("Alice"));

        assert_eq!(
            authorization,
            MockAuthorization::ClaimedOwner(String::from("Alice"))
        );
    }

    #[test]
    fn creates_transaction_input() {
        let input = TxInput {
            output_id: OutputId::new(7),
            authorization: MockAuthorization::ClaimedOwner(String::from("Alice")),
        };

        assert_eq!(input.output_id.value(), 7);
        assert_eq!(
            input.authorization,
            MockAuthorization::ClaimedOwner(String::from("Alice"))
        );
    }
    #[test]
    fn creates_transaction_output() {
        let output = TxOutput {
            owner: String::from("Bob"),
            amount: 40,
        };
        assert_eq!(output.owner, String::from("Bob"));
        assert_eq!(output.amount, 40);
    }
    #[test]
    fn creates_transaction() {
        let transaction = Transaction {
        inputs: vec![TxInput {
            output_id: OutputId::new(7),
            authorization: MockAuthorization::ClaimedOwner(
                String::from("Alice"),
            ),
        }],
        outputs: vec![
            TxOutput {
                owner: String::from("Bob"),
                amount: 40,
            },
            TxOutput {
                owner: String::from("Alice"),
                amount: 60,
            },
        ],
    };
    assert_eq!(transaction.inputs.len(), 1);
    assert_eq!(transaction.outputs.len(), 2);
    assert_eq!(transaction.outputs[0].amount, 40);
    }
}
