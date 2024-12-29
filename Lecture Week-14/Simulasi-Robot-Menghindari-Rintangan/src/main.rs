use std::collections::{HashSet, VecDeque};

fn main() {
    // Peta 2D yang merepresentasikan area dengan rintangan
    let grid = [
        [0, 0, 1, 0, 0],
        [0, 1, 1, 0, 0],
        [0, 0, 0, 0, 1],
        [1, 1, 1, 0, 0],
        [0, 0, 0, 0, 0],
    ];

    // Titik awal dan tujuan
    let start = (0, 0);
    let goal = (4, 4);

    println!("Simulasi Robot Menghindari Rintangan");
    println!("Peta:");
    print_grid(&grid, start);

    // Jalur yang ditemukan menggunakan BFS
    match bfs_find_path(&grid, start, goal) {
        Some(path) => {
            println!("Robot berhasil mencapai tujuan. Jalur yang diambil:");
            for (x, y) in path {
                println!("({},{})", x, y);
            }
        }
        None => {
            println!("Robot tidak dapat mencapai tujuan. Terhalang rintangan atau jalan buntu.");
        }
    }
}

// Fungsi untuk mencetak peta
fn print_grid(grid: &[[i32; 5]; 5], robot_position: (usize, usize)) {
    for (i, row) in grid.iter().enumerate() {
        for (j, &cell) in row.iter().enumerate() {
            if (i, j) == robot_position {
                print!("R "); // Posisi robot
            } else if cell == 1 {
                print!("X "); // Rintangan
            } else {
                print!(". "); // Jalan bebas
            }
        }
        println!();
    }
}

// Fungsi BFS untuk menemukan jalur
fn bfs_find_path(
    grid: &[[i32; 5]; 5],
    start: (usize, usize),
    goal: (usize, usize),
) -> Option<Vec<(usize, usize)>> {
    let mut queue: VecDeque<(usize, usize, Vec<(usize, usize)>)> = VecDeque::new();
    let mut visited: HashSet<(usize, usize)> = HashSet::new();

    // Memulai dari posisi awal
    queue.push_back((start.0, start.1, vec![start]));
    visited.insert(start);

    // Arah gerakan: kanan, bawah, kiri, atas
    let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];

    while let Some((x, y, path)) = queue.pop_front() {
        if (x, y) == goal {
            return Some(path); // Jika tujuan tercapai, kembalikan jalur
        }

        // Coba semua arah
        for (dx, dy) in directions.iter() {
            let nx = x as isize + dx;
            let ny = y as isize + dy;

            if nx >= 0 && ny >= 0 && nx < 5 && ny < 5 {
                let nx = nx as usize;
                let ny = ny as usize;

                // Periksa apakah jalur valid
                if grid[nx][ny] == 0 && !visited.contains(&(nx, ny)) {
                    let mut new_path = path.clone();
                    new_path.push((nx, ny));
                    queue.push_back((nx, ny, new_path));
                    visited.insert((nx, ny));
                }
            }
        }
    }

    None // Tidak ada jalur yang ditemukan
}
