use std::{cell::RefCell, rc::Rc};

use crate::gen;

pub struct MerkleNode {
    hash: Vec<u8>,
    left: Option<Rc<RefCell<MerkleNode>>>,
    right: Option<Rc<RefCell<MerkleNode>>>,
    center: Option<Rc<RefCell<MerkleNode>>>,
}

impl MerkleNode {
    /// Creates a new MerkleNode with the given hash and children.
    pub fn build(
        hash: Vec<u8>,
        left: Option<MerkleNode>,
        center: Option<MerkleNode>,
        right: Option<MerkleNode>,
    ) -> Self {
        Self {
            hash,
            left: left.map(RefCell::new).map(Rc::new),
            center: center.map(RefCell::new).map(Rc::new),
            right: right.map(RefCell::new).map(Rc::new),
        }
    }

    pub fn new() -> Self {
        Self {
            hash: gen::random_sha256(),
            left: None,
            center: None,
            right: None,
        }
    }

    /// Returns the hash of the node.
    pub fn hash(&self) -> &[u8] {
        &self.hash
    }

    /// Returns a reference to the left child of the node.
    pub fn left(&self) -> &Option<Rc<RefCell<MerkleNode>>> {
        &self.left
    }

    /// Returns a reference to the right child of the node.
    pub fn right(&self) -> &Option<Rc<RefCell<MerkleNode>>> {
        &self.right
    }

    pub fn center(&self) -> &Option<Rc<RefCell<MerkleNode>>> {
        &self.center
    }
}

/// A Merkle tree.
pub struct MerkleTree {
    root: MerkleNode,
    size: usize,
}

impl MerkleTree {
    /// Creates a new Merkle tree from the given leaf node hashes.
    pub fn new() -> Self {
        let left = MerkleNode::new();
        let center = MerkleNode::new();
        let dummy_root = MerkleNode::build(gen::random_sha256(), Some(left), Some(center), None);

        Self {
            root: dummy_root,
            size: 0,
        }
    }

    /// Returns the Merkle root of the tree.
    pub fn merkle_root(&self) -> &[u8] {
        &self.root.hash
    }

    pub fn insert(&mut self, _new_node: MerkleNode) {
        self.size += 1;

        todo!()
    }

    pub fn size(&self) -> usize {
        self.size
    }
}
