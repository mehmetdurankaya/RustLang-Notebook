# Rust'ta Fonksiyonlar

Fonksiyonlar, belirli bir işi gerçekleştirmek için oluşturulan ve gerektiğinde çağrılabilen kod bloklarıdır.

Rust'ta fonksiyon tanımlamak için `fn` anahtar kelimesi kullanılır.

## 1. Temel Syntax

Rust'ta fonksiyonun temel syntax'ı:

```rust
fn fonksiyon_adi(parametre: Tip) -> DonusTipi {
    // fonksiyon gövdesi
}
```

Örneğin:

```rust
fn selamla() {
    println!("Merhaba Turan!");
}
```

Fonksiyonu çağırmak için:

```rust
fn main() {
    selamla();
}

fn selamla() {
    println!("Merhaba Turan!");
}
```

Burada:

* `fn` → fonksiyon tanımlamak için kullanılır.
* `selamla` → fonksiyonun adı.
* `()` → parametre listesi.
* `{ }` → fonksiyon gövdesi.
* `;` → fonksiyon çağrısının statement olarak tamamlandığını belirtir.

---

## 2. Rust'ta Fonksiyon İsimlendirme

Rust'ta fonksiyon isimleri için `snake_case` kullanılır.

Doğru:

```rust
fn hesapla_toplam() {
}

fn kullanici_getir() {
}

fn dosya_kaydet() {
}
```

Tercih edilmez:

```rust
fn hesaplaToplam() {
}

fn HesaplaToplam() {
}
```

Rust'ın naming convention'ı:

```text
Fonksiyon → snake_case
Struct    → PascalCase
Enum      → PascalCase
Constant  → SCREAMING_SNAKE_CASE
```

---

## 3. Parametreler

Fonksiyon dışarıdan veri alabilir.

```rust
fn selamla(isim: &str) {
    println!("Merhaba {isim}");
}
```

Çağırırken:

```rust
fn main() {
    selamla("Turan");
}
```

Birden fazla parametre:

```rust
fn topla(sayi1: i32, sayi2: i32) {
    println!("{}", sayi1 + sayi2);
}
```

Çağırma:

```rust
fn main() {
    topla(10, 20);
}
```

### Önemli Syntax

Rust'ta parametrenin tipi belirtilmelidir:

```rust
fn topla(sayi1: i32, sayi2: i32) {
}
```

Burada:

```text
sayi1: i32
   │     │
   │     └── parametrenin tipi
   └──────── parametrenin adı
```

Fonksiyon parametrelerinde tip çıkarımı yapılmaz.

Bu nedenle aşağıdaki kullanım geçersizdir:

```rust
fn topla(sayi1, sayi2) {
}
```

---

## 4. Dönüş Değeri

Fonksiyon bir değer döndürebilir.

Syntax:

```rust
fn fonksiyon_adi() -> Tip {
    değer
}
```

Örneğin:

```rust
fn topla(a: i32, b: i32) -> i32 {
    a + b
}
```

Burada:

```text
-> i32
```

fonksiyonun `i32` tipinde bir değer döndüreceğini belirtir.

Çağırılması:

```rust
fn main() {
    let sonuc = topla(10, 20);

    println!("{sonuc}");
}
```

Çıktı:

```text
30
```

---

## 5. Rust'ta Son Expression Dönüş Değeridir

Rust'ta fonksiyonun son expression'ı `return` yazmadan dönüş değeri olabilir.

```rust
fn topla(a: i32, b: i32) -> i32 {
    a + b
}
```

Burada:

```rust
a + b
```

bir expression'dır ve fonksiyonun dönüş değeridir.

### `;` Kullanımına Dikkat

```rust
fn topla(a: i32, b: i32) -> i32 {
    a + b
}
```

Bu doğrudur.

Fakat:

```rust
fn topla(a: i32, b: i32) -> i32 {
    a + b;
}
```

burada `a + b` statement haline gelir ve artık fonksiyonun `i32` dönüş değerini sağlamaz.

Bu nedenle Rust'ta özellikle fonksiyonların sonunda `;` kullanımına dikkat edilmelidir.

---

## 6. `return` Kullanımı

Rust'ta `return` kullanılabilir:

```rust
fn topla(a: i32, b: i32) -> i32 {
    return a + b;
}
```

Bu geçerlidir.

Ancak basit fonksiyonlarda idiomatic Rust genellikle:

```rust
fn topla(a: i32, b: i32) -> i32 {
    a + b
}
```

şeklindedir.

`return` özellikle fonksiyonun ortasında erken çıkış gerektiğinde daha anlamlıdır:

```rust
fn kontrol_et(age: i32) -> bool {
    if age < 0 {
        return false;
    }

    true
}
```

---

## 7. Fonksiyonların Return Tipi Olmayabilir

Bir fonksiyon herhangi bir değer döndürmüyorsa:

```rust
fn selamla() {
    println!("Merhaba!");
}
```

şeklinde yazılır.

Rust'ın arka planda kullandığı dönüş tipi:

```rust
()
```

yani unit type'tır.

Şimdilik bunu:

> Fonksiyon herhangi bir anlamlı değer döndürmüyor.

şeklinde düşünebiliriz.

Açıkça da yazılabilir:

```rust
fn selamla() -> () {
    println!("Merhaba!");
}
```

Ancak genellikle buna gerek yoktur.

Tercih edilen:

```rust
fn selamla() {
}
```

---

## 8. Fonksiyonlar Expression Kullanabilir

Rust'ta birçok yapı expression olduğu için fonksiyon içinde `if` gibi yapılar doğrudan değer üretebilir.

```rust
fn maksimum(a: i32, b: i32) -> i32 {
    if a > b {
        a
    } else {
        b
    }
}
```

Burada `if` bir değer üretir.

Fonksiyonun dönüş değeri:

```text
a > b ise → a
değilse   → b
```

olur.

---

## 9. Scope

Fonksiyonların kendi scope'u vardır.

```rust
fn main() {
    let age = 48;

    println!("{age}");
}
```

`age`, `main` fonksiyonunun scope'u içerisindedir.

Başka bir fonksiyon:

```rust
fn test() {
    println!("{age}");
}
```

şeklinde `age` değişkenine doğrudan erişemez.

```text
main
│
└── age

test
│
└── age'e doğrudan erişemez
```

Bu konu ileride Rust'ın ownership modelini anlamak için önem kazanacaktır.

---

## 10. Fonksiyon Çağrısı

Fonksiyon tanımlamak ile fonksiyonu çalıştırmak farklıdır.

Tanımlama:

```rust
fn topla(a: i32, b: i32) -> i32 {
    a + b
}
```

Çağırma:

```rust
let sonuc = topla(10, 20);
```

Yani:

```text
fn topla(...) { ... }
       ↓
   tanımlama

topla(10, 20)
       ↓
    çağırma
```

---

## 11. Rust'ta Fonksiyon Syntax Özeti

```rust
fn fonksiyon_adi(parametre1: Tip, parametre2: Tip) -> DonusTipi {
    // kod

    sonuc
}
```

Örnek:

```rust
fn hesapla_toplam(sayi1: i32, sayi2: i32) -> i32 {
    sayi1 + sayi2
}
```

Parçaları:

```text
fn
│
├── fonksiyon_adi
│
├── (parametreler)
│       │
│       ├── isim
│       └── tip
│
├── -> DonusTipi
│
└── { fonksiyon gövdesi }
```

---

## 12. Rust Base Practices

Fonksiyon yazarken şu yaklaşımları benimsemek iyi bir Rust pratiğidir:

### İsimlendirme

```rust
fn hesapla_toplam() {}
```

`snake_case` kullan.

### Parametre tiplerini açıkça belirt

```rust
fn topla(a: i32, b: i32) -> i32 {}
```

### Gereksiz `return` kullanma

Tercih:

```rust
fn topla(a: i32, b: i32) -> i32 {
    a + b
}
```

### Son expression özelliğini kullan

```rust
fn kare(x: i32) -> i32 {
    x * x
}
```

### Fonksiyonları mümkün olduğunca tek bir sorumluluğa sahip olacak şekilde tasarla

Örneğin:

```rust
fn hesapla_toplam() {}
fn kullanici_kaydet() {}
fn rapor_olustur() {}
```

gibi fonksiyonlar, çok fazla farklı işi yapan tek bir fonksiyondan genellikle daha anlaşılırdır.

---

## 13. Önemli Rust Fonksiyon Kavramları

Temel fonksiyon syntax'ından sonra ileride şu konularla karşılaşacağız:

```text
Fonksiyonlar
│
├── Parametreler
├── Return values
├── Unit type ()
├── Expression / Statement
├── Scope
├── Ownership
├── Borrowing
├── References
├── Generic functions
├── Closures
├── Methods
├── Associated functions
└── Function pointers
```

Şimdilik özellikle şu dört noktayı sağlam öğrenmek yeterlidir:

```text
1. fn ile fonksiyon tanımlanır.
2. Parametrelerin tipi belirtilir.
3. -> ile dönüş tipi belirtilir.
4. Son expression fonksiyonun dönüş değeri olabilir.
```

### Temel örnek

```rust
fn topla(a: i32, b: i32) -> i32 {
    a + b
}

fn main() {
    let sonuc = topla(10, 20);

    println!("{sonuc}");
}
```

Bu küçük örnekte Rust'ın fonksiyon syntax'ının temel yapı taşlarının tamamını görebiliriz.
