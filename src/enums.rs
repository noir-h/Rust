// 列挙型
enum OS {
  Windows(u32, String),
  Mac(u32, String),
  Linux(u32, String),
}

pub fn run() {
  let linux = OS::Linux(1991, String::from("Linux"));
  print_os_info(linux);
  let windows = OS::Linux(1985, String::from("MicroSoft"));
  print_os_info(windows);
  let mac = OS::Linux(2001, String::from("Apple"));
  print_os_info(mac);
}
// pattern matchnig
fn print_os_info(os: OS) {
  match os {
    OS::Windows(year, who) => {
      println!("Windows : Frist release in {} by {}", year, who);
    }
    OS::Linux(year, who) => {
      println!("Linux : Frist release in {} by {}", year, who);
    }
    OS::Mac(year, who) => {
      println!("Mac : Frist release in {} by {}", year, who);
    }
  }
}
