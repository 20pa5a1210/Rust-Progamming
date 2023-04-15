fn main() {
    println!("Hello, world!");
    let sum = array_sum(&[1, 2, 3, 4, 5]);
    println!("sum = {}", sum);
    let sum1 = array_sum(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    println!("sum1 = {}", sum1);
    let sum2 = add_sum(1, 2);
    println!("sum2 = {}", sum2);
}
fn array_sum(array: &[i32]) -> i32 {
    let mut sum = 0;
    for i in array {
        sum += i;
    }
    sum
}

fn add_sum(num1: i32, num2: i32) -> i32 {
    num1 + num2
}

// Path: references/src/lib.rs

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_array_sum() {
        let sum = array_sum(&[1, 2, 3, 4, 5]);
        println!("sum1 = {}", sum);
        assert_eq!(sum, 15);
        let sum1 = array_sum(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
        println!("sum1 = {}", sum1);
        assert_eq!(sum1, 55);
        assert_eq!(add_sum(1, 2), 3);
    }
}
#[cfg(test1)]
mod tests{
    use super::*;
    #[test1]
    fn test_array_sum(){
        let sum = array_sum(&[1,2,3,4,5]);
        println!("sum1 = {}",sum);
        assert_eq!(sum,15);
        let sum1 = array_sum(&[1,2,3,4,5,6,7,8,9,10]);
        println!("sum1 = {}",sum1);
        assert_eq!(sum1,55);
        assert_eq!(add_sum(1,2),3);
    }
}