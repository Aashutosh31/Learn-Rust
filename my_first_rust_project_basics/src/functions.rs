fn main() {
  //Functions are the building blocks that execute code and return values. They are defined using the fn keyword, followed by the function name and parentheses. Functions can take parameters and return values, allowing for modular and reusable code.
  fn greet(name: &str){
    println!("Hello, {}",name);
  }
  greet("Aashutosh");

  //Functions can also return values. The return type is specified after the arrow (->) symbol. The return statement is used to specify the value to be returned from the function.
  fn add(a: i32, b: i32) -> i32 {
    return a+b;
  }
  let sum = add(3,4);
  println!("sum: {}",sum);
}