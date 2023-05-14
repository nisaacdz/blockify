use crate::Hash;

struct MerkleTree {
    tree: rs_merkle::MerkleTree<rs_merkle::algorithms::Sha256>,
}

impl MerkleTree {
    fn root(&self) -> Hash {
        match self.tree.root().map(|v| v.into()) {
            Some(v) => v,
            None => Hash::default(),
        }
    }

    fn push(&mut self, hash: Hash) {
        self.tree.insert(hash.into());
    }
}
