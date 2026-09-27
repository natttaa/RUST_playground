fn main() {

    let s: String = String::from("Hello String"); //heap
    let s_2: &str = &s[0..5];  //говорим смотреть на кусок памяти в S, тут референс. то есть s оригинал, а s_2 референс. который ссылается
    println!("{}", s_2);

    let msg: &str = "hello2"; //stack
    println!("{}", msg);

    let message: String = "hello3".to_string(); //heap
    println!("{}", message);

    // Works
    let x: i32 = 50;
    let _y: i32 = x; //уберет warning использования с поомщью _
    println!("{}", x);

    // Will not work
    /*let s: String = String::from("hello");
    let t: String = s; //тут получается, что есть два владельца, чего быть не может в  ownership
    println!("{}", t);*/

    let s: String = String::from("hello");
    let t: String = s.clone(); //так лучше не делать, но ок пока. Получается удвоение памяти
    println!("{}", t); 

    let s: String = String::from("hello5");
    let t: &String = &s; //borowing
    println!("{}", t); 

}

// &str - срез строки