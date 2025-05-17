use windows::Win32::{Foundation::{ GENERIC_READ, GENERIC_WRITE, HANDLE, TRUE}, Security::SECURITY_ATTRIBUTES, Storage::FileSystem::{FILE_SHARE_READ, FILE_SHARE_WRITE}, System::Console::{CreateConsoleScreenBuffer, FillConsoleOutputCharacterA, GetStdHandle, SetConsoleActiveScreenBuffer, SetConsoleCursorPosition, WriteConsoleA, WriteConsoleW, CONSOLE_TEXTMODE_BUFFER, COORD, STD_OUTPUT_HANDLE}};
use std::{mem::size_of, str};
use std::io;

pub struct ScreenBuffer{
    handle:HANDLE
}
impl ScreenBuffer{
    pub fn new(handle: HANDLE) -> Self {
        ScreenBuffer {
            handle: handle
        }
    }
    pub fn create() -> Result<Self, windows::core::Error> {
        let security_attr: SECURITY_ATTRIBUTES = SECURITY_ATTRIBUTES {
            nLength: size_of::<SECURITY_ATTRIBUTES>() as u32,
            lpSecurityDescriptor: std::ptr::null_mut(),
            bInheritHandle: TRUE,
        };
        let handle = unsafe {
            CreateConsoleScreenBuffer(
                (GENERIC_READ | GENERIC_WRITE).0 ,
                (FILE_SHARE_READ| FILE_SHARE_WRITE).0,
                Some(&security_attr as *const SECURITY_ATTRIBUTES),
                CONSOLE_TEXTMODE_BUFFER,
                None,
            )?
        };
        Ok(ScreenBuffer { handle })
    }
    pub fn destory() -> io::Result<()>{
        unsafe {
            match GetStdHandle(STD_OUTPUT_HANDLE) {
                Ok(hstdout)=>{
                    SetConsoleActiveScreenBuffer(hstdout)?;
                    Ok(())
                },
                Err(_)=>{
                    return Err(io::Error::last_os_error())
                }
            }
        }
    }
    pub fn show(&self) -> io::Result<()> {
        let result = unsafe { SetConsoleActiveScreenBuffer(self.handle) };
        if result.is_ok() {
            Ok(())
        } else {
            Err(io::Error::last_os_error())
        }
    }
    pub fn get_handle(&self)->HANDLE{
        return self.handle;
    }
    pub fn move_to(&self,x:i16,y:i16) -> io::Result<()>{
        if x < 0 || y < 0{
            return Err(io::Error::new(io::ErrorKind::Other, "coord must be x > 0, y > 0"))
        }
        let pos = COORD {X:x,Y:y};
        unsafe {
            SetConsoleCursorPosition(self.handle, pos)?;
        }
        Ok(())
    }
    pub fn write(&mut self, buf:&[u8]) -> io::Result<usize> {
        let utf8 = match std::str::from_utf8(buf) {
            Ok(string) =>string,
            Err(_)=> return Err(io::Error::new(
                io::ErrorKind::Other,
                "Could not parse to utf8 string"
            ))
        };

        let utf16:Vec<u16> = utf8.encode_utf16().collect();
        let mut cells_written:u32 = 0;
        unsafe {
            WriteConsoleW(self.handle, &utf16, Some(&mut cells_written), None)?;
        }
        Ok(utf8.as_bytes().len())
    }
    pub fn clear(&mut self,length:u32,start_coord:COORD)->io::Result<u32>{
        let mut chars_wirtten = 0;
        unsafe {
            FillConsoleOutputCharacterA(self.handle, ' ' as i8, length, start_coord, &mut chars_wirtten)?;
        }
        Ok(chars_wirtten)
    }
    //+attribute
}
