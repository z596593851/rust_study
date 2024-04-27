use std::slice;

#[test]
fn test1() {
    let mut num = 5;

    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;
    //let r3 = num.as_mut_ptr();

    let mut k = &mut num;
    *k += 1;

    unsafe {
        // *r1 += 1; --Cannot assign a new value to `immutable dereference of raw pointer`
        *r2 += 1;
    }

    println!("num is {num}")
}

#[test]
fn test2() {
    let mut v = vec![1,2,3,4];
    let (v2, v3) = split_at_mut(&mut v, 2);
    assert_eq!(v2, &mut [1,2]);
    assert_eq!(v3, &mut [3,4]);
}

// fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
//     let len = values.len();
//     assert!(mid <= len);
//     (&mut values[..mid], &mut values[mid..]) --cannot borrow `*values` as mutable more than once at a time
// }

fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = values.len();
    let ptr = values.as_mut_ptr();
    assert!(mid <= len);
    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}