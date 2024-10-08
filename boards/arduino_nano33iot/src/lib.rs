#![no_std]

#[cfg(feature = "rt")]
pub use cortex_m_rt::entry;

pub use atsamd_hal as hal;
pub use hal::ehal;
pub use hal::pac;

use hal::clock::GenericClockController;
use hal::sercom::{i2c, spi, uart, Sercom2, Sercom5};
use hal::time::Hertz;

#[cfg(feature = "usb")]
use hal::usb::{usb_device::bus::UsbBusAllocator, UsbBus};

hal::bsp_peripherals!(
    Sercom1 { SpiSercom }
    Sercom2 { NinaSercom }
    Sercom4 { I2cSercom }
    Sercom5 { UartSercom }
);


// The docs could be further improved with details of the specific channels etc
// Maps the pins to their arduino names and the numbers printed on the board.
// Information from: <https://github.com/arduino/ArduinoCore-samd/blob/master/variants/nano_33_iot/variant.cpp>
hal::bsp_pins!(
    PB23 {
        /// RX
        name: rx
        aliases: {
            AlternateD: Rx
        }
    }
    PB22 {
        /// TX
        name: tx
        aliases: {
            AlternateD: Tx
        }
    }
    PB10 {
        /// Digital 2: PWM, TC
        name: d2
    }
    PB11 {
        /// Digital 3: PWM, TC
        name: d3
    }
    PA07 {
        /// Digital 4: TCC
        name: d4
    }
    PA05 {
        /// Digital 5: PWM, TCC, ADC
        name: d5
    }
    PA04 {
        /// Digital 6: PWM, TCC, ADC
        name: d6
    }
    PA06 {
        /// Digital 7: ADC
        name: d7
    }
    PA18 {
        /// Digital 8
        name: d8
    }
    PA20 {
        /// Digital 9: PWM, TCC
        name: d9
    }
    PA21 {
        /// Digital 10: PWM, TCC
        name: d10
    }
    PA19 {
        /// Digital 11/SCI MISO: PWM, TCC
        name: miso
        aliases: {
            AlternateC: Miso
        }
    }
    PA16 {
        /// Digital 12/SCI MOSI: PWM, TCC
        name: mosi
        aliases: {
            AlternateC: Mosi
        }
    }
    PA17 {
        /// Digital 13/LED/SPI SCK: ON-BOARD-LED
        name: led_sck
        aliases: {
            PushPullOutput: Led,
            AlternateC: Sck
        }
    }
    PA02 {
        /// Analog 0: DAC
        name: a0
    }
    PB02 {
        /// Analog 1
        name: a1
    }
    PA11 {
        /// Analog 2: PWM, TCC
        name: a2
    }
    PA10 {
        /// Analog 3: PWM, TCC
        name: a3
    }
    PB08 {
        /// Analog 4/SDA
        name: sda
        aliases: {
            AlternateD: Sda
        }
    }
    PB09 {
        /// Analog 5/SCL: PWM< TCC
        name: scl
        aliases: {
            AlternateD: Scl
        }
    }
    PA09 {
        /// Analog 6
        name: a6
    }
    PB03 {
        /// Analog 7
        name: a7
    }
    PA03 {
        /// AREF
        name: aref
    }
    PA12 {
        /// SPI (Lefacy ICSP) 1 / NINA MOSI
        name: nina_mosi
        aliases: {
            AlternateC: NinaMosi
        }
    }
    PA13 {
        /// SPI (Lefacy ICSP) 2 / NINA MISO
        name: nina_miso
        aliases: {
            AlternateC: NinaMiso
        }
    }
    PA14 {
        /// SPI (Lefacy ICSP) 3 / NINA CS
        name: nina_cs
        aliases: {
            AlternateC: NinaCs
        }
    }
    PA15 {
        /// SPI (Lefacy ICSP) 4 / NINA SCK
        name: nina_sck
        aliases: {
            AlternateC: NinaSck
        }
    }
    PA27 {
        /// NINA GPIO0
        name: nina_gpio0
    }
    PA08 {
        /// NINA RESET_N
        name: nina_resetn
        aliases: {
            PushPullOutput: NinaResetN
        }
    }
    PA28 {
        /// NINA ACK
        name: nina_ack
        aliases: {
            FloatingInput: NinaAck
        }
    }
    PA22 {
        /// SerialNina 29: PWM, TC
        name: serial_nina29
    }
    PA23 {
        /// SerialNina 30: PWM, TC
        name: serial_nina30
    }
    PA24 {
        /// USB D- Pad
        name: usb_dm
        aliases: {
            AlternateG: UsbDm
        }
    }
    PA25 {
        /// USB D+ Pad
        name: usb_dp
        aliases: {
            AlternateG: UsbDp
        }
    }
    PA30 {
        /// SWCLK
        name: p34
    }
    PA31 {
        /// SWDIO
        name: p35
    }
);

#[cfg(feature = "usb")]
pub fn usb_allocator(
    usb: pac::USB,
    clocks: &mut GenericClockController,
    pm: &mut pac::Pm,
    dm: impl Into<UsbDm>,
    dp: impl Into<UsbDp>,
) -> UsbBusAllocator<UsbBus> {
    let gclk0 = clocks.gclk0();
    let usb_clock = &clocks.usb(&gclk0).unwrap();
    let (dm, dp) = (dm.into(), dp.into());
    UsbBusAllocator::new(UsbBus::new(usb_clock, pm, dm, dp, usb))
}



/// I2C pads for the labelled I2C peripheral
///
/// You can use these pads with other, user-defined [`i2c::Config`]urations.
pub type I2cPads = i2c::Pads<I2cSercom, Sda, Scl>;

/// I2C master for the labelled I2C peripheral
///
/// This type implements [`Read`](ehal::blocking::i2c::Read),
/// [`Write`](ehal::blocking::i2c::Write) and
/// [`WriteRead`](ehal::blocking::i2c::WriteRead).
pub type I2c = i2c::I2c<i2c::Config<I2cPads>>;
pub fn i2c_master(
    clocks: &mut GenericClockController,
    baud: impl Into<Hertz>,
    sercom: I2cSercom,
    pm: &mut pac::Pm,
    sda: impl Into<Sda>,
    scl: impl Into<Scl>,
) -> I2c {
    let gclk0 = clocks.gclk0();
    let clock = &clocks.sercom4_core(&gclk0).unwrap();
    let freq = clock.freq();
    let baud = baud.into();
    let pads = i2c::Pads::new(sda.into(), scl.into());
    i2c::Config::new(pm, sercom, pads, freq).baud(baud).enable()
}

/// UART pads
pub type UartPads = uart::Pads<Sercom5, Rx, Tx>;
/// UART device for the labelled RX & TX pins
pub type Uart = uart::Uart<uart::Config<UartPads>, uart::Duplex>;

/// Convenience for setting up the labelled RX, TX pins to
/// operate as a UART device running at the specified baud.
pub fn uart(
    clocks: &mut GenericClockController,
    baud: impl Into<Hertz>,
    sercom5: pac::Sercom5,
    pm: &mut pac::Pm,
    rx: impl Into<Rx>,
    tx: impl Into<Tx>,
) -> Uart {
    let gclk0 = clocks.gclk0();
    let clock = &clocks.sercom5_core(&gclk0).unwrap();
    let baud = baud.into();
    let pads = uart::Pads::default().rx(rx.into()).tx(tx.into());

    uart::Config::new(pm, sercom5, pads, clock.freq())
        .baud(baud, uart::BaudMode::Fractional(uart::Oversampling::Bits16))
        .enable()
}

// SPI pads for the labelled SPI peripheral
///
/// You can use these pads with other, user-defined [`spi::Config`]urations.
pub type SpiPads = spi::Pads<SpiSercom, Miso, Mosi, Sck>;

/// SPI master for the labelled SPI peripheral
///
/// This type implements [`FullDuplex<u8>`](ehal::spi::FullDuplex).
pub type Spi = spi::Spi<spi::Config<SpiPads>, spi::Duplex>;

/// Convenience for setting up the labelled SPI peripheral.
/// This powers up SERCOM1 and configures it for use as an
/// SPI Master in SPI Mode 0.
pub fn spi_master(
    clocks: &mut GenericClockController,
    baud: Hertz,
    sercom: SpiSercom,
    pm: &mut pac::Pm,
    sclk: impl Into<Sck>,
    mosi: impl Into<Mosi>,
    miso: impl Into<Miso>,
) -> Spi {
    let gclk0 = clocks.gclk0();
    let clock = clocks.sercom1_core(&gclk0).unwrap();
    let (miso, mosi, sclk) = (miso.into(), mosi.into(), sclk.into());
    let pads = spi::Pads::default().data_in(miso).data_out(mosi).sclk(sclk);

    spi::Config::new(pm, sercom, pads, clock.freq())
        .baud(baud)
        .spi_mode(spi::MODE_0)
        .enable()
}

// SPI pads for the Nina periphral
///
/// You can use these pads with other, user-defined [`spi::Config`]urations.
pub type NinaSpiPads = spi::Pads<Sercom2, NinaMiso, NinaMosi, NinaSck>;

/// SPI master for the Nina peripheral
///
/// This type implements [`FullDuplex<u8>`](ehal::spi::FullDuplex).
pub type NinaSpi = spi::Spi<spi::Config<NinaSpiPads>, spi::Duplex>;

/// Convenience for setting up the Nina peripheral.
/// This powers up SERCOM2 and configures it for use as an
/// SPI Master in SPI Mode 0.
pub fn nina_spi_master(
    clocks: &mut GenericClockController,
    baud: Hertz,
    sercom: NinaSercom,
    pm: &mut pac::Pm,
    sclk: impl Into<NinaSck>,
    mosi: impl Into<NinaMosi>,
    miso: impl Into<NinaMiso>,
) -> NinaSpi {
    let gclk0 = clocks.gclk0();
    let clock = clocks.sercom2_core(&gclk0).unwrap();
    let (miso, mosi, sclk) = (miso.into(), mosi.into(), sclk.into());
    let pads = spi::Pads::default().data_in(miso).data_out(mosi).sclk(sclk);

    spi::Config::new(pm, sercom, pads, clock.freq())
        .baud(baud)
        .spi_mode(spi::MODE_0)
        .enable()
}
