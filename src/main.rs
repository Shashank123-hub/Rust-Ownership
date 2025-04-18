
fn owner_var_example() {
    let x: i8 = 10;                            // a is the owner of the value 10
    let y: i8 = x;                             // b is the owner of the value 10
    // x is still valid here, as it is not moved
    // y is a copy of x, so it is also valid here
    
    let mut str1: String = String:: from("Hello");  // str1 is the owner of the string
    str1.push_str(" World");               // str1 is still the owner of the string
    // let str2: String = str1;                 // str2 is the owner of the string
    // str1 is moved to str2, so str1 is no longer valid
    // We can use str1 only if we use a reference to it by using &str1 or by cloning it
    // let str2: String = str1.clone();           // Clone creates a new string and assigns it to str2
    // str1 is still valid here, as it is not moved
    let str2: &String = &str1;                  // str2 is a reference to str1
    // str1 is still valid here, as it is not moved

    println!("Variable x is: {}", x);
    println!("Variable y is: {}", y);
    println!("String 1 is: {}", str1);         // This will give an error as str1 is moved to str2
    println!("String 2 is: {}", str2);         // This will work as str2 is the owner of the string
}

const VAR_C: i8 = 50;                          //A global variable - this is available throughout the program

fn scope_of_var(a: i8) {
    
    println!("Variable a is: {}", a);  //This will work

    {
        let var_b:i8 = 5;                      // scope of this variable is available within the braces
        println!("Variable b is: {}", var_b);  //This will works as var_b is in scope
        println!("Variable c is: {}", VAR_C);  //Variable c is a global variable and is available here
    }
    //println!("Variable b is: {}", var_b);    //This will give an error as scope of var_b is not available here
}

fn example1(r: u8) {
    println!("Value of integer in example1 fn is: {}", r); //This will work as r is in scope
    //Here we are passing the value of q to r
    // r is a copy of q, so it is also valid here
}

fn example2(s:String) -> usize {    //This function takes ownership of the string and calculates its length
    return s.len();                                       //This will work as s is in scope
}

fn main() {
    let q: u8 = 2;
    let t: String = String::from("Hello");               // t is the owner of the string
    let len: usize = example2(t);
    println!("Length of string is: {}", len);            //This will work as t is in scope
    example1(q);
    println!("Value of integer in main fn is: {}", q);   //This will work as q is in scope
    scope_of_var(5);                                     //This will work as var_a is in scope
    owner_var_example();    
}


