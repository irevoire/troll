use std::{sync::mpsc::sync_channel, time::Duration};

enum Print {
    Fizz,
    Buzz,
    N(u32),
}

fn main() {
    let (snd, rcv) = sync_channel(1);

    let freq = Duration::from_secs_f32(std::env::args().nth(1).unwrap_or("1.0".to_string()).parse().unwrap());
    let start = std::time::Instant::now();

    let num = snd.clone();
    std::thread::spawn(move || {
        let offset = freq / 2;
        std::thread::sleep(start + freq + offset - std::time::Instant::now());

        for i in 2.. {
            let _ = num.try_send(Print::N(i - 1));
            std::thread::sleep(start + i * freq + offset - std::time::Instant::now());
        }
    });
    let fizz = snd.clone();
    std::thread::spawn(move || {
        let offset = freq / 10;
        for i in 1.. {
            std::thread::sleep(start + i * freq * 3 + offset - std::time::Instant::now());
            fizz.send(Print::Fizz).unwrap();
        }
    });
    let buzz = snd.clone();
    std::thread::spawn(move || {
        let offset = freq / 5;
        for i in 1.. {
            std::thread::sleep(start + i * freq * 5 + offset - std::time::Instant::now());
            buzz.send(Print::Buzz).unwrap();
        }
    });
    std::thread::spawn(move || {
        let offset = freq / 2 + freq / 10;
        std::thread::sleep(offset);
        for i in 0.. {
            while let Ok(v) = rcv.try_recv() {
                match v {
                    Print::Fizz => print!("fizz"),
                    Print::Buzz => print!("buzz"),
                    Print::N(n) => print!("{n}"),
                }
                std::thread::sleep(freq / 10);
            }
            std::thread::sleep(start + i * freq + offset - std::time::Instant::now());
        }
    });

    for i in 1..=100 {
        std::thread::sleep(start + i * freq - std::time::Instant::now());
        println!();
    }
}
