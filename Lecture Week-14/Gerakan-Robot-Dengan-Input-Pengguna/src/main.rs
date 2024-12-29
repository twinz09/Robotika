use std::io;

fn main() {
    // Posisi awal robot
    let mut position = (0, 0);

    println!("Posisi awal robot: ({}, {})", position.0, position.1);

    loop {
        println!("\nMasukkan perintah untuk menggerakkan robot:");
        println!("W: Atas, S: Bawah, A: Kiri, D: Kanan, Q: Keluar");

        // Membaca input pengguna
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Gagal membaca input");

        // Menghapus spasi atau newline dari input
        let command = input.trim().to_uppercase();

        // Memproses input pengguna
        match command.as_str() {
            "W" => {
                position.1 += 1;
                println!("Robot bergerak ke atas.");
            }
            "S" => {
                position.1 -= 1;
                println!("Robot bergerak ke bawah.");
            }
            "A" => {
                position.0 -= 1;
                println!("Robot bergerak ke kiri.");
            }
            "D" => {
                position.0 += 1;
                println!("Robot bergerak ke kanan.");
            }
            "Q" => {
                println!("Keluar dari program. Posisi akhir robot: ({}, {})", position.0, position.1);
                break;
            }
            _ => {
                println!("Perintah tidak valid. Coba lagi.");
            }
        }

        // Menampilkan posisi saat ini
        println!("Posisi saat ini: ({}, {})", position.0, position.1);
    }
}
