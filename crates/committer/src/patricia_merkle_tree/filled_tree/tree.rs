use std::collections::HashMap;

use crate::hash::hash_trait::HashOutput;
use crate::patricia_merkle_tree::filled_tree::errors::FilledTreeError;
use crate::patricia_merkle_tree::filled_tree::node::FilledNode;
use crate::patricia_merkle_tree::node_data::leaf::{LeafData, LeafDataImpl};
use crate::patricia_merkle_tree::types::NodeIndex;
use crate::storage::storage_trait::{StorageKey, StorageValue};
use crate::storage::serde_trait::Serializable;

/// Consider a Patricia-Merkle Tree which has been updated with new leaves.
/// FilledTree consists of all nodes which were modified in the update, including their updated
/// data and hashes.
pub(crate) trait FilledTree<L: LeafData> {
    /// Serializes the tree into storage. Returns hash set of keys of the serialized nodes,
    /// if successful.
    #[allow(dead_code)]
    fn serialize(&self) -> HashMap<StorageKey, StorageValue>;
    #[allow(dead_code)]
    fn get_root_hash(&self) -> Result<HashOutput, FilledTreeError>;
}

pub(crate) struct FilledTreeImpl<L: LeafData> {
    tree_map: HashMap<NodeIndex, FilledNode<L>>,
}

impl<L: LeafData> FilledTreeImpl<L> {
    #[allow(dead_code)]
    pub(crate) fn new(tree_map: HashMap<NodeIndex, FilledNode<L>>) -> Self {
        Self { tree_map }
    }

    #[allow(dead_code)]
    pub(crate) fn get_all_nodes(&self) -> &HashMap<NodeIndex, FilledNode<L>> {
        &self.tree_map
    }
}

impl FilledTree<LeafDataImpl> for FilledTreeImpl<LeafDataImpl> {
    fn serialize(&self) -> HashMap<StorageKey, StorageValue> {

        let mut serialize_tree_map: HashMap<StorageKey, StorageValue> = HashMap::new();

        for (_node_index, node) in self.tree_map.iter() {
            serialize_tree_map.insert(node.db_key(), node.serialize());
            }
        serialize_tree_map
    }

    fn get_root_hash(&self) -> Result<HashOutput, FilledTreeError> {
        match self.tree_map.get(&NodeIndex::ROOT) {
            Some(root_node) => Ok(root_node.hash),
            None => Err(FilledTreeError::MissingRoot),
        }
    }
}
