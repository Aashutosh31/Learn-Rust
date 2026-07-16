fn main() {
    println!("Hello, world!");
    print!("This will be printed in same line \n");//used \n to print in new line 
    print!("This will be printed in same line\n");

    let name = "Aashutosh";
    println!("My name is {}", name);//Rust uses {} as a placeholder for variables in strings, similar to Python's f-strings or JavaScript's template literals.

    let name = "Aashutosh";
    let age = 21;
    println!("My name is {} and I am {} years old", name, age); // Rust allows you to use multiple placeholders in a single string, and you can pass multiple variables to fill those placeholders.

    let name = "Aashutosh";
    let age = 21;
    println!("My name is {} and I am {} years old", age, name); // Rust allows you to use multiple placeholders in a single string, and you can pass multiple variables to fill those placeholders. The order of the variables matters, so make sure to pass them in the correct order.

   /* 
   let x = 5;
    x = 10; // This line will cause a compile-time error because `x` is immutable by default in Rust. To make it mutable, you need to declare it with `let mut x = 5;`.
   */

    let mut x = 5;
    println!("Before:{}",x);
    x = 10; // Now this line is valid because `x` is mutable.
    println!("After:{}",x);

    // Rust does not require to specify data types explicitly, it can infer the type based on the value assigned to the variable. However, you can also specify the type explicitly if needed.

    let my_number: i32 = 10; // Here, we explicitly specify that `my_number` is of type `i32`.
    let my_float: f64 = 3.14; // Here, we explicitly specify that `my_float` is of type `f64`.
    let my_double: f64 = 2.71828; // Here, we explicitly specify that `my_double` is of type `f64`.
    let my_string: &str = "Hello, Rust!"; // Here, we explicitly specify that `my_string` is of type `&str`.
    let my_boolean: bool = true; // Here, we explicitly specify that `my_boolean` is of type `bool`.
    let my_char: char = 'A'; // Here, we explicitly specify that `my_char` is of type `char`.

    println!("My number is: {}", my_number);
    println!("My float is: {}", my_float);
    println!("My double is: {}", my_double);
    println!("My string is: {}", my_string);
    println!("My boolean is: {}", my_boolean);
    println!("My character is: {}", my_char);

    let name = "Aashutosh";
    let age = 21;
    let is_admin = true;
    println!("Name: {}",name);
    println!("Age: {}",age);
    println!("Admin: {}",is_admin);

/* 
Constants are always immutable and must have a type annotation. They can be declared in any scope, including the global scope, and can be accessed from anywhere in the code.
    const MAX_POINTS = 44; 
*/
    const MAX_POINTS: i32 = 100;// Upper case should be used while declaring constants in Rust, and they must have a type annotation. Constants are always immutable and can be declared in any scope, including the global scope, and can be accessed from anywhere in the code.
    println!("The maximum points are: {}", MAX_POINTS);

    // Operators in rust , arithematic operators and basic comparison operators works same as in Javascript and so as logical operators 
    
    const IS_LOGGEDIN: bool = true; // This is a constant boolean variable that indicates whether a user is logged in or not. It is set to true, meaning the user is logged in.
    const IS_ADMIN: bool = false; // This is a constant boolean variable that indicates whether a user is an admin or not. It is set to false, meaning the user is not an admin.

    println!("Is a regular user : {}", IS_LOGGEDIN && !IS_ADMIN);// This line prints whether the user is a regular user or not. A regular user is defined as someone who is logged in (IS_LOGGEDIN is true) and not an admin (IS_ADMIN is false). The expression IS_LOGGEDIN && !IS_ADMIN evaluates to true in this case, so the output will be "Is a regular user : true".
    println!("Has a special access : {}",IS_LOGGEDIN || IS_ADMIN);// This line prints whether the user has special access or not. A user has special access if they are either logged in (IS_LOGGEDIN is true) or an admin (IS_ADMIN is true). The expression IS_LOGGEDIN || IS_ADMIN evaluates to true in this case, so the output will be "Has a special access : true".
    println!("Unauthorized : {}", !IS_LOGGEDIN);// This line prints whether the user is unauthorized or not. A user is considered unauthorized if they are not logged in (IS_LOGGEDIN is false). The expression !IS_LOGGEDIN evaluates to false in this case, so the output will be "Unauthorized : false".

    //If Else statements in Rust are similar to other programming languages. The syntax is straightforward and easy to understand. Here's an example:
    let age = 21;
    if age >= 18 {
        println!("You are an adult.");
    } else {
        println!("You are a minor.");
    }

    //If else as an expression
    let time = 9;
    let greeting =  if time < 12 {
        "Good Morning!"
    } else {
        "Good day!"
    };
    println!("Greeting: {}", greeting);

    //if each block contains single statement, then we can write it in single line without using {} braces
    let time = 17;
    let greeting = if time < 12 {"Good morning!"} else {"Good Evening!"};
    println!("Greeting: {}", greeting);

    //Do not mix types in if else expression, it will give compile time error
    // let greeting = if time < 12 { "Good Morning!" } else { 5 }; // This will cause a compile time error

    //match statement in Rust is similar to switch statement in other languages. It allows you to compare a value against a series of patterns and execute code based on which pattern matches. Here's an example:
    let day = 4;
    match day {
        1 => println!("Monday"),
        2 => println!("Tuesday"),
        3 => println!("Wednesday"),
        4 => println!("Thursday"),
        5 => println!("Friday"),
        6 => println!("Saturday"),
        7 => println!("Sunday"),
        _ => println!("Invalid day"), // The underscore (_) is a catch-all pattern that matches any value not explicitly handled by the previous patterns.  
    }
}
