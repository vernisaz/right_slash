use std::path::MAIN_SEPARATOR;

pub fn to_unix_separator(mut path: String ) -> String {
    unsafe {
        let path_vec: &mut [u8]= path.as_bytes_mut();
    
        for comp in path_vec {
            if *comp == b'\\' { *comp = b'/';}
        }
    }
    path
}

pub fn adjust_separator(mut path: String) -> String {
    let foreign_slash = if MAIN_SEPARATOR == '\\' { '/' } else { '\\' };
    let vec = unsafe {path.as_mut_vec()};
    for c in vec {
        if *c == foreign_slash as u8 { *c = MAIN_SEPARATOR as u8;}
    }

    path
}