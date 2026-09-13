# Editörler ve IDE

Rust geliştirmek için iki güçlü seçenek öne çıkar:

RustRover
VS Code + rust-analyzer

İkisiyle de Rust geliştirilebilir. Aralarındaki temel fark, RustRover'ın Rust'a özel bir IDE, VS Code'un ise eklentilerle Rust desteği kazanan genel amaçlı bir kod editörü olmasıdır.

## 1. RustRover

RustRover, JetBrains tarafından Rust geliştirme için hazırlanmış özel bir IDE'dir.

Rust geliştirme ihtiyaçlarının büyük bölümünü doğrudan içerisinde barındırır.

Öne çıkan özellikleri
- Rust ve Cargo entegrasyonu
- Kod tamamlama
- Hata ve uyarı analizi
- Kod navigasyonu
- go to definition
- Refactoring
- Debugger
- Test çalıştırma
- Git entegrasyonu
- Terminal
- Proje ve Cargo.toml yönetimi
- Rust'a özel kod analizi

Örneğin:
```
fn topla(a: i32, b: i32) -> i32 {
a + b
}
```
topla fonksiyonunun başka bir yerde kullanıldığı durumda RustRover üzerinden fonksiyonun tanımına kolayca gidilebilir.

RustRover ne zaman avantajlı?

Rust'ı ciddi şekilde öğrenmek ve daha büyük projeler geliştirmek istediğimizde RustRover'ın IDE yaklaşımı oldukça kullanışlıdır.

Özellikle:
```
Kod
↓
Analiz
↓
Refactoring
↓
Debug
↓
Test
↓
Git
```
gibi geliştirme sürecinin büyük bölümünü tek ortamda yönetmek isteyenler için uygundur.

## 2. VS Code + rust-analyzer

VS Code, genel amaçlı bir kod editörüdür.

Rust desteğini özellikle rust-analyzer eklentisi sağlar.

rust-analyzer, Rust kodunu analiz ederek VS Code'a Rust'a özgü geliştirme özellikleri kazandırır.

- Öne çıkan özellikleri
- Kod tamamlama
- Hata gösterme
- Tür bilgisi
- Kod navigasyonu
- go to definition
- Referans bulma
- Refactoring
- Rust kod analizi
- Cargo desteği
- Test desteği
- Debugger entegrasyonu
- rustfmt entegrasyonu
- clippy entegrasyonu

Örneğin:
```
let isim = String::from("Turan");

println!("{}", isim);
```
rust-analyzer, isim değişkeninin türünü analiz ederek editör içerisinde tür bilgisi ve ilgili kod önerilerini sağlayabilir.

## 3. RustRover ve VS Code Karşılaştırması

| Özellik  |RustRover   | VS Code + rust-analyzer  |
|---|---|---|
|Rust desteği |Doğrudan   |Eklenti ile   |
|Cargo |Var   |Var   |
|Kod Tamamlama |Var   |Var   |
|Kod Analizi |Çok güçlü   |Çok güçlü   |
|Refactoring |Var   |Var   |
|Debugger |Var   |Eklenti/Entegrasyon ile   |
|Git |Var   |Var   |
|Terminal |Var   |Var   |
|Rust' a özel Özellikler |Çok kapsamlı   |rust-analyzer ile kapsamlı   |
|Eklenti İhtiyacı   |Daha az   | Dahafazla|
|Kaynak Tüketimi |Genellikle daha yüksek   | Genellikle daha düşük|
|Genel amaçlı kullanım   |Orta   | Çok güçlü|
|Rust odaklı kullanım   |Çok güçlü   | Çok güçlü|


## 4. Editör mü, IDE mi?

Bu ayrımı bilmek önemlidir.

Kod editörü

Örneğin:

VS Code

Temel olarak kod yazma ve düzenleme ortamıdır.

Çeşitli eklentiler yükleyerek farklı programlama dillerine ve araçlara destek kazandırabiliriz.

IDE

Örneğin:

RustRover

Bir programlama dili veya geliştirme ekosistemi için daha bütünleşik geliştirme ortamı sunar.

Rust açısından:
```
RustRover
│
├── Rust desteği
├── Cargo
├── Debugger
├── Test
├── Git
├── Refactoring
└── Kod analizi
```
gibi araçlar tek bir geliştirme ortamında bir araya gelir.

5. Rust Öğrenirken Hangisini Kullanmalıyım?

İki seçenek de doğrudur.

Ancak burada önemli olan hangi editörün daha popüler olduğu değil, öğrenme sürecinde hangi aracın sana daha az engel çıkardığıdır.

RustRover

Rust'a odaklanmak istiyorsan:
```
RustRover
↓
Rust
↓
Cargo
↓
Debug
↓
Test
↓
Git
```
şeklinde bütünleşik bir ortam sağlar.

VS Code

Zaten VS Code kullanıyorsan:
```
VS Code
+
rust-analyzer
+
Cargo
```
gayet güçlü bir Rust geliştirme ortamıdır.

## 6. Base Practice

Editör seçiminden daha önemli olan, Rust'ın kendi geliştirme araçlarını doğru kullanmaktır.

Temel araçlar:
```
rustup
↓
Rust toolchain
↓
cargo
├── build
├── run
├── test
└── check
↓
rustfmt
↓
clippy
```
Bu nedenle hangi editörü kullanırsak kullanalım şu araçları tanımamız gerekir:
```
cargo check
cargo build
cargo run
cargo test
cargo fmt
cargo clippy
```
Rust öğrenirken editör araçtır; Rust'ın kendisi asıl öğrenme konusudur.