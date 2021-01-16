fn main() {
    let mut s = String::new();
    let data = "initial contents";

    let d = data.to_string();

    let m = "initial contents".to_string();

    let v = String::from("initial contents");

    // hello in different languages
    let hello = String::from("السلام عليكم");
    let hello = String::from("Dobrý den");
    let hello = String::from("Hello");
    let hello = String::from("שָׁלוֹם");
    let hello = String::from("नमस्ते");
    let hello = String::from("こんにちは");
    let hello = String::from("안녕하세요");
    let hello = String::from("你好");
    let hello = String::from("Olá");
    let hello = String::from("Здравствуйте");
    let hello = String::from("Hola");

    s.push_str("Hello world!");
    println!("{}", s);

    let mut s1 = String::from("foo");
    let s2 = "bar";

    s1.push_str(s2);
    println!("s2 is {}", s2);

    let mut s3 = String::from("lo");
    s3.push('l');
    println!("s3 is {}", s3);

    println!("s3 + s2 = {}", s3 + s2);

    let (s4, s5, s6) = (
        String::from("tic"),
        String::from("tac"),
        String::from("toe"),
    );

    // let t = s4 + "-" + &s5 + "-" + &s6;
    // println!("{}", t);
    let t2 = format!("{}-{}-{}", s4, s5, s6);
    println!("{}", t2);
    let hello = "Здравствуйте";

    let s = &hello[0..4];

    println!("{}", s);
    // let z = &hello[0..1]; // panics here
    for c in "नमस्ते".chars() {
        println!("{}", c);
    }
    for b in "नमस्ते".bytes() {
        println!("{}", b);
    }
}
