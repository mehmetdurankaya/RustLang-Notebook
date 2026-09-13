# Rust'ta Veri Tipleri

Rust'ta veri tiplerini öğrenirken önce büyük resmi görelim:
```
Veri Tipleri
│
├── Skaler Tipler
│   ├── Integer      → i8, i16, i32, i64, i128, isize
│   ├── Unsigned     → u8, u16, u32, u64, u128, usize
│   ├── Floating     → f32, f64
│   ├── Boolean      → bool
│   └── Character    → char
│
└── Bileşik Tipler
├── Tuple
└── Array
```
# 1. Integer — Tam Sayılar

Negatif ve pozitif tam sayılar için kullanılır.

İşaretli integer'lar
```
let a: i8 = -10;
let b: i16 = 1000;
let c: i32 = 100_000;
let d: i64 = 1_000_000;
let e: i128 = 1_000_000_000;
```
### i → signed integer

Yani negatif değer alabilir.
```
i8
i16
i32
i64
i128
```
Rakamlar bit sayısını belirtir.

Örneğin:
```
i8 → 8 bit
i32 → 32 bit
i64 → 64 bit
```
# 2. Unsigned Integer

Bunlar negatif değer alamaz.
```
let a: u8 = 255;
let b: u32 = 1000;
let c: u64 = 1_000_000;
```
**u → unsigned**
```
u8
u16
u32
u64
u128
```
Örneğin:
```
let age: u32 = 48;
```
# 3. isize ve usize

Bunlar biraz özel.
```
let x: usize = 10;
let y: isize = -10;
```
Boyutları çalıştığın mimariye bağlıdır.

Örneğin 64-bit sistemde:
```
usize  → 64 bit
isize  → 64 bit
```
Özellikle:
```
array indeksleri
collection uzunlukları
bellekle ilgili işlemler
```
gibi yerlerde usize ile çok karşılaşacağız.

Örneğin:
```
let numbers = [10, 20, 30];


let index: usize = 1;

println!("{}", numbers[index]);
```
Burada array indeksinin usize olması önemlidir.

# 4. Floating Point

Ondalıklı sayılar için:
```
let pi: f64 = 3.14159;
let value: f32 = 10.5;
```
İki temel floating-point tipi vardır:
```
f32
f64
```
Rust'ta çoğu durumda:
```
let pi = 3.14;
```
yazarsan Rust bunu varsayılan olarak f64 kabul eder.

# 5. Boolean

İki değer alır:
```
true
false
```
Örneğin:
```
let is_active: bool = true;
let is_admin = false;
```
Özellikle kontrol yapılarında kullanılır:
```
if is_active {
println!("Aktif");
}
```
6. Character — char

Rust'ta tek karakter:
```
let letter: char = 'A';
```
Dikkat:
```
'A'
```
char'dır.

Ama:
```
"A"
```
char değildir; bu bir string slice'dır (&str).

Yani:
```
let a = 'A';   // char
let b = "A";   // &str
```
Bu ayrımı özellikle ileride String ve &str konusunda detaylandıracağız.

Rust'ın char tipi Unicode karakterleri temsil eder:
```
let c = 'ğ';
let c = '中';
let c = '🦀';
```
7. Tuple

Tuple farklı tiplerde değerleri bir arada tutabilir:
```
let person = ("Turan", 48, true);
```
Burada:
```
&str
i32
bool
```
aynı tuple içerisinde bulunuyor.

Elemanlara sıra numarasıyla erişilir:
```
let person = ("Turan", 48, true);

println!("{}", person.0);
println!("{}", person.1);
println!("{}", person.2);
```
Sonuç:
```
Turan
48
true
```
Tuple'ın önemli özelliği:

Elemanlarının tipleri farklı olabilir.

8. Array

Array aynı tipte birden fazla değeri tutar:
```
let numbers = [10, 20, 30, 40, 50];
```
Bütün elemanlar i32 olur.

Array'in uzunluğu da sabittir:
```
let numbers: [i32; 5] = [10, 20, 30, 40, 50];
```
Burada:
```
[i32; 5]
│     │
│     └── eleman sayısı
└──────── eleman tipi
```
Örneğin:
```
let numbers: [i32; 3] = [10, 20, 30];
```
ama:
```
let numbers: [i32; 3] = [10, 20, 30, 40];
```
hatalıdır.

Çünkü array'in boyutu 3 olarak belirtilmiştir.

# 9. Rust'ın Type Inference özelliği

Her değişkende tipi yazmak zorunda değiliz:
```
let age = 48;
```
Rust 48 değerinden tip çıkarabilir.

Ama gerektiğinde açıkça belirtebiliriz:
```
let age: i64 = 48;
```
Syntax:
```
let değişken: tip = değer;
```
Örneğin:
```
let age: u32 = 48;
let price: f64 = 125.50;
let active: bool = true;
let letter: char = 'T';
```
10. Rust'ta önemli bir Base Practice

Tipleri gereksiz yere yazmak zorunda değiliz.

Genellikle:
```
let age = 48;
```
yeterlidir.

Ama tipin özellikle anlaşılması gerekiyorsa:
```
let age: u64 = 48;
```
yazabiliriz.

Yani Rust'ta yaklaşım:

Compiler'ın tip çıkarımını kullan, fakat kodun anlamını netleştirmek gerektiğinde tipi açıkça belirt.

Şimdilik bilmen gerekenler
```
i32 / i64       → işaretli tam sayı
u32 / u64       → işaretsiz tam sayı
usize           → özellikle indeks/boyutlarda
f32 / f64       → ondalıklı sayı
bool            → true / false
char            → Unicode karakter
tuple           → farklı tipleri bir arada tutabilir
array           → aynı tip + sabit uzunluk
```
### Küçük ama önemli ayrım

Şu üçü birbirinden farklı:
```
let a = 'A';       // char
let b = "A";       // &str
let c = String::from("A"); // String
```