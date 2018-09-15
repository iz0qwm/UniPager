use serial::{self, SerialPort};

use config::Config;
use transmitter::Transmitter;

pub struct GP2009TRTransmitter {
    serial: Box<serial::SerialPort>
}

impl GP2009TRTransmitter {
    pub fn new(config: &Config) -> GP2009TRTransmitter {
        info!("Initializing GP2009TR transmitter...");

        let mut serial = serial::open(&config.gp2009tr.port).expect(
            "Unable to open serial port"
        );

        serial
            .configure(&serial::PortSettings {
                baud_rate: serial::BaudRate::Baud9600,
                char_size: serial::CharSize::Bits8,
                parity: serial::Parity::ParityNone,
                stop_bits: serial::StopBits::Stop1,
                flow_control: serial::FlowControl::FlowNone
            })
            .expect("Unable to configure serial port");

        GP2009TRTransmitter { serial: Box::new(serial) }
    }
}

impl Transmitter for GP2009TRTransmitter {
    fn send(&mut self, gen: &mut Iterator<Item = u32>) {
        for word in gen {
            let bytes = [
                ((word & 0xff000000) >> 24) as u8,
                ((word & 0x00ff0000) >> 16) as u8,
                ((word & 0x0000ff00) >> 8) as u8,
                (word & 0x000000ff) as u8,
            ];

            if (*self.serial).write_all(&bytes).is_err() {
                error!("Unable to write data to the serial port");
                return;
            }
        }

        // Send End of Transmission packet
        let eot = [0x18 as u8];
        if (*self.serial).write_all(&eot).is_err() {
            error!("Unable to send end of transmission byte");
            return;
        }

        if (*self.serial).flush().is_err() {
            error!("Unable to flush serial port");
        }
    }
}
