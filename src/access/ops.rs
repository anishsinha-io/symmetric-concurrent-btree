use std::sync::{Arc, RwLock};

use crate::access::anchor::Anchor;
use crate::access::objptr::ObjectPtr;
use crate::access::node::Node;
use crate::common::{INDEX_PATH, Block};
use crate::storage::bufmgr;
use crate::storage::bufmgr::{GLOBAL_BUFFER_POOL, BufferPoolFrame};
use crate::storage::page_table::GLOBAL_PAGE_TABLE;
use crate::sync::lwlock::LwLockType;

pub fn get_anchor() -> Option<BufferPoolFrame> {
    if let Some(anchor) = bufmgr::fetch_frame(0) {
        return Some(anchor);
    } else {
        if let Ok(mut pages) = GLOBAL_BUFFER_POOL.write() {
            if let Ok(mut table) = GLOBAL_PAGE_TABLE.write() {
                if let Some(anchor) = pages.get(0) {
                    if let Ok(locked) = anchor.read() {
                        return Some(locked.clone());
                    }
                } else {
                    let from_disk = Anchor::read();
                    let index = pages.len();
                    pages.insert(index, Arc::new(RwLock::new(BufferPoolFrame::new(0, from_disk))));
                    table.insert(0, index);
                    // assert because everything below should be guaranteed safe and unwrappable
                    return Some(pages.get(0).unwrap().read().unwrap().clone());
                }
            }
        }
    }
    None
}

pub fn get_node(ptr: ObjectPtr) -> Option<BufferPoolFrame> {
    if let Ok(table) = GLOBAL_PAGE_TABLE.read() {
        // if there exists a frame in the buffer pool corresponding to the above object pointer then we're good and
        // the node is already in the buffer pool
        if let Some(node) = bufmgr::fetch_frame(ptr.data.page_no) { return Some(node.clone()); };
    } else {
        if let Ok(mut blocks) = GLOBAL_BUFFER_POOL.write() {
            if let Ok(mut table) = GLOBAL_PAGE_TABLE.write() {
                if let Some(node) = blocks.get(ptr.data.page_no) {
                    if let Ok(locked) = node.read() {
                        return Some(locked.clone());
                    }
                } else {
                    let from_disk = Node::read(ptr.data.page_no);
                    let node = Node::from_block(from_disk);
                    let index = blocks.len();
                    blocks.insert(index, Arc::new(RwLock::new(BufferPoolFrame::new(node.loc.data.page_no, from_disk))));
                    table.insert(0, index);
                }
            }
        }
    }
    None
}

fn move_right(val: usize, ptr: ObjectPtr, ubleftsep: f64, lock_type: LwLockType) {
    if lock_type == LwLockType::Shared {} else {}
}

fn locate_leaf(val: usize, last_lock: LwLockType) {
    // lock page table
    let anchor = Anchor::from_block(get_anchor().unwrap().block_held);
    // root (ObjectPtr)
    let fast_ptr = anchor.fast;
    // level to enter at
    let enter_height = anchor.fast_height;
    let ubleftsep = f64::NEG_INFINITY;
    let descent: Vec<usize> = vec![];
    for i in (1..enter_height).rev() {
        // move_right();
    }
}


fn normalize() {}

pub fn insert() {}

pub fn delete() {}

pub fn search() {}