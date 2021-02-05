use std::f64;

type Function1 = dyn Fn(f64) -> f64;

fn derivative(f: &Function1, x: f64) -> f64 {
  let h = 1.0 / (10.0 as f64).powf(8.0);
  return (f(x + h) - f(x)) / h;
}

// 一引数函数を受け取って導関数を返す
// lim {h -> 0} を f64::MIN でシミュレート
pub fn ddx1(f: &'static Function1) -> Box<Function1> {
  return Box::new(move |x| derivative(&f, x));
}
