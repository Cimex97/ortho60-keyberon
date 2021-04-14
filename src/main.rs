#![no_main]
#![no_std]

use core::convert::Infallible;
use embedded_hal::digital::v2::{InputPin, OutputPin};
use generic_array::typenum::{U8, U10};
use keyberon::debounce::Debouncer;
use keyberon::impl_heterogenous_array;
use keyberon::key_code::{KbHidReport, KeyCode};
use keyberon::layout::Layout;
use keyberon::matrix::{Matrix, PressedKeys};
use panic_semihosting as _;
use rtic::app;
use stm32f1xx_hal::gpio::{gpioa::*, gpiob::*, Input, Output, PullUp, PushPull};
use stm32f1xx_hal::prelude::*;
use stm32f1xx_hal::usb::{Peripheral, UsbBus, UsbBusType};
use stm32f1xx_hal::{gpio, pac, timer};
use usb_device::bus::UsbBusAllocator;
use usb_device::class::UsbClass as _;

mod layout;

type UsbClass = keyberon::Class<'static, UsbBusType, Leds>;
type UsbDevice = usb_device::device::UsbDevice<'static, UsbBusType>;

pub struct Leds {
    caps_lock: gpio::gpioc::PC13<gpio::Output<gpio::PushPull>>,
}
impl keyberon::keyboard::Leds for Leds {
    fn caps_lock(&mut self, status: bool) {
        if status {
            self.caps_lock.set_low().unwrap()
        } else {
            self.caps_lock.set_high().unwrap()
        }
    }
}

pub struct Cols(
    pub PB0<Input<PullUp>>,
    pub PA0<Input<PullUp>>,
    pub PA1<Input<PullUp>>,
    pub PA2<Input<PullUp>>,
    pub PA3<Input<PullUp>>,
    pub PA4<Input<PullUp>>,
    pub PA5<Input<PullUp>>,
    pub PA6<Input<PullUp>>,
);
impl_heterogenous_array! {
    Cols,
    dyn InputPin<Error = Infallible>,
    U8,
    [0, 1, 2, 3, 4, 5, 6, 7]
}

pub struct Rows(
    pub PA8<Output<PushPull>>, // 1
    pub PA9<Output<PushPull>>, // 2
    pub PA10<Output<PushPull>>, // 3
    pub PB3<Output<PushPull>>, // 4
    pub PB4<Output<PushPull>>, // 5
    pub PB5<Output<PushPull>>, // 6
    pub PB6<Output<PushPull>>, // 7
    pub PB7<Output<PushPull>>, // 8
    pub PB8<Output<PushPull>>, // 9
    pub PB9<Output<PushPull>>, // 10
);
impl_heterogenous_array! {
    Rows,
    dyn OutputPin<Error = Infallible>,
    U10,
    [0, 1, 2, 3, 4, 5, 6, 7, 8, 9]
}

pub struct ShiftLeds(
    pub PB12<Output<PushPull>>,
    pub PB13<Output<PushPull>>,
    pub PB14<Output<PushPull>>,

);

#[app(device = stm32f1xx_hal::pac, peripherals = true)]
const APP: () = {
    struct Resources {
        usb_dev: UsbDevice,
        usb_class: UsbClass,
        matrix: Matrix<Cols, Rows>,
        debouncer: Debouncer<PressedKeys<U10, U8>>,
        layout: Layout,
        timer: timer::CountDownTimer<pac::TIM3>,
        shift_leds: ShiftLeds,
    }

    #[init]
    fn init(mut c: init::Context) -> init::LateResources {
        static mut USB_BUS: Option<UsbBusAllocator<UsbBusType>> = None;

        let mut flash = c.device.FLASH.constrain();
        let mut rcc = c.device.RCC.constrain();
        let mut afio = c.device.AFIO.constrain(&mut rcc.apb2);

        // set 0x424C in DR10 for dfu on reset
        let bkp = rcc
            .bkp
            .constrain(c.device.BKP, &mut rcc.apb1, &mut c.device.PWR);
        bkp.write_data_register_low(9, 0x424C);

        let clocks = rcc
            .cfgr
            .use_hse(8.mhz())
            .sysclk(72.mhz())
            .pclk1(36.mhz())
            .freeze(&mut flash.acr);

        let mut gpioa = c.device.GPIOA.split(&mut rcc.apb2);
        let mut gpiob = c.device.GPIOB.split(&mut rcc.apb2);
        let mut gpioc = c.device.GPIOC.split(&mut rcc.apb2);

        // BluePill board has a pull-up resistor on the D+ line.
        // Pull the D+ pin down to send a RESET condition to the USB bus.
        let mut usb_dp = gpioa.pa12.into_push_pull_output(&mut gpioa.crh);
        usb_dp.set_low().unwrap();
        cortex_m::asm::delay(clocks.sysclk().0 / 100);

        let mut led = gpioc.pc13.into_push_pull_output(&mut gpioc.crh);
        let mut shift_leds = ShiftLeds(
            gpiob.pb12.into_push_pull_output(&mut gpiob.crh), // led clk (SRCLR)
            gpiob.pb13.into_push_pull_output(&mut gpiob.crh), // led data (SER)
            gpiob.pb14.into_push_pull_output(&mut gpiob.crh), // led nEn (OE)
        );

        led.set_low().unwrap();
        let leds = Leds { caps_lock: led };

        let usb_dm = gpioa.pa11;
        let usb_dp = usb_dp.into_floating_input(&mut gpioa.crh);

        let usb = Peripheral {
            usb: c.device.USB,
            pin_dm: usb_dm,
            pin_dp: usb_dp,
        };

        *USB_BUS = Some(UsbBus::new(usb));
        let usb_bus = USB_BUS.as_ref().unwrap();

        let usb_class = keyberon::new_class(usb_bus, leds);
        let usb_dev = keyberon::new_device(usb_bus);

        let mut timer =
            timer::Timer::tim3(c.device.TIM3, &clocks, &mut rcc.apb1).start_count_down(1.khz());
        timer.listen(timer::Event::Update);

        let (pa15, pb3, pb4) = afio.mapr.disable_jtag(gpioa.pa15, gpiob.pb3, gpiob.pb4);

        let matrix = Matrix::new(
            Cols(
                gpiob.pb0.into_pull_up_input(&mut gpiob.crl),
                gpioa.pa0.into_pull_up_input(&mut gpioa.crl),
                gpioa.pa1.into_pull_up_input(&mut gpioa.crl),
                gpioa.pa2.into_pull_up_input(&mut gpioa.crl),
                gpioa.pa3.into_pull_up_input(&mut gpioa.crl),
                gpioa.pa4.into_pull_up_input(&mut gpioa.crl),
                gpioa.pa5.into_pull_up_input(&mut gpioa.crl),
                gpioa.pa6.into_pull_up_input(&mut gpioa.crl),
            ),
            Rows(
                gpioa.pa8.into_push_pull_output(&mut gpioa.crh),
                gpioa.pa9.into_push_pull_output(&mut gpioa.crh),
                gpioa.pa10.into_push_pull_output(&mut gpioa.crh),
                pb3.into_push_pull_output(&mut gpiob.crl),
                pb4.into_push_pull_output(&mut gpiob.crl),
                gpiob.pb5.into_push_pull_output(&mut gpiob.crl),
                gpiob.pb6.into_push_pull_output(&mut gpiob.crl),
                gpiob.pb7.into_push_pull_output(&mut gpiob.crl),
                gpiob.pb8.into_push_pull_output(&mut gpiob.crh),
                gpiob.pb9.into_push_pull_output(&mut gpiob.crh),
            ),
        );

        init::LateResources {
            usb_dev,
            usb_class,
            timer,
            shift_leds,
            debouncer: Debouncer::new(PressedKeys::default(), PressedKeys::default(), 5),
            matrix: matrix.unwrap(),
            layout: Layout::new(layout::LAYERS),
        }
    }

    #[task(binds = USB_HP_CAN_TX, priority = 2, resources = [usb_dev, usb_class])]
    fn usb_tx(mut c: usb_tx::Context) {
        usb_poll(&mut c.resources.usb_dev, &mut c.resources.usb_class);
    }

    #[task(binds = USB_LP_CAN_RX0, priority = 2, resources = [usb_dev, usb_class])]
    fn usb_rx(mut c: usb_rx::Context) {
        usb_poll(&mut c.resources.usb_dev, &mut c.resources.usb_class);
    }

    #[task(binds = TIM3, priority = 1, resources = [usb_class, matrix, debouncer, layout, timer])]
    fn tick(mut c: tick::Context) {
        c.resources.timer.clear_update_interrupt_flag();

        for event in c
            .resources
            .debouncer
            .events(c.resources.matrix.get().unwrap())
        {
            send_report(c.resources.layout.event(event), &mut c.resources.usb_class);
        }
        send_report(c.resources.layout.tick(), &mut c.resources.usb_class);
    }

    #[idle(resources = [shift_leds])]
    fn idle(c: idle::Context) -> ! {
        c.resources.shift_leds.1.set_high().unwrap();
        c.resources.shift_leds.2.set_high().unwrap();

        for x in 0..16 {
            c.resources.shift_leds.0.set_high().unwrap();
            cortex_m::asm::delay(10);
            c.resources.shift_leds.0.set_low().unwrap();
            cortex_m::asm::delay(10);
        }
        c.resources.shift_leds.0.set_high().unwrap();
        cortex_m::asm::delay(10);
        c.resources.shift_leds.0.set_low().unwrap();
        cortex_m::asm::delay(10);
        c.resources.shift_leds.2.set_low().unwrap();

        loop {
            cortex_m::asm::nop;
        }

    }

};

fn send_report(iter: impl Iterator<Item = KeyCode>, usb_class: &mut resources::usb_class<'_>) {
    use rtic::Mutex;
    let report: KbHidReport = iter.collect();
    if usb_class.lock(|k| k.device_mut().set_keyboard_report(report.clone())) {
        while let Ok(0) = usb_class.lock(|k| k.write(report.as_bytes())) {}
    }
}

fn usb_poll(usb_dev: &mut UsbDevice, keyboard: &mut UsbClass) {
    if usb_dev.poll(&mut [keyboard]) {
        keyboard.poll();
    }
}
