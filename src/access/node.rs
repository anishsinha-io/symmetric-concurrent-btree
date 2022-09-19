use std::fmt::{Display, Formatter};
use serde::{Serialize, Deserialize};

use crate::traits::encode::Encode;
use crate::access::objptr::ObjectPtr;
use crate::ioutil::fs;
use crate::common::{Block, INDEX_PATH};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Node {
    pub min_order: u32,
    pub num_keys: u32,
    pub leaf: bool,
    pub loc: ObjectPtr,
    pub link: ObjectPtr,
    pub out_link: ObjectPtr,
    pub keys: Vec<usize>,
    pub high_key: usize,
    pub children: Vec<ObjectPtr>,
}

impl Encode for Node {}

impl Node {
    pub fn read(page_no: usize) -> Block {
        let mut buf = [0u8; 512];
        fs::read_bytes(INDEX_PATH, &mut buf, page_no as u64);
        buf
    }

    pub fn from_block(buf: Block) -> Self {
        Self::decode(buf.to_vec()).unwrap()
    }
}
