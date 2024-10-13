from controller import Robot

# Inisialisasi robot
robot = Robot()

# Mendapatkan referensi ke motor
left_motor = robot.getDevice('left wheel motor')
right_motor = robot.getDevice('right wheel motor')

# Mengatur kecepatan motor
speed = 5.0  # Kecepatan robot saat bergerak maju

# Mengatur kecepatan motor
left_motor.setVelocity(speed)
right_motor.setVelocity(speed)

# Loop utama
while robot.step(32) != -1:
    # Robot bergerak maju tanpa henti
    pass  # Loop ini akan terus berjalan selama simulasi
