#![allow(dead_code)]

use serde::{Serialize, Deserialize};

// BlockData contains the page number of an Object Pointer
#[derive(Copy, Clone, Serialize, Deserialize, Debug, Ord, PartialOrd, Eq, PartialEq)]
struct BlockData {
    page_no: usize,
}

// Offset contains the offset within a block of an Object Pointer
#[derive(Copy, Clone, Serialize, Deserialize, Debug, Ord, PartialOrd, Eq, PartialEq)]
struct Offset {
    offset_bytes: usize,
}

// ObjectPtr provides an address for an item on the disk
#[derive(Copy, Clone, Serialize, Deserialize, Debug, Ord, PartialOrd, Eq, PartialEq)]
pub struct ObjectPtr {
    data: BlockData,
    offset: Offset,
}
