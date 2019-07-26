//main方法可以返回任何满足Termination trait约束的类型,包括(),bool,Result
fn main() {
    //std::env::args()可以得到传入程序的参数
    for arg in std::env::args() {
        println!("Arg: {}", arg)
    }
    //std::env::vars获取所有的环境变量
    for arg in std::env::vars() {
        println!("Env Variable: {},{}", arg.0, arg.1)
    }
    //std::env::var获取特定的某个环境变量
    let result = std::env::var("SHELL");
    match result {
        Ok(val) => println!("{}:{:?}", "SHELL", val),
        Err(err) => println!("environment variable {} not found!", "SHELL")
    }
    std::process::exit(0)
}
