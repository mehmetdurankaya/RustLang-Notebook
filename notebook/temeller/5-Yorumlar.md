## Yorumlar

Rust'ta yorumlar, kodun çalışmasını etkilemeyen ve geliştiriciye açıklama sağlamak için kullanılan metinlerdir.

### 1. Tek Satırlı Yorum: `//`

`//` ile başlayan bölüm, satırın sonuna kadar yorum olarak kabul edilir.

```rust
// Bu bir yorumdur

let age = 48;

Kodun yanında da kullanılabilir:
let age = 48; // Kullanıcının yaşı
```
## 2. Çok Satırlı Yorum: /* */

Birden fazla satırı yorum haline getirmek için kullanılır.
```
/*
    Bu birden fazla
    satırlı yorumdur.
*/

let age = 48;
````
## 3. Dokümantasyon Yorumu: ///
````
///, Rust'ta kodun dokümantasyonunu oluşturmak için kullanılır.

Genellikle fonksiyon, struct, enum, trait gibi yapıların ne yaptığını açıklamak için kullanılır.

/// İki sayıyı toplar ve sonucu döndürür.
fn topla(a: i32, b: i32) -> i32 {
    a + b
}
````
Rust'ın dokümantasyon araçları bu açıklamaları kullanarak otomatik dokümantasyon oluşturabilir.

## 4. // ve /// Arasındaki Fark
Yazım	Kullanım
````
//	Normal kod açıklaması
/* */	Çok satırlı normal yorum
///	Dokümantasyon yorumu
Base Practice

Normal kod açıklaması için:

// Kullanıcının yaşı
let age = 48;

Bir fonksiyonun veya başka bir Rust öğesinin ne yaptığını açıklamak için:

/// İki sayıyı toplar.
fn topla(a: i32, b: i32) -> i32 {
    a + b
}
````