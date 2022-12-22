use std::collections::HashMap;
use std::thread;
use std::time::Duration;

use functional_language_features;

//replaced by closure aka anonymous function.
// fn simulated_expensive_calculation(intensity: u32) -> u32 {
//     println!("Calculating slowly...");
//     thread::sleep(Duration::from_secs(2));
//     intensity
// }

//Memoization or lazy evaluation:
// Act of saving the closure in a struct and cacheing its
//      resulting value to the struct as well, so the rest of our
//      code doesnt have to be responsible for saving and reusing the result.

// To define structs,enums, or functions that hold a closures
//      we need to use generics and trait bounds, see chapter 10 and saved cargo crate.'
// Traits provided by std library are: Fn FnMut FnOnce...
struct Cacher<T>
where
    T: Fn(u32) -> u32,
{
    calculation: T,
    value: Option<u32>,
}

impl<T> Cacher<T>
where
    T: Fn(u32) -> u32,
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None,
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        let mut hash_of_values = HashMap::new();
        hash_of_values.entry(arg).or_insert(arg);
        match self.value {
            Some(_v) => hash_of_values[&arg],
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}

fn generate_workout(intensity: u32, random_number: u32) {
    // let expensive_closure = |num| {
    //     println!("Calculating slowly...");
    //     thread::sleep(Duration::from_secs(2));
    //     num
    // };

    //replace above implementation with below and update uses below.
    let mut expensive_result = Cacher::new(|num| {
        println!("Calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    // replace with closure as the value of expensive_closure variable
    //    which stores the anonymous function until needed and then is called,
    //    returning the side effects defined and the return value num at the end.

    // let expensive_result = simulated_expensive_calculation(intensity);

    if intensity < 25 {
        println!(
            "Today, do {} pushups!",
            // expensive_result replaced by below line
            // expensive_closure(intensity)
            expensive_result.value(intensity)
        );
        println!(
            "Today, do {} situps!",
            // expensive_result replaced by below line
            // expensive_closure(intensity)
            expensive_result.value(intensity)
        );
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!")
        } else {
            println!(
                "Today, run for {} minutes",
                // expensive_result replaced by below line
                // expensive_closure(intensity)
                expensive_result.value(intensity)
            )
        }
    }
}

//Closures can also:
// - They can capture their envirnment and access variables
//   from the scop in which they are defined.
fn capturing_their_enviornment() -> bool {
    let x = 4;
    let equal_to_x = |z| z == x;
    let y = 4;
    equal_to_x(y)
}

fn main() {
    functional_language_features::working_with_iterators();
    println!("{:?}", capturing_their_enviornment());
    println!("examples of all the same work in function and then closure form");
    fn add_one_v1(x: u32) -> u32 {
        x + 1
    }
    let add_one_v2 = |x: u32| -> u32 { x + 1 };
    let add_one_v3 = |x| x + 1;
    let add_one_v4 = |x| x + 1;
    //print above versions of same logic.
    println!("{}", add_one_v1(1));
    println!("{}", add_one_v2(1));
    println!("{}", add_one_v3(1));
    println!("{}", add_one_v4(1));

    let simulated_user_specified_value = 26;
    let simulated_random_number = 4;

    generate_workout(simulated_user_specified_value, simulated_random_number);
}

#[test]
fn call_with_different_values() {
    let mut c = Cacher::new(|a| a);
    let v1 = c.value(1);
    let v2 = c.value(2);

    assert_eq!(v2, 2);
}

#[test]
fn iterator_sum() {
    let v1 = vec![1,2,3];
    //iterators are lazy and must be consumed hence the v1_iter variable 
    //prior to calling sum and totaling the vector.
    //methods that call next are refered to as CONSUMING ADAPTORS
    //  an example of one of these is sum()
    //methods that change iterator into other iterators are refered to as ITERATOR ADAPTORS
    //  example of these are ..iter().map(|x| x + 1), and that fact that they are lazy is 
    //  obvious here becasue in order for this closure to be called the variable needs to be consumed.
    // collect() will capture an iterator back into a collection data structure like a vector.
    let v1_iter = v1.iter();
    let total: i32 = v1_iter.sum();
    assert_eq!(total, 6);
}

#[test]
fn iterator_adapter_filter() {
    
}

#[test]
fn iterator_collect() {
    let v1: Vec<u32> = vec![1, 2, 3];
    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();
    assert_eq!(v2, vec![2, 3, 4]);
}
