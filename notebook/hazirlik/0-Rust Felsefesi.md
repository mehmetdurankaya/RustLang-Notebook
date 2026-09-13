Rust Felsefesi / Base Practices
# 1. Güvenli kod önce gelir

Rust'ın temel amacı güvenilir ve verimli yazılım üretmektir. Memory safety, özellikle garbage collector olmadan sağlanır; ownership sistemi bu güvenliğin temel mekanizmalarındandır.

Güvenliği sonradan eklemeyiz; kodun tasarımına dahil ederiz.

# 2. Performans için kontrolü kaybetme

Rust, yüksek seviyeli abstraction'ları kullanırken düşük seviyeli performansı korumayı hedefler.

## Zero-cost abstraction:

Kullanmadığın abstraction'ın maliyetini ödeme.

Bu Rust'ın önemli tasarım prensiplerinden biridir.

# 3. Compiler düşman değil, tasarım ortağıdır

Rust compiler'ı yalnızca kodu derleyen bir araç değildir.

Kodun:
- memory safety
- ownership
- borrowing
- type safety
- concurrency

gibi konularda belirlenen kurallara uyup uymadığını kontrol eder.

Rust'ın resmi kitabı bunu compiler'ın bir **gatekeeper** rolü üstlenmesi şeklinde açıklar.

Compiler'a karşı savaşma; compiler'ın neden itiraz ettiğini anlamaya çalış.

# 4. Değişebilirlik açıkça belirtilir

Rust'ta değişkenler varsayılan olarak immutable'dır:
```
let age = 48;
```
Değişmesini istiyorsan:
```
let mut age = 48;
```
Bu nedenle:

Değişebilirlik bilinçli bir tercihtir.

# 5. Ownership temel düşünce modelidir

Rust'ta her değerin ownership ilişkisi vardır.

Temel kurallar:
```
Her değerin bir sahibi vardır.
↓
Aynı anda tek owner vardır.
↓
Owner scope dışına çıkınca değer drop edilir.
```
Bu sistem Rust'ın memory safety yaklaşımının merkezindedir.

Bu nedenle Rust öğrenirken ownership'i yalnızca bir syntax konusu olarak değil, program tasarlama biçimi olarak ele alacağız.

# 6. Idiomatic Rust yaz

Rust'ta yalnızca:

"Kod çalışıyor mu?"

diye bakmayacağız.

Aynı zamanda:

"Bu Rust'ta doğal ve idiomatic bir kullanım mı?"

diye bakacağız.

Örneğin:
```
let first_name = "Turan";
```
tercih edilir.
```
let firstName = "Turan";
```
çalışabilecek olsa da Rust naming convention'ına uygun değildir.

Genel olarak:
```
variable / function → snake_case
type / struct / enum → PascalCase
constant             → SCREAMING_SNAKE_CASE
```
# 7. Gereksiz abstraction oluşturma

Rust güçlü abstraction'lar sağlar ama:

Sırf abstraction kullanmış olmak için abstraction oluşturma.

Önce problemi basit şekilde çöz.

Sonra gerçekten ihtiyaç varsa:
```
function
↓
struct
↓
trait
↓
generic / abstraction
```
gibi yapıları kullan.

# 8. Standart araçları kullan

Rust ekosisteminde araçların kendisi de Base Practice'in parçasıdır:
```
cargo fmt
cargo clippy
cargo test
cargo check
```
Özellikle:
```
cargo fmt
```
standart formatting'i,
```
cargo clippy
```
ise daha idiomatic Rust yazmana yardımcı olur.
