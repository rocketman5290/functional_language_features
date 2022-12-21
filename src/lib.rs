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
    let v1 = vec![1,2,3];
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