
### Derleme
- Kaynak Kodu
- Derlenerek obje dosyasına dönüştürülür
- Obje dosyaları Linker ile birleştirilir
- Çalıştırılabilir dosyalara dönüştürülür



### Rust Uygulaması Çalıştırma main() fonksiyonu
- her çalıştırılabilir Rust programı main fonksiyonundan başlar.
- Bu, C/C++/Go gibi dillerle aynı gelenektir. Ama Rust'a özgü bazı detaylar var.
```rust
fn main() {
    println!("Merhaba, dünya!");
}
```

```rust
fn → fonksiyon tanımı
main → özel isim, giriş noktası (entry point)
() → parametre yok
{ } → gövde
```

|Özellik   |Açıklama   |
|---|---|
|  Zorunlu |Binary(Çalıştırılabilir) create'lerde zorunda   |
| Bezersiz  |  Bir programda sadece bir main olabilir. |
| Görünürlük  | pub yazılmaz --dışarıdan çağrılmaz.  |
|  Parametre almaz | Klasik kullanımda parametresizdir.  |
| Özel dönüş tipi  | () veya Result<(),E> olabilir  |
** Kütüphane (librarry) createlerde main yoktur.Sadece çalıştırılabilir
programlarda olur.

### main'in Farklı İmzaları
- Klasik — Parametresiz, Dönüşsüz
```rust
fn main() {
println!("Standart main");
}
```
- Result Döndüren main (Hata Yönetimi İçin) ⭐
```rust
use std::fs::File;

fn main() -> Result<(), std::io::Error> {
let _dosya = File::open("veri.txt")?;  // ? operatörü kullanılabilir
println!("Dosya açıldı");
Ok(())
}
```
### Neden önemli?

- ? operatörünü main içinde kullanmanızı sağlar
- Hata olursa program Err(...) döndürür ve Debug formatında hata mesajı yazdırır
- unwrap() ve expect() ile kod şişirmekten kurtarır

Örnek çıktı (hata durumunda):
```text
Error: Os { code: 2, kind: NotFound, message: "No such file or directory" }
```
### Termination Trait'i Uygulayan Tipler
main, Termination trait'ini implement eden herhangi bir tipi döndürebilir.
() ve Result<T, E> en yaygın olanlardır.
```
rust
use std::process::ExitCode;

fn main() -> ExitCode {
// Başarılı: ExitCode::SUCCESS (0)
// Hatalı:  ExitCode::FAILURE (1)
ExitCode::SUCCESS
}
```
Bu, OS'a dönüş kodu vermek için kullanışlıdır (CI/CD, shell script'leri için).
### Komut Satırı Argümanları
   main'in klasik imzası parametre almaz, ama argümanlara iki yolla erişirsiniz:
 - std::env ile (basit)

```
rust
use std::env;

fn main() {
let args: Vec<String> = env::args().collect();
println!("Program adı: {}", args[0]);
if args.len() > 1 {
println!("İlk argüman: {}", args[1]);
}
}
```
### clap ile (profesyonel) ⭐
``` 
rust
use clap::Parser;

#[derive(Parser)]
struct Args {
/// Girdi dosyası
#[arg(short, long)]
input: String,

    /// Ayrıntılı mod
    #[arg(short, long, default_value_t = false)]
    verbose: bool,
}

fn main() {
let args = Args::parse();
println!("Girdi: {}", args.input);
}
```
```
bash
./program --input veri.txt --verbose
Not: main kendisi parametre almaz; argümanları env::args() veya clap gibi araçlarla okursunuz.
```
### main ve Async (Asenkron) — Çok Önemli ⭐
   Rust'ta main async olamaz. Şu kod derlenmez:
```
rust
async fn main() {  // ❌ HATA
// ...
}
```
Çözüm: #[tokio::main] (veya async-std, smol) makrosu:

```
rust
#[tokio::main]
async fn main() {
let sonuc = async_islem().await;
println!("{}", sonuc);
}

async fn async_islem() -> String {
"Bitti".to_string()
}
```
Bu makro, aslında arka planda şunu üretir:
``` 
rust
fn main() {
tokio::runtime::Runtime::new()
.unwrap()
.block_on(async { /* senin kodun */ });
}
Tauri, Axum, Actix gibi framework'lerde bu yapıyı çok görürsünüz.
```
### [tokio::main] Benzeri Örnekler
```   
 Framework	Makro
   Tokio	#[tokio::main]
   async-std	#[async_std::main]
   smol	#[smol::main]
   Tauri	Genelde #[tokio::main]
 ```
### main Öncesi ve Sonrası Çalışan Şeyler
   main'den önce çalışan kodlar:

- Global statikler: static ve lazy_static / OnceLock
- ctor crate'i: main'den önce çalışacak fonksiyonlar tanımlar (nadir)
- Rust runtime: Stack kurulumu, panic handler
- main'den sonra çalışan kodlar:
- Drop implementasyonları: main sonunda scope'tan çıkan değişkenler
- atexit benzeri: Genelde Rust otomatik halleder

```
rust
struct Temizleyici;
impl Drop for Temizleyici {
fn drop(&mut self) {
println!("Temizlendi (main sonrası)");
}
}

fn main() {
let _t = Temizleyici;
println!("main içi");
}
// Çıktı:
// main içi
// Temizlendi (main sonrası)
```