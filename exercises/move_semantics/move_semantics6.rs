// move_semantics6.rs
//
// You can't change anything except adding or removing references.
//
// Execute `rustlings hint move_semantics6` or use the `hint` watch subcommand
// for a hint.



fn main() {
    let data = "Rust is great!".to_string();

    get_char(&data); // 传递一个引用给 `get_char`

    string_uppercase(&data); // 传递一个引用给 `string_uppercase`
}

// 应该不获取所有权，使用引用传递
fn get_char(data: &String) -> char {
    data.chars().last().unwrap()
}

// 应该获取可变引用来修改内容
fn string_uppercase(data: &String) {
    let data_upper = data.to_uppercase(); // 获取转换后的新字符串
    println!("{}", data_upper);  // 输出转换后的字符串
}
