use arc_swap::ArcSwap;

use crate::types::{Hex, MerkleRoot};

lazy_static::lazy_static! {
    pub static ref CURRENT_STATE_ROOT: ArcSwap<MerkleRoot> = ArcSwap::from_pointee(Default::default());
    pub static ref CHAIN_ID: ArcSwap<u64> = ArcSwap::from_pointee(Default::default());
    pub static ref PROTOCOL_VERSION: ArcSwap<Hex> = ArcSwap::from_pointee(Default::default());
}
