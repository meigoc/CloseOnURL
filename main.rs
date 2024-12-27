use CloseOnURL::close_on_url;

fn main() {
    let browser = "chrome";  // браузер для отслеживания
    let target_url = "google.com";  // URL для отслеживания

    println!("Starting to monitor browser for URL: {}", target_url);
    close_on_url(browser, target_url);  // Вызываем функцию из библиотеки
}
