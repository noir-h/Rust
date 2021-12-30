pub fn run() {
  // 所有権・参照・借用
  let s1 = String::from("Hello");
  let s2 = s1;
  println!("{}", s2);
  // println!("{} {}", s1, s2);

  let i1 = 1;
  let i2 = i1;
  println!("{}{}", i1, i2);
  println!("Stack Address of i1 is {:p}", &i1);
  println!("Stack Address of i2 is {:p}", &i2);

  let sl1 = "literal";
  let sl2 = sl1;
  println!("{}{}", sl1, sl2);
  println!("Stack Address of sl1 is {:p}", &sl1);
  println!("Stack Address of sl2 is {:p}", &sl2);

  let s3 = String::from("hello");
  let s4 = s3.clone();
  println!("Stack Address of s3 is {:p}", &s3);
  println!("Stack Address of s4 is {:p}", &s4);
  println!("Heap Memory Address of s3 is {:?}", s3.as_ptr());
  println!("Heap Memory Address of s4 is {:?}", s4.as_ptr());

  let s5 = String::from("hello");
  println!("Stack Address of s5 is {:p}", &s5);
  println!("Heap Memory Address of s5 is {:?}", s5.as_ptr());
  // println!("len of s5 is {}", s5.len());
  // println!("capacity of s5 is {}", s5.capacity());

  take_ownership(s5);
  // println!("{}", s5);

  let s6 = String::from("hello");
  println!("Stack Address of s6 is {:p}", &s6);
  println!("Heap Memory Address of s6 is {:?}", s6.as_ptr());
  println!("len of s5 is {}", s6.len());

  let s7 = take_giveback_ownership(s6);
  println!("Stack Address of s7 is {:p}", &s7);
  println!("Heap Memory Address of s7 is {:?}", s7.as_ptr());
  println!("len of s5 is {}", s7.len());

  let s8 = String::from("hello");
  let len = caluculate_length(&s8);
  println!("The length of '{}' is {}.", s8, len);

  let mut s9 = String::from("hello");
  change(&mut s9);
  println!("{}", s9);

  let s10 = String::from("hello");
  let r1 = &s10;
  let r2 = &s10;
  println!("{} {} {}", s10, r1, r2);

  // let mut s10 = String::from("hello");
  // let r1 = &s10;
  // *r1 = String::from("change");
  // let r2 = &mut s10;
  // println!("{}{}{}", s10, r1, r2);

  // let mut s11 = String::from("hello");
  // let r1 = &mut s11;
  // println!("{}", s11);
  // println!("{}", r1);
}

fn take_ownership(s: String) {
  println!("{}", s);

  println!("Stack Address of s5 is {:p}", &s);
  println!("Heap Memory Address of s5 is {:?}", s.as_ptr());
  // println!("len of s5 is {}", s.len());
  // println!("capacity of s5 is {}", s.capacity());
}

fn take_giveback_ownership(s: String) -> String {
  s
}

fn caluculate_length(s: &String) -> usize {
  s.len()
}

fn change(s: &mut String) {
  s.push_str("_world");
}
