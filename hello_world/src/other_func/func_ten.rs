pub fn func_ten(num: u32) -> u32 { //публичная функция, которую можно везде вызвать pub
    num - 10
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn func_ten_test() {
        let x = 100;
        let z = func_ten(x);
        println!("x and z from test:{} {}", x, z);
        assert_eq!(z, 90); //сравниваем, какой результат получается и какой результат мы ожидаем
    }
}