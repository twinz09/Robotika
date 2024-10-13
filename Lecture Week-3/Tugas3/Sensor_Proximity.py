from controller import Robot

robot = Robot()
time_step = int(robot.getBasicTimeStep())

# Mengambil motor
left_motor = robot.getDevice("left wheel motor")
right_motor = robot.getDevice("right wheel motor")

# Mengambil sensor proximity
proximity_sensor = robot.getDevice("ps7") 
proximity_sensor.enable(time_step) 
# Mengatur kecepatan awal motor
left_motor.setVelocity(5.0)  
right_motor.setVelocity(5.0)  

while robot.step(time_step) != -1:
    # Membaca nilai dari sensor proximity
    proximity_value = proximity_sensor.getValue()

    # objek terdeteksi
    if proximity_value > 80:  # Ambang batas jarak deteksi
        left_motor.setVelocity(0)  # Berhenti
        right_motor.setVelocity(0)  # Berhenti
    else:
        # Jika tidak ada objek, tetap bergerak maju
        left_motor.setVelocity(5.0)  # Kecepatan motor kiri
        right_motor.setVelocity(5.0)  # Kecepatan motor kanan
