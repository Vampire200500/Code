

use std::process::Command;
fn main() {
//createfile();
let result = Command::new("git").args(["pull"]).output();
println!("{:?}", result);
let result = Command::new("git").args(["add","."]).output();
println!("{:?}", result);
let result=Command::new("git").args(["commit","-m\"Updated my stuff\""]).output();
println!("{:?}", result);
let result=Command::new("git").args(["push"]).output();
println!("{:?}", result);}




/*fn createfile(){
    let result=Command::new("fsutil").args(["file","createnew","examplefile.txt","0"]).output();
println!("{:?}", result);
}*/