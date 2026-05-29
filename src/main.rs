use rodio::{Decoder};
use std::fs::{File};
use std::io::{BufReader};
use std::path::Path;
use std::process::Command;
use std::thread::sleep;
use sanitize_filename::sanitize;

fn oynattir(oynatici: &rodio::Player){
    let mut sarki_adi = String::new();
    let dosya = sarki_sectir(&mut sarki_adi);
    oynatici.append(Decoder::new(dosya).unwrap());
}

fn sarki_sectir(mut sarki_adi: &mut String) -> BufReader<File>{
    println!("Oynatmak istediginiz sarki dosyasinin adini giriniz:");
    std::io::stdin().read_line(&mut sarki_adi).unwrap();
    let temiz_isim = sarki_adi.trim();
    let dosya_yolu = Path::new("music").join(temiz_isim);
    println!("Dosya yolu: {}", dosya_yolu.display());
        BufReader::new(File::open(Path::new(&dosya_yolu)).unwrap())
}

fn sarki_listele(dosya_yolu: &Path) {
    if let Ok(sarki_listesi) = std::fs::read_dir(dosya_yolu) {
        println!("\nSarki Listen");
        println!("============================================================\n");
        for sarki in sarki_listesi {
            println!("{}", sarki.unwrap().file_name().into_string().unwrap());
        }
        println!("\n\n============================================================");
    }
}
fn yol_temizle(yol: &str) -> String {
        let os_safe = sanitize(yol);
        let no_spaces = os_safe.replace(" ", "_");
        no_spaces
            .chars()
            .filter(|c| c.is_alphanumeric() || *c == '_' || *c == '.')
            .collect()

    }
fn online_indir() {
        println!("Youtube linkini giriniz:");
        let mut link = String::new();
        std::io::stdin().read_line(&mut link).unwrap();
        let link = link.trim();
        let isim_yakalama = Command::new("yt-dlp")
            .arg("-x")
            .arg("--audio-format")
            .arg("mp3")
            .arg("--get-filename")
            .arg("-o")
            .arg("%(title)s.mp3")
            .arg(link)
            .output()
            .expect("Dosya adı alınamadı!");
        let ham_dosya_adi = String::from_utf8_lossy(&isim_yakalama.stdout).trim().to_string();
        let temiz_dosya_adi = yol_temizle(&ham_dosya_adi);
        let yuklenecek_yol = Path::new("music").join(temiz_dosya_adi);
        let yuklenecek_yol_str = yuklenecek_yol.to_str().unwrap();
        println!("Dosya yolu: {}", yuklenecek_yol_str);
        let mut yt_dlp = Command::new("yt-dlp");
        yt_dlp
            .arg("-t")
            .arg("mp3")
            .arg("-o")
            .arg(yuklenecek_yol_str)
            .arg(link);
        yt_dlp.output().expect("Youtube indirme islemi basarisiz!");
    }
    fn sarki_yukle() {
        let mut sarki_yolu = String::new();

        println!("Cihazinizda yuklu olan sarkinin dosya yolunu gosterin, ardindan sisteme yuklenecek.");
        println!("Bu islem sonrasinda orijinal dosyayi silebilirsiniz");
        println!("Ornek dosya yolu gosterimi (Linux icin): /home/kullanici_adi/Masaustu/yuklenecek_sarki.mp3 ");
        println!("\nSarki yuklemek istediginiz dosyanin yolunu giriniz:");

        std::io::stdin().read_line(&mut sarki_yolu).unwrap();
        let sarki_yolu_str = sarki_yolu.trim();
        let kaynak_yol = Path::new(sarki_yolu_str);

        if let Some(dosya_adi) = kaynak_yol.file_name() {
            let yuklenen_yol = Path::new("music").join(dosya_adi);

            match std::fs::copy(kaynak_yol, &yuklenen_yol) {
                Ok(_) => println!("Sarki yukleme islemi basarili! Dosya 'music' klasorune eklendi."),
                Err(e) => println!("Sarki yukleme islemi basarisiz: {}", e),
            }
        } else {
            println!("Gecerli bir dosya adı bulunamadi!");
        }
    }
    fn main() {
        let handle = rodio::DeviceSinkBuilder::open_default_sink()
            .expect("Default cihaza ulasilamadi!");
        let oynatici = rodio::Player::connect_new(&handle.mixer());

        loop {
            println!("MP-3 oynatiyiciya hos geldiniz, ne yapmak istersiniz? \n");
            println!("1-Sarki oynat");
            println!("2-Sarkilari Listele");
            println!("3-Ses arttir");
            println!("4-Ses azalt");
            println!("5-Durdur/Devam et");
            println!("6-Sarki yukle");
            println!("7-Online mp3 indir (yt-dlp gerektirir)");
            println!("8-Cikis\n\n");
            let mut secim = String::new();
            std::io::stdin().read_line(&mut secim).unwrap();
            let secim_sayi: i8 = secim.trim().parse().unwrap();
            match secim_sayi {
                1 => {
                    oynattir(&oynatici);
                    Command::new("clear").status().unwrap();
                }
                2 => {
                    sarki_listele(Path::new("music"));
                    println!("\n\n");
                }
                3 => {
                    oynatici.set_volume(oynatici.volume() + 0.1);
                    Command::new("clear").status().unwrap();
                }
                4 => {
                    oynatici.set_volume(oynatici.volume() - 0.1);
                    Command::new("clear").status().unwrap();
                }
                5 => {
                    if oynatici.is_paused() {
                        oynatici.play();
                    } else {
                        oynatici.pause();
                    }
                    Command::new("clear").status().unwrap();
                }
                6 => {
                    sarki_yukle();
                    sleep(std::time::Duration::from_secs(1));
                    Command::new("clear").status().unwrap();
                }
                7 => {
                    online_indir()
                }
                8 => {
                    break;
                }
                _ => {
                    println!("Gecersiz Islem!");
                }
            }
        }
    }
