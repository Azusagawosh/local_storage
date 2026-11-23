use std::collections::HashMap;
use std::io;
 

pub fn create(my_map: &mut HashMap<String, String>) {
    println!("введите ключ:");
    let mut key = String::new();
    io::stdin().read_line(&mut key).expect("Err");

    println!("введите значение:");
    let mut value = String::new();
    io::stdin().read_line(&mut value).expect("Err");
    my_map.insert(key.trim().to_string(), value.trim().to_string());
    println!("Добавлено: {} -> {}", key.trim().to_string(), value.trim().to_string());
}

pub fn delete(my_map: &mut HashMap<String, String>) {
    println!("Выберите действие 0 удалить ключ");
    let del = 0;
    io::stdin().read_line(&del).expect("Err");
    my_map.remove()
}
