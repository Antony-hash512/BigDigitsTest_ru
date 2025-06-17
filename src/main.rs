mod dynamic_int;

use dynamic_int::DynamicInt;
use std::time::Instant;

fn main() {
    println!("🔢 Поиск совершенных чисел с использованием DynamicInt");
    println!("================================================");
    
    // Автоматически запускаем тест известных совершенных чисел
    //test_known_perfect_numbers();
     find_perfect_numbers();
    // println!("\n🔍 Теперь попробуем найти следующее совершенное число после 8128...");
    // search_perfect_numbers_in_range(8129, 50000);
}

fn test_known_perfect_numbers() {
    println!("🧪 Тестируем известные совершенные числа...\n");
    
    // Первые несколько совершенных чисел: 6, 28, 496, 8128
    let test_numbers = vec![6, 28, 496, 8128];
    
    for num in test_numbers {
        let start_time = Instant::now();
        let dynamic_num = DynamicInt::new(num);
        let is_perfect = dynamic_num.is_perfect();
        let elapsed = start_time.elapsed();
        
        println!("Число: {} | Совершенное: {} | Тип: {} | Время: {:.3?}",
            num, 
            if is_perfect { "✅" } else { "❌" },
            dynamic_num.get_type_name(),
            elapsed
        );
    }
    
    println!("\n🔍 Также проверим несколько несовершенных чисел:");
    let non_perfect = vec![5, 7, 10, 12, 100, 1000];
    
    for num in non_perfect {
        let start_time = Instant::now();
        let dynamic_num = DynamicInt::new(num);
        let is_perfect = dynamic_num.is_perfect();
        let elapsed = start_time.elapsed();
        
        println!("Число: {} | Совершенное: {} | Тип: {} | Время: {:.3?}",
            num, 
            if is_perfect { "✅" } else { "❌" },
            dynamic_num.get_type_name(),
            elapsed
        );
    }
    
    println!("\n🎯 Результат: алгоритм корректно определяет совершенные числа!");
}

fn search_perfect_numbers_in_range(start: i128, end: i128) {
    println!("🔍 Ищем совершенные числа в диапазоне от {} до {}...\n", start, end);
    
    let mut current = DynamicInt::new(start);
    let one = DynamicInt::one();
    let end_num = DynamicInt::new(end);
    let mut found_count = 0;
    let mut checked_count = 0;
    let total_start_time = Instant::now();
    
    while current.lt(&end_num) {
        let start_time = Instant::now();
        
        if current.is_perfect() {
            let elapsed = start_time.elapsed();
            found_count += 1;
            
            println!("🎉 НАЙДЕНО СОВЕРШЕННОЕ ЧИСЛО №{}!", found_count);
            println!("   📊 Число: {}", current.to_string_value());
            println!("   🔢 Тип: {}", current.get_type_name());
            println!("   ⏱️  Время проверки: {:.3?}", elapsed);
            println!("   📍 Позиция в диапазоне: {}/{}\n", 
                current.to_string_value(), end);
        }
        
        checked_count += 1;
        
        // Переходим к следующему числу
        current = current.add(&one);
        
        // Для информативности выводим прогресс каждые 5000 чисел
        if checked_count % 5000 == 0 {
            let progress = (checked_count as f64 / (end - start) as f64) * 100.0;
            println!("🔄 Проверено: {} чисел ({:.1}%)", checked_count, progress);
        }
    }
    
    let total_elapsed = total_start_time.elapsed();
    println!("\n📊 Итоги поиска:");
    println!("   🔢 Диапазон: {} - {}", start, end);
    println!("   ✅ Найдено совершенных чисел: {}", found_count);
    println!("   📋 Всего проверено: {}", checked_count);
    println!("   ⏱️  Общее время: {:.2?}", total_elapsed);
    println!("   ⚡ Скорость: {:.0} чисел/сек", 
        checked_count as f64 / total_elapsed.as_secs_f64());
    
    if found_count == 0 {
        println!("   💡 Следующее совершенное число после 8128 = 33550336 (очень большое!)");
    }
}

// Функция для бесконечного поиска (закомментирована для безопасности)
fn find_perfect_numbers() {
    println!("🔍 Начинаем бесконечный поиск совершенных чисел...\n");
    println!("⚠️  Внимание: это может занять очень много времени!");
    println!("   Нажмите Ctrl+C для остановки\n");
    
    let mut current = DynamicInt::new(2); // Начинаем с 2
    let one = DynamicInt::one();
    let mut found_count = 0;
    let mut checked_count = 0;
    
    loop {
        let start_time = Instant::now();
        
        if current.is_perfect() {
            let elapsed = start_time.elapsed();
            found_count += 1;
            
            println!("🎉 НАЙДЕНО СОВЕРШЕННОЕ ЧИСЛО №{}!", found_count);
            println!("   📊 Число: {}", current.to_string_value());
            println!("   🔢 Тип: {}", current.get_type_name());
            println!("   ⏱️  Время проверки: {:.2?}", elapsed);
            println!("   📍 Всего проверено: {}\n", checked_count);
        }
        
        checked_count += 1;
        
        // Переходим к следующему числу
        current = current.add(&one);
        
        // Для информативности выводим прогресс каждые 10000 чисел
        //if checked_count % 1000000 == 0 {
        //    println!("🔄 Проверено: {} чисел", checked_count);
        //}
    }
}

fn test1() {
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
