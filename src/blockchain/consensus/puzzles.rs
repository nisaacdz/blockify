pub trait ConsensusPuzzle {
    type AttemptType;
    fn verify(&self, attempt: Self::AttemptType) -> bool;
}

pub struct HashPrefixPuzzle<const D: usize> {
    prefix: [u8; D],
    input: crate::Hash,
}

impl<const D: usize> HashPrefixPuzzle<D> {
    pub fn new(prefix: [u8; D], input: crate::Hash) -> Self {
        Self { prefix, input }
    }

    pub fn test_value(&self, value: &crate::Hash) -> bool {
        let res = crate::sha_from_x([value, &self.input]);
        res.starts_with(&self.prefix)
    }
}

impl<const D: usize> ConsensusPuzzle for HashPrefixPuzzle<D> {
    type AttemptType = crate::Hash;
    fn verify(&self, attempt: Self::AttemptType) -> bool {
        self.test_value(&attempt)
    }
}