#[test]
fn test_format_string(){
    //默认的格式化字符串
    assert_eq!("1",format!("{}",1));
    //八进制
    assert_eq!("11",format!("{:o}",9));
    //十六进制小写
    assert_eq!("ff",format!("{:x}",255));
    //十六进制大写
    assert_eq!("FF",format!("{:X}",255));
    //指针
    println!("{:p}",&0);
    //二进制
    assert_eq!("1111",format!("{:b}",15));
    //科学计数法小写
    assert_eq!("1e4",format!("{:e}",10000f32));
    //科学计数法大写
    assert_eq!("1E4",format!("{:E}",10000f32));
    //打印实现了Debug接口的信息
    println!("{:?}","test");
    assert_eq!("\"test\"",format!("{:?}","test"));
    //打印实现了Debug接口的带换行和缩进信息
    assert_eq!("(\n    \"test1\",\n    \"test2\"\n)",format!("{:#?}",("test1","test2")));
    //命名参数
    assert_eq!("x y y",format!("{a} {b} {b}",a="x",b="y"));
}