// Сохраните в файл `src/main.rs` в новом проекте cargo (cargo new big_bob)
// Запуск: cargo run

fn main() {
    // ANSI-коды для цвета (работает в большинстве терминалов)
    let red = "\x1b[31m";
    let yellow = "\x1b[33m";
    let bold = "\x1b[1m";
    let reset = "\x1b[0m";

    // Небольшой ASCII-арт-баннер
    let banner = r#"
  ____                 _     _    ____  _     
 |  _ \ ___  __ _  ___| |__ | |  | __ )| |    
 | |_) / _ \/ _` |/ __| '_ \| |  |  _ \| |    
 |  _ <  __/ (_| | (__| | | | |  | |_) | |___ 
 |_| \_\___|\__,_|\___|_| |_|_|  |____/|_____|
"#;

    println!("{}{}{}{}", bold, yellow, banner, reset);

    // Главная строка
    println!("{}{}орешки{} {}{}биг{} {}{}боб{}",
        bold, red, reset,   // "орешки" красным и жирно
        bold, yellow, reset, // "биг" желтым и жирно
        bold, red, reset    // "боб" снова красным и жирно
    );

    // Немного «флерта»: рамка
    let line = "—".repeat(28);
    println!("\n{}{}{}{}", bold, line, reset, "\n");
}
