fn main() {
    let characters = vec!['a', 'b', 'c', 'd'];

    for (index, character) in characters.iter().enumerate() {
        println!("{}: {}", index, character);
    }
}
