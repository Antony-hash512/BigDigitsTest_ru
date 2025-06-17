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

    // Проверка на простое число
    pub fn is_prime(&self) -> bool {
        let zero = DynamicInt::new(0);
        let one = DynamicInt::one();
        let two = DynamicInt::new(2);
        let three = DynamicInt::new(3);
        
        // 0 и 1 не являются простыми
        if self.eq(&zero) || self.eq(&one) {
            return false;
        }
        
        // 2 и 3 - простые числа
        if self.eq(&two) || self.eq(&three) {
            return true;
        }
        
        // Четные числа больше 2 не простые
        if self.rem(&two).is_zero() {
            return false;
        }
        
        // Числа, делящиеся на 3, не простые
        if self.rem(&three).is_zero() {
            return false;
        }
        
        // Проверяем делители вида 6k±1 до квадратного корня
        let mut i = DynamicInt::new(5);
        let sqrt_n = self.sqrt();
        let six = DynamicInt::new(6);
        
        while i.lt(&sqrt_n) || i.eq(&sqrt_n) {
            // Проверяем i и i+2 (вида 6k-1 и 6k+1)
            if self.rem(&i).is_zero() {
                return false;
            }
            
            let i_plus_2 = i.add(&two);
            if (i_plus_2.lt(&sqrt_n) || i_plus_2.eq(&sqrt_n)) && self.rem(&i_plus_2).is_zero() {
                return false;
            }
            
            // Переходим к следующей паре (следующее 6k-1)
            i = i.add(&six);
        }
        
        true
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

    // Возведение в степень
    pub fn pow(&self, exponent: u32) -> Self {
        match self {
            DynamicInt::Small(base) => {
                // Для малых степеней пытаемся использовать i128
                if exponent <= 63 && *base > 0 && *base <= (i128::MAX as f64).powf(1.0 / exponent as f64) as i128 {
                    if let Some(result) = base.checked_pow(exponent) {
                        return DynamicInt::Small(result);
                    }
                }
                // Если переполнение, используем BigInt
                let big_base = base.to_bigint().unwrap();
                DynamicInt::Big(num_traits::pow(big_base, exponent as usize))
            }
            DynamicInt::Big(base) => {
                DynamicInt::Big(num_traits::pow(base.clone(), exponent as usize))
            }
        }
    }

    // Вычисление 2^x
    pub fn power_of_two(x: u32) -> Self {
        let two = DynamicInt::new(2);
        two.pow(x)
    }

    // Вычисление числа Мерсенна: 2^x - 1
    pub fn mersenne_number(x: u32) -> Self {
        let power_of_two = Self::power_of_two(x);
        let one = DynamicInt::one();
        
        match power_of_two {
            DynamicInt::Small(n) => DynamicInt::Small(n - 1),
            DynamicInt::Big(n) => DynamicInt::Big(n - 1),
        }
    }

    // Вычисление совершенного числа по формуле Евклида: 2^(x-1) * (2^x - 1)
    pub fn euclid_perfect_number(x: u32) -> Self {
        if x < 2 {
            return DynamicInt::new(0); // Не может быть совершенным для x < 2
        }
        
        let mersenne = Self::mersenne_number(x);
        let power_factor = Self::power_of_two(x - 1);
        
        mersenne.mul(&power_factor)
    }

    // Проверка, является ли число Мерсенна простым
    pub fn is_mersenne_prime(x: u32) -> bool {
        let mersenne = Self::mersenne_number(x);
        mersenne.is_prime()
    }

    // Вычитание
    pub fn sub(&self, other: &DynamicInt) -> Self {
        match (self, other) {
            (DynamicInt::Small(a), DynamicInt::Small(b)) => {
                let (result, overflow) = a.overflowing_sub(*b);
                if overflow {
                    let big_a = a.to_bigint().unwrap();
                    let big_b = b.to_bigint().unwrap();
                    DynamicInt::Big(big_a - big_b)
                } else {
                    DynamicInt::Small(result)
                }
            }
            (DynamicInt::Big(a), DynamicInt::Small(b)) => {
                DynamicInt::Big(a - b.to_bigint().unwrap())
            }
            (DynamicInt::Small(a), DynamicInt::Big(b)) => {
                DynamicInt::Big(a.to_bigint().unwrap() - b)
            }
            (DynamicInt::Big(a), DynamicInt::Big(b)) => {
                DynamicInt::Big(a - b)
            }
        }
    }

    // Вычисление факториала
    pub fn factorial(&self) -> Self {
        let zero = DynamicInt::new(0);
        let one = DynamicInt::one();
        
        // 0! = 1, 1! = 1
        if self.eq(&zero) || self.eq(&one) {
            return one;
        }
        
        let mut result = one.clone();
        let mut current = DynamicInt::new(2);
        
        while current.lt(self) || current.eq(self) {
            result = result.mul(&current);
            current = current.add(&one);
        }
        
        result
    }

    // Статический метод для вычисления факториала от числа
    pub fn factorial_of(n: i128) -> Self {
        let num = DynamicInt::new(n);
        num.factorial()
    }

    // Проверка, является ли число факториалом некоторого числа
    pub fn is_factorial(&self) -> (bool, i128) {
        let one = DynamicInt::one();
        let mut n = DynamicInt::new(1);
        let mut factorial = one.clone();
        
        loop {
            if factorial.eq(self) {
                return (true, match n {
                    DynamicInt::Small(val) => val - 1,
                    DynamicInt::Big(_) => -1, // Слишком большое число
                });
            }
            
            if factorial.lt(self) {
                factorial = factorial.mul(&n);
                n = n.add(&one);
                
                // Ограничиваем поиск разумными пределами
                if let DynamicInt::Small(val) = n {
                    if val > 200 {
                        break;
                    }
                }
            } else {
                break;
            }
        }
        
        (false, -1)
    }

    // Вычисление факториала в диапазоне (для многопоточности)
    pub fn factorial_range(start: i128, end: i128) -> Self {
        let mut result = DynamicInt::one();
        let mut current = DynamicInt::new(start);
        let end_num = DynamicInt::new(end);
        
        while current.lt(&end_num) || current.eq(&end_num) {
            result = result.mul(&current);
            current = current.add(&DynamicInt::one());
        }
        
        result
    }

    // Вычисление произведения в диапазоне (оптимизированное для многопоточности)
    pub fn product_range(start: i128, end: i128) -> Self {
        if start > end {
            return DynamicInt::one();
        }
        
        if start == end {
            return DynamicInt::new(start);
        }
        
        let mut result = DynamicInt::new(start);
        for i in (start + 1)..=end {
            result = result.mul(&DynamicInt::new(i));
        }
        result
    }
} 