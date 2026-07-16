fn main() {
//loops in rust are similar to other programming languages, but they have some unique features. There are three main types of loops in Rust: `loop`, `while`, and `for`.

//1.Loop is an infinite loop that will continue to execute until it is explicitly broken out of using the `break` statement. Here's an example:
let mut count = 1;

loop {
  println!("Count: {}", count);
  if count >= 10 {
    break; 
  }
  count += 1;
};

//only loop can return a value, and the value is returned using the `break` statement or even without it. Here's an example:
let result = loop {
  println!("Count: {}", count);
  if count > 10 {
    break count; // The value of `count` will be returned from the loop and assigned to `result`. 
  }
  count += 1;
};
println!("Result: {}", result); // 11 will be printed as the value of `result` because the loop was broken when `count` was greater than 10, and the value of `count` at that point was 11.

//2. The `while` loop will continue to execute as long as the specified condition is true. Here's an example:

let mut win_percent = 50;
let mut score = 1;

//we can not use final_score = while score != 20 { ... } because the while loop does not return a value, so we need to use a separate variable to store the final score after the loop has completed.Even with break statement, the while loop does not return a value, so we need to use a separate variable to store the final score after the loop has completed. 
while score != 20 {

  //skipping the iteration of the loop using the `continue` statement. The `continue` statement will skip the rest of the code in the current iteration and move on to the next iteration of the loop. Here's an example:
  if score == 10 {
    score += 1; // This line is necessary to increment the score before continuing to the next iteration. Without this line, the loop would get stuck in an infinite loop when `score` is 10, because the condition `score == 10` would always be true, and the `continue` statement would keep skipping the rest of the code in that iteration without ever incrementing `score`. By adding `score += 1;`, we ensure that `score` will eventually become 11, allowing the loop to continue and eventually exit when `score` reaches 20.

    continue; // This will skip the rest of the code in the current iteration and move on to the next iteration of the loop. So, when `score` is 10, it will be incremented to 11, and the `println!` statement will not be executed for that iteration.
  }

  if score == 19 {
    break; //value of score will be 19 when the loop is broken, and we can use that value to determine the final score after the loop has completed.
  }
  println!("Scoreline: {}",score);
  score += 1;
};

let final_score = score; 

if final_score >= 19  {
  win_percent += 50;
}else {
  win_percent -= 50;
}
println!("win_percent:{}",win_percent);

//3. The `for` loop is used to iterate over a range of values or a collection. Here's an example:
//Rust handles the counter variable (i) automatically, unlike many other programming languages. You don't need to declare or increment it manually.
for i in 1..10 { // The range `1..10` includes numbers from 1 to 9 (10 is exclusive). If you want to include 10, you can use `1..=10`.
  println!("Iteration: {}", i);
}

for i in 1..=10{
  if i == 3 {
    continue; // This will skip the rest of the code in the current iteration and move on to the next iteration of the loop. So, when `i` is 3, it will skip the `println!` statement and continue with the next iteration.
  }
  if i == 7 {
    break; // like while loop, the for loop can be broken using the `break` statement, and the value of `i` will be 7 when the loop is broken. We can use that value to determine the final value of `i` after the loop has completed.
  }
  println!("i : {}",i); // It will not print 7 becuase the loop is broken when `i` is 7, so the `println!` statement will not be executed for that iteration.
}

//To print the final value of `i` after the loop has completed, we can use a separate variable to store the final value of `i` after the loop has completed. Here's an example:
let mut final_i = 0; // Initialize a variable to store the final value of `i` after the loop has completed.
for i in 1..=10 {
  if i == 3 {
    continue; // This will skip the rest of the code in the current iteration and move on to the next iteration of the loop. So, when `i` is 3, it will skip the `println!` statement and continue with the next iteration.
  }
  if i == 7 {
    final_i = i; // Store the value of `i` when the loop is broken
    break; // like while loop, the for loop can be broken using the `break` statement, and the value of `i` will be 7 when the loop is broken. We can use that value to determine the final value of `i` after the loop has completed.    
}
}
println!("Final value of i after the loop: {}", final_i); // It will print 7 because the loop was broken when `i` was 7, and we stored that value in `final_i`.
}