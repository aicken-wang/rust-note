use std::collections::HashMap;
fn main() {
    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        // 判断entry 判断word是否存在
        // or_insert 不存在就传入新值
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:#?}",map);

}
