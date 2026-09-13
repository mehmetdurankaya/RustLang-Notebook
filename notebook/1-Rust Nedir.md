## Rust Lang
### Rust Nedir? (Kısa Özet)
Rust, Mozilla tarafından geliştirilen,
bellek güvenliği ve performansı bir arada sunan,
derlenen (compiled) bir sistem programlama dilidir.
İlk kararlı sürümü 2015'te çıktı.
### Öne Çıkan Özellikler
- Bellek güvenliği — Garbage Collector (GC) olmadan,
- "borrow checker" sayesinde bellek hatalarını (null pointer,
- dangling pointer, data race) derleme zamanında yakalar.
- Sıfır maliyetli soyutlamalar — C/C++ kadar hızlı, ama daha güvenli.
- Eşzamanlılık (concurrency) — "Fearless concurrency" — veri yarışlarını derleyici engeller.
- Modern araçlar — Cargo (paket yöneticisi + build sistemi), rustfmt, clippy.
- Tip sistemi — Güçlü, statik, trait tabanlı.
### Nerede Kullanılır?
- Sistem programlama, işletim sistemi bileşenleri
- WebAssembly
- CLI araçları
- Gömülü sistemler
- Ağ / sunucu yazılımları (ör. Deno, Discord'un bazı kısımları)
- Linux kernel (artık resmi olarak destekleniyor)
- **Rust = C++ hızı + bellek güvenliği + modern araçlar.**