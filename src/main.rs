fn variables()
{
    // Declaring a variable without the `mut` keyword makes it immutable.
    let x = 5; // Immutable variable
    println!("x = {}", x);
    // The line below would produce a compile-time error because you cannot assign to an immutable variable:
    //x = 10; 

    // Declaring a variable with the `mut` keyword makes it mutable.
    let mut b = 5;
    println!("b = {}", b);
    
    // These are valid because `b` is mutable:
    // Mutable variables can be changed as many times as needed:
    b = 20;
    println!("The new value of b = {}", b);
    b = 30;
    println!("The new value of b = {}", b);

    //You can also have mutable and none mutible strings as well.
    let mut my_string = String::from("Hello");
    println!("The value of my_string is: {}", my_string);
    my_string.push_str(", world!");
    println!("The new value of my_string is: {}", my_string);

    // Rust allows you to "shadow" a variable by declaring a new variable with the same name
    // as an existing variable. This effectively creates a new variable that "hides" the old one.
    let z = 5;
    let z = z + 1; // Shadowing the old `z` with a new `z`

    // This will print "6":
    println!("z = {}", z);

    

}

fn expressions()
{
    // Simple expressions

    let a = 2 + 3;                   // Addition
    println!("a = {}", a);
    
    let b = 5 * 6;                   //Multiplication
    println!("b = {}", b);
    
    let c = (10 - 2) / 3;            //Division and Subtraction
    println!("c = {}", c);
    
    let d = 7 % 3;                   //Modulus
    println!("d = {}", d);
    
    let e = true && false;           //Logical AND
    println!("e = {}", e);
    
    let f = true || false;           //Logical OR
    println!("f = {}", f);
    
    let g = !false;                  //Logical NOT
    println!("g = {}", g);
    
    // Function call expressions
    
    let i= add(2, 3);               //Calling a function
    println!("i = {}", i);
    
    let j = multiply(a, b);          //Passing expressions as arguments to a function
    println!("j = {}", j);    
}

fn add(x: i32, y: i32) -> i32 
{
    x + y
}

fn multiply(x: i32, y: i32) -> i32 
{
    x * y
}

fn conditionals()
{
    let x = 10;

    //If-else-if statement
    if x > 10 
    {
        println!("x is greater than 10");
    } 
    else if x == 10 
    {
        println!("x is equal to 10");
    } 
    else 
    {
        println!("x is less than 10");
    }
    
    //Match statement
    match x 
    {
        1 => println!("x is 1"),
        2 => println!("x is 2"),
        3..=9 => println!("x is between 3 and 9"),
        10 => println!("x is 10"),
        _ => println!("x is something else"),
    }
}

fn loops()
{
    let mut i = 0;
    
    //While loop
    while i < 5 {
        println!("i is {}", i);
        i += 1;
    }
    
    //For loop using range
    //You can create a range using the '..' operator
    for j in 0..5 {
        println!("j is {}", j);
    }
    
    //For loop using iterator
    //This is how you create a vector
    let nums = vec![1, 2, 3, 4, 5];
    for num in nums.iter() {
        println!("num is {}", num);
    }
    
    //Loop will repeatedly execute a block of code until a 'break' point is hit
    let mut k = 0;
    loop {
        println!("k is {}", k);
        k += 1;
        if k == 5 {
            break;
        }
    }
}

fn slicing()
{
    //Create an initial vector
    let numbers = vec![1, 2, 3, 4, 5];
    
    //Slice the vector to get the first three elements
    let slice = &numbers[0..3];
    
    //Display the original vector
    println!("numbers: {:?}", numbers);
    //Display the indexes that were sliced out
    println!("slice: {:?}", slice);    
}
fn main() 
{
    println!("\nVariables");
    variables();
    println!("\nExpressions");
    expressions();
    println!("\nConditionals");
    conditionals();
    println!("\nLoops");
    loops();
    println!("\nSlicing");
    slicing();
}
