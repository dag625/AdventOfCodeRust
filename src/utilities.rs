use std::result;
use std::vec::Vec;

pub type Result = std::result::Result<(), String>;

pub fn list_result<T, E>(res: Vec<result::Result<T, E>>) -> result::Result<Vec<T>, E> {
    let mut retval : Vec<T> = Vec::new();
    for item in res {
        match item {
            Ok(i) => { retval.push(i); },
            Err(e) => { return Err(e); }
        }
    }
    Ok(retval)
}

pub fn list_result_and_convert<T, AltE, E>(res: Vec<result::Result<T, AltE>>, conv: fn(AltE) -> E) -> result::Result<Vec<T>, E> {
    let mut retval : Vec<T> = Vec::new();
    for item in res {
        match item {
            Ok(i) => { retval.push(i); },
            Err(e) => { return Err(conv(e)); }
        }
    }
    Ok(retval)
}