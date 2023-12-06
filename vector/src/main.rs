fn main() {
    let mut v:Vec<i32> = Vec::new();
    // v = [0];  error
    v.push(1);
    v.push(2);
    v.push(3);
    v.push(4);
    println!("{:?}", v);
    // vec!  隐藏shadow
    let v = vec![1, 2, 3, 4, 5];
    println!("{:?}", v);
    // 获取vector中的元素
    let v = vec![1, 2, 3, 4, 5];
    let v2 = &v[1]; 
     println!("v[1] = {:?}", v2);
    //let v11 = &v[10]; // 获取11个元素 越界会panic index = 10  panic !!!
    // println!("v[10] = {:?}", v11);
    match v.get(10) {
        // 越界返回None 
        Some(val) => println!("match val = {:?}", val),
        None => println!(" None "),
    }
    //所有权和借用
    let mut v = vec![1, 2, 3, 4, 5];
    let v2 = &v[1];//不可变，
    //新增元素可能导致扩容,原数组的内存被释放,编译器禁用
    // v.push(6); // mutable borrow occurs here
    println!("v[1] = {}", v2);// 不可以变
    // 遍历 vector中的值
    let mut v = vec![1, 2, 3, 4, 5];
    for e in & mut v {
        println!("element {}",e);
        *e += 50;
    }
    // 打印
    for e in &v {
        println!("element {}",e);
    }
    // 使用枚举类型在vec里存储不同数据类型
    let data =vec![SheetCell::Int(0),SheetCell::Float(1.0), SheetCell::Text(String::from("title"))];
    for d in &data {
        println!("data element {:?}",d);
    }
}
// 实现debug的trait
#[derive(Debug)]
enum SheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}
