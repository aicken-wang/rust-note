fn main() {
    println!("Hello, world!");
    let mut x = 1;
    let number = loop {
        x = x +1;
        if x == 10 {
            break x * 2;
        }
    };
    println!("{:?} numbers:{:?}", x,number);
    let mut x = 3; 
    while x != 0 {
        println!("{:?}!", x);
        x = x -1;
        
    }
    println!("fire");
    let arr  = vec![1,2,3,4,5,6];
    for e in arr.iter() {
        println!("{:?}", e);
    }
    println!("########################");
    for number in (1..4).rev() {
        println!("{:?}", number);
    }
    println!("game over!");

    let  mut str = String::from("Hello, world!");
    str.push_str("^=^ ");
    println!("{:?}", str);
}
