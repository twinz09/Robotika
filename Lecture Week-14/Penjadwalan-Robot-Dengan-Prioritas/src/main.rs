use std::collections::BinaryHeap;
use std::cmp::Reverse;

#[derive(Debug)]
struct Task {
    name: String,
    priority: u32,
}

fn main() {
    // Daftar tugas robot
    let tasks = vec![
        Task {
            name: String::from("Ambil barang dari lokasi A"),
            priority: 2,
        },
        Task {
            name: String::from("Antar barang ke lokasi B"),
            priority: 1,
        },
        Task {
            name: String::from("Isi ulang daya baterai"),
            priority: 3,
        },
    ];

    // Antrian prioritas
    let mut priority_queue: BinaryHeap<Reverse<Task>> = BinaryHeap::new();

    // Masukkan semua tugas ke dalam antrian prioritas
    for task in tasks {
        priority_queue.push(Reverse(task));
    }

    println!("Robot mulai menyelesaikan tugas berdasarkan prioritas:");
    while let Some(Reverse(task)) = priority_queue.pop() {
        println!("Menyelesaikan tugas: {}", task.name);
    }
}

impl Eq for Task {}

impl PartialEq for Task {
    fn eq(&self, other: &Self) -> bool {
        self.priority == other.priority
    }
}

impl Ord for Task {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.priority.cmp(&self.priority) // Prioritas tertinggi lebih dulu
    }
}

impl PartialOrd for Task {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
