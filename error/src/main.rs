use std::fs::File;
use std::io::ErrorKind;
use std::io;
use std::io::Read;
pub struct Guess {
    value:i32,
}
impl Guess {
    pub fn new(value:i32) -> Guess { 
        if value < 1 || value > 100 {
            panic!("value must be between 1 and 100,got {} ",value);
        }
        Guess {value}
    }
    pub fn value(&self) -> i32 { self.value }
}

fn main() {
    // rust 的可靠性，错误处理
    // 大部分情况 在编译时提示错误，并处理
    // 错误分类
    // - 可恢复，如文件没有找到，可以再次尝试  Result<T,E>
    // - 不可恢复， bug 访问数组下标超出了范围 panic! 宏
    println!("Hello, world!");
    let f = File::open("hello.txt"); 
    println!("文件是否打开成功,{:?}",f);
    let f = match f {
        Ok(file) => file,
        Err(err) =>match err.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(file_create) => file_create,
                Err(e) =>panic!("Error Creating file error, err msg: {:?}.", e),
            },
            other_err=>panic!("Error opening the file {:?}",other_err),
        }
    };
    println!("文件是否打开成功,{:?}",f);
    // 使用闭包
    let fd = File::open("fd.log").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("fd.log").unwrap_or_else(|error| {
                panic!("Error creating file:{:?}",error);
            })
        }else {
            panic!("Error opening file {:?}",error)
        }
    });
    println!("文件是否打开成功,{:?}",fd);

    // unwrap()方法等效示例
    let  f = File::open("hello.txt");
    let f = match f {
        Ok(f) => f,
        Err(e) => panic!("Error opening file:{:?}",e),
    };
    println!("match 文件是否打开成功,{:?}",f);
    //上面等效于
    let f = File::open("hello.txt").unwrap();
    println!("unwrap 文件是否打开成功,{:?}",f);
    //expect
    let f = File::open("hello.txt").expect("无法打开文件");
    println!("expect 文件是否打开成功,{:?}",f);
    // 传播错误
    let  result = read_userinfo_from_file();
    let g = Guess::new(10);
    
    println!("guess.value = {}",g.value());

}

fn read_userinfo_from_file() -> Result<String,io::Error> {
    let f = File::open("user.txt");
    let mut f = match f {
        Ok(f) => f,
        Err(e) => return Err(e),
    };
    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => return Err(e),
    }
}
// 使用?运算符简化代码
fn read_userinfo_from_file1() -> Result<String,io::Error> {
    let mut f = File::open("user.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}
// 使用?运算符简化代码
fn read_userinfo_from_file2() -> Result<String,io::Error> {
    let mut s = String::new();
    File::open("user.txt")?.read_to_string(&mut s)?;
    Ok(s)
}