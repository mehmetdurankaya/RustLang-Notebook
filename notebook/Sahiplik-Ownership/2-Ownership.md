# Ownership Nedir?

Ownership, Rust'ın belleği yönetmek için kullandığı sistemdir.

Rust'ın temel yaklaşımı şudur:

Her değerin bir sahibi vardır.

Bu sahiplik sayesinde Rust şunları compile-time'da kontrol edebilir:

- Bir değer ne zaman kullanılabilir?
- Bir değer ne zaman artık geçerli değildir?
- Bellek ne zaman serbest bırakılmalıdır?
- Aynı belleğe güvenli şekilde nasıl erişilebilir?

## Ownership'in 3 Temel Kuralı

Rust Ownership sisteminin temelinde üç önemli kural vardır.

## Kural 1

### Rust'taki her değerin bir sahibi (owner) vardır.

Örneğin:
```
let isim = String::from("Turan");
```
Burada:
```
isim
│
▼
String verisinin sahibi
```
isim, oluşturulan String değerinin sahibidir.

## Kural 2

Aynı anda bir değerin yalnızca bir sahibi olabilir.

Örneğin:
```
let isim1 = String::from("Turan");

let isim2 = isim1;
```
Burada String değeri kopyalanmış gibi görünse de aslında Ownership aktarılmıştır.
```
isim1 ──────┐
│
▼
String
```
sonrasında:
```
isim2 ──────┐
│
▼
String
```
Ownership isim1'den isim2'ye geçmiştir.

Bu işlem:

> Move

olarak adlandırılır.

## Kural 3

Sahip scope dışına çıktığında değer otomatik olarak drop edilir.

Örneğin:
```
{
let isim = String::from("Turan");

    println!("{}", isim);
}
```
Scope sona erdiğinde:
```
isim
↓
scope dışına çıktı
↓
drop
↓
Heap belleği serbest bırakılır
```
Rust bunu otomatik olarak gerçekleştirir.

## Scope

Ownership'i anlamak için Scope kavramını bilmek gerekir.
```
{
let isim = String::from("Turan");

    println!("{}", isim);
}
```
Burada isim değişkeninin scope'u:
```
{
┌───────────────────────────┐
│ let isim = ...            │
│                           │
│ println!("{}", isim);     │
└───────────────────────────┘
}
```
} karakterine ulaşıldığında isim artık scope dışındadır.

Rust bu noktada String'in drop edilmesini sağlar.

## Move

Rust'ta bazı değerler basit şekilde kopyalanmaz.

Özellikle Heap kullanan türlerde bu çok önemlidir.

Örneğin:
```
let s1 = String::from("Merhaba");

let s2 = s1;
```
Burada s1 artık kullanılabilir değildir.

Şu kod hata verir:
```
let s1 = String::from("Merhaba");

let s2 = s1;

println!("{}", s1);
```
Çünkü Ownership s1'den s2'ye taşınmıştır.

Bu işleme:

> MOVE

denir.

### Move neden var?

Şöyle bir durum olduğunu düşünelim:
```
s1 ────────┐
│
▼
HEAP DATA
```
Eğer:
```
let s2 = s1;
```
işleminde Heap verisi gerçekten kopyalanmasaydı ve iki değişken de aynı Heap alanının sahibi olsaydı:
```
s1 ────────┐
├──────> HEAP DATA
s2 ────────┘
```
iki farklı owner ortaya çıkardı.

Scope sona erdiğinde ikisi de aynı belleği serbest bırakmaya çalışabilirdi.

Bu durum:

> **Double Free**

gibi ciddi bellek hatalarına yol açabilir.

Rust bunun yerine Ownership'i taşır:
```
s1 ──X

s2 ────────> HEAP DATA
```
Böylece tek bir owner kalır.

### Copy

Bazı türlerde Move yerine Copy gerçekleşir.

Örneğin:
```
let x = 10;
let y = x;

println!("{}", x);
println!("{}", y);
```
Bu kod geçerlidir.

Çünkü i32 gibi basit türler Copy trait'ine sahiptir.

Burada:
```
x = 10
│
├────> y = 10
│
└────> x hâlâ kullanılabilir
```
Copy semantiğinde değer kopyalanır.

Copy olan türlere örnekler

Genel olarak:
```
i32
u32
i64
f64
bool
char
```
gibi basit türler Copy olabilir.

Örneğin:
```
let a = 10;
let b = a;

println!("{}", a);
println!("{}", b);
```
### Move ve Copy Arasındaki Fark
   |Özellik	|Move|	Copy|
   |--------|----|------|
   |Ownership aktarılır	|✅	|❌|
   |Eski değişken kullanılabilir|❌|✅|
   |Heap verisi açısından önemlidir	|✅	|Genellikle hayır|
   |Örnek	|String	|i32|

Özet:
```
String
│
└── Move

i32
│
└── Copy
8. Clone
```
Bazen gerçekten bağımsız bir kopya oluşturmak isteriz.

Bunun için clone() kullanabiliriz.
```
let s1 = String::from("Merhaba");

let s2 = s1.clone();

println!("{}", s1);
println!("{}", s2);
```
Burada iki bağımsız String vardır.
```
s1 ───────> "Merhaba"

s2 ───────> "Merhaba"
```
Heap verisi de kopyalanır.

Bu nedenle clone():

Move'dan farklı olarak gerçek bir veri kopyalama işlemi gerçekleştirebilir.

### Move vs Copy vs Clone

En önemli ayrımlardan biridir.
```
MOVE
s1 ─────────────> s2
s1 artık kullanılamaz
COPY
s1 ─────────────> s2
s1 kullanılmaya devam edebilir
CLONE
s1 ───────> bağımsız veri
s2 ───────> bağımsız veri
```
Basit şekilde:

|İşlem	|Sonuç|
|-------|----|
|Move	|Sahiplik taşınır|
|Copy	|Basit değer kopyalanır|
|Clone	|Açıkça bağımsız kopya oluşturulur|