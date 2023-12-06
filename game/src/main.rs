use std::io;//prelude
use rand::Rng; // trait
use std::cmp::Ordering; // 枚举
fn main() {
    println!("猜数字游戏~");
    let secret_number = rand::thread_rng().gen_range(1..=101);
    println!("神秘数字:{}",secret_number);
    // immutable  可变的，rust默认const属性是不可以变
    let mut input_str = String::new();
    loop {
        input_str.clear();
        // & 引用，程序可以通过变量在不同地方访问同一块内存地址, mut是要numbers的引用是可变的。
        io::stdin().read_line(&mut input_str).expect("无法读取行");
        // io::Result Ok,Err, 返回Ok代表成功了,返回Err失败里面包含失败的原因。 expect,是Err会panic打印错误信息，若是Ok会提取结果值返回给用户Ok(t) => t
        
        //print!(" str= {:?}",input_str);
        let numbers:u32 = match input_str.trim().parse() {
                Ok(num) => num,
                Err(e)=> {
                    println!("Not a number.Error:[{:?}]",e);
                    continue
                }
        };
        match numbers.cmp(&secret_number) {
            Ordering::Less=> println!("too samll "),
            Ordering::Greater=> println!(" too big "),
            Ordering::Equal=> {
                println!("win");
                break;
        },

        };
        println!("你猜测的数是:{}",numbers);
    }
 
    
}
