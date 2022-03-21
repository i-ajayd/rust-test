pub fn run(){
  let mut line = String::new();
  println!("Enter your name :");
  let _b = std::io::stdin().read_line(&mut line);
  println!("Hello , {}", line);
  // println!("no of bytes read , {}", _b);
}

struct Person {
  name: String;
  surname: String
}