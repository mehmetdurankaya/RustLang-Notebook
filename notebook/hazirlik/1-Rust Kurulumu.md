## 1. Rust kurulumu: rustup

Rust'u doğrudan tek bir program olarak düşünme. rustup Rust araç zincirini yönetir.

Kurulumdan sonra temel olarak şunlara sahip olursun:
```
rustup
│
├── rustc      → Rust kodunu derler
├── cargo      → Projeyi yönetir
├── rustfmt    → Kodu biçimlendirir
└── clippy     → Kod kalitesini kontrol eder
```
Windows'ta kurulumu yaptıktan sonra kontrol:
```
rustc --version
cargo --version
rustup --version
```
Örneğin:
```
rustc 1.xx.x
cargo 1.xx.x
rustup 1.xx.x
```
## 2. rustc nedir?

rustc, Rust compiler yani Rust derleyicisidir.

Örneğin:
```
fn main() {
println!("Merhaba Rust!");
}
```
Bunu rustc derler.

Kabaca:
```
Rust kodu
↓
rustc
↓
çalıştırılabilir program
```
Ama gerçek projelerde genellikle doğrudan rustc kullanmayacağız.

Burada Cargo devreye giriyor.

## 3. Cargo nedir?

Cargo'yu Rust'ın proje yöneticisi olarak düşünebilirsin.

Şunları yapar:

- proje oluşturur
- derleme yapar
- çalıştırır
- bağımlılıkları yönetir
- testleri çalıştırır
- kodu formatlar
- bazı kalite kontrollerini yapar

Örneğin:
```
cargo new hello_world
```
Bu komut yeni bir Rust projesi oluşturur.

Ortaya kabaca:
```
hello_world/
│
├── Cargo.toml
└── src/
└── main.rs
```
çıkar.

main.rs

Programımız:
```
fn main() {
println!("Hello, world!");
}
```
Buradaki:
```
fn main()
```
programın başlangıç noktasıdır.

## 4. Cargo.toml nedir?

Bunu daha önce kullandığın bazı proje yapılandırmalarına benzetebilirsin.

Örneğin .NET'te:
```
.csproj
```
Node.js'te:
```
package.json
```
Rust'ta:
```
Cargo.toml
```
vardır.

Burada projenin adı, sürümü, bağımlılıkları vb. tutulur.

## 5. cargo run

Projeye gir:
```
cd hello_world
```
Sonra:
```
cargo run
```
Cargo kabaca şunu yapar:
```
Cargo
↓
Projeyi bul
↓
Derle
↓
Programı çalıştır
```
Sonuç:
```
Hello, world!
```
Burada önemli nokta:

Rust projelerinde günlük kullanımda rustc yerine çoğunlukla Cargo kullanacağız.

## 6. rustfmt ve clippy

Bunları şimdilik sadece görevleriyle bilmen yeterli.

**rustfmt**

Kodun biçimini düzeltir:
```
cargo fmt
```
Örneğin kötü biçimlendirilmiş kodu Rust'ın standart biçimine getirir.

**Clippy**

Kodunu inceler ve daha iyi Rust yazman için önerilerde bulunur:

**cargo clippy**

Bunu ileride özellikle Effective Rust öğrenirken önemseyeceğiz.

## 7. VS Code + rust-analyzer

VS Code'a:

rust-analyzer

eklenti­sini kurmanı öneririm.

Bu sana:

- hata gösterme
- otomatik tamamlama
- tip bilgisi
- kod navigasyonu
- bazı Rust analizleri

sağlar.

Yani:
```
VS Code 
+ 
rust-analyzer
↓
Rust geliştirme ortamı
```
## Şimdilik aklında kalması gereken 5 şey
- rustup  → Rust araçlarını yönetir
- rustc   → Rust derleyicisi
- cargo   → Rust proje yöneticisi
- rustfmt → Kodu biçimlendirir
- clippy  → Kod kalitesini analiz eder

Ve ilk çalışma akışımız:
```
cargo new hello_world
cd hello_world
cargo run
```