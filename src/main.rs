use std::io;

fn main() {
   /* let x = 5;
    let x = x + 1;
    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {}", x);
    }
    println!("The value of x in the outer scope is: {}", x);
    let spaces = "    ";
    let spaces = spaces.len();
    print!("The value of spaces is: {}", spaces);

    let a = [1, 2, 3, 4, 5];
    print!("\nThe value of a is: {:?}", a);
    let months = ["January", "February", "March", "April", "May", "June", "July", "August", "September", "October", "November", "December"];
    print!("\nThe value of months is: {:?}", months[10]);


    let z: [i32; 5] = [1, 2, 3, 4, 5];
    print!("\nThe value of z is: {:?}", z);


    let x = [3; 5];
    print!("\nThe value of z is: {:?}", x);
 */
    let a = [1, 2, 3, 4, 5];

    println!("\nPlease enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");
}
