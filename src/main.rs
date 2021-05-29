use std::io;
use std::io::Read;
use std::fs::File;

fn read_text_from_file(path: &str) -> Result<String, io::Error> {
    let mut f = File::open(path)?;//？如果打开文件失败,返回error类型
    let mut s = String::new();//创建一个空字符串变量
    f.read_to_string(&mut s)?;//读取文件，以字符串的方式
    Ok(s)//返回读取后的内容
}

fn main() {
    let str_file = read_text_from_file("./test.txt");
    match str_file {
        Ok(s) => println!("{}", s),
        Err(e) => {
            match e.kind() {
                io::ErrorKind::NotFound => {
                    println!("No such file");
                },//文件不存在
                _ => {
                    println!("Cannot read the file");//文件存在但读取失败
                }
            }
        }
    }
}
