use std::fmt::{Display, Formatter};
use serde::{Serialize, Deserialize};

use crate::traits::encode::Encode;
use crate::access::objptr::ObjectPtr;

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
