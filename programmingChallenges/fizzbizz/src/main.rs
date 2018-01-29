fn main() {
    for x in 1..100{
        match x {
            //x if (x % 2) == 0 => println!("{}", x),
            x if (x % 5)==0&&((x % 3)==0) => println!("FizzBizz; {}", x),
            x if (x % 5)==0&&((x % 3)!=0) => println!("Bizz; {}", x),
            x if (x % 5)!=0&&((x % 3)==0) => println!("Fizz; {}", x),
            _ => println!("{}", x),
        };
    }
}
