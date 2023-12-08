use cortex_m::{peripheral::SYST, delay::Delay};
use rp_pico::{pac::{WATCHDOG, XOSC, CLOCKS, PLL_SYS, PLL_USB, RESETS}, hal::{self, Clock}};

pub fn init_delay(watchdog: WATCHDOG, xosc: XOSC, clocks: CLOCKS, pll_sys: PLL_SYS, pll_usb: PLL_USB, syst: SYST, mut resets: &mut RESETS) -> Delay {
        // Set up the watchdog driver - needed by the clock setup code
        let mut watchdog = hal::Watchdog::new(watchdog);
    
        // Configure the clocks
        //
        // The default is to generate a 125 MHz system clock
        let clocks = hal::clocks::init_clocks_and_plls(
            rp_pico::XOSC_CRYSTAL_FREQ,
            xosc,
            clocks,
            pll_sys,
            pll_usb,
            &mut resets,
            &mut watchdog,
        )
            .ok()
            .unwrap();
    
        // The delay object lets us wait for specified amounts of time (in
        // milliseconds)
        cortex_m::delay::Delay::new(syst, clocks.system_clock.freq().to_Hz())
    }
    