use rs_oc_runtime::Encoding;

fn main() {
    let encode = Encoding::Struct("A", &[Encoding::Char, Encoding::Int]);
    println!("{}", encode);
    println!("{}", encode.eq("{A=ci}"));
    // assert_eq!(encode, "{A=ci}");
    //
}
