//! Note: Without additional hardware, PC13 should not be used to drive an LED, see page 5.1.2 of
//! the reference manual for an explanation. This is not an issue on the blue pill.

#![deny(unsafe_code)]
#![no_std]
#![no_main]

use panic_rtt_target as _;

use nb::block;

use cortex_m_rt::entry;
use embedded_hal::digital::v2::OutputPin;
use stm32f1xx_hal::{pac, prelude::*, timer::Timer, rtc::Rtc};
use rtt_target::rprintln;
use chrono::{Utc, TimeZone, Datelike, Timelike};

#[entry]
fn main() -> ! {
    let cp = cortex_m::Peripherals::take().unwrap();
    let dp = pac::Peripherals::take().unwrap();
    rtt_target::rtt_init_print!();
    rprintln!("rise-and-shine");
    // writeln!(output, "Build time: {}", env!("VERGEN_BUILD_TIMESTAMP")).ok();
    // writeln!(output, "SHA: {}", env!("VERGEN_GIT_SHA_SHORT")).ok();

    let mut flash = dp.FLASH.constrain();
    let mut rcc = dp.RCC.constrain();
    let clocks = rcc.cfgr.freeze(&mut flash.acr);

    // Init RTC
    let mut pwr = dp.PWR;
    let mut backup_domain = rcc.bkp.constrain(dp.BKP, &mut rcc.apb1, &mut pwr);
    let mut rtc = Rtc::rtc(dp.RTC, &mut backup_domain);
    if rtc.current_time() == 0 {
        let t = Utc.ymd(2021, 10, 11).and_hms(22, 08, 00).timestamp();
        rtc.set_time(t as u32);
    }

    // Acquire GPIOs
    let mut gpioa = dp.GPIOA.split(&mut rcc.apb2);
    let mut gpiob = dp.GPIOB.split(&mut rcc.apb2);

    // Release pins taken by hw by default
    let mut afio = dp.AFIO.constrain(&mut rcc.apb2);
    let (_pa15, _pb3, pb4) = afio.mapr.disable_jtag(gpioa.pa15, gpiob.pb3, gpiob.pb4);

    let mut led = pb4.into_push_pull_output(&mut gpiob.crl);

    // Configure the syst timer to trigger an update every second
    let mut timer = Timer::syst(cp.SYST, &clocks).start_count_down(2.hz());

    loop {
        block!(timer.wait()).unwrap();
        led.set_high().unwrap();
        block!(timer.wait()).unwrap();
        led.set_low().unwrap();

        let t = rtc.current_time();
        let t = Utc.timestamp(t as i64, 0);
        rprintln!("Time: {}/{:02}/{:02} {:02}:{:02}:{:02}", t.year(), t.month(), t.day(), t.hour(), t.minute(), t.second());
    }
}
