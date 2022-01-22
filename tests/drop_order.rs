pub mod utils;
pub use utils::*;

use std::mem::forget;

#[test]
fn pipe_ref_drop() {
    let record = Default::default();

    let a = FamilyMember::new(&record);
    let b = a.clone();
    drop(a);
    let c = b.clone();
    drop(b);
    let d = c.clone();
    drop(c);
    let expected_record = record.lock().unwrap().clone();
    forget(d);

    let record = Default::default();

    let d = FamilyMember::new(&record) // a = new
        .pipe_ref_drop(FamilyMember::clone) // b = a.clone()
        .pipe_ref_drop(FamilyMember::clone) // c = b.clone()
        .pipe_ref_drop(FamilyMember::clone);
    let actual_record = record.lock().unwrap().clone();
    forget(d);

    eprintln!("Expected: {:?}", &expected_record);
    eprintln!("Actual: {:?}", &actual_record);

    assert!(actual_record.contains(&RecordItem::Drop(0)));
    assert!(actual_record.contains(&RecordItem::Drop(1)));
    assert!(actual_record.contains(&RecordItem::Drop(2)));
    assert!(!actual_record.contains(&RecordItem::Drop(3)));

    assert_eq!(&actual_record, &expected_record);
}
