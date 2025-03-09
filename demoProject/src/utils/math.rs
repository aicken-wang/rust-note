pub fn add(a: i32, b: i32) -> i32 {
    a + b
}
// 模块级别的函数, 只能在模块内部调用
pub(crate) fn sub(a: i32, b: i32) -> i32 {
    a - b
}
pub fn mul(a: i32, b: i32) -> i32 {
    a * b
}
pub fn div(a: i32, b: i32) -> i32 {
    a / b
}
