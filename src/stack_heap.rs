enum List {
  Node(i32, Box<List>),
  Nil,
}

pub fn run() {
  // stack overflow => 8MBまでしか保存できない
  // let a1: [u8; 9000000] = [1; 9000000];

  // Vector型
  let mut v1 = vec![1, 2, 3, 4];
  let v2 = vec![5, 6, 7, 8];
  let mut v3 = vec![9, 10];
  println!("Stack address of v1 is : {:p}", &v1);
  println!("Stack address of v2 is : {:p}", &v2);
  println!("Heap memoty address of v1 is : {:?}", v1.as_ptr());
  println!("Len of v1 is : {}", v1.len());
  println!("Capacity of v1 is : {}", v1.capacity());
  v1.insert(1, 10);
  println!("{:?}", v1);
  v1.remove(0);
  println!("{:?}", v1);
  v1.append(&mut v3);
  println!("{:?}", v1);
  println!("{:?}", v3);

  // box pointer型
  let t1: (i64, String) = (10, String::from("Hello"));
  println!("Stack Address of tuple data is {:p}", &t1);
  println!("Heap Memory Address of t1.1 is {:?}", &t1.1.as_ptr());
  println!("Len of t1.1 is {}", t1.1.len());
  println!("Capacity of t1.1 is {}", t1.1.capacity());

  let mut b1 = Box::new(t1);
  (*b1).1 += "World";
  println!("{} {}", b1.0, b1.1);
  println!("Stack Address of b1 is {:p}", &b1);
  println!("Heap Address of tapule data is {:p}", b1);
}
