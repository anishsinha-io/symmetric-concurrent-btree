#![allow(dead_code)]

use serde::{Serialize, Deserialize};

use crate::access::objptr::ObjectPtr;
use crate::ioutil::fs;
use crate::common::{Block, INDEX_PATH};
use crate::traits::encode::Encode;

// Anchor is essentially the header page
#[derive(Serialize, Deserialize)]
pub struct Anchor {
    // fast height
    pub fast_height: usize,
    // actual height
    pub top_height: usize,
    // fast pointer. this is the logical root location but may not be the physical root location
    pub fast: ObjectPtr,
    // pointer to actual top of tree
    pub top: ObjectPtr,
}

impl Encode for Anchor {}

impl Anchor {
    pub fn read() -> Block {
        let mut buf = [0u8; 512];
        fs::read_bytes(INDEX_PATH, &mut buf, 0);
        buf
    }

    pub fn from_block(buf: Block) -> Self {
        Self::decode(buf.to_vec()).unwrap()
    }
}