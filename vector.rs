fn main() {
  let h: Vec<String> = ["Hello".to_string(), ",".to_string(), " World".to_string(), "!".to_string()].to_vec();
  for s in h.iter() {
      print!("{}",s);
  }
}
