use rs_oc_runtime::Encoding;

fn main() {
    let encode = Encoding::Struct("A", &[Encoding::Char, Encoding::Int]);
    println!("{:?}", encode.to_string());
    println!("{}", encode);
    assert_eq!(&encode, "{A=ci}");
    println!("{}", encode.eq("{A=ci}"));
    // let my_str = "123";
    // my_str.eq("123");
    // assert_eq!(encode, "{A=ci}");
    //
}
