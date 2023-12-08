use embedded_hal::digital::v2::InputPin;
use rp_pico::hal::gpio::{Pin, PinId, Function, PullType, FunctionSio, SioOutput, ValidFunction, PullUp, SioInput};

pub struct PushButton<I: PinId> {
    pin: Pin<I, FunctionSio<SioInput>, PullUp>
}

impl<I: PinId> PushButton<I> {
    pub fn new<F: Function, P: PullType>(pin: Pin<I, F, P>) -> Self where I: ValidFunction<FunctionSio<SioInput>>, {
        PushButton {
            pin: pin.into_pull_up_input()
        }
    }

    pub fn is_pressed(&self) -> bool {
        self.pin.is_low().unwrap()
    }
}