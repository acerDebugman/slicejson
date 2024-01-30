pub fn main() {
    // let data = r#"
    // {
    //     "zgc": null,
    //     "zgc": -0.03e+10,
    //     "zgc": false,
    //     "zgc": true,
    //     "zgc": {"kkk2":"abc"},
    //     "zgc": ["kkk2","abc"]
    // }
    // "#;
    // let data = r#"{"zgc":"kkk"}"#;
    // let data = r#"
    // [
    //     {"zgc": null,
    //     "zgc": -0.03e+10,
    //     "zgc": false,
    //     "zgc": true,
    //     "zgc": {"kkk2":"abc"},
    //     "zgc": ["kkk2","abc"]}
    // ]
    // "#;
    // let data = r#"
    // [
    //     "zgc", "ab c  ", true, false, 123.10, 0, -0
    // ]
    // "#;
    // let data = "-0";
    let data = "-0.0";
    println!("data: {:?}", data);
    let v = slicejson::parser::parse(data);

    println!("Value: {:?}", v);
    println!("****show:");
    v.show(data);

    println!("parse again:");
    let _ = slicejson::parser::parse(&v.to_string(data));
}
