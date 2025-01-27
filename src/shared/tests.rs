#[test]
fn test_shared() {
    fn unpack(shared: Shared<i32>) -> i32 {
        let shared = shared.lock().unwrap();
        let unwraped_val = *shared;
        unwraped_val
    }

    use crate::shared::{IntoShared, Shared};
    let shared = 42.into_shared();
    let shared = shared.lock().unwrap();
    assert_eq!(*shared, 42);
    
    let packed_val = 420.into_shared();
    let unpackaged_val = unpack(packed_val);
    
    assert_eq!(unpackaged_val, 420);
}
