pub fn run() {
  let st1 = String::from("x");
  let st2 = String::from("y");
  let res1 = get_longest(&st1, &st2);
  println!("{}", res1);

  let st3 = String::from("x");
  {
    let st4 = String::from("x");
    // st4の方がライフタイムが短いためres2もst4と同じライフタイムとみなされる
    let res2 = get_longest(&st3, &st4);
    println!("{}", res2);
  }

  let st5 = String::from("x");
  // 型が逆算されて推論される
  let res3;
  {
    let st6 = String::from("x");
    res3 = get_longest(&st5, &st6);
  }
  // ダングリングポインタが発生する
  // println!("{}", res3);
}

// 'a 返り値のリファレンスのライフタイムは引数で受け取った値の短い方を採用する
fn get_longest<'a>(x: &'a str, y: &'a str) -> &'a str {
  if x.len() > y.len() {
    x
  } else {
    y
  }
}

// fn dummy<'a>() -> &'a str {
//   let s = String::from("demo");
//   // このfnのスコープを抜ける時にsがドロップするからリファレンスを返せない
//   &s
// }
