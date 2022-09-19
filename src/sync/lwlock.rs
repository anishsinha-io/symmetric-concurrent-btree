#![allow(dead_code)]

// LwLocks have two modes: shared and exclusive
#[derive(Eq, PartialEq)]
pub enum LwLockType {
    Shared,
    Exclusive,
}

// A task can either be an insert operation or a delete operation
#[derive(Eq, PartialEq)]
pub enum Task {
    Ins,
    Del,
}
