fn scope_of_var(a: i8) {
    {
        let var_b:i8 = 5;  // scope of this variable is available within the braces
        println!("Variable b is: {}", var_b);  //This will works as var_b is in scope
    }
    println!("Variable a is: {}", a);  //This will work
    //println!("Variable b is: {}", var_b);  //This will give an error as scope of var_b is not available here;
}

fn main() {
    scope_of_var(5); //This will work as var_a is in scope
}
