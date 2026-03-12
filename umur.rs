use std::io;

fn main() {
    println!("=== Age Calculator ===");
    let tahun_lahir = input_tahun_lahir();
    let tahun_sekarang = input_tahun_sekarang();
    let umur = hitung_umur(tahun_lahir, tahun_sekarang);
    tampilin_hasil(tahun_lahir, tahun_sekarang, umur);
}

fn input_tahun_lahir() -> u32 {
    loop{
        println!("Masukan tahun lahir disini!");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Gagal membaca input");

        match input.trim().parse::<u32>(){
            Ok(tahun)=> {
                if tahun > 1800 && tahun <= 2026 {
                    return tahun;
                } else {
                    println!("❌ Tahun lahir harus diantara 1800 -2026, okay!");
                }
            }
            Err (_) => {
                println! ("❌ Input tidak valid, harus sesuai format bro!");
            }
        }
    }
}

fn input_tahun_sekarang() -> u32 {
    loop{
        println! ("Masukan tahun sekarang disini! (Jika tidak tahu (atau tekan Enter untuk menggunakan 2026); ");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Gagal membaca input");

        if input.trim().is_empty(){
            return 2026;
        }

        match input.trim().parse::<u32>(){
            Ok(tahun) => {
                if tahun >= 2000 && tahun <= 2026 {
                    return tahun;
                } else {
                    println!("❌ Tahun sekarang harus diantara 2000 - 2026, okay!");
                }
            }
            Err(_) => {
                println!("❌ Input tidak valid, harus sesuai format bro!");
            }
        }
    }
}

fn hitung_umur(tahun_lahir:u32, tahun_sekarang:u32) -> u32 {
    tahun_sekarang - tahun_lahir
}

fn tampilin_hasil(tahun_lahir:u32, tahun_sekarang:u32, umur:u32){
    println!("Tahun lahir kamu: {}", tahun_lahir);
    println!("Tahun sekarang: {}", tahun_sekarang);
    println!("Umur kamu adalah: {} tahun", umur);

    if umur < 17 {
        println! ("Kamu masih kecil!");
    } else if umur < 60 {
        println! ("Selamat menjadi dewasa!");
    } else if umur > 60 {
        println!("Selamat menjadi lansia!");
    }
}