EESchema Schematic File Version 4
EELAYER 30 0
EELAYER END
$Descr A4 11693 8268
encoding utf-8
Sheet 1 1
Title ""
Date ""
Rev ""
Comp ""
Comment1 ""
Comment2 ""
Comment3 ""
Comment4 ""
$EndDescr
$Comp
L Connector:Raspberry_Pi_2_3 J?
U 1 1 6097F771
P 3450 3400
F 0 "J?" H 3450 4881 50  0000 C CNN
F 1 "Raspberry_Pi_2_3" H 3450 4790 50  0000 C CNN
F 2 "" H 3450 3400 50  0001 C CNN
F 3 "https://www.raspberrypi.org/documentation/hardware/raspberrypi/schematics/rpi_SCH_3bplus_1p0_reduced.pdf" H 3450 3400 50  0001 C CNN
	1    3450 3400
	1    0    0    -1  
$EndComp
$Comp
L Motor:Stepper_Motor_unipolar_6pin M?
U 1 1 60984FA0
P 7300 3550
F 0 "M?" H 7488 3674 50  0000 L CNN
F 1 "Stepper_Motor_unipolar_6pin" H 7488 3583 50  0000 L CNN
F 2 "" H 7310 3540 50  0001 C CNN
F 3 "http://www.infineon.com/dgdl/Application-Note-TLE8110EE_driving_UniPolarStepperMotor_V1.1.pdf?fileId=db3a30431be39b97011be5d0aa0a00b0" H 7310 3540 50  0001 C CNN
	1    7300 3550
	1    0    0    -1  
$EndComp
Wire Wire Line
	3250 2100 3250 1900
Wire Wire Line
	5850 4400 5850 4700
Wire Wire Line
	5850 4700 3750 4700
Wire Wire Line
	5850 1900 5850 2850
Wire Wire Line
	3250 1900 5850 1900
Wire Wire Line
	6350 3650 6800 3650
Wire Wire Line
	6800 3650 6800 3450
Wire Wire Line
	6800 3450 7000 3450
Wire Wire Line
	6350 3750 6950 3750
Wire Wire Line
	6950 3750 6950 3650
Wire Wire Line
	6950 3650 7000 3650
Wire Wire Line
	7200 3250 6700 3250
Wire Wire Line
	6700 3250 6700 3550
Wire Wire Line
	6700 3550 6350 3550
Wire Wire Line
	6600 3450 6600 3150
Wire Wire Line
	6600 3150 7400 3150
Wire Wire Line
	7400 3150 7400 3250
$Comp
L Driver_Motor:Pololu_Breakout_A4988 A?
U 1 1 6097E596
P 5850 3550
F 0 "A?" H 5900 4431 50  0000 C CNN
F 1 "Pololu_Breakout_A4988" H 5900 4340 50  0000 C CNN
F 2 "Module:Pololu_Breakout-16_15.2x20.3mm" H 6125 2800 50  0001 L CNN
F 3 "https://www.pololu.com/product/2980/pictures" H 5950 3250 50  0001 C CNN
	1    5850 3550
	1    0    0    -1  
$EndComp
$Comp
L power:+12V #PWR?
U 1 1 609B9CD1
P 6050 1800
F 0 "#PWR?" H 6050 1650 50  0001 C CNN
F 1 "+12V" H 6065 1973 50  0000 C CNN
F 2 "" H 6050 1800 50  0001 C CNN
F 3 "" H 6050 1800 50  0001 C CNN
	1    6050 1800
	1    0    0    -1  
$EndComp
Wire Wire Line
	6050 1800 6050 2050
Wire Wire Line
	5450 3550 4850 3550
Wire Wire Line
	4850 3550 4850 2500
Wire Wire Line
	4850 2500 4250 2500
Wire Wire Line
	5450 3650 4750 3650
Wire Wire Line
	4750 3650 4750 2600
Wire Wire Line
	4750 2600 4250 2600
$Comp
L power:Earth #PWR?
U 1 1 609D2FFC
P 6450 1800
F 0 "#PWR?" H 6450 1550 50  0001 C CNN
F 1 "Earth" H 6450 1650 50  0001 C CNN
F 2 "" H 6450 1800 50  0001 C CNN
F 3 "~" H 6450 1800 50  0001 C CNN
	1    6450 1800
	-1   0    0    1   
$EndComp
Wire Wire Line
	6050 4350 6450 4350
Wire Wire Line
	6350 3450 6600 3450
Wire Wire Line
	6450 1800 6450 2050
$Comp
L Device:CP C?
U 1 1 609DE0DB
P 6250 2050
F 0 "C?" V 6505 2050 50  0000 C CNN
F 1 "100 uF" V 6414 2050 50  0000 C CNN
F 2 "" H 6288 1900 50  0001 C CNN
F 3 "~" H 6250 2050 50  0001 C CNN
	1    6250 2050
	0    -1   -1   0   
$EndComp
Wire Wire Line
	6100 2050 6050 2050
Connection ~ 6050 2050
Wire Wire Line
	6050 2050 6050 2850
Wire Wire Line
	6400 2050 6450 2050
Connection ~ 6450 2050
Wire Wire Line
	6450 2050 6450 4350
$EndSCHEMATC
