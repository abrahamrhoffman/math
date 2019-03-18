fn main() {
    // A vector to contain all real numbers up to 49
    let mut reals = Vec::new();
    // Populate the vector
    let mut number: i32 = 0;
    while number != 50 {
        reals.push(number);
        number = number + 1;
    }
    // Display the results
    println!("{:?}", &reals[0..10]);
    println!("{:?}", &reals[10..20]);
    println!("{:?}", &reals[20..30]);
    println!("{:?}", &reals[30..40]);
    println!("{:?}", &reals[40..50]);
}