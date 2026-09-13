## Rust'ta temel coding conventions
### 1. İsimlendirme kuralları

Rust'ın standart isimlendirme yaklaşımı oldukça nettir:

| Öğe          | Rust'ta tercih                        |
| ------------ | ------------------------------------- |
| Değişken     | `snake_case`                          |
| Fonksiyon    | `snake_case`                          |
| Metot        | `snake_case`                          |
| Modül        | `snake_case`                          |
| Constant     | `SCREAMING_SNAKE_CASE`                |
| Static       | `SCREAMING_SNAKE_CASE`                |
| Struct       | `PascalCase`                          |
| Enum         | `PascalCase`                          |
| Trait        | `PascalCase`                          |
| Type alias   | `PascalCase`                          |
| Generic type | Genellikle `T`, `TKey`, `TValue` gibi |

Örneğin:
```
let first_name = "Turan";
let user_age = 48;

fn calculate_total() {
}

struct UserAccount {
}

enum UserRole {
Admin,
User,
}

const MAX_RETRY_COUNT: u32 = 3;
```
Rust'ta camelCase kullanmak genel convention değildir.

Yani:
```
let firstName = "Turan"; // ❌
let first_name = "Turan"; // ✅
```
15. Rust'ın genel felsefesi

Şimdilik aklımızda şu prensipler olsun:
``` 
Rust
 │
 ├── Açık ve anlaşılır kod
 │
 ├── Varsayılan olarak immutable
 │
 ├── Değişebilirlik → açıkça mut
 │
 ├── snake_case → değişken/fonksiyon
 │
 ├── PascalCase → type/struct/enum/trait
 │
 ├── SCREAMING_SNAKE_CASE → constant
 │
 ├── Compiler → ciddi şekilde kullanılır
 │
 ├── cargo fmt → standart formatting
 │
 ├── clippy → idiomatic Rust
 │
 ├── expression → değer üretir
 │
 └── Gereksiz koddan kaçınma

``` 