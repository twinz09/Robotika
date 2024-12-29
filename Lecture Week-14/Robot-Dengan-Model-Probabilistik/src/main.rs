use rand::Rng;

#[derive(Debug)]
struct SensorReading {
    actual_position: (f32, f32),
    noisy_position: (f32, f32),
}

fn main() {
    // Posisi awal robot dan tujuan
    let mut robot_position = (0.0, 0.0);
    let goal = (4.0, 4.0);

    println!("Simulasi Robot dengan Model Probabilistik\n");
    println!("Posisi awal robot: ({:.1}, {:.1})", robot_position.0, robot_position.1);
    println!("Tujuan robot: ({:.1}, {:.1})\n", goal.0, goal.1);

    // Simulasi pergerakan robot hingga mencapai tujuan
    while distance(robot_position, goal) > 0.5 {
        let sensor_reading = simulate_sensor(robot_position);

        println!(
            "Pembacaan sensor: Posisi aktual ({:.1}, {:.1}), Posisi dengan noise ({:.1}, {:.1})",
            sensor_reading.actual_position.0,
            sensor_reading.actual_position.1,
            sensor_reading.noisy_position.0,
            sensor_reading.noisy_position.1
        );

        // Gunakan model probabilistik untuk memperbaiki posisi berdasarkan pembacaan sensor
        robot_position = kalman_filter(robot_position, sensor_reading.noisy_position);

        println!(
            "Robot bergerak ke posisi baru: ({:.1}, {:.1})",
            robot_position.0, robot_position.1
        );
    }

    println!("\nRobot telah mencapai tujuan di ({:.1}, {:.1})", goal.0, goal.1);
}

// Fungsi untuk menghitung jarak antara dua titik
fn distance(a: (f32, f32), b: (f32, f32)) -> f32 {
    ((a.0 - b.0).powi(2) + (a.1 - b.1).powi(2)).sqrt()
}

// Fungsi untuk mensimulasikan pembacaan sensor dengan noise
fn simulate_sensor(position: (f32, f32)) -> SensorReading {
    let mut rng = rand::thread_rng();
    let noise_x: f32 = rng.gen_range(-0.5..0.5); // Noise pada sumbu x
    let noise_y: f32 = rng.gen_range(-0.5..0.5); // Noise pada sumbu y

    SensorReading {
        actual_position: position,
        noisy_position: (position.0 + noise_x, position.1 + noise_y),
    }
}

// Fungsi untuk memperbaiki posisi menggunakan Kalman Filter sederhana
fn kalman_filter(actual_position: (f32, f32), noisy_position: (f32, f32)) -> (f32, f32) {
    let kalman_gain = 0.8; // Gain untuk Kalman Filter

    (
        actual_position.0 + kalman_gain * (noisy_position.0 - actual_position.0),
        actual_position.1 + kalman_gain * (noisy_position.1 - actual_position.1),
    )
}
