use std::collections::VecDeque;

fn main() {
    // Matriks 2D yang merepresentasikan peta
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

    // Jalur yang dihasilkan
    match find_path(&grid, start, goal) {
        Some(path) => {
            println!("Jalur ditemukan:");
            for (x, y) in path {
                println!("({},{})", x, y);
            }
        }
        None => println!("Tidak ada jalur yang ditemukan."),
    }
}

// Fungsi untuk menemukan jalur menggunakan algoritma BFS
fn find_path(grid: &[[i32; 5]; 5], start: (usize, usize), goal: (usize, usize)) -> Option<Vec<(usize, usize)>> {
    let mut queue: VecDeque<(usize, usize, Vec<(usize, usize)>)> = VecDeque::new();
    let mut visited = [[false; 5]; 5];
    
    // Menambahkan titik awal ke antrian
    queue.push_back((start.0, start.1, vec![start]));
    visited[start.0][start.1] = true;

    // Arah gerakan: kanan, bawah, kiri, atas
    let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];

    while let Some((x, y, path)) = queue.pop_front() {
        // Jika mencapai tujuan, kembalikan jalur
        if (x, y) == goal {
            return Some(path);
        }

        // Periksa semua arah
        for (dx, dy) in directions.iter() {
            let nx = x as isize + dx;
            let ny = y as isize + dy;

            // Pastikan gerakan berada dalam batas peta dan tidak terhalang
            if nx >= 0 && ny >= 0 && nx < 5 && ny < 5 {
                let nx = nx as usize;
                let ny = ny as usize;
                if grid[nx][ny] == 0 && !visited[nx][ny] {
                    let mut new_path = path.clone();
                    new_path.push((nx, ny));
                    queue.push_back((nx, ny, new_path));
                    visited[nx][ny] = true;
                }
            }
        }
    }

    None // Tidak ada jalur yang ditemukan
}
