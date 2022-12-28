//! Memoization or lazy evaluation:
//! Act of saving the closure in a struct and cacheing its
//!      resulting value to the struct as well, so the rest of our
//!      code doesnt have to be responsible for saving and reusing the result.

//! To define structs,enums, or functions that hold a closures
//!      we need to use generics and trait bounds, see chapter 10 and saved cargo crate.'
//! Traits provided by std library are: Fn FnMut FnOnce...

// Comments to use to describe on the cover page of the generated documentation that is used to
// describe the crate as a whole instead of individual functions with examples which uses '///' and
// are ran through the test suite.

//! # Art
//! 
//! A library for modeling artistic concepts.
//re-exporting with pub self:: to help shape the public facing API, and help
//it make more sense for the consumers of this code.
pub use self::kinds::PrimaryColor;
pub use self::kinds::SecondaryColor;
pub use self::utils::mix;

pub mod kinds {
    /// The primary colors according to the RYB color model.
    pub enum PrimaryColor {
        Red,
        Yellow,
        Blue,
    }

    /// The secondary colors according to the RYB color model.
    pub enum SecondaryColor {
        Orange,
        Green,
        Purple,
    }
}

pub mod utils {
    use crate::kinds::*;
    /// Combines two primary colors in equal amounts to create
    /// a secondary color.
    pub fn mix(c1: PrimaryColor, c2: PrimaryColor ) -> SecondaryColor {
       return SecondaryColor::Green
    }
}

/// Examples of iterators in use
pub fn working_with_iterators() {
    println!("working with iterators!!");

    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();

    for val in v1_iter {
        println!("Got: {}", val);
    }
    iterator_demo();
}

fn iterator_demo() {
    let v1 = vec![1, 2, 3];
    //calling next on iterator changes internal state and
    //therfore need mut added to it so it can be changed.
    let mut v1_iter = v1.iter();
    println!("{:?}", v1_iter.next());
    println!("{:?}", v1_iter.next());
    println!("{:?}", v1_iter.next());
    println!("{:?}", v1_iter.next());
    // assert_eq!(v1_iter.next(), Some(&2));
    // assert_eq!(v1_iter.next(), Some(&3));
    // assert_eq!(v1_iter.next(), None);
}

#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

fn shoes_in_my_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    //call into_iter() to create an iterator that takes ownership of the vector,
    //and can then filter it and collect it into a new iterator that contains only the
    //shoe instances that return true based on the equality expression found in the closure.
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}

struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

//Implementing Iterator trait for counter which allows us to use
//associated iterator functions.
impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count < 5 {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}

/// Adds on to the number given.
///
/// # Examples
///
/// ```
/// let arg = 5;
/// let answer = functional_language_features::add_one(arg);
///
/// assert_eq!(6, answer);
/// ```
pub fn add_one(num: u32) -> u32 {
    num + 1
}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;

    #[test]
    fn using_other_iterator_trait_methods() {
        let sum: u32 = Counter::new()
            .zip(Counter::new().skip(1))
            .map(|(a, b)| a * b)
            .filter(|x| x % 3 == 0)
            .sum();
        assert_eq!(18, sum);
    }

    #[test]
    fn calling_next_directly() {
        let mut counter = Counter::new();
        assert_eq!(counter.next(), Some(1));
        assert_eq!(counter.next(), Some(2));
        assert_eq!(counter.next(), Some(3));
        assert_eq!(counter.next(), Some(4));
        assert_eq!(counter.next(), Some(5));
        assert_eq!(counter.next(), None);
    }

    #[test]
    fn filter_by_size() {
        let shoes = vec![
            Shoe {
                size: 10,
                style: String::from("sneaker"),
            },
            Shoe {
                size: 13,
                style: String::from("sandal"),
            },
            Shoe {
                size: 10,
                style: String::from("boot"),
            },
        ];

        let in_my_size = shoes_in_my_size(shoes, 10);
        assert_eq!(
            in_my_size,
            vec![
                Shoe {
                    size: 10,
                    style: String::from("sneaker")
                },
                Shoe {
                    size: 10,
                    style: String::from("boot")
                },
            ]
        )
    }
}
