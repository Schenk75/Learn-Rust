use std::io;    // 将io库引入当前作用域
use rand::Rng;  // Rng是一个trait，定义了随机数生成器实现的方法，想使用这些方法的话，此trait必须在作用域中
use std::cmp::Ordering;  // Ordering是一个枚举，成员是 Less、Greater 和 Equal


fn main() {
    println!("Guess the number!");      // 在屏幕上打印字符串的宏
    /*
    rand::thread_rng 函数提供实际使用的随机数生成器, 位于当前执行线程的本地环境中，并从操作系统获取seed
    gen_range 方法获取两个数字作为参数，并生成一个范围在两者之间的随机数
    */
    let secret_number = rand::thread_rng().gen_range(1, 101);
    // println!("The secret number is: {}", secret_number);
    loop{
        println!("Please input your guess:");
        /*
        let 创建变量，变量默认不可变
        mut 使得变量可变
        :: 表明new是String类型的一个关联函数
        */
        let mut guess = String::new();
        /*
        io::stdin函数返回一个终端标准输入句柄
        read_line将标准输入存入字符串
        &表示这个参数是一个引用
        read_line函数返回一个Result类型（枚举），成员有Ok和Err
        Result实例有expect方法：
            若Result值为Ok，expect获取Ok中的值并原样返回
            若Result值为Err，expect导致程序崩溃，并显式当做参数传给expect的信息
        */
        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        // println!的占位符{}
        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
    
}