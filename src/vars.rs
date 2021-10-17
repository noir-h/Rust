pub mod sub_a;
mod sub_b;

const MAX_POINTS: u32 = 100_000;

pub fn run() {
  println!("Here is vars module");
  // sub_a::func_a();
  // sub_b::func_b();
  let mut x = 5;
  println!("The value of x is: {}", x);
  x = 6;
  println!("The value of x is: {}", x);
  // デフォルトではi1に32bit,f1に64bitの型の推論が割り当てられる
  // _を文頭につけることで使用していない変数ということをコンパイラーに伝える
  let _i1 = 3;
  let _f1 = 0.1;

  println!("{}", usize::BITS);
  println!("Memory address of const is: {:p}", &MAX_POINTS);

  let i2: i64 = 1;
  let i3: i64 = 2;
  println!("Stack address of i2 is: {:p}", &i2);
  println!("Stack address of i3 is: {:p}", &i3);

  let y = 5;
  println!("Stack address of y is: {:p}", &y);
  let y = y + 1;
  println!("Stack address of y is: {:p}", &y);
  let y = y * 2;
  println!("Stack address of y is: {:p}", &y);
  println!("The value of y is: {}", y);
  {
    let y = 0;
    println!("The value of y is: {}", y);
  }
  println!("The value of y is: {}", y);

  // タプル型
  let t1 = (500, 6.4, "dummy");
  let (_x, _y, _z) = t1;
  println!("The value of t1 is: {},{},{}", t1.0, t1.1, t1.2);

  let mut t2 = ((0, 1), (2, 3));
  let ((ref mut x1_ptr, ref mut x2_ptr), _) = t2;
  *x1_ptr = 5;
  *x2_ptr = -5;
  println!("{:?}", t2);

  // 配列
  // サイズや要素数の変更はできない
  let a1 = [0, 1, 2, 3, 4, 5];
  let a2 = [0; 10];
  println!("{:?} {:?} {} {}", a1, a2, a1[2], a1[3]);
}
