use num_bigint::{BigInt, ToBigInt};
use num_traits::{Zero, One}; // Добавляем One для единицы

// Определяем наш собственный тип числа
#[derive(Debug, Clone)]
pub enum DynamicInt {
    Small(i128),
    Big(BigInt),
}

impl DynamicInt {
    // Конструктор из i128
    pub fn new(val: i128) -> Self {
        DynamicInt::Small(val)
    }

    // Конструктор единицы
    pub fn one() -> Self {
        DynamicInt::Small(1)
    }

    // Пример операции сложения
    pub fn add(&self, other: &DynamicInt) -> Self {
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
    pub fn mul(&self, other: &DynamicInt) -> Self {
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

    // Операция деления
    pub fn div(&self, other: &DynamicInt) -> Self {
        match (self, other) {
            (DynamicInt::Small(a), DynamicInt::Small(b)) => {
                DynamicInt::Small(a / b)
            }
            (DynamicInt::Big(a), DynamicInt::Small(b)) => {
                DynamicInt::Big(a / b.to_bigint().unwrap())
            }
            (DynamicInt::Small(a), DynamicInt::Big(b)) => {
                DynamicInt::Big(a.to_bigint().unwrap() / b)
            }
            (DynamicInt::Big(a), DynamicInt::Big(b)) => {
                DynamicInt::Big(a / b)
            }
        }
    }

    // Операция получения остатка
    pub fn rem(&self, other: &DynamicInt) -> Self {
        match (self, other) {
            (DynamicInt::Small(a), DynamicInt::Small(b)) => {
                DynamicInt::Small(a % b)
            }
            (DynamicInt::Big(a), DynamicInt::Small(b)) => {
                DynamicInt::Big(a % b.to_bigint().unwrap())
            }
            (DynamicInt::Small(a), DynamicInt::Big(b)) => {
                DynamicInt::Big(a.to_bigint().unwrap() % b)
            }
            (DynamicInt::Big(a), DynamicInt::Big(b)) => {
                DynamicInt::Big(a % b)
            }
        }
    }

    // Проверка на равенство нулю
    pub fn is_zero(&self) -> bool {
        match self {
            DynamicInt::Small(n) => *n == 0,
            DynamicInt::Big(n) => n.is_zero(),
        }
    }

    // Сравнение чисел
    pub fn eq(&self, other: &DynamicInt) -> bool {
        match (self, other) {
            (DynamicInt::Small(a), DynamicInt::Small(b)) => a == b,
            (DynamicInt::Big(a), DynamicInt::Small(b)) => a == &b.to_bigint().unwrap(),
            (DynamicInt::Small(a), DynamicInt::Big(b)) => &a.to_bigint().unwrap() == b,
            (DynamicInt::Big(a), DynamicInt::Big(b)) => a == b,
        }
    }

    // Сравнение: меньше чем
    pub fn lt(&self, other: &DynamicInt) -> bool {
        match (self, other) {
            (DynamicInt::Small(a), DynamicInt::Small(b)) => a < b,
            (DynamicInt::Big(a), DynamicInt::Small(b)) => a < &b.to_bigint().unwrap(),
            (DynamicInt::Small(a), DynamicInt::Big(b)) => &a.to_bigint().unwrap() < b,
            (DynamicInt::Big(a), DynamicInt::Big(b)) => a < b,
        }
    }

    // Получение квадратного корня (приблизительно)
    pub fn sqrt(&self) -> Self {
        match self {
            DynamicInt::Small(n) => {
                let sqrt_val = (*n as f64).sqrt() as i128;
                DynamicInt::Small(sqrt_val)
            }
            DynamicInt::Big(n) => {
                // Для BigInt используем приблизительный алгоритм
                DynamicInt::Big(n.sqrt())
            }
        }
    }

    // Проверка на совершенное число
    pub fn is_perfect(&self) -> bool {
        let one = DynamicInt::one();
        let two = DynamicInt::new(2);
        
        // Совершенными могут быть только числа >= 2
        if self.lt(&two) {
            return false;
        }

        let mut sum = DynamicInt::one(); // Начинаем с 1 (всегда делитель)
        let mut i = DynamicInt::new(2);
        let sqrt_n = self.sqrt();

        // Ищем делители до квадратного корня
        while i.lt(&sqrt_n) || i.eq(&sqrt_n) {
            if self.rem(&i).is_zero() {
                sum = sum.add(&i);
                
                // Добавляем парный делитель, если он не равен корню
                let pair_divisor = self.div(&i);
                if !i.eq(&pair_divisor) {
                    sum = sum.add(&pair_divisor);
                }
            }
            i = i.add(&one);
        }

        sum.eq(self)
    }

    // Проверяем, является ли число типом Small или Big
    pub fn is_small(&self) -> bool {
        matches!(self, DynamicInt::Small(_))
    }

    pub fn get_type_name(&self) -> &'static str {
        match self {
            DynamicInt::Small(_) => "i128",
            DynamicInt::Big(_) => "BigInt",
        }
    }

    // Метод для получения значения в виде строки
    pub fn to_string_value(&self) -> String {
        match self {
            DynamicInt::Small(n) => n.to_string(),
            DynamicInt::Big(n) => n.to_string(),
        }
    }
} 