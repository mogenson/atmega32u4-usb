#![allow(dead_code)]

extern "C" {
    /* setup */
    fn usb_init();
    fn usb_configured() -> u8;

    /* receiving data */
    fn usb_serial_getchar() -> i16;
    fn usb_serial_available() -> u8;
    fn usb_serial_flush_input();

    /* transmitting data */
    fn usb_serial_putchar(c: u8) -> i8;
    fn usb_serial_putchar_nowait(c: u8) -> i8;
    fn usb_serial_write(buffer: *const u8, size: u16) -> i8;
    fn usb_serial_flush_output();

    /* serial parameters */
    fn usb_serial_get_baud() -> u32;
    fn usb_serial_get_stopbits() -> u8;
    fn usb_serial_get_paritytype() -> u8;
    fn usb_serial_get_numbits() -> u8;
    fn usb_serial_get_control() -> u8;
    fn usb_serial_set_control(signals: u8) -> i8;

    /* interrupt service routines */
    pub fn usb_gen_handler();
    pub fn usb_com_handler();
}

pub struct UsbSerial();

impl UsbSerial {
    pub fn init(&self) {
        unsafe { usb_init() };
    }

    pub fn is_configured(&self) -> bool {
        unsafe { usb_configured() != 0 }
    }

    pub fn get_available(&self) -> u8 {
        unsafe { usb_serial_available() }
    }

    pub fn get_dtr(&self) -> bool {
        unsafe { usb_serial_get_control() & 0x01 != 0 }
    }

    pub fn get_rts(&self) -> bool {
        unsafe { usb_serial_get_control() & 0x02 != 0 }
    }
}

impl embedded_hal::serial::Read<u8> for UsbSerial {
    type Error = ();

    fn read(&mut self) -> nb::Result<u8, Self::Error> {
        if unsafe { usb_serial_available() } > 0 {
            let data = unsafe { usb_serial_getchar() };
            if data != -1 {
                return Ok(data as u8);
            }
        }
        Err(nb::Error::WouldBlock)
    }
}

impl embedded_hal::serial::Write<u8> for UsbSerial {
    type Error = ();

    fn write(&mut self, word: u8) -> nb::Result<(), Self::Error> {
        if unsafe { usb_serial_putchar(word) == 0 } {
            Ok(())
        } else {
            Err(nb::Error::WouldBlock)
        }
    }

    fn flush(&mut self) -> nb::Result<(), Self::Error> {
        unsafe { usb_serial_flush_output() };
        Ok(())
    }
}

impl ufmt::uWrite for UsbSerial {
    type Error = ();

    fn write_str(&mut self, s: &str) -> Result<(), Self::Error> {
        if unsafe { usb_serial_write(s.as_ptr(), s.len() as u16) == 0 } {
            Ok(())
        } else {
            Err(())
        }
    }
}
