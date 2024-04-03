
use std::fs::File;
use std::io::Read;
use std::string::String;
use std::str;

static FILE_NAME: &'static str = "nyan";

fn main(){

println!("Hi");
image_to_base64("png");
}

fn image_to_base64(file_type : &str) {
    let mut file = match File::open(format!("res/{}_data", file_type)) {
        Err(why) => panic!("couldn't open {}", why),
        Ok(file) => file,
    };
    let mut buffer = String::new();
    match file.read_to_string(&mut buffer) {
        Err(why) => panic!("couldn't read {}", why),
        Ok(_) => {},
    }
    let base64 = image_base64::to_base64(&format!("res/{}.{}", FILE_NAME, file_type)); 
    assert_eq!(base64, buffer);
}