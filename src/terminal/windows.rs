use std::io::{self, stdout, Stdout, Write};
use std::thread::sleep;
use std::time::Duration;
use windows::Win32::{
    Foundation::HANDLE,
    System::Console::{
        CONSOLE_SCREEN_BUFFER_INFO, CONSOLE_MODE, GetStdHandle, GetConsoleScreenBufferInfo,
        GetConsoleMode, STD_INPUT_HANDLE, STD_OUTPUT_HANDLE,COORD
    },
};
use super::screen_buffer::ScreenBuffer;

pub struct Terminal {
    stdout: Stdout,
    hstdout: HANDLE,
    hstdin: HANDLE,
    original_csbi: CONSOLE_SCREEN_BUFFER_INFO,
    original_console_mode: CONSOLE_MODE,
    height: i16,
    width: i16,
    haltbuf:HANDLE
}
pub enum ClearType{
    ALL,
    ROW,
    COL
}
impl Terminal {
    pub fn new() -> io::Result<Self> {
        let mut terminal = Terminal {
            stdout: stdout(),
            hstdout: HANDLE::default(),
            hstdin: HANDLE::default(),
            original_csbi: CONSOLE_SCREEN_BUFFER_INFO::default(),
            original_console_mode: CONSOLE_MODE::default(),
            height: 0,
            width: 0,
            haltbuf:HANDLE::default()
        };
        terminal.init_handles()?;
        Ok(terminal)
    }

    fn init_handles(&mut self) -> io::Result<()> {
        self.hstdout = unsafe { GetStdHandle(STD_OUTPUT_HANDLE)? };
        self.hstdin = unsafe { GetStdHandle(STD_INPUT_HANDLE)? };
        unsafe {
            GetConsoleScreenBufferInfo(self.hstdout, &mut self.original_csbi)?;
            GetConsoleMode(self.hstdout, &mut self.original_console_mode)?;
        }
        self.width = self.original_csbi.dwSize.X;
        self.height = self.original_csbi.dwSize.Y;
        Ok(())
    }
    pub fn enter_alternate_buffer(&mut self) -> io::Result<()> {
        let alternate_screen = ScreenBuffer::create()?;
        alternate_screen.show()?;
        self.haltbuf = alternate_screen.get_handle();
        self.stdout.flush()?;
        Ok(())
    }

    pub fn leave_alternate_buffer(&self) -> io::Result<()> {
        ScreenBuffer::destory()?;
        Ok(())
    }
    fn get_alternate_buffer_info(&mut self) -> io::Result<CONSOLE_SCREEN_BUFFER_INFO>{
        let mut csbi : CONSOLE_SCREEN_BUFFER_INFO = CONSOLE_SCREEN_BUFFER_INFO::default();
        unsafe {
            GetConsoleScreenBufferInfo(self.haltbuf, &mut csbi)?;
        }
        Ok(csbi)
    }
    pub fn clear(&mut self,clear_type:ClearType,start_x:Option<i16>,start_y:Option<i16>) -> io::Result<()>{
        let alt_csbi = self.get_alternate_buffer_info();
        if !alt_csbi.is_ok() {return Err(io::Error::new(io::ErrorKind::Other, "Failed to fetch alt csbi"))};
        let alt_csbi = alt_csbi.unwrap();
        let mut alternate_screen = ScreenBuffer::new(self.haltbuf);
        match clear_type {
            ClearType::ALL =>{
                let length = alt_csbi.dwMaximumWindowSize.X * alt_csbi.dwMaximumWindowSize.Y;
                let start_coord = COORD {X:0,Y:0};
                ScreenBuffer::clear(&mut alternate_screen, length as u32, start_coord)?;
            },
            ClearType::ROW => {
                if start_y.is_none(){ return Err(io::Error::new(io::ErrorKind::Other, "When you use LINE Clear function, you need to set start_y."));}
                let length = alt_csbi.dwMaximumWindowSize.X;
                let start_coord = COORD{X:0,Y:start_y.unwrap()};
                ScreenBuffer::clear(&mut alternate_screen, length as u32, start_coord)?;
            },
            ClearType::COL => {
                if start_x.is_none(){ return Err(io::Error::new(io::ErrorKind::Other, "When you use LINE Clear function, you need to set start_x."));}
                let length = alt_csbi.dwMaximumWindowSize.Y;
                let start_coord = COORD{X:start_x.unwrap(),Y:0};
                ScreenBuffer::clear(&mut alternate_screen, length as u32, start_coord)?;
            }}
        Ok(())
    }
    pub fn write(&mut self,str:&str) -> io::Result<()>{
        let string = str.as_bytes();
        let mut alternate_screen = ScreenBuffer::new(self.haltbuf);
        alternate_screen.write(string)?;
        Ok(())
    }
    pub fn move_to(&mut self,x:i16,y:i16) -> io::Result<()>{
        let alternate_screen = ScreenBuffer::new(self.haltbuf);
        alternate_screen.move_to(x, y)?;
        Ok(())
    }
    pub fn test(&mut self) -> io::Result<()> {
        self.enter_alternate_buffer()?;
        sleep(Duration::from_secs(3));
        self.move_to(10, 10)?;
        sleep(Duration::from_secs(3));
        self.write("This message will be printed in alternate con")?;
        sleep(Duration::from_secs(3));
        self.leave_alternate_buffer()?;
        println!("This message will be printed in default console");
        sleep(Duration::from_secs(3));
        Ok(())
    }


}
