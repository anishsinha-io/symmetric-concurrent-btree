#![allow(dead_code)]

// LwLocks have two modes: shared and exclusive
pub enum LwLockType {
    Shared,
    Exclusive,
}

// A task can either be an insert operation or a delete operation
pub enum Task {
    Ins,
    Del,
}
