use serde::{Deserialize, Serialize};

use super::Hash;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MerkleNode {
    hash: Hash,
    left: Option<Box<MerkleNode>>,
    right: Option<Box<MerkleNode>>,
    center: Option<Box<MerkleNode>>,
}

impl MerkleNode {
    pub fn build(
        hash: Hash,
        left: Option<MerkleNode>,
        center: Option<MerkleNode>,
        right: Option<MerkleNode>,
    ) -> Self {
        Self {
            hash,
            left: left.map(Box::new),
            center: center.map(Box::new),
            right: right.map(Box::new),
        }
    }

    pub fn new() -> Self {
        Self {
            hash: super::random_sha256(),
            left: None,
            center: None,
            right: None,
        }
    }

    pub fn dummy() -> Self {
        Self::new()
    }

    /// Returns the hash of the node.
    pub fn hash(&self) -> &Hash {
        &self.hash
    }

    /// Returns a reference to the left child of the node.
    pub fn left(&self) -> &Option<Box<MerkleNode>> {
        &self.left
    }

    /// Returns a reference to the right child of the node.
    pub fn right(&self) -> &Option<Box<MerkleNode>> {
        &self.right
    }

    pub fn center(&self) -> &Option<Box<MerkleNode>> {
        &self.center
    }
}

/// A Merkle tree.
///
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MerkleTree {
    root: MerkleNode,
    size: usize,
}

impl std::hash::Hash for MerkleTree {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        state.write(self.merkle_root());
    }
}

impl MerkleTree {
    /// Creates a new Merkle tree from the given leaf node hashes.
    pub fn new() -> Self {
        let left = MerkleNode::dummy();
        let center = MerkleNode::dummy();
        let right = MerkleNode::dummy();
        let dummy_root = MerkleNode::build(
            super::random_sha256(),
            Some(left),
            Some(center),
            Some(right),
        );

        Self {
            root: dummy_root,
            size: 0,
        }
    }

    /// Returns the Merkle root of the tree.
    pub fn merkle_root(&self) -> &Hash {
        &self.root.hash
    }

    pub fn push(&mut self, hash: &Hash) {
        self.size += 1;

        let left_hash = self.root.left().as_deref().unwrap().hash();

        if let None = &self.root.center {
            let new_hash = super::sha_from_x([hash, left_hash, self.merkle_root()]);

            let mut new_node = MerkleNode::build(new_hash, None, None, None);

            new_node.left = self.root.left().clone();

            self.root.left = None;
            new_node.center = Some(Box::new(self.root.clone()));

            self.root = new_node;
        } else if let None = self.root.right {
            let center_hash = self.root.center().as_deref().unwrap().hash();
            let new_hash = super::sha_from_x([hash, left_hash, center_hash, self.merkle_root()]);

            let mut new_node = MerkleNode::build(new_hash, None, None, None);

            new_node.left = self.root.left().clone();
            new_node.center = self.root.center().clone();

            self.root.left = None;
            self.root.center = None;
            new_node.center = Some(Box::new(self.root.clone()));

            self.root = new_node;
        } else {
            let center_hash = self.root.center().as_deref().unwrap().hash();
            let right_hash = self.root.right().as_deref().unwrap().hash();
            let new_hash =
                super::sha_from_x([hash, left_hash, center_hash, right_hash, self.merkle_root()]);

            let mut new_node = MerkleNode::build(new_hash, None, None, None);

            new_node.left = Some(Box::new(self.root.clone()));

            self.root = new_node;
        }
    }

    pub fn pop(&self) -> bool {
        todo!()
    }

    pub fn size(&self) -> usize {
        self.size
    }
}
