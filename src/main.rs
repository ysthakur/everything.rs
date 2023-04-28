fn main() {
  let args = std::env::args().collect::<Vec<_>>();
  if args.len() == 1 {
    println!("You need to give at least one argument");
  } else {
    println!("Something went wrong: {}", exec::Command::new(&args[1]).args(&args[2..]).exec());
  }
}
