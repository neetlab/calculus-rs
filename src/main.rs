mod differential;

fn quadratic_function (x: f64) -> f64 {
  return x.powi(2);
}

fn main() {
  let derivative = differential::ddx1(&quadratic_function);

  let y = quadratic_function(2.0);
  let dy = derivative(1.0);

  println!("{}, {}", y, dy);
}
 