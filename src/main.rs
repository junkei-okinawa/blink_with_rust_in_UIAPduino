#![no_std]
#![no_main]

use panic_halt as _;
use ch32v0::ch32v003;

#[riscv_rt::entry]
fn main() ->! {
    let p = ch32v003::Peripherals::take().unwrap();
    
    // GPIO初期化 (UIAPduino の PC0 は BUILT IN LED に接続されている)
    p.RCC.apb2pcenr().modify(|_, w| w.iopcen().set_bit());
    p.GPIOC.cfglr().modify(|_, w| w.mode0().variant(0b01).cnf0().variant(0b00)); // PC0をPush-Pull出力に設定

    loop {
        for _ in 0..2 {
            p.GPIOC.outdr().modify(|_, w| w.odr0().clear_bit()); // LED点灯
            for _ in 0..300_000 { riscv::asm::nop(); }
    
            p.GPIOC.outdr().modify(|_, w| w.odr0().set_bit()); // LED消灯
            for _ in 0..300_000 { riscv::asm::nop(); }
        }
        p.GPIOC.outdr().modify(|_, w| w.odr0().clear_bit()); // LED点灯
        for _ in 0..1_000_000 { riscv::asm::nop(); }

        p.GPIOC.outdr().modify(|_, w| w.odr0().set_bit()); // LED消灯
        for _ in 0..1_000_000 { riscv::asm::nop(); }
    }
}
