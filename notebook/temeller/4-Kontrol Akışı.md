# Kontrol Akışı

Rust'ta kontrol akışı, programın hangi kodu, hangi koşulda ve kaç kez çalıştıracağını belirler.

## Rust'ta temel kontrol akışı yapıları:

- if / else
- loop
- while
- for
- match → ileride daha detaylı ele alınacak
## if / else
Sözdizimi
```
if koşul {
// koşul doğruysa çalışır
} else {
// koşul yanlışsa çalışır
}
```
Örnek:
```
let age = 20;

if age >= 18 {
println!("Reşit");
} else {
println!("Reşit değil");
}
```
Önemli Rust kuralı

Rust'ta if koşulunun sonucu mutlaka bool olmalıdır.
```
if age >= 18 {
println!("Reşit");
}
```
Doğrudur.

Ancak:
```
if age {
println!("...");
}
```
Rust'ta geçerli değildir.

Rust, bazı dillerde olduğu gibi 0 = false, 1 = true gibi otomatik dönüşüm yapmaz.

### else if

Birden fazla koşul kontrol edilebilir:
```
let age = 20;

if age < 18 {
println!("Çocuk");
} else if age < 65 {
println!("Yetişkin");
} else {
println!("65 yaş üstü");
}
```
Yapı:
```
if koşul1 {
// ...
} else if koşul2 {
// ...
} else {
// ...
}
```
if Bir Expression'dır

Rust'ın önemli özelliklerinden biri:

if sadece kontrol yapmak için değil, değer üretmek için de kullanılabilir.

Örneğin:
```
let age = 20;

let durum = if age >= 18 {
"Reşit"
} else {
"Reşit değil"
};

println!("{}", durum);
```
Burada:
```
let durum = if age >= 18 {
"Reşit"
} else {
"Reşit değil"
};

if bir değer üretmektedir.
```
Rust Base Practice

if bir değer üretiyorsa, gereksiz yere ayrı bir mut değişken kullanmak yerine doğrudan expression olarak kullanmak genellikle daha idiomatiktir.

## loop

loop, kod bloğunu sonsuz olarak tekrarlar.

Sözdizimi
```
loop {
// tekrar tekrar çalışır
}
```
Örnek:
```
loop {
println!("Merhaba");
}
```
Bu program durmadan "Merhaba" yazdırır.

Bu nedenle genellikle break ile kullanılır.

###  break

break, döngüyü sonlandırır.
```
let mut sayi = 1;

loop {
println!("{}", sayi);

    if sayi == 5 {
        break;
    }

    sayi += 1;
}
```
Çıktı:
```
1
2
3
4
5
```
Akış:
```
loop
↓
sayi yazdır
↓
sayi == 5 ?
├── Hayır → sayi += 1 → tekrar loop
└── Evet  → break
```
## loop Değer Döndürebilir

Rust'ın güçlü expression yaklaşımının başka bir örneği:
```
let sonuc = loop {
break 42;
};

println!("{}", sonuc);
```
Burada loop:
```
42
```
değerini üretir.

Dolayısıyla:
```
let sonuc = loop {
break 42;
};
```
sonucunda:
```
sonuc == 42
```
olur.

Bu Rust'ın önemli özelliklerinden biridir:

break sadece döngüyü bitirmez, istersek döngüden bir değer de döndürebilir.

## while

while, koşul doğru olduğu sürece döngüyü çalıştırır.

Sözdizimi
```
while koşul {
// çalışacak kod
}
```
Örnek:
```
let mut sayi = 1;

while sayi <= 5 {
println!("{}", sayi);
sayi += 1;
}
```
Akış:
```
sayi = 1
↓
sayi <= 5 ?
↓
Evet
↓
kodu çalıştır
↓
sayi += 1
↓
tekrar kontrol et
```
Koşul false olduğunda döngü sona erer.

## for

for, Rust'ta koleksiyonlar ve aralıklar üzerinde gezinmek için çok sık kullanılır.

Sözdizimi
```
for değişken in ifade {
// kod
}
```
Örneğin:
```
for sayi in 1..6 {
println!("{}", sayi);
}
```
Çıktı:
```
1
2
3
4
5
```
Buradaki:
```
1..6
```
şu anlama gelir:
```
1, 2, 3, 4, 5
```
6 dahil değildir.

4.9 ..= Aralığı

..= kullanırsak başlangıç ve bitiş dahil olur.
```
for sayi in 1..=5 {
println!("{}", sayi);
}
```
Çıktı:
```
1
2
3
4
5
```
Karşılaştırma:
```
1..5

→ 1, 2, 3, 4

1..=5

→ 1, 2, 3, 4, 5
```
## Diziler Üzerinde for

Daha sonra göreceğimiz array yapısıyla:

let sayilar = [10, 20, 30, 40];
```
for sayi in sayilar {
println!("{}", sayi);
}
```
Burada for, array'in elemanları üzerinde tek tek gezinir.

Bu kullanım Rust'ta oldukça önemlidir.

## continue

continue, döngünün mevcut turunun geri kalanını atlar ve sonraki tura geçer.
```
for sayi in 1..=5 {
if sayi == 3 {
continue;
}

    println!("{}", sayi);
}
```
Çıktı:
```
1
2
4
5
```
3 atlandı.

Akış:
```
1 → yazdır
2 → yazdır
3 → continue
4 → yazdır
5 → yazdır
```
## break ve continue Farkı

|Yapı   |Görevi   |
|---|---|
| break| Döngüyü tamamen bitirir|
| continue  |Mevcut turu atlar, sonraki tura geçer    |

Örneğin:
```
for sayi in 1..=10 {
if sayi == 3 {
continue;
}

    if sayi == 7 {
        break;
    }

    println!("{}", sayi);
}
```
Çıktı:
```
1
2
4
5
6
```
Çünkü:
```
3 → continue
7 → break
8, 9, 10 → hiç çalışmaz
```
## Rust Base Practices

Kontrol akışında Rust'ın temel yaklaşımı:

1. if koşulu bool olmalıdır
```
   if age >= 18 {
   // ...
   }
   ```
2. if expression olarak kullanılabilir
   ```
   let sonuc = if age >= 18 {
   "Yetişkin"
   } else {
   "Çocuk"
   };
   ```
3. Basit tekrarlar için for tercih edilir
   ```
   for sayi in 1..=10 {
   println!("{}", sayi);
   }
```
Özellikle koleksiyonlar üzerinde gezinirken for idiomatiktir.

4. Koşula bağlı tekrar için while
   ```
while koşul {
   // ...
   }
```
5. Bilinçli sonsuz döngü için loop
   ```
loop {
   // ...
   if bitir {
   break;
   }
   }
```
6. Döngü kontrolü için break ve continue
  ```
```
break;
```
→ tamamen çık
```
continue;
```
→ bu turu atla

## Kontrol Akışının Büyük Resmi

Rust'ta şu dört yapıyı şimdilik net olarak ayır:
```
if
│
├── Koşula göre seçim
│
└── if / else / else if
```
```
loop
│
├── Sonsuz döngü
└── break ile çıkılır
```
```
while
│
├── Koşul true olduğu sürece çalışır
└── Koşul false → döngü biter
```
```
for
│
├── Aralıklar üzerinde gezinme
└── Koleksiyonlar üzerinde gezinme
```
Ve Rust açısından en önemli noktalardan biri:
```
if
loop
bloklar
↓
expression
↓
değer
```
Yani Rust'ta kontrol akışı yalnızca "hangi kod çalışacak?" sorusunu değil, bazı durumlarda "hangi değer üretilecek?" sorusunu da cevaplar.

## Şimdilik Bilmemiz Gerekenler

Kontrol akışı bölümünü tamamlamak için şu sözdizimlerini bilmen yeterli:
```
if koşul {
}
if koşul {
} else {
}
if koşul1 {
} else if koşul2 {
} else {
}
loop {
break;
}
while koşul {
}
for item in koleksiyon {
}
continue;
```