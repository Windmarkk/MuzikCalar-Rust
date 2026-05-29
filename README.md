# MüzikÇalar-R (mp3-oynatici)

Rust programlama dilini öğrenirken geliştirdiğim, terminal tabanlı çevrimdışı ve çevrimiçi özelliklere sahip bir MP3 çalar uygulaması.

## Özellikler

- **Şarkı Oynatma:** `music` klasöründeki MP3 dosyalarını oynatma.
- **Şarkı Listeleme:** Mevcut şarkıları listeleyebilme.
- **Ses Kontrolü:** Sesi arttırıp azaltabilme.
- **Durdur/Devam Et:** Çalan şarkıyı duraklatma ve kaldığı yerden devam ettirme.
- **Şarkı Yükleme:** Bilgisayarınızdaki herhangi bir dizinden `music` klasörüne şarkı kopyalama.
- **Online MP3 İndirme:** YouTube bağlantısı vererek doğrudan `music` klasörüne MP3 indirebilme (bunun için sisteminizde `yt-dlp` kurulu olmalıdır).

## Gereksinimler

- [Rust ve Cargo](https://www.rust-lang.org/tools/install) (Projeyi derlemek ve çalıştırmak için)
- [yt-dlp](https://github.com/yt-dlp/yt-dlp) (Online müzik indirme özelliğini kullanmak için sisteminizde kurulu olmalıdır)

## Kurulum ve Çalıştırma

1. Projeyi bilgisayarınıza klonlayın.
2. Terminali projenin ana dizininde açın.
3. Uygulamayı derleyip çalıştırmak için şu komutu girin:

```bash
cargo run
```

## Eklenecekler (Will be added)

- [ ] Çalma listesi (Playlist) oluşturma ve oynatma desteği.
- [ ] Şarkılar arası geçiş (Sonraki/Önceki şarkı) ve atlama özelliği.
- [ ] Şarkı süresi ve anlık oynatma süresi (Progress bar) göstergesi.
- [ ] Şarkı döngüye alma (Loop) veya rastgele oynatma (Shuffle) özellikleri.
- [ ] Gelişmiş terminal arayüzü (TUI) entegrasyonu.
- [ ] Diğer ses formatları (.wav, .flac vb.) için genişletilmiş destek.

