use num_bigint::{BigInt, ToBigInt};
use num_traits::Zero; // Для BigInt::zero()

// Определяем наш собственный тип числа
#[derive(Debug, Clone)]
enum DynamicInt {
    Small(i128),
    Big(BigInt),
}

impl DynamicInt {
    // Конструктор из i128
    fn new(val: i128) -> Self {
        DynamicInt::Small(val)
    }

    // Пример операции сложения
    fn add(&self, other: &DynamicInt) -> Self {
        match (self, other) {
            (DynamicInt::Small(a), DynamicInt::Small(b)) => {
                // Пытаемся сложить как i128
                let (result, overflow) = a.overflowing_add(*b);
                if overflow {
                    // Если переполнение, конвертируем оба числа в BigInt и складываем
                    let big_a = a.to_bigint().unwrap();
                    let big_b = b.to_bigint().unwrap();
                    DynamicInt::Big(big_a + big_b)
                } else {
                    DynamicInt::Small(result)
                }
            }
            // Если одно из чисел уже Big, то сразу работаем с BigInt
            (DynamicInt::Big(a), DynamicInt::Small(b)) => {
                DynamicInt::Big(a + b.to_bigint().unwrap())
            }
            (DynamicInt::Small(a), DynamicInt::Big(b)) => {
                DynamicInt::Big(a.to_bigint().unwrap() + b)
            }
            (DynamicInt::Big(a), DynamicInt::Big(b)) => {
                DynamicInt::Big(a + b)
            }
        }
    }

    // Пример операции умножения
    fn mul(&self, other: &DynamicInt) -> Self {
        match (self, other) {
            (DynamicInt::Small(a), DynamicInt::Small(b)) => {
                let (result, overflow) = a.overflowing_mul(*b);
                if overflow {
                    let big_a = a.to_bigint().unwrap();
                    let big_b = b.to_bigint().unwrap();
                    DynamicInt::Big(big_a * big_b)
                } else {
                    DynamicInt::Small(result)
                }
            }
            (DynamicInt::Big(a), DynamicInt::Small(b)) => {
                DynamicInt::Big(a * b.to_bigint().unwrap())
            }
            (DynamicInt::Small(a), DynamicInt::Big(b)) => {
                DynamicInt::Big(a.to_bigint().unwrap() * b)
            }
            (DynamicInt::Big(a), DynamicInt::Big(b)) => {
                DynamicInt::Big(a * b)
            }
        }
    }

    // Метод для получения значения в виде строки
    fn to_string_value(&self) -> String {
        match self {
            DynamicInt::Small(n) => n.to_string(),
            DynamicInt::Big(n) => n.to_string(),
        }
    }
}

fn main() {
    // Пример использования
    let num1 = DynamicInt::new(i128::MAX - 10);
    let num2 = DynamicInt::new(20);

    println!("num1: {}", num1.to_string_value());
    println!("num2: {}", num2.to_string_value());

    // Сложение, которое вызывает переполнение i128
    let sum = num1.add(&num2);
    println!("Сумма (ожидается Big): {}", sum.to_string_value());
    if let DynamicInt::Big(_) = sum {
        println!("Сумма действительно стала BigInt!");
    }

    let small_sum = DynamicInt::new(5).add(&DynamicInt::new(10));
    println!("Малая сумма (ожидается Small): {}", small_sum.to_string_value());
    if let DynamicInt::Small(_) = small_sum {
        println!("Малая сумма осталась Small!");
    }

    // Умножение
    let large_num_i128 = i128::MAX / 2;
    let mul1 = DynamicInt::new(large_num_i128);
    let mul2 = DynamicInt::new(3); // Умножение на 3 переполнит i128::MAX

    println!("\nmul1: {}", mul1.to_string_value());
    println!("mul2: {}", mul2.to_string_value());

    let product = mul1.mul(&mul2);
    println!("Произведение (ожидается Big): {}", product.to_string_value());
    if let DynamicInt::Big(_) = product {
        println!("Произведение действительно стало BigInt!");
    }
}
