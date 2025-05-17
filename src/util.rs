use std::io::{
    Result,
    Error
};
pub fn result(return_val:Result<()>)->Result<()>{
    if return_val.is_ok() {
            Ok(())
    } else {
        Err(Error::last_os_error())
    }
}