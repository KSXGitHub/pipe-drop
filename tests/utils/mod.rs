use std::sync::Mutex;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum RecordItem {
    Create(usize),
    Drop(usize),
}

lazy_static::lazy_static! {
    pub static ref RECORD: Mutex<Vec<RecordItem>> = Default::default();
}

impl RecordItem {
    fn record(self) {
        RECORD.lock().unwrap().push(self);
        dbg!(&*RECORD.lock().unwrap());
    }
}

#[derive(Debug)]
pub struct Object {
    count: usize,
}

impl Default for Object {
    fn default() -> Self {
        RecordItem::Create(0).record();
        Object { count: 0 }
    }
}

impl Clone for Object {
    fn clone(&self) -> Self {
        let count = self.count + 1;
        RecordItem::Create(count).record();
        Object { count }
    }
}

impl Drop for Object {
    fn drop(&mut self) {
        RecordItem::Drop(self.count).record();
    }
}
