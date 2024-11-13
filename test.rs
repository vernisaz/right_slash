fn main() {
    let mut line2: String = "yes\\num\\β3".to_string();
    line2 = rslash::to_unix_separator(line2);
    assert_eq!(line2,"yes/num/β3".to_string());

    #[cfg(target_os = "windows")]
    {
        let mut path = "yes/num\\β3".to_string();
        assert_eq!{adjust_separator(path),"yes\\num\\β3".to_string()} 
    }
    
    #[cfg(target_family = "unix")]
    {
        let path = "yes\\num/β3".to_string();
        assert_eq!{rslash::adjust_separator(path),"yes/num/β3".to_string()} 
    }
}