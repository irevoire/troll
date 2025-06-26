use std::{sync::mpsc::sync_channel, time::Duration};

enum Print {
    Fizz,
    Buzz,
    N(u32),
}

fn main() {
    let (snd, rcv) = sync_channel(1);

    let num = snd.clone();
    std::thread::spawn(move || {
        std::thread::sleep(Duration::from_secs_f32(0.5));
        for i in 0.. {
            let _ = num.try_send(Print::N(i));
            std::thread::sleep(Duration::from_secs(1));
        }
    });
    let fizz = snd.clone();
    std::thread::spawn(move || {
        std::thread::sleep(Duration::from_secs_f32(0.1));
        let mut now = std::time::Instant::now();
        loop {
            std::thread::sleep(Duration::from_secs(3) - now.elapsed());
            fizz.send(Print::Fizz).unwrap();
            now = std::time::Instant::now();
        }
    });
    let buzz = snd.clone();
    std::thread::spawn(move || {
        std::thread::sleep(Duration::from_secs_f32(0.2));
        let mut now = std::time::Instant::now();
        loop {
            std::thread::sleep(Duration::from_secs(5) - now.elapsed());
            buzz.send(Print::Buzz).unwrap();
            now = std::time::Instant::now();
        }
    });
    std::thread::spawn(move || {
        std::thread::sleep(Duration::from_secs_f32(0.6));
        let mut now = std::time::Instant::now();
        loop {
            while let Ok(v) = rcv.try_recv() {
                match v {
                    Print::Fizz => print!("fizz"),
                    Print::Buzz => print!("buzz"),
                    Print::N(n) => print!("{n}"),
                }
                std::thread::sleep(Duration::from_secs_f32(0.1));
            }
            std::thread::sleep(Duration::from_secs(1) - now.elapsed());
            now = std::time::Instant::now();
        }
    });

    loop {
        std::thread::sleep(Duration::from_secs(1));
        println!();
    }
}
