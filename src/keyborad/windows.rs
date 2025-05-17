use std::{io, slice};
use windows::Win32::System::Console::{
    GetStdHandle, ReadConsoleInputW, INPUT_RECORD, STD_INPUT_HANDLE,
};
use windows::Win32::Foundation::{HANDLE};

pub fn read_input(buf: &mut [INPUT_RECORD]) -> io::Result<usize> {
    let mut num_records: u32 = 0;
    unsafe {
        let handle: HANDLE = GetStdHandle(STD_INPUT_HANDLE)?;
        let result: Result<(), windows::core::Error> = ReadConsoleInputW(handle, buf,  &mut num_records);
        if result.is_ok() {
            Ok(num_records as usize)
        } else {
            Err(io::Error::last_os_error())
        }
    }
}
