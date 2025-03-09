
fn  main() {
    // 循环    
    let mut i = 0;
    let result = loop {
        i += 1;
        if i == 10 {
            break i;
        }
    };
    println!("result = {}", result);  
    // while
    let mut i = 0;  
    while i < 10 {
        i += 1;
    }
    println!("result = {}", i);
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
 
    return ();
    // for
    let arr = [1, 2, 3, 4, 5];
    for i in arr.iter() {
        println!("i = {}", i);
    }
    // for
    for i in (0..10).rev() {
        println!("i = {}", i);
    }
    // for
    for i in (0..10).step_by(2) {
        println!("i = {}", i);
    }
    // for
    for i in (0..10).filter(|x| x % 2 == 0) {
        println!("i = {}", i);
    }
    // for
    for i in (0..10).map(|x| x * 2) {
        println!("i = {}", i);
    }
    // for
    for i in (0..10).enumerate() {
        println!("i = {}", i.0);
    }
    // for
    for i in (0..10).zip(10..20) {
        println!("i = {}", i.0);
    }
    // for
    for i in (0..10).zip(10..20).map(|(x, y)| x + y) {
        println!("i = {}", i);
    }
    // for
    for i in (0..10).zip(10..20).filter(|(x, y)| x % 2 == 0) {
        println!("i = {}", i.0);
    }
    // for
    for i in (0..10).zip(10..20).filter(|(x, y)| x % 2 == 0).map(|(x, y)| x + y) {
        println!("i = {}", i);
    }
    // for
    for i in (0..10).zip(10..20).filter(|(x, y)| x % 2 == 0).map(|(x, y)| x + y).enumerate() {
        println!("i = {}", i.0);
    }
    // for
    for i in (0..10).zip(10..20).filter(|(x, y)| x % 2 == 0).map(|(x, y)| x + y).enumerate().map(|(i, x)| x * i) {
        println!("i = {}", i);
    }
    // for
    for i in (0..10).zip(10..20).filter(|(x, y)| x % 2 == 0).map(|(x, y)| x + y).enumerate().map(|(i, x)| x * i).filter(|x| x % 2 == 0) {
        println!("i = {}", i);
    }  
}