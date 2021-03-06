// 構造体,genericsを適用させることで何らかの型でインスタンス化できる
struct Point<T> {
  x: T,
  y: T,
}
struct PointOnother<T, U> {
  x: T,
  y: U,
}
impl<T, U> PointOnother<T, U> {
  fn mixup<V, W>(self, other: PointOnother<V, W>) -> PointOnother<T, W> {
    PointOnother {
      x: self.x,
      y: other.y,
    }
  }
}

pub fn run() {
  let number_list = vec![34, 50, 25, 100, 65];
  // ''で囲うとchar型になり4バイトのサイズを持つ
  let char_list = vec!['a', 'b', 'c', 'd'];
  println!("{}", largest(char_list));
  println!("{}", largest(number_list));
  // インスタンス化
  let p1 = Point { x: 1, y: 2 };
  let p2 = Point { x: 1.0, y: 2.0 };
  let p3 = PointOnother { x: 5, y: 10.4 };
  let p4 = PointOnother { x: "Rust", y: 'a' };
  let p5 = p3.mixup(p4);
  println!("{}{}", p5.x, p5.y);
}

fn largest_i32(list: Vec<i32>) -> i32 {
  let mut largest = list[0];
  for item in list {
    if item > largest {
      largest = item;
    }
  }
  largest
}
// <T>何らかのデータ型を意味する
// Trait境界、PartisalOrd + Copyで比較が可能な型のみを許可する
fn largest<T: PartialOrd + Copy>(list: Vec<T>) -> T {
  let mut largest = list[0];
  for item in list {
    if item > largest {
      largest = item;
    }
  }
  largest
}
