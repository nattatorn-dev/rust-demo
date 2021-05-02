// into_iter - This consumes the collection so that on each iteration the exact data is provided. Once the collection has been consumed it is no longer available for reuse as it has been 'moved' within the loop.
fn push(vector1: Vec<i32>, vector2: Vec<i32>) -> Vec<i32> {
    let mut temp: Vec<i32> = Vec::new();

    for num in vector1.into_iter() {
        temp.push(num);
    }

    for num in vector2.into_iter() {
        temp.push(num);
    }

    return temp;
}

#[test]
fn test_push() {
    assert_eq!(merge(vec![1], vec![2, 3]), vec![1, 2, 3]);
}

fn merge(vector1: Vec<i32>, vector2: Vec<i32>) -> Vec<i32> {
    let mut temp: Vec<i32> = vector1.clone();

    for num in vector2.into_iter() {
        temp.push(num);
    }

    return temp;
}

#[test]
fn test_merge() {
    assert_eq!(merge(vec![1], vec![2, 3]), vec![1, 2, 3]);
}

fn main() {
    let number1 = vec![1, 2, 3, 4, 5];
    let number2 = vec![5, 6, 7, 8, 9];
    let result1 = push(number1, number2);
    println!("{:?}", result1);

    let number3 = vec![1, 2, 3, 4, 5];
    let number4 = vec![5, 6, 7, 8, 9];
    let result2 = merge(number3, number4);
    println!("{:?}", result2);

}
