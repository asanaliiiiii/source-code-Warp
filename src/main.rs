// src/main.rs
use std::env;
use std::fs;

// Подключаем модуль sysinfo, который мы настраивали ранее
mod big_functions;

fn main() {
    // Собираем аргументы командной строки
    let args: Vec<String> = env::args().collect();
    
    // Если файл не указан, выводим красивую профессиональную справку
    if args.len() < 2 {
        println!("⚡ Warp Language Runtime v1.0.0");
        println!("Usage: warp <filename.warp>");
        println!("\nExample: cargo run -- studio.warp");
        return;
    }

    // Берём путь к файлу (первый аргумент после имени программы)
    let file_path = &args[1];

    // Пробуем прочитать файл с кодом Warp
    match fs::read_to_string(file_path) {
        Ok(code) => {
            // Файл успешно прочитан, запускаем парсинг и выполнение
            execute_warp_code(&code);
        }
        Err(_) => {
            // Если файла нет или к нему нет доступа, выводим строгую ошибку
            println!("Error: Could not open or read file '{}'", file_path);
        }
    }
}

// Функция, которая будет отвечать за разбор и выполнение твоего языка
fn execute_warp_code(code: &str) {
    // Пока что выведем отладочную информацию (потом этот блок можно убрать)
    println!("--- [Warp Interpreter Execution] ---");
    
    // Пример вызова системной функции, которую мы фиксили через sysinfo
    let sys_specs = big_functions::sysinfo::fetch_system_specs();
    println!("{}", sys_specs);
    
    println!("------------------------------------");
    
    // TODO: Сюда вставляется вызов твоего лексера, парсера и связки HTML + Tkinter
    // Например:
    // let mut parser = WarpParser::new(code);
    // parser.interpret();
}