from controller import Robot

# Inisialisasi robot
robot = Robot()

# Mendapatkan referensi ke motor
left_motor = robot.getDevice('left wheel motor')
right_motor = robot.getDevice('right wheel motor')

# Mengatur kecepatan motor
left_speed = 2.0   # Kecepatan roda kiri lebih lambat
right_speed = 5.0  # Kecepatan roda kanan lebih cepat

# Mengatur kecepatan motor
left_motor.setVelocity(left_speed)
right_motor.setVelocity(right_speed)

# Loop utama
while robot.step(32) != -1:
    # Robot bergerak melingkar tanpa henti
    pass  # Loop ini akan terus berjalan selama simulasi
