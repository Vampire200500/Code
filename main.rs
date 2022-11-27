

use std::process::Command;
fn main() {
let result = Command::new("git").args(["pull"]).output();
println!("{:?}", result);
let result = Command::new("git").args(["add","."]).output();
println!("{:?}", result);
let result=Command::new("git").args(["commit","-m\"Updated my stuff\""]).output();
println!("{:?}", result);
let result=Command::new("git").args(["push"]).output();
println!("{:?}", result);
let result=Command::new("touch").args(["test.rs"]).output();
}