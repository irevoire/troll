use rand::Rng;
use std::env;

fn main() {
    let freq: usize = env::args()
        .nth(1)
        .unwrap_or("100000000".to_string())
        .parse()
        .unwrap();

    let mut current_number = 1;
    let mut rng = rand::rng();

    let fizz_range =  0.0..33.0;
    let buzz_range = 33.0..=(33.0+20.0);


    // to get up to speed we're going to randomely set each counter
    let mut fizz_count = ((33.0/100.0) * freq as f64) as usize;
    let mut buzz_count = ((20.0/100.0) * freq as f64) as usize;
    //let mut number_count = ((53.0/100.0) * freq as f64) as usize;
    let mut number_count = freq - 1;


    for i in 0..(100 * freq * 3) {
        let angle = rng.random_range(0.0..100.0);

        if fizz_range.contains(&angle) {
            fizz_count += 1;
            if fizz_count == freq {
                print!("fizz");
                fizz_count = 0;
            }
        } else if buzz_range.contains(&angle) {
            buzz_count += 1;
            if buzz_count == freq {
                print!("buzz");
                buzz_count = 0;
            }
        } else {
            number_count += 1;
            if number_count == freq {
                print!("{}", current_number);
                number_count = 0;
            }
        }

        if i % freq == 0 {
            current_number += 1;
            println!();
        }
    }
}
