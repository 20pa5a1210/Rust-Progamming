fn main() {
    println!("Hello, world!");
    enum Hello {
        description,
        rel,
        is_ready,
    }
    reverse("Hello, world!");
    struct City {
        name: String,
        population: u32,
        is_capital: bool,
    }

    impl City {
        fn new(name: &str, population: u32, is_capital: bool) -> City {
            City {
                name: name.to_string(),
                population,
                is_capital,
            }
        }
    }

    let london = City::new("London", 8_615_000, true);
}

fn reverse(input: &str) -> String {
    let mut chars: Vec<char> = input.chars().collect();
    chars.reverse();
    chars.into_iter().collect()
}

// enum City {
//     London,
//     Paris,
//     Berlin,
//     NewYork,
// }
