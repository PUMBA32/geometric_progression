use std::io;

#[derive(Debug)]
struct Progression {
    a1: f64,
    a2: f64,
    n: f64
}

impl Progression {
    // создание прогрессии
    fn new(a1: f64, a2:f64, n: f64) -> Progression {
        Progression {a1, a2, n}
    }

    // нахождение делителя
    fn get_q(&self) -> f64 {
        self.a2 / self.a1
    }

    // нахождение элемента "a" с индексом n 
    fn get_an(&self, q: f64) -> f64 {
        self.a1*q.powf(self.n - 1.0)
    }

    // нахождение суммы элементов до n-ого
    fn sum(&self, q: f64) -> f64 {
        (self.a1*(q.powf(self.n) - 1.0)) / (q - 1.0) 
    }
}

fn main() {
    println!("Добро пожаловать!");
    println!("В этой программе ты можешь:\n- Найти сумму элементов\n- Найти q\n- Найти n-q элемент прогрессии");
    println!("\nДля этого тебе нужно (вводить так: 4.0, 3.6 ..):\n\t1. Знать первые два элемента прогрессии\n\t2. Ввести нужный индекс\n");

    loop {
        let mut str_a1 = String::new();
        let mut str_a2 = String::new();
        let mut str_n = String::new();

        println!("Введи a1: ");
        match io::stdin().read_line(&mut str_a1) {
            Ok(_) => {},
            Err(e) => println!("{}", e)
        }

        println!("Введи a2: ");
        match io::stdin().read_line(&mut str_a2) {
            Ok(_) => {},
            Err(e) => println!("{}", e)
        }

        println!("Введи n: ");
        match io::stdin().read_line(&mut str_n) {
            Ok(_) => {},
            Err(e) => println!("{}", e)
        }

        let a1: f64 = str_a1.trim().parse().unwrap();
        let a2: f64 = str_a2.trim().parse().unwrap();
        let n: f64 = str_n.trim().parse().unwrap();

        let prog = Progression::new(a1, a2, n);
        let q = prog.get_q();
        let sum = prog.sum(q);
        let an = prog.get_an(q);

        println!("q: {}", q);
        println!("Сумма первых {} элементов: {}", n, sum);
        println!("{}-й элемент прогрессии: {}", n, an);

        
        let n = get_num();
        if n == 1 {
            continue;
        } else {
            break;
        }
    }
}

fn get_num() -> i32 {
    loop {
        println!("Продолжить?\n[1] - Да\n[2] - Нет");
        let mut str_choice = String::new();
        match io::stdin().read_line(&mut str_choice) {
            Ok(_) => {},
            Err(e) => println!("{}", e)
        }
        let num: i32 = str_choice.trim().parse().unwrap();

        if num == 1{
            return num; 
        } else if num == 2 {
            return num;
        } else {
            println!();
            continue;
        }
    }
}