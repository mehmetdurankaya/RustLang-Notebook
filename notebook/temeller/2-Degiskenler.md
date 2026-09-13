## Değişken isimlendirme kuralları

Rust'ta değişken isimleri için temel kurallar:
```rust 
let user_name = "Turan";
let age = 48;
let total_price = 150.5;
```
### Geçerli
- Harf ile başlayabilir.
- _ ile başlayabilir.
- Rakam içerebilir.
- snake_case tercih edilir.

Örneğin:
```
let user1 = "Ali";
let user2 = "Veli";
 ```
geçerlidir.

Ama:
```
let 1user = "Ali"; // ❌
```
geçersizdir.

3. Keyword'ler

Rust'ta bazı kelimeleri değişken adı olarak kullanamazsın:
```
let fn = 10; // ❌
let struct = 20; // ❌
let match = 30; // ❌
```
Çünkü bunlar Rust'ın keyword'leridir.

Örneğin:
```
fn
let
mut
struct
enum
trait
impl
match
if
else
loop
while
for
return
```
gibi kelimelerin dil içerisinde özel anlamları vardır.

4. _ Rust'ta çok önemlidir

Rust'ta _ sadece isimlendirmede kullanılan bir karakter değildir.

Örneğin:
```
let _x = 10;
```
Buradaki _x, normal bir değişkendir fakat kullanılmadığında Rust'ın uyarı davranışları açısından farklıdır.

Daha önemlisi:
 ```
let _ = 10;
```
Burada değeri bilinçli olarak yok sayıyoruz.

İleride pattern matching öğrenirken _ çok önemli olacak.

5. Kullanılmayan değişkenler

Rust compiler sana kullanılmayan değişkenler konusunda uyarı verebilir.

Örneğin:
```
fn main() {
    let age = 48;
}
```
age kullanılmadığı için Rust sana uyarı verebilir.

Bilinçli olarak kullanılmayacaksa:
```
let _age = 48;
```
şeklinde yazmak yaygın bir Rust yaklaşımıdır.

Bu küçük gibi görünüyor ama Rust'ın compiler'ı ciddiye alan felsefesinin bir parçası.

6. Sabitler

Sabitler için:

const MAX_USERS: u32 = 100;

Rust convention:

SCREAMING_SNAKE_CASE

Yani:
```
const MAX_USERS: u32 = 100; // ✅

const maxUsers: u32 = 100;  // ❌ convention

Burada ayrıca önemli bir syntax farkı var:

const MAX_USERS: u32 = 100;
                ^^^
``` 
Constant için tip açıkça belirtilir.

7. let ile const aynı şey değil

Bunu şimdiden ayıralım:
```
let max_users = 100;
```
ve:
```
const MAX_USERS: u32 = 100;
 ```
aynı kavram değildir.
 ``` 
let → binding/değişken
 ``` 
const → compile-time constant

Bu ayrımı ileride daha detaylı ele alacağız.

8. Rust'ta mut varsayılan değildir

Bu Rust'ın çok önemli bir yaklaşımıdır.
``` 
let age = 48;
 ``` 
age değiştirilemez.

Değiştirmek istiyorsan açıkça:
``` 
let mut age = 48;

age = 49;
 ``` 
demelisin.

Rust'ın felsefesini burada görmeye başlıyoruz:

Değişebilirlik açıkça belirtilir.

Bu yaklaşım ileride ownership ve concurrency konularında çok önemli olacak.

9. Shadowing Rust'ta idiomatic bir özelliktir

Örneğin:
``` 
let age = 48;

let age = age + 1;
``` 
Bunu Rust'ta oldukça doğal göreceksin.

Hatta tip değiştirmek için bile kullanılabilir:
``` 
let value = "48";

let value = value.parse::<i32>().unwrap();
``` 
İlk:
``` 
value → &str
 ``` 
sonra:
 ``` 
value → i32
``` 
olabilir.

Bu, mut ile yapılamayacak bir şeydir çünkü mut aynı binding'in değerini değiştirirken shadowing yeni binding oluşturur.

10. Rust'ta bloklar değer üretebilir

Bu çok önemli bir Rust özelliği.
``` 
let result = {
    let a = 10;
    let b = 20;

    a + b
};

result:

30
``` 
olur.

Burada:
 ``` 
a + b
``` 
sonunda ; olmadığı için bloğun değeridir.

Bu Rust syntax'ının çok önemli özelliklerinden biri.

11. Expression ve Statement ayrımı

Rust öğrenirken bunu erkenden öğrenmeni öneririm.
``` 
let x = 10;
``` 
bir statement.

Ama:
``` 
x + 10
``` 
bir expression.

Expression değer üretir.

Örneğin:
``` 
let result = if age >= 18 {
    "adult"
} else {
    "child"
};
``` 
Burada if bile değer üretebiliyor.

Bu, Rust'ın C#/Java'dan ayrıldığı noktalardan biridir ve ileride çok kullanışlı olacak.

12. Rust'ta return her zaman gerekli değil

Şunu yazabiliriz:
``` 
fn add(a: i32, b: i32) -> i32 {
    a + b
}
``` 
veya:
``` 
fn add(a: i32, b: i32) -> i32 {
    return a + b;
}
``` 
İkincisi geçerli olsa da normal durumda Rust'ın idiomatic yaklaşımı birincisidir.

Yani:
``` 
a + b
``` 
son expression ise dönüş değeridir.

13. cargo fmt → Rust'ın standart biçimlendirmesi

Rust'ta formatting konusunda fazla tartışma yapmamız gerekmiyor.

cargo fmt

Rust'ın standart formatter'ını çalıştırır.

Bu da Rust ekosisteminin güzel taraflarından biri:

Kod biçimlendirme konusunda takım içinde "ben böyle seviyorum" tartışmasını azaltır.

14. Clippy → sadece hata arayan bir araç değil
cargo clippy

Rust'ta idiomatic olmayan veya iyileştirilebilecek kodlar konusunda öneriler verir.

Örneğin compiler:

Bu kod çalışıyor.

diyebilir.

Clippy ise:

Çalışıyor ama Rust'ta bunu daha iyi yazabilirsin.

diyebilir.

Bu nedenle Rust öğrenirken Clippy'yi öğretmen yardımcısı gibi kullanacağız.


