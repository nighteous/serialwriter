use serialport;
use std::time::Duration;

fn get_ram() -> u64 {

    use sysinfo::{System, SystemExt};
    let mut sys = System::new_all();

    sys.refresh_all();

    let tm_byte: u64 = sys.total_memory();
    let um_byte: u64 = sys.used_memory();

    let tmum = (um_byte * 100) / tm_byte;

    // percentage of ram
    return tmum
}

fn get_led() -> &'static [u8] {

   let ramused = get_ram();
   let mut output;

   if ramused >= 85 {
       output = "5".as_bytes();
   }
   else if ramused < 75 && ramused >= 60 {
       output = "4".as_bytes();
   }
   else if ramused < 60 && ramused >= 45 {
       output = "3".as_bytes();
   }
   else if ramused < 45 && ramused >= 20 {
       output = "2".as_bytes();
   }
   else {
       output = "1".as_bytes();
   }

   return output;

}

fn main() {

    let mut serial_port = serialport::new("/dev/ttyACM0", 57600)
        .timeout(Duration::from_secs(5))
        .open()
        .expect("Failed just like you");

    // let output = "1".as_bytes();

    loop {
        let output = get_led();
        serial_port.write(output).expect("Write Failed!");
        serial_port.flush().unwrap();
    }


}
