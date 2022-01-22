pub use pipe_drop::PipeDrop;
pub use std::sync::Mutex;

/// Creation or destruction of a generation within a family line.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum RecordItem {
    Create(usize),
    Drop(usize),
}

impl RecordItem {
    /// Add the item to a family record.
    fn record(self, record: &Mutex<Vec<RecordItem>>) {
        record.lock().expect("get record to write to").push(self);
    }
}

/// A member of a family.
#[derive(Debug)]
pub struct FamilyMember<'a> {
    record: &'a Mutex<Vec<RecordItem>>,
    generation: usize,
}

impl<'a> FamilyMember<'a> {
    /// Spawn a new family line.
    pub fn new(record: &'a Mutex<Vec<RecordItem>>) -> Self {
        RecordItem::Create(0).record(record);
        FamilyMember {
            record,
            generation: 0,
        }
    }
}

impl<'a> Clone for FamilyMember<'a> {
    fn clone(&self) -> Self {
        let record = self.record;
        let generation = self.generation + 1;
        RecordItem::Create(generation).record(record);
        FamilyMember { record, generation }
    }
}

impl<'a> Drop for FamilyMember<'a> {
    fn drop(&mut self) {
        RecordItem::Drop(self.generation).record(self.record);
    }
}
