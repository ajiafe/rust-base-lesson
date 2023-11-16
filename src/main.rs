use std::io;
use rand::Rng;
use std::cmp::Ordering; // 枚举类型

fn main (){
    println!("Guess the number!");
    // gen_range(1,101);
    let secret_number = rand::thread_rng().gen_range(1..=100); // i32 u32 
    // println!("The secret number is: {secret_number}");
    
    loop {
        println!("输入你猜测的数字.");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        
        // 1.同名变量 shadow 
        // 2. u32 无符号整数类型 
        // 3. trim() 去掉字符串 左右两次的空白 注意这里是用户输入 需要回车 会自动带入\n 所以要把他去掉
        // let guess: u32 = guess.trim().parse().expect("Please type a number!");
        // 优雅的处理错误 match 枚举类型 Ok 和 Err 
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
    
        println!("Your guessed: {guess}");
    
        // 字符串参数
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You win!");
                break;
            },
        }
    }
}