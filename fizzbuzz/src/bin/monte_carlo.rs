use rand::Rng;
use std::env;

fn main() {
    let freq: usize = env::args()
        .nth(1)
        .unwrap_or("100000000".to_string())
        .parse()
        .unwrap();

    println!("1");
    let mut current_number = 2;
    let mut rng = rand::rng();

    let fizz_range =  0.0..=33.0;
    let buzz_range = 33.0..=(33.0+20.0);

    // to get up to speed we're going to randomely set each counter
    let mut fizz_count = ((33.0/100.0) * freq as f64) as usize;
    let mut buzz_count = ((20.0/100.0) * freq as f64) as usize;
    let mut number_count = ((53.0/100.0) * freq as f64) as usize;

    let mut line_count = 0;
    let mut iter = 0;
    let mut ret = String::new();

    loop {
        let angle = rng.random_range(0.0..100.0);

        if fizz_range.contains(&angle) {
            fizz_count += 1;
            if fizz_count == freq {
                ret.push_str("fizz");
                fizz_count = 0;
            }
        } else if buzz_range.contains(&angle) {
            buzz_count += 1;
            if buzz_count == freq {
                ret.push_str("buzz");
                buzz_count = 0;
            }
        } else {
            number_count += 1;
            if number_count == freq {
                ret.push_str(&current_number.to_string());
                number_count = 0;
            }
        }

        if iter >= freq && !ret.is_empty() {
            current_number += 1;
            line_count += 1;
            if line_count == 100 {
                break;
            }
            println!("{ret}");
            ret.clear();
            iter = 0;
        }
        iter += 1;
    }
}
