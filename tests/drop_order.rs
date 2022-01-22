pub mod utils;
pub use utils::*;

#[test]
fn pipe_ref_drop() {
    let expected_record = vec![
        RecordItem::Create(0),
        RecordItem::Create(1),
        RecordItem::Drop(0),
        RecordItem::Create(2),
        RecordItem::Drop(1),
        RecordItem::Create(3),
        RecordItem::Drop(2),
    ];

    let record = Default::default();

    let _persistent = FamilyMember::new(&record) // a = new
        .pipe_ref_drop(FamilyMember::clone) // b = a.clone()
        .pipe_ref_drop(FamilyMember::clone) // c = b.clone()
        .pipe_ref_drop(FamilyMember::clone);
    let actual_record = record.lock().unwrap().clone();

    eprintln!("Expected: {:?}", &expected_record);
    eprintln!("Actual: {:?}", &actual_record);

    assert!(actual_record.contains(&RecordItem::Drop(0)));
    assert!(actual_record.contains(&RecordItem::Drop(1)));
    assert!(actual_record.contains(&RecordItem::Drop(2)));
    assert!(!actual_record.contains(&RecordItem::Drop(3)));

    assert_eq!(&actual_record, &expected_record);
}
