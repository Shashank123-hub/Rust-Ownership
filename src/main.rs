
fn owner_var_example() {
    let x: i8 = 10;   // a is the owner of the value 10
    let y: i8 = x;    // b is the owner of the value 10
    // x is still valid here, as it is not moved
    // y is a copy of x, so it is also valid here
    println!("Variable x is: {}", x);
    println!("Variable y is: {}", y);
}

const VAR_C: i8 = 50; //A global variable - this is available throughout the program

fn scope_of_var(a: i8) {
    {
        let var_b:i8 = 5;  // scope of this variable is available within the braces
        println!("Variable b is: {}", var_b);  //This will works as var_b is in scope
        println!("Variable c is: {}", VAR_C);  //Variable c is a global variable and is available here
    }
    println!("Variable a is: {}", a);  //This will work
    //println!("Variable b is: {}", var_b);  //This will give an error as scope of var_b is not available here
}

fn main() {
    scope_of_var(5); //This will work as var_a is in scope
    owner_var_example();    
}


