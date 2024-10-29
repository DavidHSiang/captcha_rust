use captcha_rust::Captcha;

fn main() {
    let a = Captcha::new(5, 130, 40);
    println!("test:{},base_img:{}", a.text, a.base_img);
}
