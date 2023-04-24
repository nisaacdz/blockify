use crate::net::node::NodeId;

pub trait ConsensusProtocol {
    // The ConsensusMessage type represents the messages exchanged between nodes in the network.
    type Message;

    // The Block type represents the blocks of data added to the blockchain.
    type Block;

    // Initializes the consensus protocol with the current state of the blockchain.
    fn initialize(&mut self, current_state: Vec<Self::Block>, nodes: Vec<NodeId>);

    // Processes an incoming message and returns a response message.
    fn process_message(
        &mut self,
        message: &Self::Message,
        sender_id: &NodeId,
    ) -> Option<Self::Message>;

    // Generates a new block based on the current state of the blockchain and the consensus rules.
    fn generate_block(&self, current_state: Vec<Self::Block>) -> Option<Self::Block>;

    // Determines whether a given block is valid according to the consensus rules.
    fn is_block_valid(&self, block: &Self::Block, current_state: Vec<Self::Block>) -> bool;

    // Returns the IDs of the nodes currently participating in the consensus protocol.
    fn participating_nodes(&self) -> Vec<NodeId>;
}
