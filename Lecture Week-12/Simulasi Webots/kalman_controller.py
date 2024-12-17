from controller import Robot

# Konstanta
TIME_STEP = 32
WHEEL_RADIUS = 0.02  # Radius roda dalam meter
AXLE_LENGTH = 0.052  # Jarak antar roda kiri dan kanan
RANGE = 1024  # Rentang nilai sensor proximity
THRESHOLD = 80.0  # Ambang batas jarak sensor untuk deteksi rintangan
MAX_SPEED = 6.28  # Kecepatan maksimum motor

# Fungsi Kalman Filter
def kalman_filter(z, u, x, P):
    # Prediksi langkah
    x_pred = x + u
    P_pred = P + 0.1  # Noise proses

    # Koreksi langkah
    K = P_pred / (P_pred + 1)  # Gain Kalman
    x = x_pred + K * (z - x_pred)  # Pembaruan posisi
    P = (1 - K) * P_pred  # Pembaruan ketidakpastian
    return x, P

# Fungsi menghitung odometri
def compute_odometry(left_encoder, right_encoder, last_left, last_right):
    left_value = left_encoder.getValue()
    right_value = right_encoder.getValue()
    dl = (left_value - last_left) * WHEEL_RADIUS
    dr = (right_value - last_right) * WHEEL_RADIUS
    delta_orientation = (dr - dl) / AXLE_LENGTH  # Perubahan orientasi
    distance_travelled = (dl + dr) / 2.0  # Jarak rata-rata
    return distance_travelled, delta_orientation, left_value, right_value

# Inisialisasi robot
robot = Robot()

# Motor roda
left_motor = robot.getDevice("left wheel motor")
right_motor = robot.getDevice("right wheel motor")
left_motor.setPosition(float('inf'))
right_motor.setPosition(float('inf'))
left_motor.setVelocity(0.0)
right_motor.setVelocity(0.0)

# Encoder roda
left_encoder = robot.getDevice("left wheel sensor")
right_encoder = robot.getDevice("right wheel sensor")
left_encoder.enable(TIME_STEP)
right_encoder.enable(TIME_STEP)

# Sensor proximity
sensors = []
sensor_names = ["ps0", "ps1", "ps2", "ps3", "ps4", "ps5", "ps6", "ps7"]
for name in sensor_names:
    sensor = robot.getDevice(name)
    sensor.enable(TIME_STEP)
    sensors.append(sensor)

# Variabel Kalman Filter
x = 0.0  # Posisi awal
P = 1.0  # Ketidakpastian awal

# Variabel untuk odometri
last_left_encoder = 0.0
last_right_encoder = 0.0

# Loop utama
while robot.step(TIME_STEP) != -1:
    # Ambil nilai sensor proximity
    sensors_value = [sensor.getValue() for sensor in sensors]

    # Deteksi rintangan di depan
    front_obstacle = sensors_value[0] > THRESHOLD or sensors_value[7] > THRESHOLD

    if front_obstacle:
        print("Rintangan terdeteksi! Berbelok untuk menghindar.")
        # Logika untuk menghindari rintangan
        left_motor.setVelocity(-MAX_SPEED / 2)  # Mundur roda kiri
        right_motor.setVelocity(MAX_SPEED / 2)  # Berbelok tajam ke kanan
        robot.step(500)  # Tunggu sebentar untuk membebaskan diri
        continue  # Lewati iterasi untuk menghindari stuck

    # Hitung odometri
    distance_travelled, delta_orientation, last_left_encoder, last_right_encoder = compute_odometry(
        left_encoder, right_encoder, last_left_encoder, last_right_encoder
    )

    # Gunakan sensor proximity sebagai input pengukuran
    z = (sensors_value[0] + sensors_value[7]) / 2.0  # Rata-rata sensor depan
    z = z / RANGE  # Normalisasi nilai sensor

    # Estimasi pergerakan robot (input u)
    u = distance_travelled

    # Terapkan Kalman Filter
    x, P = kalman_filter(z, u, x, P)

    # Set kecepatan motor untuk maju
    left_motor.setVelocity(MAX_SPEED / 2)
    right_motor.setVelocity(MAX_SPEED / 2)

    # Debugging
    print(f"Sensor Proximity: {[round(val, 2) for val in sensors_value]}")
    print(f"Jarak tempuh: {distance_travelled:.4f} m, Perubahan orientasi: {delta_orientation:.4f} rad")
    print(f"Estimasi Posisi Robot (Kalman Filter): {x:.4f} m")
    print("-" * 50)
