fn main() {

    println!("Hello, world!");
    println!("");
    println!("");

    println!("### Data Types ###");

    // Struct Klasik. Termasuk nama field dan tipe datanya
    struct Student {
        name: String,
        level: u8,
        remote: bool
    }

    // Tuple Struct. Hanya tipe data yang disebutkan di sini
    struct Grades(char, char, char, f32);

    // Memulai fungsi utama. 'fn' adalah kata kunci di Rust yang menunjukkan bahwa ini adalah sebuah fungsi.
    // Fungsi dapat memiliki parameter yang kita berikan, dan juga dapat mengembalikan nilai. Penjelasan lebih lanjut nanti.

    // println! adalah makro bawaan dari Rust. Ini mencetak output ke konsol.
    // Kita dapat menggunakan substitusi nilai untuk argumen {}
    println!("Hello, {} {}!", "Will", "Velida");

    // Binding variabel secara default bersifat immutable, artinya setelah kita menetapkan nilai ke variabel, nilainya tidak bisa diubah.
    // Kita bisa menggunakan kata kunci 'mut' untuk membuat variabel menjadi mutable (artinya kita bisa mengubah nilainya SETELAH dideklarasikan).
    let mut age = 33;
    let birth_year = 1991;
    println!("Saya berusia {} tahun", age);

    // Kita juga bisa mendeklarasikan variabel baru yang menggunakan nama variabel yang sudah ada.
    // Ini disebut 'variable shadowing' karena variabel ini 'menaungi' variabel sebelumnya.
    // Variabel sebelumnya masih ada, tetapi tidak dapat dirujuk lagi di scope ini.
    let birth_year = birth_year - 1;
    age = 34;
    println!("Sekarang saya berusia {} tahun", age);
    println!("Saya lahir tahun {}", birth_year);

    // Rust adalah bahasa dengan tipe statis. Compiler harus mengetahui tipe data yang tepat untuk semua variabel dalam kode Anda agar dapat dikompilasi dan dijalankan.
    // Compiler biasanya dapat menyimpulkan tipe data dari variabel berdasarkan nilai yang diberikan.
    // Di sini, kita memberitahu compiler untuk membuat variabel nephew_age sebagai bilangan bulat 32-bit.
    // Kita menentukan tipe data u32 setelah nama variabel
    let nephew_age: u32 = 14;
    println!("Keponakan saya berusia {} tahun", nephew_age);

    // Rust memiliki beberapa tipe data bawaan.

    // Float (bilangan desimal)
    let _float: f32 = 4.0;
    println!("1 x 2 = {}", 1*2);

    // Boolean: nilai benar atau salah.
    let is_bigger_num = 2 < 4;
    println!("Apakah 2 < 4: {}", is_bigger_num);

    // String
    // Tipe karakter. Sebuah karakter tunggal.
    let first_char: char = 'W';
    let last_char: char = 'l';
    let second_char = 'i';

    // String adalah rangkaian karakter.
    // Sebagian besar waktu, literal String bertipe &str
    let my_name = "Will";
    println!("{} adalah karakter pertama, {} adalah karakter terakhir, {} adalah karakter kedua dari nama saya {}", first_char, last_char, second_char, my_name);

    // Tuple adalah pengelompokan nilai dengan berbagai tipe yang dikumpulkan menjadi satu nilai gabungan.
    // Nilai individu di dalam tuple disebut elemen.
    // Tuple memiliki panjang tetap. Setelah dideklarasikan, panjangnya tidak dapat bertambah atau berkurang, dan elemen tidak dapat ditambahkan atau dihapus.
    let my_dog = ("Toby", 15, false);
    println!("Nama anjing saya {}, usianya {} tahun, apakah dia masih hidup? {}", my_dog.0, my_dog.1, my_dog.2);

    // Setelah kita mendefinisikan tipe struct, kita bisa menggunakannya dengan membuat instance tipe tersebut dan menentukan nilai untuk setiap field.
    let student_1 = Student{
        name: String::from("Will Velida"),
        remote: true,
        level: 5
    };

    let grades = Grades('A', 'A', 'B', 3.5);

    // Untuk struct klasik, kita bisa mendapatkan nilai dengan merujuk pada nama propertinya
    println!("{}, adalah programmer level {}. Apakah dia bekerja secara remote: {}",
        student_1.name, student_1.level, student_1.remote);

    // Untuk tuple struct, kita mendapatkan nilai dengan merujuk pada posisinya dalam indeks.
    println!("{},{},{},GPA = {}", grades.0, grades.1, grades.2, grades.3);

    println!("### If Else ###");

    // Pernyataan if/else dasar
    if 1 == 2 {
        println!("Angkanya sama");
    } else {
        println!("Angkanya tidak sama");
    }

    // Mengikat nilai ke variabel menggunakan pernyataan if/else
    let sunny_day = true;
    let take_jacket = if sunny_day {
        "Jangan bawa jaket"
    } else {
        "Bawa jaket"
    };

    println!("{}", take_jacket);

    // Menggunakan beberapa pernyataan if/else untuk mengevaluasi beberapa kondisi
    let num = 100;
    let out_of_range: bool;

    if num < 0 {
        out_of_range = true;
    } else if num == 0 {
        out_of_range = true;
    } else if num > 101 {
        out_of_range = true;
    } else {
        out_of_range = false;
    }
    println!("{}", out_of_range);

    println!("");
    println!("");
    println!("### Arrays ###");

    // Array: Koleksi objek dengan tipe yang sama disimpan secara berurutan di memori
    // Anda dapat mendeklarasikan array, menginisialisasi semua nilai, dan compiler akan menyimpulkan panjangnya
    let _working_days_num = ["Senin", "Selasa", "Rabu", "Kamis", "Jumat"];

    // Anda juga dapat mendeklarasikan array, menginisialisasi semua nilai, dan menentukan panjangnya
    let _working_days_num = [0; 5];

    // Kita juga bisa mengakses elemen dalam array menggunakan indeks.
    println!("{}", _working_days_num[0]);

    // Vektor juga menyimpan beberapa nilai dengan tipe data yang sama
    // Kita bisa mendeklarasikan vektor, menginisialisasi semua nilai
    let nephews_age = vec![14, 9, 0];
    println!("Usia keponakan: {:?}", nephews_age);

    // Anda juga bisa mendeklarasikan vektor, menginisialisasi semua nilai, dan menentukan panjangnya
    let zeroes = vec![0; 5];
    println!("Zeroes: {:?}", zeroes);

    // Kita bisa menambah atau menghapus nilai pada vektor menggunakan metode push atau pop.
    let mut names = Vec::new();

    names.push("Will");
    names.push("Isaac");
    names.push("Sam");

    println!("Nama: {:?}", names);

    names.pop();
    println!("Nama: {:?}", names);

    // Kita juga bisa mengakses elemen di vektor dengan posisinya dalam vektor
    let mut fruit = vec!["Apel", "Melon", "Jeruk"];
    let orange = fruit[2];
    fruit[0] = "Stroberi";
    println!("Buah-buahan: {:?}, Jeruk = {}", fruit, orange);

    println!("");
    println!("");
    println!("### For While Loops ###");

    // Penggunaan loop untuk terus menjalankan aksi
    let mut counter = 1;
    let loop_stop = loop {
        counter *= 4;
        if counter > 100 {
            break counter;
        }
    };

    println!("Berhenti pada counter = {}", loop_stop);

    // While loops menggunakan kondisi. Loop akan diulang selama kondisi benar.
    let mut num = 0;
    while num < 10 {
        println!("Halo!");
        num = num + 1;
    }

    // For loop menggunakan iterator untuk memproses koleksi item.
    let shopping_list = ["susu", "keju", "roti", "apel"];

    // Nilai iterator terikat pada hasil metode iter()
    for item in shopping_list.iter() {
        println!("Barang belanjaan berikutnya adalah {}", item);
    }

    // Cara lain untuk membuat iterator adalah menggunakan notasi range a..b
    for number in 0..10 {
        println!("Angka {}", number);
    }

    println!("");
    println!("");
    println!("### Hash Maps ###");

    // Perintah 'use' memasukkan definisi HashMap dari pustaka standar Rust.
    use std::collections::HashMap;
    let mut items: HashMap<String, String> = HashMap::new();

    // Menambahkan elemen ke hash map menggunakan metode insert(<key>, <value>)
    items.insert(String::from("Satu"), String::from("Buku"));
    items.insert(String::from("Dua"), String::from("Keyboard"));
    items.insert(String::from("Tiga"), String::from("Kacamata"));

    // Mendapatkan nilai dari hash map menggunakan metode get(<key>)
    let keyboard = items.get("Dua");
    println!("{:?}", keyboard);

    // Menghapus entri dari hash map menggunakan metode remove(<key>)
    items.remove("Tiga");
    println!("{:?}", items.get("Tiga"));
}
