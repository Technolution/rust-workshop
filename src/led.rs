use embedded_hal::digital::v2::{InputPin, PinState, OutputPin};
use rp_pico::hal::{gpio::{Pin, PinId, Function, PullType, FunctionSio, SioOutput, ValidFunction, PullUp, SioInput, PullDown, DynPullType}};

pub struct Led<I: PinId> {
    pin: Pin<I, FunctionSio<SioOutput>, PullDown>
}

impl<I: PinId> Led<I> {
    pub fn new<F: Function>(pin: Pin<I, F, PullDown>) -> Self where I: ValidFunction<FunctionSio<SioOutput>>, {
        Led {
            pin: pin.into_push_pull_output_in_state(PinState::Low)
        }
    }

    pub fn set_led(&mut self, b: bool) {
        if b {
            self.pin.set_high().unwrap()
        } else {
            self.pin.set_low().unwrap()
        }
        
    }
}