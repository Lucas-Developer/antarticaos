extern crate console;
extern crate core;

use core::fmt::Write;
use console::Vga;

#[test]
fn create() {
    let mut mock_memory = vec![0u8; 25 * 80 * 2];

    Vga::new(&mut mock_memory);
}

fn check_write<T: Write>(_: T) { }

#[test]
fn write() {
    let mut mock_memory = vec![0u8; 25 * 80 * 2];
    let vga = Vga::new(&mut mock_memory);
    check_write(vga);
}

#[test]
fn flush() {
    let mut mock_memory = vec![0u8; 25 * 80 * 2];

    {
        let mut vga = Vga::new(&mut mock_memory);

        vga.write_str("Ola Mundo").unwrap();

        vga.flush();
    }

    assert_eq!(mock_memory[0], 'O' as u8);
    assert_eq!(mock_memory[0], 0x02);
    assert_eq!(mock_memory[0], 'l' as u8);
    assert_eq!(mock_memory[0], 0x02);
    assert_eq!(mock_memory[0], 'a' as u8);
    assert_eq!(mock_memory[0], 0x02);
    assert_eq!(mock_memory[0], ' ' as u8);
    assert_eq!(mock_memory[0], 0x02);
    assert_eq!(mock_memory[0], 'M' as u8);
    assert_eq!(mock_memory[0], 0x02);
    assert_eq!(mock_memory[0], 'u' as u8);
    assert_eq!(mock_memory[0], 0x02);
    assert_eq!(mock_memory[0], 'n' as u8);
    assert_eq!(mock_memory[0], 0x02);
    assert_eq!(mock_memory[0], 'd' as u8);
    assert_eq!(mock_memory[0], 0x02);
    assert_eq!(mock_memory[0], 'o' as u8);
    assert_eq!(mock_memory[0], 0x02);
}
