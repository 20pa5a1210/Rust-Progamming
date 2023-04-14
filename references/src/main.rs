fn main() {
    println!("Hello, world!");
    let sum = array_sum(&[1, 2, 3, 4, 5]);
    println!("sum = {}", sum);
}
fn array_sum(array: &[i32]) -> i32 {
    let mut sum = 0;
    for i in array {
        sum += i;
    }
    sum
}
