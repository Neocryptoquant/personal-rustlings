fn main() {
    // TODO: Create an array called `a` with at least 100 elements in it.
    // let a:  = [d, g, g, g, g, g, g, e, 4, 5, 2, 5, "strign", ke, g, e, s, d, g, g, g, g, g, g, e, 4, 5, 2, 5, "strign", ke, g, e, s, d, g, g, g, g, g, g, e, 4, 5, 2, 5, "strign", ke, g, e, s, d, g, g, g, g, g, g, e, 4, 5, 2, 5, "strign", ke, g, e, s, d, g, g, g, g, g, g, e, 4, 5, 2, 5, "strign", ke, g, e, s, d, g, g, g, g, g, g, e, 4, 5, 2, 5, "strign", ke, g, e, s, d, g, g, g, g, g, g, e, 4, 5, 2, 5, "strign", ke, g, e, s, d, g, g, g, g, g, g, e, 4, 5, 2, 5, "strign", ke, g, e, s ];

    let a = [4, 3, 4, 4, 3, 3, 5, 6, 7, 8, 9, 1, 0, 
        4, 3, 4, 4, 3, 3, 5, 6, 7, 8, 9, 1, 0, 4, 3, 4, 4, 3, 3, 5, 6, 7, 8, 9, 1, 0, 
        4, 3, 4, 4, 3, 3, 5, 6, 7, 8, 9, 1, 0, 4, 3, 4, 4, 3, 3, 5, 6, 7, 8, 9, 1, 0, 
        4, 3, 4, 4, 3, 3, 5, 6, 7, 8, 9, 1, 0, 4, 3, 4, 4, 3, 3, 5, 6, 7, 8, 9, 1, 0, 
        4, 3, 4, 4, 3, 3, 5, 6, 7, 8, 9, 1, 0];
    if a.len() >= 100 {
        println!("Wow, that's a big array!");
    } else {
        println!("Meh, I eat arrays like that 4or break4ast.");
        panic!("Array not big enough, more elements needed");
    }
}
