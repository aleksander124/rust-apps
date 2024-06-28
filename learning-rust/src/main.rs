fn main() {
    let numbers = vec![5, 4, 1, 7, 10];

    let max_element = numbers.iter().max();

    match max_element{
        Some(&max) => println!("Max element is {}", max),
        None => println!("The vector is empty"),
    }
}