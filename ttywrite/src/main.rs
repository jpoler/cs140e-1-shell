extern crate serial;
extern crate structopt;
extern crate xmodem;
#[macro_use]
extern crate structopt_derive;

use std::fs::File;
use std::io::{self, Read, Write};
use std::path::PathBuf;
use std::time::Duration;

use serial::core::{BaudRate, CharSize, FlowControl, SerialDevice, SerialPortSettings, StopBits};
use structopt::StructOpt;
use xmodem::{Progress, Xmodem, DEBUG_BUFFER, DEBUG_BUFFER_OFFSET};

mod parsers;

use parsers::{
    parse_baud_rate, parse_flow_control, parse_mode, parse_stop_bits, parse_width, Mode,
};

#[derive(StructOpt, Debug)]
#[structopt(about = "Write to TTY using the XMODEM protocol by default.")]
struct Opt {
    #[structopt(
        short = "m",
        long = "mode",
        help = "Read or write mode",
        parse(try_from_str = "parse_mode"),
        default_value = "write"
    )]
    mode: Mode,

    #[structopt(
        short = "i",
        help = "Input file (defaults to stdin if not set)",
        parse(from_os_str)
    )]
    input: Option<PathBuf>,

    #[structopt(
        short = "b",
        long = "baud",
        parse(try_from_str = "parse_baud_rate"),
        help = "Set baud rate",
        default_value = "115200"
    )]
    baud_rate: BaudRate,

    #[structopt(
        short = "t",
        long = "timeout",
        parse(try_from_str),
        help = "Set timeout in seconds",
        default_value = "10"
    )]
    timeout: u64,

    #[structopt(
        short = "w",
        long = "width",
        parse(try_from_str = "parse_width"),
        help = "Set data character width in bits",
        default_value = "8"
    )]
    char_width: CharSize,

    #[structopt(help = "Path to TTY device", parse(from_os_str))]
    tty_path: PathBuf,

    #[structopt(
        short = "f",
        long = "flow-control",
        parse(try_from_str = "parse_flow_control"),
        help = "Enable flow control ('hardware' or 'software')",
        default_value = "none"
    )]
    flow_control: FlowControl,

    #[structopt(
        short = "s",
        long = "stop-bits",
        parse(try_from_str = "parse_stop_bits"),
        help = "Set number of stop bits",
        default_value = "1"
    )]
    stop_bits: StopBits,

    #[structopt(short = "r", long = "raw", help = "Disable XMODEM")]
    raw: bool,
}

struct Tty {
    serial: serial::SystemPort,
    input: Option<PathBuf>,
    raw: bool,
}

impl Tty {
    fn read(mut self) -> io::Result<()> {
        let mut file;
        let mut stdout;

        let mut writer: &mut io::Write = if let Some(pathbuf) = self.input {
            file = File::create(pathbuf)?;
            &mut file
        } else {
            stdout = io::stdout();
            &mut stdout
        };

        if self.raw {
            io::copy(&mut self.serial, &mut writer)?;
            Ok(())
        } else {
            fn progress(progress: Progress) {
                println!("progress: {:?}", progress);
            }
            let buf = vec![];
            let mut cursor = io::Cursor::new(buf);
            let n = Xmodem::receive_with_progress(self.serial, &mut cursor, progress)?;
            writer.write_all(&cursor.into_inner()[..n])?;
            Ok(())
        }
    }

    fn write(mut self) -> io::Result<()> {
        let mut file;
        let mut stdin;

        let mut reader: &mut io::Read = if let Some(pathbuf) = self.input {
            file = File::open(pathbuf)?;
            &mut file
        } else {
            stdin = io::stdin();
            &mut stdin
        };

        if self.raw {
            io::copy(&mut reader, &mut self.serial)?;
            Ok(())
        } else {
            fn progress(progress: Progress) {
                println!("progress: {:?}", progress);
            }

            let mut buf = vec![];
            reader.read_to_end(&mut buf)?;
            Xmodem::transmit_with_progress(io::Cursor::new(buf), self.serial, progress)?;
            Ok(())
        }
    }
}

fn run() -> io::Result<()> {
    let opt = Opt::from_args();

    let mut serial = serial::open(&opt.tty_path).expect("path points to invalid TTY");
    let mut tty_settings = serial.read_settings()?;
    tty_settings.set_baud_rate(opt.baud_rate)?;
    tty_settings.set_char_size(opt.char_width);
    tty_settings.set_stop_bits(opt.stop_bits);
    tty_settings.set_flow_control(opt.flow_control);
    tty_settings.set_parity(serial::Parity::ParityNone);
    serial.write_settings(&tty_settings)?;
    serial.set_timeout(Duration::from_secs(opt.timeout))?;

    let tty = Tty {
        serial,
        input: opt.input,
        raw: opt.raw,
    };

    let res = match opt.mode {
        Mode::Read => tty.read(),
        Mode::Write => tty.write(),
    };

    Ok(())
}

fn main() {
    run().unwrap();
}

// fn read()
