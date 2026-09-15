# Borrowing'in Avantajı

Bir fonksiyon sadece veriyi okuyacaksa Ownership'i devretmemize gerek yoktur.
```
fn uzunluk(metin: &String) -> usize {
metin.len()
}
```
Kullanımı:
```
let isim = String::from("Turan");

let uzunluk = uzunluk(&isim);

println!("{}", isim);
println!("{}", uzunluk);
```
Burada isim hâlâ main scope'unun sahibidir.

## Mutable Borrowing — &mut

Bir veriyi sadece okumak değil, değiştirmek istiyorsak mutable reference kullanırız.
```
fn degistir(metin: &mut String) {
metin.push_str(" Kaya");
}
```
Kullanımı:
```
let mut isim = String::from("Turan");

degistir(&mut isim);

println!("{}", isim);
```
Burada:
```
&mut isim
```
isim değerinin mutable referansını verir.

Fonksiyon:
```
metin.push_str(...)
```
ile veriyi değiştirebilir.

## Neden mut İki Yerde Görünüyor?

Şuna dikkat:
```
let mut isim = String::from("Turan");
```
ve:
```
degistir(&mut isim);
```
İlk mut:

> Değişkenin değiştirilebilir olduğunu belirtir.

İkinci mut:

> Mutable reference oluşturduğumuzu belirtir.

## Borrowing Kuralları

Rust'ın en önemli güvenlik kurallarından biri:

Aynı anda ya bir tane mutable reference veya birden fazla immutable reference olabilir.

Yani:
```
N tane immutable reference
VEYA
1 tane mutable reference
```
## Birden Fazla Immutable Reference

Bu geçerlidir:
```
let isim = String::from("Turan");

let r1 = &isim;
let r2 = &isim;
let r3 = &isim;

println!("{}", r1);
println!("{}", r2);
println!("{}", r3);
```
Çünkü hiçbiri veriyi değiştirmiyor.
```
       ┌── r1 ──┐
       │        │
isim ──┼── r2 ──┼──> String
│        │
└── r3 ──┘
```
Birden fazla okuyucu olabilir.

## Aynı Anda Tek Mutable Reference

Şu kullanım geçerlidir:
```
let mut isim = String::from("Turan");

let r1 = &mut isim;

r1.push_str(" Kaya");
```
Fakat aynı anda başka bir mutable reference oluşturmak sorun yaratır:
```
let mut isim = String::from("Turan");

let r1 = &mut isim;
let r2 = &mut isim;
```
Rust bunu engeller.

Çünkü iki farklı mutable reference aynı veriyi aynı anda değiştirebilir.

## Mutable ve Immutable Reference Birlikte

Şu durum da problem oluşturabilir:

let mut isim = String::from("Turan");
```
let r1 = &isim;
let r2 = &mut isim;
```
Çünkü r1 hâlâ immutable reference olarak kullanılabilecek durumdayken r2 mutable reference oluşturmaya çalışıyor.

Rust bunu güvenli değildir diye engeller.

## Rust'ın Temel Referans Modeli

Şunu akılda tutmak çok önemlidir:
```
                    String
                      │
          ┌───────────┴───────────┐
          │                       │
     &String                   &mut String
          │                       │
       OKUMA                  OKUMA + YAZMA
```
Aynı anda:
```
&String + &String + &String
```
olabilir.

Ama:
```
&mut String + &mut String
```
aynı veriye aynı anda verilemez.

Ve genel kural:
```
&String
veya
&mut String
```
## Borrow Checker

Rust compiler'ın Ownership ve Borrowing kurallarını kontrol eden mekanizmasına:

Borrow Checker

denir.

Örneğin:
```
let mut isim = String::from("Turan");

let r1 = &isim;
let r2 = &mut isim;
```
Rust burada programı derlemeyi reddedebilir.

Çünkü potansiyel olarak:
```
r1 ──────> isim
▲
│
r2 ────────┘
```
şeklinde aynı anda okuma ve yazma erişimi oluşabilir.

Borrow Checker'ın amacı:

Bellek güvenliğini runtime'da değil, mümkün olduğunca compile-time'da garanti etmektir.

## Lifetime — 'a

Lifetime, bir referansın geçerli olduğu yaşam süresini ifade eder.

Örneğin:
```
let isim = String::from("Turan");

let referans = &isim;

println!("{}", referans);
```
Burada referans, isim yaşadığı sürece kullanılabilir.

Basitleştirilmiş olarak:
```
isim lifetime
├──────────────────────┤

referans lifetime
├───────────────┤
```
Referansın yaşam süresi, referans verdiği veriden daha uzun olamaz.

## Dangling Reference

Rust'ın engellediği önemli hatalardan biri dangling reference'dır.

Mantıksal olarak şöyle bir durum tehlikelidir:
```
reference
│
▼
artık var olmayan veri
```
C/C++ gibi dillerde bu tür hatalar ciddi problemlere yol açabilir.

Rust ise Borrow Checker sayesinde geçersiz referansların oluşmasını engellemeye çalışır.

Temel prensip:

Bir referans, referans verdiği veriden daha uzun yaşayamaz.

## Lifetime Annotation — 'a

Bazı durumlarda Rust compiler'a referansların yaşam süreleri arasındaki ilişkiyi açıkça belirtmemiz gerekir.

Örneğin:
```
fn en_uzun<'a>(x: &'a str, y: &'a str) -> &'a str {
if x.len() > y.len() {
x
} else {
y
}
}
```
Buradaki:

> 'a

bir lifetime annotation'dır.

Bu:

x, y ve dönen referansın lifetime ilişkisini belirtir.

Ancak önemli nokta:

> 'a bellekte oluşturulan bir değişken değildir.

Bir lifetime:

runtime değeri

değil,

compiler'a referansların yaşam ilişkisini anlatan bilgi

olarak düşünülmelidir.

## Slices

Slice, bir koleksiyonun tamamına sahip olmadan onun bir bölümüne erişmemizi sağlayan referanstır.

En önemli slice türlerinden biri:

> &str

dir.

Bir diğeri:

> &[T]

şeklindedir.

## &str

Şu kodu düşünelim:
```
let isim = String::from("Turan");

let parca = &isim[0..3];

println!("{}", parca);
```
Burada parca yeni bir String değildir.

Sadece isim içindeki bir bölgeye referanstır.
```
String

T u r a n
0 1 2 3 4

      │
      └──── &str

&str:
```
String verisinin tamamının sahibi değildir.

## String ve &str

Bu iki tür arasındaki fark çok önemlidir.

String
- Owned bir türdür.
- Heap üzerinde veri tutabilir.
- Büyüyebilir/değişebilir.
- Ownership'e sahiptir.
&str
- Borrow edilmiş string slice'tır.
- Başka bir string verisine referans verebilir.
- Kendisi verinin sahibi değildir.
- Okuma amaçlı fonksiyon parametrelerinde çok kullanışlıdır.

Örneğin:
```
fn yazdir(metin: &str) {
println!("{}", metin);
}
```
Bu fonksiyon hem:

> String

hem de:

> &str

ile kullanılabilir.

Örneğin:
```
let isim = String::from("Turan");

yazdir(&isim);
yazdir("Merhaba");
```
28. &[T]

Slice sadece String'lerde kullanılmaz.

Bir array'in veya vector'ün bir bölümünü referans olarak alabiliriz.

Örneğin:
```
let sayilar = vec![10, 20, 30, 40, 50];

let parca = &sayilar[1..4];

println!("{:?}", parca);
```
Sonuç:
```
[20, 30, 40]
```
Burada:

parca

yeni bir Vec değildir.

Vec içindeki verilere borrow edilmiş bir slice'dır.

## Ownership ve Function Parametreleri

Fonksiyonlarda Ownership davranışını anlamak çok önemlidir.
```
Ownership alan fonksiyon
fn kullan(metin: String) {
println!("{}", metin);
}
```
Çağrı:
```
let isim = String::from("Turan");
```
kullan(isim);

Burada:
```
main
isim
│
│ ownership
▼
kullan()
Borrow alan fonksiyon
fn oku(metin: &String) {
println!("{}", metin);
}
```
Çağrı:
```
let isim = String::from("Turan");

oku(&isim);
```
Burada:
```
main
isim
│
│ borrow
▼
oku()
```
Ownership main içinde kalır.

### En Önemli Zihinsel Model

Rust öğrenirken aşağıdaki modeli sürekli kullanmak faydalıdır:
```

                    DEĞER
                      │
               ┌──────┴──────┐
               │             │
             OWNER        BORROWER
               │             │
             sahip          & / &mut
               │             │
               ▼             ▼
           ownership       reference
```
Bir fonksiyona değer gönderirken kendimize şu soruyu sormalıyız:

Fonksiyon bu verinin sahibi mi olacak, yoksa sadece geçici olarak mı kullanacak?

Eğer sadece kullanacaksa çoğu durumda:

> &T

veya:

> &mut T

düşünülmelidir.

## String Üzerinden Örnek

Aşağıdaki kod Ownership, Borrowing ve Mutable Borrowing kavramlarını birlikte gösterir:
```
fn oku(metin: &str) {
println!("Metin: {}", metin);
}

fn degistir(metin: &mut String) {
metin.push_str(" Kaya");
}

fn main() {
let mut isim = String::from("Turan");

    oku(&isim);

    degistir(&mut isim);

    oku(&isim);
}
```
Akış:
```
main
│
├── isim oluştur
│
├── &isim
│      │
│      └── oku()
│
├── &mut isim
│      │
│      └── degistir()
│
└── &isim
│
└── oku()
```
Ownership hiçbir zaman main dışına aktarılmamıştır.

## Borrow Checker Hatalarını Öğrenme

Rust öğrenirken compiler hatalarından kaçmak yerine onları anlamaya çalışmak çok önemlidir.

Örneğin:
```
let mut isim = String::from("Turan");

let r1 = &isim;
let r2 = &mut isim;

println!("{}", r1);
println!("{}", r2);
```
Bu kod hata verdiğinde sadece:

"Rust izin vermiyor."

demek yeterli değildir.

Şu soruları sormalıyız:


1. isim'in sahibi kim?
2. r1 neyin referansı?
3. r1 immutable mı?
4. r2 mutable mı?
5. r1 hâlâ kullanılabilir durumda mı?
6. Aynı anda kaç borrow var?
7. Rust burada hangi güvenlik problemini önlemeye çalışıyor?

Bu sorular Ownership sistemini gerçekten anlamaya yardımcı olur.

## Pratik Alıştırmalar
###    Alıştırma 1 — Move

Aşağıdaki kodun neden hata verdiğini açıklayın:
```
fn main() {
let isim1 = String::from("Turan");

    let isim2 = isim1;

    println!("{}", isim1);
}
```
Soru

isim1 neden artık kullanılamıyor?

### Alıştırma 2 — Copy

Aşağıdaki kod neden çalışıyor?
```
fn main() {
let x = 10;

    let y = x;

    println!("{}", x);
    println!("{}", y);
}
```
Soru

String ile i32 arasındaki Ownership davranışı neden farklı?

### Alıştırma 3 — Clone

Aşağıdaki kodu inceleyin:
```
fn main() {
let isim1 = String::from("Turan");

    let isim2 = isim1.clone();

    println!("{}", isim1);
    println!("{}", isim2);
}
```
Soru

Burada neden isim1 hâlâ kullanılabiliyor?

### Alıştırma 4 — Immutable Borrow

Şu fonksiyonu yazın:

fn uzunluk(metin: &String) -> usize {
// ...
}

Fonksiyon:

- String'in ownership'ini almamalı.
- String'i değiştirmemeli.
- Uzunluğunu döndürmeli.

### Alıştırma 5 — Mutable Borrow

Şu fonksiyonu yazın:
```
fn ekle(metin: &mut String) {
// ...
}
```
Fonksiyon String'in sonuna:

> " Rust"

eklemeli.

### Alıştırma 6 — Slice

Şu fonksiyonu yazın:
```
fn ilk_uc_harf(metin: &str) -> &str {
// ...
}
```
Fonksiyon verilen metnin ilk üç karakterini temsil eden bir &str döndürmeli.

Not: UTF-8 nedeniyle Rust'ta String'i byte index'leriyle dilimlerken dikkatli olunmalıdır.

## Mini Proje — String İşleme

Bu aşamanın sonunda aşağıdaki fonksiyonları kendiniz yazmaya çalışın:
```
fn yazdir(metin: &str) {
// ...
}
```
```
fn uzunluk(metin: &str) -> usize {
// ...
}

fn buyut(metin: &mut String) {
// ...
}

fn ilk_parca(metin: &str) -> &str {
// ...
}

main içerisinde:

fn main() {
let mut metin = String::from("Rust Ownership");

    yazdir(&metin);

    let uzunluk = uzunluk(&metin);

    println!("Uzunluk: {}", uzunluk);

    buyut(&mut metin);

    yazdir(&metin);

    let parca = ilk_parca(&metin);

    println!("Parça: {}", parca);
}
```
Bu küçük proje şu kavramları birlikte kullanmayı amaçlar:
```
String
│
├── Ownership
│
├── Immutable Borrow
│
├── Mutable Borrow
│
├── &str
│
└── Lifetime
```
## Özet

Bu aşamada öğrenilmesi gereken temel zihinsel model:
```
OWNERSHIP
│
├── Her değerin bir sahibi vardır.
│
├── Aynı anda tek owner vardır.
│
└── Owner scope dışına çıkınca değer drop edilir.


MOVE
│
└── Ownership başka değişkene geçer.


COPY
│
└── Basit değer kopyalanır.


CLONE
│
└── Açıkça bağımsız kopya oluşturulur.


BORROWING
│
├── &T
│    └── immutable borrow
│
└── &mut T
└── mutable borrow


BORROW RULES
│
├── N immutable reference
│
└── VEYA
│
└── 1 mutable reference


LIFETIME
│
└── Referansın geçerli olduğu yaşam süresi.


SLICE
│
├── &str
│
└── &[T]
```
🎯 Bu Aşamanın Gerçek Hedefi

Bu konuları ezberlemek değil, aşağıdaki soruya cevap verebilmektir:

"Bu değerin sahibi kim ve şu anda kim bu değere erişiyor?"

Rust kodu okurken sürekli şu dört soruyu sormak gerekir:

1. Owner kim?

2. Ownership taşındı mı?

3. Borrow var mı?
   & / &mut

4. Aynı anda kaç kişi erişiyor?

Bu dört soruya cevap verebiliyorsanız Rust'ın Ownership sisteminin temel mantığını anlamaya başlamışsınız demektir.

🧠 Hatırlanması Gereken Tek Cümle

Rust'ta belleği güvenli yapan şey, verinin kime ait olduğunun ve kimlerin ne şekilde erişebileceğinin compile-time'da kontrol edilmesidir.

Ownership Rust'ın sadece bir özelliği değildir.

Ownership;
```
Memory Management
↓
Ownership
↓
Borrowing
↓
Borrow Checker
↓
Lifetime
↓
Memory Safety
```
-
zincirinin temelidir.