fn main() {
  let all_args = std::env::args().collect::<Vec<_>>();
  if all_args.len() == 1 {
    println!("You need to give at least one argument");
  } else {
    let err = exec::Command::new(&all_args[1]).args(&all_args[2..]).exec();
    println!("Something went wrong: {}", err);
  }
}
