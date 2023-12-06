use std::collections::HashMap;
fn main() {
    // 统计单词出现的个数，合并现有的value和新的value,value = old_value + new_value
    let text = "Reduce expectations, exercise, and get through the cycle";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        // 判断entry 判断word是否存在
        // or_insert 不存在就传入新值
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:#?}",map);

    // 使用collect方法创建
    let description_en = vec![
        String::from("Project Manager"),
        String::from("Product Director"),
        String::from("Operations Director"),
        String::from("Marketing Director"),
        String::from("Team Leader"),
        String::from("People Leader"),
        String::from("Product Engineer"),
        String::from("Software Engineer"),
        String::from("Assistant Software Engineer"),
        String::from("Database Administrator"),
        String::from("Research and Development"),
        String::from("Front-End"),
        String::from("Business Analyst"),
        ];
    let description_cn = vec![
        String::from("项目经理"),
        String::from("产品主管"),
        String::from("运营总监"),
        String::from("市场总监"),
        String::from("团队领导"),
        String::from("人事领导"),
        String::from("运维工程师"),
        String::from("软件工程师(程序猿/媛)"),
        String::from("助理软件工程师"),
        String::from("数据库管理员(程序员身兼数职，一般没有)"),
        String::from("研发工程师"),
        String::from("前端"),
        String::from("业务需求分析师"),
    ];
    let desc_dict:HashMap<_, _> = description_en.iter().zip(description_cn).collect();
    //for循环，注意HashMap 是无序的,这里要使用引用，否则desc_dict变量的所有权被转移给for loop的 desc_dict
    for (en, cn) in &desc_dict {
        println!("[{}] => {} ", en,cn);
    }

    // 所有权示例demo
    let key = String::from("key");
    let val = String::from("value"); // --- move occurs because `val` has type `String`, which does not implement the `Copy` trait
    let mut hm = HashMap::new();
    hm.insert(key, val); //  --- value moved here
    // println!("value insert [{}, {}]",key,val); // value borrowed here after move

    // 值的引用插入HashMap中
    let key = String::from("key");
    let val = String::from("value");
    let mut hm = HashMap::new();
    hm.insert(&key, &val); 
    println!("ref insert [{}, {}]",key,val);
    // get方法
    let my_desc_en = String::from("Software Engineer");
    let my_desc_cn = desc_dict.get(&my_desc_en);
    // 返回值是Option<&T>
    match my_desc_cn {
        Some(desc)=> println!("find key => {}, value => {}",my_desc_en, desc),
        None => {
            println!("not found key: {}",my_desc_en);
        },
    };

    // HashMap插入
    // 已经存在的key更新value
    let  mut name_dict = HashMap::new();
    name_dict.insert(String::from("xiaochun"),18);
    name_dict.insert(String::from("xiaochun"),32);
    println!("key exist {:?}",&name_dict);
    //key存在就不更新，不存在就更新
    
    name_dict.entry(String::from("小川")).or_insert(18);
    name_dict.entry(String::from("xiaochun")).or_insert(33);
    
    println!("Update if the key not exist {:?}",&name_dict);



}
