// Some Pins I have prepared before wanted to save them some where

// // Acquire the GPIO peripheral
//     let mut gpioa = dp.GPIOA.split();
//     let mut gpiob = dp.GPIOB.split();
//     let mut gpioc = dp.GPIOC.split();
//     let mut gpiod = dp.GPIOD.split();

//     // RGB controll LEDs
//     let mut rled = gpioa.pa12.into_push_pull_output(&mut gpioa.crh);
//     let mut bled = gpioa.pa11.into_push_pull_output(&mut gpioa.crh);
//     let mut gled = gpiob.pb12.into_push_pull_output(&mut gpiob.crh);

//     // Stepper Control Pins
//     let mut StepperPWM01 = gpioc.pc7.into_alternate_open_drain(&mut gpioc.crl);
//     let mut StepperPWM01 = gpiob.pb5.into_alternate_open_drain(&mut gpiob.crl);
//     let mut StepperDir01 = gpiod.pd0.into_push_pull_output(&mut gpiod.crl);
//     let mut StepperDir02 = gpiod.pd1.into_push_pull_output(&mut gpiod.crl);
    
//     // Magnetometer
//     let mut mag_sda = gpioc.pc9.into_analog(&mut gpioc.crh);
//     let mut mag_sda = gpiob.pb8.into_analog(&mut gpiob.crh);