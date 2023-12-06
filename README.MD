# TODO:有时间就补充
[学习链接]
- [rust编译器安装](https://www.rust-lang.org/tools/install)
- [rust std标准库文档](https://doc.rust-lang.org/std/index.html)
- [rust 文档](https://doc.rust-lang.org/rustdoc/index.html)

### 变量与可变性
- 声明变量使用`let`关键字
- 默认情况下，变量是不可变的(immutable)
### 变量与常量
- 常量(constant),常量在绑定值以后也是不可变的，但是它与不可变的变量有很多区别：
- 不可以使用mut，常量永远都是不可变的
- 声明常量使用const关键字,它的类型必须被标注
- 常量可以在任何作用域内进行声明，包括全局作用域
- 常量只可以绑定到常量表达式，无法绑定到函数的调用结果或只能在运行时才能计算出的值
- 在程序运行期间，常量在其声明的作用域内一直有效
- 命名规范：Rust里常量使用全大写字母，每个单词之间用下划线分开。

#### 所有权规则
- 每一值都有一个变量，这个变量是该值的所有者
- 每个值同时只能有一个所有者
-当所有者超出作用域时，该值将被删除。{}代表一个作用域，cpp一样


#### String类型
```rust
fn main() {
    let mut str = String::from("Hello, world!");
    str.push_str(" ^=^ ");
    println!("{:?}", str);
} //  String drop函数
```
### 常用的集合
- Vector
    - Vec<T>，叫vector向量，底层是数组实现，注意扩容时数据迁移
        - 标准库提供
        - 数组，可以存储多个元素
        - 只能存储相同类型的数据
        - 元素在内存中连续存放
    - 创建Vector
        - Vec::new 函数
        - 使用初始值创建Vec<T>，使用vec!宏
    - 更新Vector
        - 向vector中添加元素 push
    - 删除vector
        - 和其它struct一样 当vec离开作用域后
        - vector对象就会清理掉
        - vector对象所有的元素就会被清理掉
    - 读取vector的元素
        - 两种方式可以引用vector里的值
        - 索引
        - get方法
    - 所有权和借用规则
        - 不能在同一作用域内同时拥有可变和不可变引用
    - 使用enum来存储多种数据类型
        - enum的变体可以附加不同类型的数据
        - enum的变体定义在同一个enum类型下
        示例参考:vector目录

- String
    - rust开发者被字符串困扰的原因
    - Rust倾向于暴露可能的错误
    - 字符串数据结构复杂
    - UTF-8
    - 字符串是Byte的集合，提供了操作的方法，能将byte解析为文本
    - 在Rust的核心语言层面，只有一个字符串类型：字符串切片str(或&str)
    - 字符串切片：对存储在其它地方，UTF-8编码的字符串的引用
        - 字符串字面值：存储在二进制文件中，也是字符串切片
    - String类型：
        - 来自标准库而不是核心语言
        - 可增长，可修改，可拥有
        - UTF-8 编码
        - 标准库其他字符串类型:OsString OsStr CString CRStr
        - String类和str相比，str是拥有或借用的变体
        - 可存储不同编码的文本或在内存中以不同的形式展现
        - 第三方库，Library crate针对存储字符串可提供更多的选项
    -创建字符串(String)
        - String::new函数
        - 使用初始值来创建String
            - to_string()方法，可用于实现Display trait的类型，包括字符串字面值
            - String::from()函数，从字面值创建String
        - 更新String
            - push_str()方法，把一个字符串切片附加到String
            - push()方法，把单个字符附加到String
            - +:连接字符串
                - 使用了类似fn add(self,s:&str) -> String {...}的签名方法
                    - 标准库中的add方法使用了泛型
                    - 只能把&str添加到String
                    - 解引用强制转换(deref coercion)
            - format!：连接多个字符串
        - 对String按索引的形式进行访问
            - 按索引语法访问String的某部分，会报错
            - Rust的字符串不支持索引语法访问
            - String是对Vec<u8>的封装
            - String的len()方法返回字符串的长度
            - 字节（bytes()方法）、标量值(chars()方法)、字形簇(第三分库) 
            - Bytes,Scalar values, Grapheme Clusters
            - Rust不允许对String进行索引的原因是
                - 索引操作应消耗一个常量时间(O(1))
                - 而String无法保证：需要遍历的所有内容，来确定有多少个合法的字符
            - 切割String
                - 可以使用[]和一个范围来创建字符串的切片，
                    - TODO:注意切片如果跨越了字符边界，程序会panic
- HashMap
    - 键值对的形式存储数据，一个键(key)，对应一个值(value)
    - hash 函数:决定如何在内存中存放K和V
    - 使用场景：通过K(任意类型)来查找数据，而不是通过索引
    - 创建HashMap
        - 创建空HashMap::new() 函数
        - 添加数据使用insert()方法
        - hashmap用的较少，不在preclude中
        - 标注库对其支持较少，没有内置的宏来创建hashmap
        - 数据存储在heap上
        - HashMap是同构的，所有的k必须是同一种类型，所有的value必须是同一种类型 `let hm:HashMap<String, i32> = HashMap::new();`
    - 特定场景下创建HashMap, collect()方法
        - 在元素类型为Tuple的Vector上使用collect()方法，可以组件一个HashMap
            - 要求Tuple有两个值：一个作为K，一个作为V
            - collect()方法可以把数据整合成很多种集合类型，包括HashMap
                - 返回值需要显示指明类型
            - HashMap和所有权
                - 对于实现了Copy trait的类型(基础类型都实现了 如i32)，值会被复制到HashMap中
                - 对于拥有所有权的值(String)，值会被移动，所有权会转移给HashMap
                - 若将值的引用插入到HashMap中值本身不会移动
                - 在HashMap有效的期间，被引用的值必须保持有效
            - 访问HashMap中的值
                - get方法 
                    - 参数是key
                    - 返回值Option<&value>
            - 遍历HashMap
                - for循环
            - 更新HashMap<key,value>
                - HashMap大小可变
                - 每一个key同时只能对应一个value
                - 更新HashMap中的数据
                    - key已经存在，对应一个value
                        - 替换现有的value
                            - 若向HashMap插入一对key-value，然后在插入同样的key,但是value不同，原来的value会被替换掉,使用`insert()方法`
                        - 保留现有的value,忽略新的value
                            - 只在key不对应任何值的情况下，才插入value
                                - `entry()方法`检查指定的key是否对应一个value
                                    - 参数为key
                                    - 返回enum Entry 代表值是否存在
                                - Entry的or_insert()方法
                                - 返回
                                    - 若key存在，返回对应的value的一个可变引用
                                    - 若value不存在，将方法参数作为key的新值插入进去，返回这个值的可变引用
                        - 合并现有的value和新的value，例如统计单词出现的次数
                    - key 不存在
                        - 添加一个key，value
                    - hash函数
                        - 默认情况下HashMap使用加密功能强大的Hash函数，可以抵抗拒绝服务(Dos)攻击
                        - 不是可用的最快的hash算法
                        - 但是具有更好的安全性
                        - 可以指定不同的hasher来切到另一个函数
                            - hasher是实现BuildHasher trait的类型
#### Rust错误处理
- Rust的可靠性：错误处理
        - 大部分情况下，在编译时提示错误，并处理
    - 错误的分类
        - 可恢复
            - 例如文件未找到，可再次尝试
        - 不可恢复
            - bug 例如访问的索引超出范围

    - Rust没有类似异常的机制
            - 可恢复错误 Result<T, E>
            - 不可恢复 panic!宏
    - 不可恢复的错误和panic!
    - 当panic!宏执行
        - 你的程序会打印一个错误信息
        - 展开(unwind),清理调用栈(stack)
        - 退出程序
    - 为应对panic 展开或终止(abort)调用栈
    - 默认情况下，当panic发生
        - 程序展开调用栈(工作量大)
            - Rust沿着调用栈往回走，栈展开
            - 清理每一个遇到的函数中的数据
        - 或立即终止调用栈
            - 不进行清理操作，直接停止程序
            - 内存需要OS进行清理
        - 想要二进制文件更小，把设置从展开改为终止
            - Cargo.toml中的profile部分设置
            ```toml
            [profile.release]
            panic = 'abort'
            ```
        - 通过设置环境变量RUST_BACKTRACE=0来查看调用栈的信息
        - Result枚举
            - enum Result<T,E> {
                Ok(T),
                Err(E),
            }
            - T 是操作成功情况下，Ok变体里返回的数据类型
            - E 操作失败情况下，Err变体里返回的错误的类型
        - 出来Result的一种方式match 表达式
        - 和Option枚举一样Result及其变体也是由prelude带入作用域
        - 匹配不同的错误
        - match匹配很好用，处理比较原始，啰嗦
        - 闭包 closure，Result<T,E>有很多方法
            - 他们接收闭包作为参数
            - 使用match实现,
            - 使用这些方法会让业务代码更加简洁。
        - unwrap
            - unwrap: match表达式的一个快捷方法
            - 若Result结果是Ok,返回Ok里面的值，就是上面的T类型值
            - 若Result结果是Err，调用panic!宏
        - expect
            - expect 和unwrap类似，但可指定错误信息.
        - 传播错误
            - 在函数出处理错误
            - 将错误返回给调用者
        - ?运算符
            - ?运算符传播错误的一种快捷方式
                - 若Result是Ok，Ok中的值就是表达式的结果，然后继续执行程序
                - 若Result是Err,Err就是整个函数的返回值，就像使用了return
        - ?与from函数
            - trait std::convert::From上的from函数
                - 用于错误之间的转换
                - 被? 所应用的错误，会隐士的被from函数处理
                - 当?调用from函数时
                    - 它所接收的错误类型会被转换为当前函数返回类型所定义的错误类型
                - 用于 针对不同错误原因，返回同一个错误类型
                    - 只要每一个错误类型实现了转换为所返回的错误类型的from函数
        - 可以使用链式调用简化代码
        ```rust
        use std::fs::File;
        use std::io;
        use std::io::Read;
        fn main() {
            let r = read_file();
        }
        fn read_file() -> Result<String,io::Error> {
            let mut s = String::new();
            File::open("filepath")?.read_to_string(&mut s)?;
            Ok(s)
        }
        ```
        - ?运算符只能用于返回Result或Option的函数
        ```rust
        use std::fs::File;
        use std::error::Error;
        // Box<dyn Error>是一个trait对象
        // 简单理解 任何可能得错误类型
        fn main() -> Result<(),Box<dyn Error>> {
             let f = File::open("hello.txt")?;
             Ok(())
        }
        ```

    - 什么时候使用panic
        - 编写示例，原型代码，测试代码
        - 可以使用panic!
            - 演示Demo unwrap
            - 原型代码 unwrap expect
            - 测试 unwrap expect

#### 泛型

#### trait

#### 生命周期
- Rust的每一个引用都有自己的生命周期
- 生命周期:引用保持有效的作用域
- 大多数情况：生命周期是隐式的、可被推断的
- 当引用的生命周期可能以不同的方式互相关联时:手动标注生命周期
- 生命周期-避免悬垂引用(dangling reference)
- 生命周期的主要目标：避免悬垂引用
```rust
fn main() {
    let r; // 声明未初始化
    //`r` used here but it is possibly-uninitialized
    let y = r; // error  r 未初始化，无法赋值
    {
        //作用域
        let x = 5;
        r = &x; // error  borrowed value does not live long enough
    }
    println!("{}",r);
}

```
- 借用检查器
- Rust编译器的借用检查器:比较作用域来判断所有的借用是否合法
- 生命周期标注语法
    - 生命周期的标注不会改变引用的生命周期长度
    - 当指定了泛型生命周期参数，函数可以接收带有任何生命周期的引用
    - 生命周期的标注：描述了多个引用的生命周期的关系，但不影响生命周期

- 生命周期参数名
    - 以‘开头
    - 通常全小写且非常短
    - 很多人使用’a
- 生命周期标注的位置
    - 在引用的&符号后
    - 使用空格将标注和引用类型分开
    ```rust
    &i32 // 引用
    &‘a i32 // 带有显式生命周期的引用
    &’a mut i32 // 带有显式生命周期的可变引用
    // 单个生命周期标注本身没有意义
    ```
#### 函数签名中的生命周期标注
- 泛型生命周期参数声明在：函数名和参数列表之间的<>里
- 生命周期‘a的实际生命周期是x和y两个生命周期中较小的那个
#### 深入理解生命周期
- 指定生命周期参数的方式依赖于函数所做的事情
- 从函数返回引用时，返回类型的生命周期参数需要与其中一个参数的生命周期匹配
- 注意不要返回函数内部的变量的引用,要使用值传递转移所有权。C++中是返回函数内部变量的指针，这些都是会有悬垂指针，Rust通过编译器生命周期检查编译不通过，而C++会导致运行时未定义的行为。(乱码 crash等)

#### Struct定义中的生命周期标注
- Struct里可包括
    - 自持有的类型
    - 引用：需要在每个引用上添加生命周期标注
#### 生命周期的省略
- 每一个引用都有生命周期
- 需要为使用生命周期的函数或struct指定生命周期参数
###### 生命周期省略规则
- 在Rust引用分析中所编入的模式称为生命周期省略规则
- 这些规则无需开发者来遵守
- 它们是一些特殊情况，由编译器来考虑
- 如果你的代码符合这些情况，那么无需显式标注生命周期
- 生命周期省略规则不会提供完整的推断
- 如果应用规则后，引用的生命周期任然模糊不清***编译错误***
- 解决办法：添加生命周期标注，表明引用之间的相互关系
###### 输入、输出生命周期
- 生命周期子啊
    - 函数/方法的参数：输入生命周期
    - 函数/方法的返回值：输出生命周期

##### 生命周期省略的三个规则
- 编译器使用3个柜子在没有显式标注生命周期的情况下，来确定引用的生命周期
- 规则1应用与输入生命周期
- 规则2,3应用于输出生命周期
- 如果编译器应用完3个规则之后，仍然有无法确定生命周期的引用就编译**报错**
- 这些规则适用于fn定义和impl块
- **规则1** 每个引用类型的参数都有自己的生命周期
- **规则2** 如果只有1个输入生命周期参数，那么该生命周期被赋所有的输出生命周期参数
- **规则3** 如果有多个输入生命周期参数，但其中一个是&self 或 &mut self(是方法),那么self的生命周期会被赋给所有的输出生命周期参数
###### 方法定义中的生命周期标注
- 在struct上使用生命周期实现方法，语法和泛型参数的语法一样
- 在哪神功和使用生命周期参数，依赖于：
    - 生命周期参数是否和字段，方法的参数或返回值有关
- struct 字段的生命周期名
    - 在impl后声明
    - 在struct名后使用
    - 这些生命周期是struct类型的一部分
- impl块内的方法签名中
    - 引用必须绑定与struct字段引用的生命周期，或者引用是独立的也可以
    - 生命周期省略规则经常使得方法中的生命周期标注不是必须的

##### 静态生命周期
- ’static是一个特殊的生命周期，整个程序的持续时间
- 例如 所有的字符串字面值都拥有‘static生命周期
```rust
let s &'static str = "hello world!";
```
- 为引用指定’static生命周期前要三思
- 是否需要引用在程序整个生命周期内都存活
##### 泛型参数类型、trait bound 生命周期