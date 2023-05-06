
fn odd(n: i32) -> Result<i32, String> {
    if n % 2 == 1 {
        Ok(n)
    } else {
        Err(format!("{} is not odd", n))
    }
}

fn my_fn2(n: i32) -> Result<&'static str, &'static str> {
    if n % 2 == 1 {
        Ok("n is odd")
    } else {
        Err("n is even")
    }
}

fn my_fn3(n: i32) -> Result<String, String> {
    if n % 2 == 1 {
        Ok(String::from("n is odd"))
    } else {
        Err(String::from("n is even"))
    }
}

fn my_function(n: i32) -> () {
    if n % 2 == 1 {
        println!("n is odd");
    } else {
        println!("n is even");
    }
}

fn main() {
    let double = |n| -> Result<i32, String> {
        let n = odd(n)?; // odd が Err ならすぐに return する
        return Ok(n * 2);
    };

    for n in 0 .. 4 {
        println!("number: {}", n);
        println!("double result: {:?}\n", double(n));
    }

    let my_fn_result = my_fn2(123).unwrap();
    println!("{}",  my_fn_result);
    let my_fn3_result = my_fn3(125).unwrap();
    println!("{}",  my_fn3_result);
}
