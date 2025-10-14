import serial

ser: Serial = serial.Serial('/dev/ttyACM0')
ser.baudrate = 115200
ser.timeout = 3.5
ser.stopbit = 2

while True:
    message: string = ser.read(25)
    print(message)
