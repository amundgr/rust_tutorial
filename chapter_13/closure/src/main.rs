use std::thread;
use std::time::Duration;

fn _simulated_expencive_calculation(intensity: u32) -> u32 {
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    intensity
}

struct Cacher<T>
where
    T: Fn(u32) -> u32,
{
    calculation: T,
    value: Option<u32>,
}

fn generate_workout(intensity: u32, random_number: u32) {
    let expensive_result = |num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    };
    if intensity < 25 {
        println!(
            "Today, to {} pushups!",
            expensive_result(intensity)
        );
        println!(
            "Next, do {} situps!",
            expensive_result(intensity)
        );
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated");
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_result(intensity)
            );
        }
    }
}

fn main() {
    let simulated_user_spesified_value = 10;
    let simulated_random_number = 7;

    generate_workout(simulated_user_spesified_value, simulated_random_number);
}
