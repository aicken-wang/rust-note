# Rust基础语法
    
### Rust基础类型
- 赋值语句: let关键字
```rust
let a: u32 = 10;
// 或
let a = 10u32;
```
- 数字类型: 
  - 整数

|长度| 有符号 | 无符号 |
| --- | --- | ---|
|8-bit|i8|u8|
|16-bit|i16|u16|
|32-bit|i32|u32|
|64-bit|i64|u64|
|128-bit|i128|u126|
|CPU arch|isize|usize|
- isize/usize和CPU架构有关，32位就是i32/u32,64位就是i64/u64

```plaintext
十进制 10_222 等效于10222，使用下划线按3位数一组隔开，方便人阅读
十六进制 0x开头， 如：0xffff
八进制 0o(小写字母o)开头，如：0o12
二进制 0b开头按四位数字一组隔开 如：0b1000_0001
字符的子节表示 b'A'  对应一个ASCII字符
```
- 浮点数 f32和f64
```rust
let a = 1.0f32;
let b = 2.0f64;
```
- 布尔类型 true和false
```rust
let a = true;
let b: bool = false;
```
- 字符 char Unicode散列值
```rust
    let b: bool = false;
    let a = 'A';
    let b: char = '吖';
    let dog = '🐶';
    let zh = '中';
    let heart_eyed_cat = '😻';
```
- 字符串 
  - String Unicode字符串的UTF-8编码
```rust
fn main() {
    // 多语言问候语集合
    let greetings = vec![
        ("中文", "你好，世界！"),          // 中文
        ("English", "Hello, World!"),      // 英语
        ("Español", "¡Hola, Mundo!"),      // 西班牙语
        ("Français", "Bonjour le monde!"), // 法语
        ("Deutsch", "Hallo Welt!"),        // 德语
        ("Italiano", "Ciao, mondo!"),      // 意大利语
        ("Português", "Olá Mundo!"),       // 葡萄牙语
        ("العربية", "مرحبا بالعالم!"),     // 阿拉伯语
        ("日本語", "こんにちは、世界！"),  // 日语
        ("Русский", "Здравствуй, мир！"),  // 俄语
        ("ไทย", "สวัสดีชาวโลก！"),           // 泰语
        ("한국어", "안녕하세요, 세계！"),  // 韩语
        ("Swahili", "Hujambo, Dunia！"),   // 斯瓦希里语
    ];

    // 遍历输出所有问候语
    for (lang, text) in greetings {
        println!("[{}] {}", lang, text);
    }
/*
    [中文] 你好，世界！
    [English] Hello, World!
    [Español] ¡Hola, Mundo!
    [Français] Bonjour le monde!
    [Deutsch] Hallo Welt!
    [Italiano] Ciao, mondo!
    [Português] Olá Mundo!
    [العربية] مرحبا بالعالم!
    [日本語] こんにちは、世界！
    [Русский] Здравствуй, мир！
    [ไทย] สวัสดีชาวโลก！
    [한국어] 안녕하세요, 세계！
    [Swahili] Hujambo, Dunia！

*/
}
```
```rust

    let s = String::from("你好");
    // String不支持通过下标访问，和C/C++/Go语言有区别，
    // 因为String存储的是Unicode序列的UTF-8是可变长的,取出来没有意义,Rust直接对String禁止了这个索引操作
    // let first = s[0]; //String cannot be indexed by `{integer}`
    let first = s.chars().next().unwrap();
    println!("{}", first); // 你

```
```go
package main
func main() {
	s := "你好"
	println(s[0]) // 228
}
```
  - 字符串字面量的转义 反斜杠\
```rust
// 和其它语言差不多，支持\x 输入等值ASCII \u{}输入等值的Unicode字符
fn main() {
    let s1 = " I am a \"string\"";
    let s2 = "I am \r\n a \\string.\0"; // 等效C语言一样\0结束符
    println!("{}", s1);
    println!("{}", s2);
    let s2 = " a = \x61 \u{56FD}";
    println!("{}", s3); // a 国
}
```
  - 禁止转义的字符串字面量 r""或r#""#
```rust

fn main(){
    // 字符串字面量前面加r，表示原始字符串，不会转义
    let s = r"好好学习，天天向上 \x61 \u{1234} \r\n \\Ok";
    println!("{}", s);  // 输出：好好学习，天天向上 \x61 \u{1234} \r\n \\Ok

    // 加#后编译器不会把字符串字面量中的"当作结束符
    let s1 = r#"好好学习，天天向上 \x61 \u{1234} \r\n \\Ok""#;
    println!("{}", s1);  // 输出：好好学习，天天向上 \x61 \u{1234} \r\n \\Ok"
    // 字符串中包含#号的情况，可以使用多个#号前后配对
    let s2 = r##"好好学习，天天向上 \x61 \u{1234} \r\n \\Ok#"##; 
    println!("{}", s2);  // 输出：好好学习，天天向上 \x61 \u{1234} \r\n \\Ok#
    // 多行字符串，Rust支持换行写，默认会把字符串中的换行符保留
    let s3 = "
    你好，
    Rust!";
    println!("{}", s3);  // 输出：你好，
                        // Rust!            
}
```
  - 字符串 b"字符串"
```rust

fn main(){
    // [u8; 13] 是一个固定大小的字节数组，它包含13个元素，每个元素都是一个字节
    let byte_string = b"Hello, World!";
    println!("{:?}", byte_string);  // Output: [72, 101, 108, 108, 111, 44, 32, 87, 111, 114, 108, 100, 33]
    // 说明紧凑的字节字符串的每个字符都是一个字节
    // 字节数组的每个元素都是一个字节，所以我们可以通过索引访问它们
    println!("{:?}", byte_string[0]);  // Output: 72
    // 字节数组在做系统编程和网络协议开发很有用
}
```
- 数组：array类型，连续内存，存储同一类型的多个值，固定长度
```rust
fn main(){
    let  arr: [i32,5] = [1,2,3,4,5]; // 数组有5个元素，每个元素的类型是i32
    // 或者，注意没有加mut的数组是不可变的，不能修改;编译器没有报错是因为第二个arr遮蔽了第一个arr
    let  arr = [1,2,3,4,5];
    // 固定尺寸的数组存放在栈上，能在编译器就可以根据数据类型计算运行时占用的内存空间
    // 通过下标访问数组元素
    println!("arr[0] = {}", arr[0]);
    // 数组越界访问会导致panic
    // println!("arr[5] = {}", arr[5]); // 编译器会报错 index out of bounds
    // 数组的长度是固定的，不能动态增加或减少
    // arr.push(6); // 编译器会报错    
}
```
- 动态数组：Vec向量，存储同一类型的多个值，容量可变，支持动态扩容
```rust
fn main(){
    let v1: Vec<i32> = Vec::new();
    println!("v1 = {:?}", v1); // v1 = []
    let v2 = vec![1, 2, 3];
    println!("v2 = {:?}", v2); // v2 = [1, 2, 3]
    let v3 = vec![0; 10];// v3数组中有10个元素，每个元素的值都是0
    println!("v3 = {:?}", v3); // v3 = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
    let v4 = vec![1, 2, 3, 4, 5];
    let third: &i32 = &v4[2]; // 支持索引访问, 从0开始, v4[2]访问第3个元素， 但是如果索引超出范围，运行时会导致panic
    println!("third = {}", third); // third = 3
    match v4.get(2) {
        Some(third) => println!("third = {}", third), // third = 3
        None => println!("There is no third element."),
    }
    let v5 = vec![100, 32, 57];
    for i in &v5 {
        println!("{}", i); // 100 32 57
    }
    let mut v6 = vec![100, 32, 57]; // 可变的动态数组      

    for i in &mut v6 {
        *i += 50;
    }
    println!("v6 = {:?}", v6); // v6 = [150, 82, 107]
    for i in &v6 {
        println!("{}", i); // 150 82 107
    }
    v6.push(200);
    println!("v = {:?}", v6);
    println!("v = {:?}", v6[5]); 
    // cargo build 编译，不会检查越界访问，运行时越界访问panic
    // thread 'main' panicked at src/main.rs:29:28:
    // index out of bounds: the len is 4 but the index is 5 
}
```
- 哈希表：存储key-value映射关系(字典)
```rust
use std::collections::HashMap;
fn main(){
    let mut map = HashMap::new();
    map.insert("name", "wang"); 
    map.insert("age", "33");
    map.insert("city", "深圳");
    println!("{:?}", map); // {"city": "深圳", "name": "wang", "age": "33"}
    println!("Name: {}", map["name"]); // Name: wang
    println!("Age: {}", map["age"]);   // Age: 33
    println!("City: {}", map["city"]); // City: 深圳
    map.remove("age"); // 删除age   
    println!("{:?}", map); // {"city": "深圳", "name": "wang"}
    println!("Age: {}", map.get("age").unwrap_or(&"Age not found")); // Age: Age not found
    println!("City: {}", map.get("city").unwrap_or(&"City not found")); // City: 深圳
    println!("Country: {}", map.get("country").unwrap_or(&"Country not found")); // Country: Country not found
    map.insert("city", "广州"); // 更新city 
    println!("{:?}", map); // {"city": "广州", "name": "wang"}
    map.clear(); // 清空map
    println!("{:?}", map); // {}
    map.insert("name", "aicken-wang"); // 插入name
    println!("{:?}", map); // {"name": "aicken-wang"}
}
```
  
### Rust复合类型
- 元组:固定(元素可以不同)长度的列表
```rust
fn main() {
    let tuple: (i32, f64, String, u8) = (1, 2.0, "Hello".to_string(), b'A');
    println!("Tuple: {:?}", tuple); // Tuple: (1, 2.0, "Hello", 65)
    println!("第1个元素: {}", tuple.0); // 第1个元素: 1
    println!("第2个元素: {}", tuple.1); // 第2个元素: 2
    println!("第3个元素: {}", tuple.2); // 第3个元素: Hello
    println!("第4个元素: {}", tuple.3); // 第4个元素: 65
                                        // 可以用作返回多个值
    let (a, b, c, d) = tuple; // 解构
    println!("a: {}, b: {}, c: {}, d: {}", a, b, c, d); // a: 1, b: 2, c: Hello, d: 65
    // 没有任何元素的元组称为unit，用()表示，类似于空集合, 当函数没有返回值,实际上返回的是一个unit值
    let unit: () = ();
    println!("Unit: {:?}", unit); // Unit: ()
    return unit; // 返回unit值
}
```
- 结构体: struct关键字
```rust
// use std::fmt;

#[derive(Debug)] // 自动实现 fmt::Debug trait
#[allow(dead_code)] // 禁用未使用代码警
// #[warn(unused_variables)] // 未使用变量警告
// #[warn(unused_imports)]   // 未使用导入警告
// #[warn(unused_mut)]       // 未使用可变变量警告
// #[warn(unused_unsafe)]    // 未使用不安全代码警告
// #[warn(unused_assignments)] // 未使用赋值警告
// #[warn(unused_attributes)]  // 未使用属性警告
// #[warn(unused_macros)]      // 未使用宏警告
// #[warn(unused_import_braces)] // 未使用导入大括号警告
// #[warn(unused_extern_crates)] // 未使用外部包警告
// #[warn(unused_features)]      // 未使用特性警告
// #[warn(unused_parens)]        // 未使用括号警告
// #[warn(unused_results)]       // 未使用结果警告
// #[warn(unused_doc_comments)]  // 未使用文档注释警告     
struct User {
    username: String,
    sex: bool,
    age: u8,
    email: String,
    address: String,
    phone: String,
    password: String,
    is_active: bool,
    is_admin: bool,
    is_staff: bool,
    is_superuser: bool,
    date_joined: String,
    last_login: String,
    date_of_birth: String,
    last_name: String,
    first_name: String,
    is_online: bool,
    is_verified: bool,
    is_blocked: bool,
    is_deleted: bool,
    is_suspended: bool,
    is_banned: bool,
    is_reported: bool,  
}
// 实现 fmt::Debug trait
// impl fmt::Debug for User {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         // 隐藏敏感字段（例如 `name`），仅显示 `age`
//         write!(f, "User {{ username:{}, age: {} }}", self.username, self.age) 
//     }
// }
fn main() {
    let u1  = User {
        username: String::from("aicken-wang"),   // 用户名
        sex: true,                                // 性别
        age: 33,                                  // 年龄
        email: String::from("aickenwang@163.com"),// 邮箱
        address: String::from("China"),           // 地址
        phone: String::from("12345678901"),       // 电话
        password: String::from("$2b$12$1Q"),      // 密码
        is_active: true,                          // 是否激活
        is_admin: false,                          // 是否管理员
        is_staff: false,                          // 是否员工
        is_superuser: false,                      // 是否超级用户
        date_joined: String::from("2021-01-01"),  // 注册时间
        last_login: String::from("2021-01-01"),   // 最后登录时间
        date_of_birth: String::from("2021-01-01"),// 出生日期
        last_name: String::from("Wang"),          // 姓
        first_name: String::from("Aicken"),       // 名 
        is_online: false,                         // 是否在线
        is_verified: false,                       // 是否验证       
        is_blocked: false,                        // 是否被屏蔽 
        is_deleted: false,                        // 是否被删除
        is_suspended: false,                      // 是否被暂停 
        is_banned: false,                         // 是否被封禁
        is_reported: false,                       // 是否被举报
    };
    println!("username: {}", u1.username);  // 输出用户名
    println!("{:#?}", u1); // 输出用户信息
}
```
- 枚举: enum关键字
```rust

#[derive(Debug)] // 加这一行，才能使用 {:?} 打印
#[derive(PartialEq)] // 加这一行，才能使用 == 比较
enum IpAddrKind {
    V4,
    V6,
}
fn  main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
    if four == IpAddrKind::V4 {
        println!("four == IpAddrKind::V4");
    }
    if six == IpAddrKind::V6 {
        println!("six == IpAddrKind::V6");
    }   
    println!("IpAddrKind::V4 = {:?}", IpAddrKind::V4);
    println!("IpAddrKind::V6 = {:?}", IpAddrKind::V6);   
}
```
### Rust控制流
- 分支语句:if/else 分支，支持表达式返回
```rust
// 和C/C++不同
// rust和go一样认为if后面的条件表达式不推荐用()包裹起来，
// 认为这样if(condition)是不必要的，是多余的语法噪音
fn  main() {
    let a = 1;
    // if else 返回了一个值
    let b = if a == 1 {
        // 代码最后不加分号，表示返回值
        2
    } else {
        3
    }; // 这里加分号，表示赋值语句结束
    println!("b = {}", b);
}
  // 三目运算符
  //let b = if a == 1 { 2 } else { 3 };
```
- 循环语句
  - loop:无条件循环
```rust
    // 循环    
    let mut i = 0;
    let result = loop {
        i += 1;
        if i == 10 {
            break i;
        }
    };
    println!("result = {}", result);  
```
  - while:条件判断循环
```rust
  // while
  let mut i = 0;  
  while i < 10 {
      i += 1;
  }
  println!("result = {}", i);
```
  - for:迭代器的遍历
```rust
    // for
    for i in 0..10 {
        println!("i = {}", i);
    }
    println!("----------------------");
    for i in (0..10).rev() {
        println!("i = {}", i);
    }
    println!("----------------------");
    for ch in 'A'..'Z' {
        println!("{:?}", ch); // 'A' 'B' 'C' ... 'Y'
    }
```
### Rust中的函数和模块
- 函数
  - fn关键字
  - 形参与实参
  - 栈帧
- 闭包
  - 保存上下文变量的函数，两个竖线符合 `||`
- 模块