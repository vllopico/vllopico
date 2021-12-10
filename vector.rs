fn suma_vec(v:Vec<i32>) -> i32 {
  let mut sum:i32 = 0;
  for n in v.iter(){
      sum = sum + n;
  }
  return sum;
}

fn main() {
  let sv:i32 = suma_vec(v);
  let mut v = Vec::new();
  v.push(1);
  v.push(2);
  v.push(3);
  v.push(4);
  v.push(5);
  println!("Suma elements vector -> {}",sv);
}
