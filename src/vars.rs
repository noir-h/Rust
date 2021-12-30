pub mod sub_a;
mod sub_b;

const MAX_POINTS: u32 = 100_000;
const MAX2_POINTS: u32 = 4294967295;
pub fn run() {
  println!("test{:p}", &MAX_POINTS);
  println!("test{:p}", &MAX2_POINTS);
  println!("Here is vars module");
  // sub_a::func_a();
  // sub_b::func_b();
  let mut x = 5;
  println!("The value of x is: {}", x);
  x = 6;
  println!("The value of x is: {}", x);
  /*
   *  デフォルトではi1に32bit,f1に64bitの型の推論が割り当てられる
   *  _を文頭につけることで使用していない変数ということをコンパイラーに伝える
   */
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

  /*
   *  タプル型
   *  下記のようにハードコーディングされた文字列を文字列スライスという
   *  データへのアクセス方法は2通り
   */
  let t1 = (500, 6.4, "dummy");
  let (_x, _y, _z) = t1;
  println!("The value of t1 is: {},{},{}", t1.0, t1.1, t1.2);

  let mut t2 = ((0, 1), (2, 3));
  let ((ref mut x1_ptr, ref mut x2_ptr), _) = t2;
  // 参照外し、ポインタが差す参照ではなく値にアクセスすることができる
  *x1_ptr = 5;
  *x2_ptr = -5;
  // タプル型、構造体などは{:?}でデータを取り出せる
  println!("{:?}", t2);

  /*
   *  配列
   *  サイズや要素数の変更はできない,コンパイル時にサイズが決まっている必要がある => stackに積まれる
   */
  let a1 = [0, 1, 2, 3, 4, 5];
  let a2 = [0; 10];
  println!("{:?} {:?} {} {}", a1, a2, a1[2], a1[3]);

  // 文字列スライス
  let s1 = "helloこんにちは挨拶";
  let s2 = "hello";
  println!("Stack address of s1 is: {:p}", &s1);
  println!("Stack address of s2 is: {:p}", &s2);
  println!("Static memory address of s1 is: {:p}", s1.as_ptr());
  println!("Static memory address of s2 is: {:p}", s2.as_ptr());
  println!("Len of s1 is: {}", s1.len());
  println!("Len of s2 is: {}", s2.len());

  // String型
  let mut s1 = String::from("hello");
  let mut s2 = String::from("helloworld");
  println!("Stack address of s1 is: {:p}", &s1);
  println!("Stack address of s2 is: {:p}", &s2);
  println!("Heap memory address of s1 is: {:p}", s1.as_ptr());
  println!("Heap memory address of s2 is: {:p}", s2.as_ptr());
  println!("Len of s1 is: {}", s1.len());
  println!("Len of s2 is: {}", s2.len());
  println!("Capacity of s1 is: {}", s1.capacity());
  println!("Capacity of s2 is: {}", s2.capacity());
  s1.push_str("_new1");
  s1.push_str("_new2");
  println!("{} {}", s1, s2);
  println!("Stack address of s1 is: {:p}", &s1);
  println!("Stack address of s2 is: {:p}", &s2);
  println!("Heap memory address of s1 is: {:p}", s1.as_ptr());
  println!("Heap memory address of s2 is: {:p}", s2.as_ptr());
}
