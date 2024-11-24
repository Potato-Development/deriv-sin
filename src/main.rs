const CONST_PI: f64 = 3.1415926535_8979323846_2643383279_5028841971_6939937510;
const CONST_H: f64 = 0.0001;

fn main() {
    let mut print_gradient: String = String::new();

    let mut i: f64 = 0.0;
    while i <= 2.0_f64 * CONST_PI {
        let m: f64 = ((i + CONST_H).sin() - i.sin()) / CONST_H;
        println!("{}|{}|{}", i, i.sin(), m);

        // エクセル用の出力
        print_gradient = format!("{}{}, ", print_gradient, m);

        i += CONST_PI / 180.0_f64;
    }

    // エクセル用の出力
    println!("{}", print_gradient);
}
