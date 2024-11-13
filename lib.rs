use std::path::MAIN_SEPARATOR;

pub fn to_unix_separator(mut path: String ) -> String {
    unsafe {
        let path_vec: &mut [u8]= path.as_bytes_mut();
    
        for c in 0..path_vec.len() {
            if path_vec[c] == b'\\' { path_vec[c] = b'/';}
        }
    }
    path
}

pub fn adjust_separator(mut path: String) -> String {
    let foreign_slash = if MAIN_SEPARATOR == '\\' { '/' } else { '\\' };
    let vec = unsafe {path.as_mut_vec()};
    for c in 0..vec.len() {
        if vec[c] == foreign_slash as u8 { vec[c] = MAIN_SEPARATOR as u8;}
    }

    path
}