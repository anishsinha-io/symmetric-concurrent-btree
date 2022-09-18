use std::sync::{Arc, RwLock};
use lazy_static::lazy_static;
use std::fmt::{Display, Formatter};

use crate::common::Block;

// each buffer pool frame contains the following information:
// - dirty represents whether this frame has been modified since it was read into memory
// - pins represents how many threads are using this frame
// - page_no_held is the page number of the buffer held
// - page_held is the actual page held
#[derive(Debug, Clone)]
pub struct BufferPoolFrame {
    pub dirty: bool,
    pub pins: usize,
    pub block_no_held: usize,
    pub block_held: Block,
}

impl Display for BufferPoolFrame {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "BufferPoolFrame [pins={} page_no_held={} dirty={}]", self.pins, self.block_no_held, self.dirty)
    }
}

impl BufferPoolFrame {
    pub fn new(page_no: usize, block: Block) -> Self {
        BufferPoolFrame {
            pins: 0,
            block_no_held: page_no,
            dirty: false,
            block_held: block,
        }
    }
}


// a buffer pool is a thread safe structure. it consists of a vector of BufferPoolFrames which are also thread safe
// due to the RwLock and Arc wrapping them. The entire structure is wrapped in a RwLock and an Arc
type BufferPool = Arc<RwLock<Vec<Arc<RwLock<BufferPoolFrame>>>>>;

// initialized only once. this is the only buffer pool that should ever be used
lazy_static! {
   pub static ref GLOBAL_BUFFER_POOL: BufferPool = Arc::new(RwLock::new(Vec::new()));
}

// function to fetch a frame from the pool
pub fn fetch_frame(page_no: usize) -> Option<BufferPoolFrame> {
    if let Ok(pages) = GLOBAL_BUFFER_POOL.read() {
        if let Some(frame) = pages.get(page_no) {
            if let Ok(locked) = frame.read() {
                return Some((*locked).clone());
            }
        }
    }
    None
}

// when performing modifying operations on the pool, this is what is used as the return type
pub type FrameResult = Result<usize, String>;

pub fn add_frame(block_no: usize, block: Block) -> FrameResult {
    // acquire write lock on global buffer pool
    if let Ok(mut pages) = GLOBAL_BUFFER_POOL.write() {
        let mut new_frame = BufferPoolFrame::new(block_no, block);
        new_frame.pins = 1;
        let size = pages.len();
        pages.insert(size, Arc::new(RwLock::new(new_frame)));
        return Ok(block_no);
    }
    Err("could not add frame to buffer pool".to_string())
}

pub fn drop_frame(block_no: usize) -> FrameResult {
    if let Ok(mut pages) = GLOBAL_BUFFER_POOL.write() {
        pages.remove(block_no);
        return Ok(block_no);
    }
    Err("could not remove frame from buffer pool".to_string())
}



