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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creates_and_reads_output_id() {
        let output_id = OutputId::new(42);
        assert_eq!(output_id.value(), 42);
    }
}
