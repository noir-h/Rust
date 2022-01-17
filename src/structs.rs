#[derive(Debug)]
struct User {
  username: String,
  email: String,
  sign_in_count: u64,
  active: bool,
}
#[derive(Debug)]
struct Rectangle {
  width: u32,
  height: u32,
}
impl Rectangle {
  fn create(width: u32, height: u32) -> Self {
    // SelfがRectangleのデータ型自体を意味する
    Self { width, height }
  }
  // &selfで渡さないと所有権がmoveしてしまい作成したインスタンスにアクセスできなくなる
  fn area(&self) {
    println!("{}", self.width * self.height);
  }
}

pub fn run() {
  let user1 = User {
    username: String::from("hogehoge"),
    email: String::from("hoge@example.com"),
    active: true,
    sign_in_count: 1,
  };
  // user2にuser1をバインドすると所有権moveが発生
  // let user2 = user1;
  // println!("{}", user1); // error
  let mut user1 = User {
    username: String::from("hogehoge"),
    email: String::from("hoge@example.com"),
    active: true,
    sign_in_count: 1,
  };
  user1.email = String::from("another@example.com");
  println!("{:#?}", user1);
  let user2 = build_user(String::from("user2@example.com"), String::from("user2"));
  println!("{:#?}", user2);

  let rect = Rectangle::create(20, 20);
  println!("{:#?}", rect);
  rect.area();
}

fn build_user(email: String, username: String) -> User {
  User {
    email,
    username,
    active: true,
    sign_in_count: 1,
  }
}
