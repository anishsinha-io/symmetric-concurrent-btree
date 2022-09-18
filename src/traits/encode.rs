use serde::Serialize;
use serde::de::{DeserializeOwned};

use crate::common::{Block, INDEX_PATH};
use crate::ioutil::fs;

pub trait Encode {
    fn encode(&self) -> Option<Vec<u8>>
        where Self: Serialize + Sized
    {
        let encoded = bincode::serialize(&self);
        match encoded {
            Ok(encoded) => Some(encoded),
            Err(_) => None,
        }
    }

    fn decode(bytes: Vec<u8>) -> Option<Self>
        where
            Self: Sized + Serialize + DeserializeOwned,
    {
        let decoded = bincode::deserialize(&bytes[..]);
        match decoded {
            Ok(decoded) => Some(decoded),
            Err(_) => None,
        }
    }
}