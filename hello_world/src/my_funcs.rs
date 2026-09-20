pub fn add_five(num: u32) -> u32 { //публичная функция, которую можно везде вызвать pub
    num + 5
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn add_five_test() {
        let x = 100;
        let z = add_five(x);
        println!("x and z from test:{} {}", x, z);
        assert_eq!(z, 105); //сравниваем, какой результат получается и какой результат мы ожидаем
    }
}