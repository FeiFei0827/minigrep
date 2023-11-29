use std::error::Error;
use std::fs;

pub struct Config {
    pub query: String,
    pub file_path: String,
}

impl Config {
    pub fn new(args: &[String]) -> Config {
        if args.len() < 3 {
            panic!("\x1b[31mnot enough arguments\x1b[0m");
        }//panic
        let query = args[1].clone();
        let file_path = args[2].clone();

        Config { query, file_path }
    }
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        Ok(Config { query, file_path })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;
// ?运算符用于将可能的错误传播给调用者进行处理。如果读取文件出现错误，函数将立即返回错误结果并终止执行。
    println!("With text:\n{contents}");

    Ok(())
    //如果函数执行成功并成功打印文件内容，那么将返回一个表示成功的 Result 对象，其中结果为空。
}
pub fn parse_config(args: &[String]) -> Config {
    let query = args[1].clone();
    let file_path = args[2].clone();

    Config { query, file_path }
}
// fn run(config: Config) {
//     let contents = fs::read_to_string(config.file_path)
//         .expect("Should have been able to read the file");
//
//     println!("With text:\n{contents}");
// }

