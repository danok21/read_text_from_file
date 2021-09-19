    use std::fs::File;
    use std::io::prelude::*;
    use std::path::Path;

    pub  fn read_config_file(file_path: &str) -> String{
        let path = Path::new(file_path);
        let display_path = path.display();
        let mut file = match File::open(&path) {
            Ok(file) => file,
            Err(err) => panic!("couldn't open {}: {}",display_path, err)
        };
        let mut text = String::new();
        match file.read_to_string(&mut text) {
            Err(err) => println!("couldn't read {}",err),
            Ok(_) => println!("contains: {}",text),
        }
        text
    }




