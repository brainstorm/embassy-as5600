// use avr_tester::*;

// fn avr() -> AvrTester {
//     AvrTester::atmega328p()
//         .with_clock_of_16_mhz()
//         .load("target/atmega2560/debug/yourproject.elf")
// }

// // Assuming `yourproject` implements a ROT-13 encoder:

// #[test]
// fn short_text() {
//     let mut avr = avr();

//     // Let's give our firmware a moment to initialize:
//     avr.run_for_ms(1);

//     // Now, let's send the string:
//     avr.uart0().write("Hello, World!");

//     // ... give the AVR a moment to retrieve it & send back, encoded:
//     avr.run_for_ms(1);

//     // ... and, finally, let's assert the outcome:
//     assert_eq!("Uryyb, Jbeyq!", avr.uart0().read::<String>());
// }
