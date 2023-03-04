use crate::gen;

#[derive(Clone)]
pub struct MerkleNode {
    hash: Vec<u8>,
    left: Option<Box<MerkleNode>>,
    right: Option<Box<MerkleNode>>,
    center: Option<Box<MerkleNode>>,
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
            left: left.map(Box::new),
            center: center.map(Box::new),
            right: right.map(Box::new),
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

    pub fn insert(&mut self, hash: Vec<u8>) {
        self.size += 1;

        let left_hash = self.root.left().as_deref().unwrap().hash();

        if let None = &self.root.center {
            let new_hash = gen::sha_from_3(&hash, left_hash, self.merkle_root());

            let mut new_node = MerkleNode::build(new_hash, None, None, None);

            new_node.left = self.root.left().clone();

            self.root.left = None;
            new_node.center = Some(Box::new(self.root.clone()));

            self.root = new_node;
        } else if let None = self.root.right {
            let center_hash = self.root.center().as_deref().unwrap().hash();
            let new_hash = gen::sha_from_4(&hash, left_hash, center_hash, self.merkle_root());

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
            let new_hash = gen::sha_from_5(
                &hash,
                left_hash,
                center_hash,
                right_hash,
                self.merkle_root(),
            );

            let mut new_node = MerkleNode::build(new_hash, None, None, None);

            new_node.left = Some(Box::new(self.root.clone()));

            self.root = new_node;
        }

    }

    pub fn size(&self) -> usize {
        self.size
    }
}
