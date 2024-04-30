use std::marker::PhantomData;

use crate::hash::hash_trait::{HashFunction, HashInputPair, HashOutput};
use crate::patricia_merkle_tree::node_data::inner_node::{
    BinaryData, EdgeData, NodeData, PathToBottom,
};
use crate::patricia_merkle_tree::node_data::leaf::{LeafData, LeafDataTrait};
use crate::types::Felt;

#[cfg(test)]
#[path = "hash_function_test.rs"]
pub mod hash_function_test;

pub(crate) trait TreeHashFunction<L: LeafDataTrait, H: HashFunction> {
    /// Computes the hash of given node data.
    fn compute_node_hash(node_data: &NodeData<L>) -> HashOutput;
}

pub(crate) struct TreeHashFunctionImpl<H: HashFunction> {
    _hash_function: PhantomData<H>,
}

/// Implementation of TreeHashFunction. The implementation is based on the following reference:
/// https://docs.starknet.io/documentation/architecture_and_concepts/Network_Architecture/starknet-state/#trie_construction
// TODO(Aner, 11/4/24): Verify the correctness of the implementation.
const CONTRACT_STATE_HASH_VERSION: Felt = Felt::ZERO;
impl<H: HashFunction> TreeHashFunction<LeafData, H> for TreeHashFunctionImpl<H> {
    fn compute_node_hash(node_data: &NodeData<LeafData>) -> HashOutput {
        match node_data {
            NodeData::Binary(BinaryData {
                left_hash,
                right_hash,
            }) => H::compute_hash(HashInputPair(left_hash.0, right_hash.0)),
            NodeData::Edge(EdgeData {
                bottom_hash: hash_output,
                path_to_bottom: PathToBottom { path, length },
            }) => HashOutput(
                H::compute_hash(HashInputPair(hash_output.0, path.0)).0 + Felt::from(length.0),
            ),
            NodeData::Leaf(LeafData::StorageValue(storage_value)) => HashOutput(*storage_value),
            NodeData::Leaf(LeafData::CompiledClassHash(compiled_class_hash)) => {
                HashOutput(compiled_class_hash.0)
            }
            NodeData::Leaf(LeafData::StateTreeTuple {
                class_hash,
                contract_state_root_hash,
                nonce,
            }) => H::compute_hash(HashInputPair(
                H::compute_hash(HashInputPair(
                    H::compute_hash(HashInputPair(class_hash.0, *contract_state_root_hash)).0,
                    nonce.0,
                ))
                .0,
                CONTRACT_STATE_HASH_VERSION,
            )),
        }
    }
}
