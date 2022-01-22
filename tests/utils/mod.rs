use std::{
    collections::HashMap,
    sync::{
        atomic::{AtomicUsize, Ordering::Relaxed},
        Mutex,
    },
};

/// Creation or destruction of a generation within a family line.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum RecordItem {
    Create(usize),
    Drop(usize),
}

/// ID to assign to new family lines.
static NEXT_FAMILY: AtomicUsize = AtomicUsize::new(0);
lazy_static::lazy_static! {
    /// All records of every family lines.
    static ref FAMILY_RECORDS: Mutex<HashMap<usize, Vec<RecordItem>>> = Default::default();
}

impl RecordItem {
    /// Add the item to a family record.
    fn record(self, family: usize) {
        FAMILY_RECORDS
            .lock()
            .unwrap()
            .entry(family)
            .or_default()
            .push(self);
    }
}

/// A member of a family.
#[derive(Debug)]
pub struct FamilyMember {
    family: usize,
    generation: usize,
}

impl Default for FamilyMember {
    fn default() -> Self {
        let family = NEXT_FAMILY.fetch_add(1, Relaxed);
        RecordItem::Create(0).record(0);
        FamilyMember {
            family,
            generation: 0,
        }
    }
}

impl Clone for FamilyMember {
    fn clone(&self) -> Self {
        let family = self.family;
        let generation = self.generation + 1;
        RecordItem::Create(generation).record(family);
        FamilyMember { family, generation }
    }
}

impl Drop for FamilyMember {
    fn drop(&mut self) {
        RecordItem::Drop(self.generation).record(self.family);
    }
}
