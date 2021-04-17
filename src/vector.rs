// into_iter - This consumes the collection so that on each iteration the exact data is provided. Once the collection has been consumed it is no longer available for reuse as it has been 'moved' within the loop.
pub fn push(vector1: Vec<i32>, vector2: Vec<i32>) -> Vec<i32> {
  let mut temp: Vec<i32> = Vec::new();

  for num in vector1.into_iter() {
    temp.push(num);
  }

  for num in vector2.into_iter() {
    temp.push(num);
  }

  return temp;
}

pub fn merge(vector1: Vec<i32>, vector2: Vec<i32>) -> Vec<i32> {
  let mut temp: Vec<i32> = vector1.clone();

  for num in vector2.into_iter() {
    temp.push(num);
  }

  return temp;
}
