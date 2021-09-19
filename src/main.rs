mod config;
use clap::{App, load_yaml, ArgMatches};
use crate::config::{read_config_file};

fn main() {

    let yaml = load_yaml!("clap.yaml");  // src/clap.yal
    let matches = App::from(yaml).get_matches();
    //读取命令行参数文件中的内容
    //let config_file = matches.value_of("config").unwrap_or("src/test.conf");
    //读取命令参数内容
    let config_file = matches.value_of("config").unwrap();
    //let toml_string = read_config_file(config_file);

    //println!("配置文件内容：{}",toml_string.as_str());
    loop {
        println!("{}", config_file);
    }
}