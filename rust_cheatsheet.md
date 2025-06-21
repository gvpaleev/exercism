# Шпаргалка по Rust (на основе Exercism)

## 1. Основные конструкции
```rust
fn main() { // Объявление функции
    let x = 5; // Неизменяемая переменная
    let mut y = 10; // Изменяемая переменная
    pub fn public_func() {} // Публичная функция
}
```
*Комментарий:* В Rust переменные по умолчанию неизменяемы - это предотвращает случайные изменения.

## 2. Работа со строками
```rust
let s1 = String::from("hello"); // String (владеющая)
let s2 = "world"; // &str (срез)
let chars: Vec<char> = s1.chars().collect(); // Итерация по символам
```
*Комментарий:* String выделяет память в куче, &str - это только представление.

## 3. Время и даты
```rust
use std::time::Duration;
use chrono::{DateTime, Utc};

let gigasecond = Duration::from_secs(1_000_000_000);
let dt: DateTime<Utc> = Utc::now() + gigasecond;
```
*Комментарий:* Для сложных операций с датами используйте крейт `chrono`.

## 4. Тестирование
```rust
#[test]
fn test_reverse() {
    assert_eq!(reverse("hello"), "olleh");
}
```
*Комментарий:* Тесты пишутся рядом с кодом или в tests/.

## 5. Сложные концепции
### Владение (Ownership)
- У каждого значения есть владелец
- При присваивании владение передается
- Владелец отвечает за освобождение памяти

### Заимствование (Borrowing)
```rust
let s = String::from("hello");
let len = calculate_length(&s); // Передача ссылки
```
*Комментарий:* Ссылки бывают изменяемые (&mut) и неизменяемые (&).

## 6. Полезные идиомы
- `match` для pattern matching
- `unwrap()` и `expect()` для работы с Option/Result
- `?` оператор для обработки ошибок
