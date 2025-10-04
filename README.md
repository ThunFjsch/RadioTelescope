# Radio Image Telescop project

## Introduction

The Radio Telescope utilises a Satelite Dish to scan the night sky controlled with an Embedded interface that communicates with a client. The Embedded Interface transmits the information gathered to the client. After the scan is finished and all the information is transmitted, the client creates an image from the received radio frequencies.  

## Telescope Controller



### Planned Components:
- Magnetometer (HMC5883L) [Documentation](http://https://m.media-amazon.com/images/I/B1fYZAVkZFL.pdf)
- Stepper Driver (A4988) [Documentation](https://https://cdn.shopify.com/s/files/1/1509/1638/files/A4988_Stepper_Motor_Driver_Datenblatt_AZ-Delivery_Vertriebs_GmbH.pdf?v=1608626085)
- Microcontroller (STM32 F1) 
  - [Shop Page](https://www.reichelt.de/de/de/shop/produkt/nucleo-64_arm_cortex_m3_stm32_f1-serie-154270?PROVID=2788)
  -  [Quick Pin Layout](https://os.mbed.com/platforms/ST-Nucleo-L073RZ/)
  -  [Documentation](https://cdn-reichelt.de/documents/datenblatt/A300/NUCLEO_MANUAL_EN.pdf)
- RGB LED
- Nema 17 Stepper Motor
- BJT NPN Transistor [Documentation](https://cdn.sparkfun.com/assets/d/5/e/5/d/BC547.pdf)
- USB to serial chip CH340 [Documentation](https://www.mpja.com/download/35227cpdata.pdf)

## Current informations
For the first milestone of this project, the radio telescop should be able to measure the temperature of the sun.
For this milestone I will use the lab experiment that I found. There are many points where knowledge can be deepend and a direction what the telescope controller should be capable of.

[Lab Course Physics](https://teaching.astro.uni-koeln.de/sites/default/files/praktikum_m/Radio_astronomy_v1.4_engl.pdf)

The added challenge is that the reading of the Signal requires to read the Satelite dish input. Which will probably be an [Low-noise block downconverter](http://https://en.wikipedia.org/wiki/Low-noise_block_downconverter).
As far as I understand it filters the incoming signal to low noise, up to 11.1GHz. I think that the input signal will be read by the Analog to Digital converter of the Microcontroller. Then later processed by the Client Application.

My current guess is that I need to implement some form of [Spherical coordinate system](https://en.wikipedia.org/wiki/Spherical_coordinate_system), for the localisation of the target scan area. My Plan to reach this milestone is to track the movement of the moon.
[More to the subject](https://www.youtube.com/watch?v=ACHACvEAXUE)