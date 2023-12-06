use std::fmt::Display;
fn  account<'a ,T>(x:&'a str, y:&'a str, acc: T) ->&'a str 
where
    T: Display,
{
    println!("account:{}",acc);
    if x.len() > y.len() { 
        x
    } else { 
        y
    }
}
// fn main() {
//     let s1 = String::from("hello world");
//     let s2 = String::from("abc");
//     let s3 = longest(&s1,&s2);
//     println!("{}", s3);

// }
// fn longest<'a>(x:&'a str, y:&'a str) -> &'a str {
//     if x.len() > y.len() {
//         x
//     } else {
//         y
//     }
// }

// demo
#[derive(Debug)]
pub  struct UserExample<'a> {
    name: &'a str,
}
impl <'a> UserExample<'a> {
   pub fn get_age(&self) -> u32 { 
        18
    }
   pub fn get_name(&self, account:&str) -> &str {
        println!("account:{}", account);
        self.name
    }
}
fn main() {
    let s = String::from("hello, world !");
    let n = s.split(',').next().expect("没有找到 ',' in string");
    let u = UserExample { name:n};
    println!("{:?} ,{:?}", u, u.get_name("10086"));

}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}

