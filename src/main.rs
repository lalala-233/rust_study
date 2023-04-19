use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn guess() {
    let mut time = 0;
    let random = rand::thread_rng().gen_range(1..=100);
    loop {
        println!("\n这是你第 {time} 次猜测，你猜的数字为（范围：1~100）：");
        time += 1;
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("错误：无法获取输入！");
        let guess: u32 = match guess.trim().parse() {
            Ok(guess) => guess,
            Err(_) => {
                println!("错误：请输入数字！");
                time -= 1;
                continue;
            }
        };
        match guess.cmp(&random) {
            Ordering::Equal => {
                println!("你猜对了！");
                break;
            }
            Ordering::Greater => println!("你猜的数大了～"),
            Ordering::Less => println!("你猜的数小了～"),
        }
    }
}

fn main() {
    println!("这是一个猜数字游戏，程序会生成一个随机数。");
    loop {
        guess();
        println!("\n恭喜你获得胜利！再来一盘或者关闭游戏～");
    }
}
