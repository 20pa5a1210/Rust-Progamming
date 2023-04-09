

fn main() {
    let total_orders: i64 = get_final_orders();
    println!("Total orders: {}", total_orders);
}

fn get_final_orders() -> i64 {
    let orders: Vec<i64> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let mut total_orders: i64 = 0;
    for order in orders.iter() {
        total_orders += order;
    }
    return total_orders;
}
