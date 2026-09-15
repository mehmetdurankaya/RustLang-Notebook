# 🦀 Ownership (Sahiplik)

Amaç: Rust'ın bellek yönetimini, Ownership sistemini ve Borrow Checker'ın neden var olduğunu anlamak.

Rust öğrenirken karşılaşılan en önemli farklardan biri Ownership (Sahiplik) sistemidir.

C, C++ gibi dillerde belleği manuel olarak yönetmek mümkündür.
Java, C#, Python gibi dillerde ise çoğunlukla Garbage Collector bu işi bizim için yapar.

Rust ise farklı bir yol izler:

Rust, Garbage Collector kullanmadan bellek güvenliğini compile-time'da garanti etmeye çalışır.

Bunu sağlayan temel mekanizma:

- Ownership
- Borrowing
- References
- Lifetimes
- Borrow Checker

Bu aşamanın amacı sadece kuralları ezberlemek değil, Rust'ın neden böyle tasarlandığını anlamaktır.

1. Stack ve Heap

Ownership'i anlamadan önce belleğin temel çalışma mantığını anlamak gerekir.

Bir program çalışırken veriler kabaca iki farklı bellek bölgesinde tutulabilir:

- Stack
- Heap
1.1 Stack

Stack, boyutu compile-time'da bilinen veya kolayca belirlenebilen veriler için oldukça hızlı bir bellek alanıdır.

Örneğin:
```
let x = 10;
let y = 20;
```
Buradaki x ve y gibi basit değerler Stack üzerinde tutulabilir.

Stack'in temel özellikleri:

- Çok hızlıdır.
- Veriler belirli bir düzen içerisinde tutulur.
- Fonksiyon çağrılarıyla yakından ilişkilidir.
- Bellek yönetimi otomatik ve düzenlidir.
- Değerlerin boyutunun bilinmesi önemlidir.

Basit bir model:

STACK
```
┌──────────────┐
│ y = 20       │
├──────────────┤
│ x = 10       │
└──────────────┘
```
1.2 Heap

Heap, çalışma zamanında boyutu değişebilen veya boyutu compile-time'da bilinmeyen veriler için kullanılır.

Örneğin:
```
let isim = String::from("Turan");
```
Burada String nesnesinin kendisi Stack üzerinde bulunurken, tuttuğu karakterlerin bulunduğu veri Heap üzerinde olabilir.

Basitleştirilmiş model:
```
STACK                         HEAP

┌───────────────┐             ┌──────────────┐
│ isim          │────────────>│ T u r a n    │
│ ptr           │             └──────────────┘
│ len = 5       │
│ capacity = 5  │
└───────────────┘
```
Burada önemli nokta:

String tek parça halinde sadece Heap'te duran bir şey değildir.

String'in yönetim bilgileri Stack'te bulunurken, karakter verisi Heap'te bulunabilir.