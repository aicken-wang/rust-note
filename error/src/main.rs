//  use std::fs::File

// use std::io::Read;
fn main() {
    // rust 的可靠性，错误处理
    // 大部分情况 在编译时提示错误，并处理
    // 错误分类
    // - 可恢复，如文件没有找到，可以再次尝试  Result<T,E>
    // - 不可恢复， bug 访问数组下标超出了范围 panic! 宏
    println!("Hello, world!");
    let f = std::fs::File::open("hello.txt"); 
    println!("文件是否打开成功,{:?}",f);
}
