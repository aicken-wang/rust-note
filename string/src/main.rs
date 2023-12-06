use unicode_segmentation::UnicodeSegmentation;

fn main() {
    // 创建一个String
    let s = String::new();
    let data = "hello, world!";
    let s = data.to_string();

    let s = "hello, world!".to_string(); // 同上
    let s = String::from("hello, world!"); // 同上
    let mut s1 = String::from("hello");
    s1.push_str(", world!");
    println!("{}", s);
    println!("{}", s1);
    let mut s = String::from("hello, world");
    s.push('!'); //添加单个字符
    // 字符串拼接
    let s1 = String::from("hello");
    let s2 = String::from(", world!");
    let s3 = s1 + &s2;
    // println!("{}", s1); // value borrowed here after move
    println!("{}",s2);
    println!("{}",s3);
    //format!宏拼接字符串
    let s = format!("{}-{}-{}",s,s2,s3);
    println!("{}",s);
    // 获取长度
    println!("s length {}",s.len());
    //bytes
    for b in s.bytes() {
        println!("{} ",b);
    }
    //标量值
    for b in s.chars() {
        println!("{} ",b);
    }
    // 字形簇
    get_grapheme_clusters();
}
fn get_grapheme_clusters() {
    /*
    vim Cargo.toml 添加依赖

    [dependencies]
    rand = "0.8.5"
    unicode-segmentation = "1.10.1"

    */

    println!("-----------------------------------------------------------");
    let s = "नमस्कार";
    for cluster in s.grapheme_indices(true) {
        let (idx,graphemes) = cluster;
        println!("index: {}  Grapheme: {}",idx,graphemes);
    }
    for b in s.chars() {
       println!("chars: {}",b);     
    }
}