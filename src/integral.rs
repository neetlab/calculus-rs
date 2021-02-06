use crate::algebra::Function1;
use std::ops::Range;

pub fn integrate1(f: &Function1, from: i32, to: i32) -> f64 {
  let interval = to - from;
  let dx = 1.0 / (10.0 as f64).powf(8.0);

  return (from..to)
    .map(|n| n as f64 * dx)
    .fold(0.0, |acc, n| acc + f(n));
}
