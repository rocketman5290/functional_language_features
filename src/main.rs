use std::thread;
use std::time::Duration;

//replaced by closure aka anonymous function.
fn simulated_expensive_calculation(intensity: u32) -> u32 {
    println!("Calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    intensity
}

fn generate_workout(intensity: u32, random_number: u32) {
    let expensive_closure = |num| {
        println!("Calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    };

    // replace with closure as the value of expensive_closure variable 
    //    which stores the anonymous function until needed and then is called,
    //    returning the side effects defined and the return value num at the end.
    
    // let expensive_result = simulated_expensive_calculation(intensity);
    
    if intensity < 25 {
        println!(
            "Today, do {} pushups!",
            // expensive_result replaced by below line
            expensive_closure(intensity)
        );
        println!(
            "Today, do {} situps!",
            // expensive_result replaced by below line
            expensive_closure(intensity)
        );
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!")
        } else {
            println!(
                "Today, run for {} minutes",
                // expensive_result replaced by below line
                expensive_closure(intensity)
            )
        }
    }
}
fn main() {
    println!("examples of all the same work in function and then closure form");
    fn add_one_v1(x:u32) -> u32 {x + 1}
    let add_one_v2 = |x: u32| -> u32 {x + 1};
    let add_one_v3 = |x| { x + 1 };
    let add_one_v4 = |x| x + 1;
    //print above versions of same logic.
    println!("{}", add_one_v1(1));
    println!("{}", add_one_v2(1));
    println!("{}", add_one_v3(1));
    println!("{}", add_one_v4(1));


    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout(
        simulated_user_specified_value,
        simulated_random_number
    );
}
