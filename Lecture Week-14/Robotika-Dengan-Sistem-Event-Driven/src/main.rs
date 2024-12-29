use std::collections::VecDeque;

#[derive(Debug)]
enum Event {
    ObstacleDetected,
    GoalChanged((usize, usize)),
    BatteryLow,
}

fn main() {
    // Posisi awal robot dan tujuan
    let mut robot_position = (0, 0);
    let mut goal = (4, 4);

    // Antrian event
    let mut event_queue: VecDeque<Event> = VecDeque::new();

    // Tambahkan beberapa event ke dalam antrian
    event_queue.push_back(Event::ObstacleDetected);
    event_queue.push_back(Event::GoalChanged((3, 3)));
    event_queue.push_back(Event::BatteryLow);

    println!("Simulasi Robotika dengan Sistem Event-Driven\n");
    println!("Posisi awal robot: ({}, {})", robot_position.0, robot_position.1);
    println!("Tujuan awal robot: ({}, {})\n", goal.0, goal.1);

    while let Some(event) = event_queue.pop_front() {
        match event {
            Event::ObstacleDetected => {
                println!("Event: Rintangan terdeteksi!");
                // Robot akan mengubah jalurnya untuk menghindari rintangan
                avoid_obstacle(&mut robot_position);
                println!("Robot menghindari rintangan. Posisi saat ini: ({}, {})", robot_position.0, robot_position.1);
            }
            Event::GoalChanged(new_goal) => {
                println!("Event: Tujuan diubah ke ({}, {})!", new_goal.0, new_goal.1);
                goal = new_goal;
            }
            Event::BatteryLow => {
                println!("Event: Baterai lemah!");
                // Robot akan kembali ke stasiun pengisian daya
                go_to_charging_station(&mut robot_position);
                println!("Robot menuju stasiun pengisian daya. Posisi saat ini: ({}, {})", robot_position.0, robot_position.1);
            }
        }
    }

    println!("\nSemua event telah diproses.");
    println!("Posisi akhir robot: ({}, {})", robot_position.0, robot_position.1);
    println!("Tujuan akhir robot: ({}, {})", goal.0, goal.1);
}

// Fungsi untuk menghindari rintangan
fn avoid_obstacle(position: &mut (usize, usize)) {
    position.0 += 1; // Misalnya, robot bergerak ke bawah untuk menghindari rintangan
}

// Fungsi untuk menuju stasiun pengisian daya
fn go_to_charging_station(position: &mut (usize, usize)) {
    *position = (0, 0); // Stasiun pengisian daya berada di (0, 0)
}
