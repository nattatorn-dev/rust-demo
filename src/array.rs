use std::mem;

fn get_first(slice: &[i32]) -> i32 {
  return slice[0];
}

#[test]
fn test_get_first() {
  assert_eq!(get_first([1,100,99]), 1);
}

fn main() {
    // fix size
    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    let arr5000: [i128; 5000] = [99; 5000];

    // Arrays are stack allocated
    println!("array occupies {} bytes", mem::size_of_val(&arr));
    println!("array occupies {} bytes", mem::size_of_val(&arr5000));

    let first = get_first(&arr);
    println!("first {}", first);

    println!("borrow a section of the array as a slice");
    let borrowed = &arr5000[1 .. 4];
    println!("array occupies {} bytes", mem::size_of_val(borrowed));
    println!("{:?}", borrowed);
}