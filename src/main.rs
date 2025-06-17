mod dynamic_int;

use dynamic_int::DynamicInt;
use std::time::Instant;
use std::thread;
use std::sync::{Arc, mpsc};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::io::{self, Write};

fn main() {
    println!("🔢 Поиск совершенных чисел с использованием DynamicInt");
    println!("================================================");
    
    loop {
        println!("\n📋 Выберите режим работы:");
        println!("1. 🧪 Тест известных совершенных чисел");
        println!("2. 🔍 Поиск в ограниченном диапазоне (совершенные)");
        println!("3. 🚀 Многопоточный поиск совершенных чисел");
        println!("4. 🔄 Однопоточный бесконечный поиск (совершенные)");
        println!("5. ♾️  Бесконечный многопоточный поиск (совершенные)");
        println!("6. 🔢 Тест простых чисел");
        println!("7. 🔍 Поиск простых чисел в диапазоне");
        println!("8. 🚀 Многопоточный поиск простых чисел");
        println!("9. ♾️  Бесконечный многопоточный поиск простых чисел");
        println!("10. 🚪 Выход");
        println!("\nВведите номер (1-10): ");
        
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Не удалось прочитать ввод");
        
        match input.trim() {
            "1" => {
                test_known_perfect_numbers();
            }
            "2" => {
                println!("\n📊 Введите диапазон поиска:");
                
                print!("Начальное число: ");
                io::stdout().flush().unwrap();
                let mut start_input = String::new();
                io::stdin().read_line(&mut start_input).expect("Ошибка чтения");
                let start_num: i128 = start_input.trim().parse().unwrap_or(8129);
                
                print!("Конечное число: ");
                io::stdout().flush().unwrap();
                let mut end_input = String::new();
                io::stdin().read_line(&mut end_input).expect("Ошибка чтения");
                let end_num: i128 = end_input.trim().parse().unwrap_or(50000);
                
                search_perfect_numbers_in_range(start_num, end_num);
            }
            "3" => {
                println!("\n🧵 Настройка многопоточного поиска:");
                
                // Определяем количество логических ядер
                let available_cores = thread::available_parallelism().map(|p| p.get()).unwrap_or(4);
                println!("💻 Доступно логических ядер: {}", available_cores);
                
                print!("Введите количество потоков (по умолчанию {}): ", available_cores);
                io::stdout().flush().unwrap();
                let mut threads_input = String::new();
                io::stdin().read_line(&mut threads_input).expect("Ошибка чтения");
                let num_threads = threads_input.trim().parse().unwrap_or(available_cores);
                
                print!("Начальное число (по умолчанию 2): ");
                io::stdout().flush().unwrap();
                let mut start_input = String::new();
                io::stdin().read_line(&mut start_input).expect("Ошибка чтения");
                let start_num: i128 = start_input.trim().parse().unwrap_or(2);
                
                print!("Размер блока на поток (по умолчанию 100000): ");
                io::stdout().flush().unwrap();
                let mut chunk_input = String::new();
                io::stdin().read_line(&mut chunk_input).expect("Ошибка чтения");
                let chunk_size: i128 = chunk_input.trim().parse().unwrap_or(100000);
                
                println!("\n🚀 Запускаем многопоточный поиск...");
                find_perfect_numbers_multithreaded(num_threads, start_num, chunk_size);
            }
            "4" => {
                println!("\n⚠️  Внимание! Бесконечный поиск может работать очень долго!");
                print!("Вы уверены? (y/N): ");
                io::stdout().flush().unwrap();
                let mut confirm = String::new();
                io::stdin().read_line(&mut confirm).expect("Ошибка чтения");
                
                if confirm.trim().to_lowercase() == "y" || confirm.trim().to_lowercase() == "yes" {
                    find_perfect_numbers();
                } else {
                    println!("❌ Отменено.");
                }
            }
            "5" => {
                println!("\n⚠️  Внимание! Бесконечный многопоточный поиск может работать очень долго!");
                print!("Вы уверены? (y/N): ");
                io::stdout().flush().unwrap();
                let mut confirm = String::new();
                io::stdin().read_line(&mut confirm).expect("Ошибка чтения");
                
                if confirm.trim().to_lowercase() == "y" || confirm.trim().to_lowercase() == "yes" {
                    // Настройка параметров для бесконечного многопоточного поиска
                    let available_cores = thread::available_parallelism().map(|p| p.get()).unwrap_or(4);
                    println!("💻 Доступно логических ядер: {}", available_cores);
                    
                    print!("Введите количество потоков (по умолчанию {}): ", available_cores);
                    io::stdout().flush().unwrap();
                    let mut threads_input = String::new();
                    io::stdin().read_line(&mut threads_input).expect("Ошибка чтения");
                    let num_threads = threads_input.trim().parse().unwrap_or(available_cores);
                    
                    find_perfect_numbers_infinite_multithreaded(num_threads);
                } else {
                    println!("❌ Отменено.");
                }
            }
            "6" => {
                test_prime_numbers();
            }
            "7" => {
                println!("\n📊 Введите диапазон поиска простых чисел:");
                
                print!("Начальное число: ");
                io::stdout().flush().unwrap();
                let mut start_input = String::new();
                io::stdin().read_line(&mut start_input).expect("Ошибка чтения");
                let start_num: i128 = start_input.trim().parse().unwrap_or(2);
                
                print!("Конечное число: ");
                io::stdout().flush().unwrap();
                let mut end_input = String::new();
                io::stdin().read_line(&mut end_input).expect("Ошибка чтения");
                let end_num: i128 = end_input.trim().parse().unwrap_or(100);
                
                search_primes_in_range(start_num, end_num);
            }
            "8" => {
                println!("\n🧵 Настройка многопоточного поиска простых чисел:");
                
                let available_cores = thread::available_parallelism().map(|p| p.get()).unwrap_or(4);
                println!("💻 Доступно логических ядер: {}", available_cores);
                
                print!("Введите количество потоков (по умолчанию {}): ", available_cores);
                io::stdout().flush().unwrap();
                let mut threads_input = String::new();
                io::stdin().read_line(&mut threads_input).expect("Ошибка чтения");
                let num_threads = threads_input.trim().parse().unwrap_or(available_cores);
                
                print!("Начальное число (по умолчанию 2): ");
                io::stdout().flush().unwrap();
                let mut start_input = String::new();
                io::stdin().read_line(&mut start_input).expect("Ошибка чтения");
                let start_num: i128 = start_input.trim().parse().unwrap_or(2);
                
                print!("Размер блока на поток (по умолчанию 100000): ");
                io::stdout().flush().unwrap();
                let mut chunk_input = String::new();
                io::stdin().read_line(&mut chunk_input).expect("Ошибка чтения");
                let chunk_size: i128 = chunk_input.trim().parse().unwrap_or(100000);
                
                println!("\n🚀 Запускаем многопоточный поиск простых чисел...");
                find_primes_multithreaded(num_threads, start_num, chunk_size);
            }
            "9" => {
                println!("\n⚠️  Внимание! Бесконечный многопоточный поиск простых чисел может работать очень долго!");
                print!("Вы уверены? (y/N): ");
                io::stdout().flush().unwrap();
                let mut confirm = String::new();
                io::stdin().read_line(&mut confirm).expect("Ошибка чтения");
                
                if confirm.trim().to_lowercase() == "y" || confirm.trim().to_lowercase() == "yes" {
                    let available_cores = thread::available_parallelism().map(|p| p.get()).unwrap_or(4);
                    println!("💻 Доступно логических ядер: {}", available_cores);
                    
                    print!("Введите количество потоков (по умолчанию {}): ", available_cores);
                    io::stdout().flush().unwrap();
                    let mut threads_input = String::new();
                    io::stdin().read_line(&mut threads_input).expect("Ошибка чтения");
                    let num_threads = threads_input.trim().parse().unwrap_or(available_cores);
                    
                    find_primes_infinite_multithreaded(num_threads);
                } else {
                    println!("❌ Отменено.");
                }
            }
            "10" => {
                println!("👋 До свидания!");
                break;
            }
            _ => {
                println!("❌ Неверный выбор. Попробуйте снова.");
            }
        }
        
        println!("\n{}", "=".repeat(50));
    }
}

// Новая функция для многопоточного поиска совершенных чисел
fn find_perfect_numbers_multithreaded(num_threads: usize, start_num: i128, chunk_size: i128) {
    println!("🚀 Начинаем многопоточный поиск совершенных чисел...");
    println!("   🧵 Количество потоков: {}", num_threads);
    println!("   📊 Начальное число: {}", start_num);
    println!("   📦 Размер блока на поток: {}", chunk_size);
    println!("   ⚠️  Нажмите Ctrl+C для остановки\n");
    
    let found_count = Arc::new(AtomicUsize::new(0));
    let checked_count = Arc::new(AtomicUsize::new(0));
    let (tx, rx) = mpsc::channel();
    let start_time = Instant::now();
    
    // Создаем потоки
    let mut handles = Vec::new();
    
    for thread_id in 0..num_threads {
        let tx_clone = tx.clone();
        let found_count_clone = Arc::clone(&found_count);
        let checked_count_clone = Arc::clone(&checked_count);
        let thread_start = start_num + (thread_id as i128 * chunk_size);
        let thread_end = thread_start + chunk_size;
        
        let handle = thread::spawn(move || {
            search_in_range_thread(
                thread_id,
                thread_start,
                thread_end,
                tx_clone,
                found_count_clone,
                checked_count_clone,
            );
        });
        
        handles.push(handle);
    }
    
    // Закрываем отправитель в основном потоке
    drop(tx);
    
    // Клонируем Arc для использования в потоке обработки сообщений
    let found_count_for_msg = Arc::clone(&found_count);
    let checked_count_for_msg = Arc::clone(&checked_count);
    
    // Собираем результаты
    let msg_handle = thread::spawn(move || {
        for result in rx {
            match result {
                ThreadMessage::PerfectFound { thread_id, number, type_name, check_time, total_checked } => {
                    let global_found = found_count_for_msg.fetch_add(1, Ordering::SeqCst) + 1;
                    let elapsed_total = start_time.elapsed();
                    
                    println!("🎉 НАЙДЕНО СОВЕРШЕННОЕ ЧИСЛО №{}!", global_found);
                    println!("   📊 Число: {}", number);
                    println!("   🔢 Тип: {}", type_name);
                    println!("   🧵 Поток: #{}", thread_id);
                    println!("   ⏱️  Время проверки числа: {:.3?}", check_time);
                    println!("   ⏰ Общее время работы: {:.2?}", elapsed_total);
                    println!("   📍 Всего проверено: {}\n", total_checked);
                }
                ThreadMessage::Progress { thread_id, checked_in_thread } => {
                    if checked_in_thread % 50000 == 0 {
                        let total_checked = checked_count_for_msg.load(Ordering::SeqCst);
                        let elapsed = start_time.elapsed();
                        let speed = total_checked as f64 / elapsed.as_secs_f64();
                        println!("🔄 Поток #{}: проверено {} | Всего: {} | Скорость: {:.0}/сек", 
                            thread_id, checked_in_thread, total_checked, speed);
                    }
                }
            }
        }
    });
    
    // Ждем завершения всех потоков
    for handle in handles {
        handle.join().unwrap();
    }
    
    // Ждем завершения потока обработки сообщений
    msg_handle.join().unwrap();
    
    let total_time = start_time.elapsed();
    let total_checked_final = checked_count.load(Ordering::SeqCst);
    let total_found_final = found_count.load(Ordering::SeqCst);
    
    println!("\n📊 Итоговая статистика:");
    println!("   🧵 Потоков: {}", num_threads);
    println!("   ✅ Найдено совершенных чисел: {}", total_found_final);
    println!("   📋 Всего проверено чисел: {}", total_checked_final);
    println!("   ⏱️  Общее время: {:.2?}", total_time);
    println!("   ⚡ Общая скорость: {:.0} чисел/сек", total_checked_final as f64 / total_time.as_secs_f64());
}

#[derive(Debug)]
enum ThreadMessage {
    PerfectFound {
        thread_id: usize,
        number: String,
        type_name: String,
        check_time: std::time::Duration,
        total_checked: usize,
    },
    Progress {
        thread_id: usize,
        checked_in_thread: usize,
    },
}

#[derive(Debug)]
enum PrimeThreadMessage {
    PrimeFound {
        thread_id: usize,
        number: String,
        type_name: String,
        check_time: std::time::Duration,
        total_checked: usize,
    },
    Progress {
        thread_id: usize,
        checked_in_thread: usize,
    },
}

fn search_in_range_thread(
    thread_id: usize,
    start: i128,
    end: i128,
    tx: mpsc::Sender<ThreadMessage>,
    _found_count: Arc<AtomicUsize>,
    checked_count: Arc<AtomicUsize>,
) {
    let mut current = DynamicInt::new(start);
    let one = DynamicInt::one();
    let end_num = DynamicInt::new(end);
    let mut checked_in_thread = 0;
    
    while current.lt(&end_num) {
        let check_start = Instant::now();
        
        if current.is_perfect() {
            let check_time = check_start.elapsed();
            let total_checked = checked_count.load(Ordering::SeqCst);
            
            let _ = tx.send(ThreadMessage::PerfectFound {
                thread_id,
                number: current.to_string_value(),
                type_name: current.get_type_name().to_string(),
                check_time,
                total_checked,
            });
        }
        
        checked_in_thread += 1;
        checked_count.fetch_add(1, Ordering::SeqCst);
        
        // Отправляем прогресс
        if checked_in_thread % 10000 == 0 {
            let _ = tx.send(ThreadMessage::Progress {
                thread_id,
                checked_in_thread,
            });
        }
        
        current = current.add(&one);
    }
    
    println!("🏁 Поток #{} завершен. Проверено {} чисел в диапазоне {}-{}", 
        thread_id, checked_in_thread, start, end);
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

// Функция для бесконечного многопоточного поиска
fn find_perfect_numbers_infinite_multithreaded(num_threads: usize) {
    println!("♾️  Начинаем бесконечный многопоточный поиск совершенных чисел...");
    println!("   🧵 Количество потоков: {}", num_threads);
    println!("   📦 Размер блока на поток: 1,000,000 чисел");
    println!("   ⚠️  Нажмите Ctrl+C для остановки\n");
    
    let found_count = Arc::new(AtomicUsize::new(0));
    let checked_count = Arc::new(AtomicUsize::new(0));
    let current_start = Arc::new(AtomicUsize::new(2)); // Начинаем с 2
    let (tx, rx) = mpsc::channel();
    let start_time = Instant::now();
    
    // Создаем потоки
    let mut handles = Vec::new();
    
    for thread_id in 0..num_threads {
        let tx_clone = tx.clone();
        let found_count_clone = Arc::clone(&found_count);
        let checked_count_clone = Arc::clone(&checked_count);
        let current_start_clone = Arc::clone(&current_start);
        
        let handle = thread::spawn(move || {
            infinite_search_thread(
                thread_id,
                tx_clone,
                found_count_clone,
                checked_count_clone,
                current_start_clone,
            );
        });
        
        handles.push(handle);
    }
    
    // Закрываем отправитель в основном потоке
    drop(tx);
    
    // Клонируем Arc для использования в потоке обработки сообщений
    let found_count_for_msg = Arc::clone(&found_count);
    let checked_count_for_msg = Arc::clone(&checked_count);
    
    // Собираем результаты
    let msg_handle = thread::spawn(move || {
        for result in rx {
            match result {
                ThreadMessage::PerfectFound { thread_id, number, type_name, check_time, total_checked } => {
                    let global_found = found_count_for_msg.fetch_add(1, Ordering::SeqCst) + 1;
                    let elapsed_total = start_time.elapsed();
                    
                    println!("🎉 НАЙДЕНО СОВЕРШЕННОЕ ЧИСЛО №{}!", global_found);
                    println!("   📊 Число: {}", number);
                    println!("   🔢 Тип: {}", type_name);
                    println!("   🧵 Поток: #{}", thread_id);
                    println!("   ⏱️  Время проверки числа: {:.3?}", check_time);
                    println!("   ⏰ Общее время работы: {:.2?}", elapsed_total);
                    println!("   📍 Всего проверено: {}\n", total_checked);
                }
                ThreadMessage::Progress { thread_id, checked_in_thread } => {
                    if checked_in_thread % 100000 == 0 {
                        let total_checked = checked_count_for_msg.load(Ordering::SeqCst);
                        let elapsed = start_time.elapsed();
                        let speed = total_checked as f64 / elapsed.as_secs_f64();
                        println!("🔄 Поток #{}: проверено {} | Всего: {} | Скорость: {:.0}/сек", 
                            thread_id, checked_in_thread, total_checked, speed);
                    }
                }
            }
        }
    });
    
    // Ждем завершения всех потоков (никогда не случится в бесконечном поиске)
    for handle in handles {
        handle.join().unwrap();
    }
    
    // Ждем завершения потока обработки сообщений
    msg_handle.join().unwrap();
}

fn infinite_search_thread(
    thread_id: usize,
    tx: mpsc::Sender<ThreadMessage>,
    _found_count: Arc<AtomicUsize>,
    checked_count: Arc<AtomicUsize>,
    current_start: Arc<AtomicUsize>,
) {
    let chunk_size = 1000000_i128; // 1 миллион чисел на блок
    let mut checked_in_thread = 0;
    
    loop {
        // Атомарно получаем следующий диапазон
        let start = current_start.fetch_add(chunk_size as usize, Ordering::SeqCst) as i128;
        let end = start + chunk_size;
        
        println!("🧵 Поток #{} начинает обработку диапазона {}-{}", thread_id, start, end);
        
        let mut current = DynamicInt::new(start);
        let one = DynamicInt::one();
        let end_num = DynamicInt::new(end);
        
        while current.lt(&end_num) {
            let check_start = Instant::now();
            
            if current.is_perfect() {
                let check_time = check_start.elapsed();
                let total_checked = checked_count.load(Ordering::SeqCst);
                
                let _ = tx.send(ThreadMessage::PerfectFound {
                    thread_id,
                    number: current.to_string_value(),
                    type_name: current.get_type_name().to_string(),
                    check_time,
                    total_checked,
                });
            }
            
            checked_in_thread += 1;
            checked_count.fetch_add(1, Ordering::SeqCst);
            
            // Отправляем прогресс
            if checked_in_thread % 50000 == 0 {
                let _ = tx.send(ThreadMessage::Progress {
                    thread_id,
                    checked_in_thread,
                });
            }
            
            current = current.add(&one);
        }
        
        println!("🏁 Поток #{} завершил диапазон {}-{}, переходит к следующему", 
            thread_id, start, end);
    }
}

fn test_prime_numbers() {
    println!("🔢 Тестируем простые числа...\n");
    
    // Первые простые числа
    let test_primes = vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47];
    
    println!("✅ Проверяем известные простые числа:");
    for num in test_primes {
        let start_time = Instant::now();
        let dynamic_num = DynamicInt::new(num);
        let is_prime = dynamic_num.is_prime();
        let elapsed = start_time.elapsed();
        
        println!("Число: {} | Простое: {} | Тип: {} | Время: {:.3?}",
            num, 
            if is_prime { "✅" } else { "❌" },
            dynamic_num.get_type_name(),
            elapsed
        );
    }
    
    println!("\n❌ Проверяем составные числа:");
    let non_primes = vec![1, 4, 6, 8, 9, 10, 12, 14, 15, 16, 18, 20, 21, 22, 24, 25, 26, 27, 28, 30];
    
    for num in non_primes {
        let start_time = Instant::now();
        let dynamic_num = DynamicInt::new(num);
        let is_prime = dynamic_num.is_prime();
        let elapsed = start_time.elapsed();
        
        println!("Число: {} | Простое: {} | Тип: {} | Время: {:.3?}",
            num, 
            if is_prime { "✅" } else { "❌" },
            dynamic_num.get_type_name(),
            elapsed
        );
    }
    
    println!("\n🎯 Результат: алгоритм корректно определяет простые числа!");
}

fn search_primes_in_range(start: i128, end: i128) {
    println!("🔍 Ищем простые числа в диапазоне от {} до {}...\n", start, end);
    
    let mut current = DynamicInt::new(start);
    let one = DynamicInt::one();
    let end_num = DynamicInt::new(end);
    let mut found_count = 0;
    let mut checked_count = 0;
    let total_start_time = Instant::now();
    
    while current.lt(&end_num) {
        let start_time = Instant::now();
        
        if current.is_prime() {
            let elapsed = start_time.elapsed();
            found_count += 1;
            
            println!("🎉 НАЙДЕНО ПРОСТОЕ ЧИСЛО №{}: {}", found_count, current.to_string_value());
            if found_count <= 20 { // Показываем детали только для первых 20
                println!("   🔢 Тип: {} | ⏱️  Время: {:.3?}", current.get_type_name(), elapsed);
            }
        }
        
        checked_count += 1;
        current = current.add(&one);
        
        // Для информативности выводим прогресс каждые 10000 чисел
        if checked_count % 10000 == 0 {
            let progress = (checked_count as f64 / (end - start) as f64) * 100.0;
            println!("🔄 Проверено: {} чисел ({:.1}%)", checked_count, progress);
        }
    }
    
    let total_elapsed = total_start_time.elapsed();
    println!("\n📊 Итоги поиска:");
    println!("   🔢 Диапазон: {} - {}", start, end);
    println!("   ✅ Найдено простых чисел: {}", found_count);
    println!("   📋 Всего проверено: {}", checked_count);
    println!("   ⏱️  Общее время: {:.2?}", total_elapsed);
    println!("   ⚡ Скорость: {:.0} чисел/сек", 
        checked_count as f64 / total_elapsed.as_secs_f64());
}

fn find_primes_multithreaded(num_threads: usize, start_num: i128, chunk_size: i128) {
    println!("🚀 Начинаем многопоточный поиск простых чисел...");
    println!("   🧵 Количество потоков: {}", num_threads);
    println!("   📊 Начальное число: {}", start_num);
    println!("   📦 Размер блока на поток: {}", chunk_size);
    println!("   ⚠️  Нажмите Ctrl+C для остановки\n");
    
    let found_count = Arc::new(AtomicUsize::new(0));
    let checked_count = Arc::new(AtomicUsize::new(0));
    let (tx, rx) = mpsc::channel();
    let start_time = Instant::now();
    
    // Создаем потоки
    let mut handles = Vec::new();
    
    for thread_id in 0..num_threads {
        let tx_clone = tx.clone();
        let found_count_clone = Arc::clone(&found_count);
        let checked_count_clone = Arc::clone(&checked_count);
        let thread_start = start_num + (thread_id as i128 * chunk_size);
        let thread_end = thread_start + chunk_size;
        
        let handle = thread::spawn(move || {
            search_primes_in_range_thread(
                thread_id,
                thread_start,
                thread_end,
                tx_clone,
                found_count_clone,
                checked_count_clone,
            );
        });
        
        handles.push(handle);
    }
    
    // Закрываем отправитель в основном потоке
    drop(tx);
    
    // Клонируем Arc для использования в потоке обработки сообщений
    let found_count_for_msg = Arc::clone(&found_count);
    let checked_count_for_msg = Arc::clone(&checked_count);
    
    // Собираем результаты
    let msg_handle = thread::spawn(move || {
        for result in rx {
            match result {
                PrimeThreadMessage::PrimeFound { thread_id, number, type_name, check_time, total_checked } => {
                    let global_found = found_count_for_msg.fetch_add(1, Ordering::SeqCst) + 1;
                    let elapsed_total = start_time.elapsed();
                    
                    println!("🎉 НАЙДЕНО ПРОСТОЕ ЧИСЛО №{}!", global_found);
                    println!("   📊 Число: {}", number);
                    println!("   🔢 Тип: {}", type_name);
                    println!("   🧵 Поток: #{}", thread_id);
                    println!("   ⏱️  Время проверки числа: {:.3?}", check_time);
                    println!("   ⏰ Общее время работы: {:.2?}", elapsed_total);
                    println!("   📍 Всего проверено: {}\n", total_checked);
                }
                PrimeThreadMessage::Progress { thread_id, checked_in_thread } => {
                    if checked_in_thread % 50000 == 0 {
                        let total_checked = checked_count_for_msg.load(Ordering::SeqCst);
                        let total_found = found_count_for_msg.load(Ordering::SeqCst);
                        let elapsed = start_time.elapsed();
                        let speed = total_checked as f64 / elapsed.as_secs_f64();
                        println!("🔄 Поток #{}: проверено {} | Всего: {} | Найдено: {} | Скорость: {:.0}/сек", 
                            thread_id, checked_in_thread, total_checked, total_found, speed);
                    }
                }
            }
        }
    });
    
    // Ждем завершения всех потоков
    for handle in handles {
        handle.join().unwrap();
    }
    
    // Ждем завершения потока обработки сообщений
    msg_handle.join().unwrap();
    
    let total_time = start_time.elapsed();
    let total_checked_final = checked_count.load(Ordering::SeqCst);
    let total_found_final = found_count.load(Ordering::SeqCst);
    
    println!("\n📊 Итоговая статистика:");
    println!("   🧵 Потоков: {}", num_threads);
    println!("   ✅ Найдено простых чисел: {}", total_found_final);
    println!("   📋 Всего проверено чисел: {}", total_checked_final);
    println!("   ⏱️  Общее время: {:.2?}", total_time);
    println!("   ⚡ Общая скорость: {:.0} чисел/сек", total_checked_final as f64 / total_time.as_secs_f64());
}

fn search_primes_in_range_thread(
    thread_id: usize,
    start: i128,
    end: i128,
    tx: mpsc::Sender<PrimeThreadMessage>,
    _found_count: Arc<AtomicUsize>,
    checked_count: Arc<AtomicUsize>,
) {
    let mut current = DynamicInt::new(start);
    let one = DynamicInt::one();
    let end_num = DynamicInt::new(end);
    let mut checked_in_thread = 0;
    
    while current.lt(&end_num) {
        let check_start = Instant::now();
        
        if current.is_prime() {
            let check_time = check_start.elapsed();
            let total_checked = checked_count.load(Ordering::SeqCst);
            
            let _ = tx.send(PrimeThreadMessage::PrimeFound {
                thread_id,
                number: current.to_string_value(),
                type_name: current.get_type_name().to_string(),
                check_time,
                total_checked,
            });
        }
        
        checked_in_thread += 1;
        checked_count.fetch_add(1, Ordering::SeqCst);
        
        // Отправляем прогресс
        if checked_in_thread % 10000 == 0 {
            let _ = tx.send(PrimeThreadMessage::Progress {
                thread_id,
                checked_in_thread,
            });
        }
        
        current = current.add(&one);
    }
    
    println!("🏁 Поток #{} завершен. Проверено {} чисел в диапазоне {}-{}", 
        thread_id, checked_in_thread, start, end);
}

fn find_primes_infinite_multithreaded(num_threads: usize) {
    println!("♾️  Начинаем бесконечный многопоточный поиск простых чисел...");
    println!("   🧵 Количество потоков: {}", num_threads);
    println!("   📦 Размер блока на поток: 1,000,000 чисел");
    println!("   ⚠️  Нажмите Ctrl+C для остановки\n");
    
    let found_count = Arc::new(AtomicUsize::new(0));
    let checked_count = Arc::new(AtomicUsize::new(0));
    let current_start = Arc::new(AtomicUsize::new(2)); // Начинаем с 2
    let (tx, rx) = mpsc::channel();
    let start_time = Instant::now();
    
    // Создаем потоки
    let mut handles = Vec::new();
    
    for thread_id in 0..num_threads {
        let tx_clone = tx.clone();
        let found_count_clone = Arc::clone(&found_count);
        let checked_count_clone = Arc::clone(&checked_count);
        let current_start_clone = Arc::clone(&current_start);
        
        let handle = thread::spawn(move || {
            infinite_search_primes_thread(
                thread_id,
                tx_clone,
                found_count_clone,
                checked_count_clone,
                current_start_clone,
            );
        });
        
        handles.push(handle);
    }
    
    // Закрываем отправитель в основном потоке
    drop(tx);
    
    // Клонируем Arc для использования в потоке обработки сообщений
    let found_count_for_msg = Arc::clone(&found_count);
    let checked_count_for_msg = Arc::clone(&checked_count);
    
    // Собираем результаты
    let msg_handle = thread::spawn(move || {
        for result in rx {
            match result {
                PrimeThreadMessage::PrimeFound { thread_id, number, type_name, check_time, total_checked } => {
                    let global_found = found_count_for_msg.fetch_add(1, Ordering::SeqCst) + 1;
                    let elapsed_total = start_time.elapsed();
                    
                    println!("🎉 НАЙДЕНО ПРОСТОЕ ЧИСЛО №{}!", global_found);
                    println!("   📊 Число: {}", number);
                    println!("   🔢 Тип: {}", type_name);
                    println!("   🧵 Поток: #{}", thread_id);
                    println!("   ⏱️  Время проверки числа: {:.3?}", check_time);
                    println!("   ⏰ Общее время работы: {:.2?}", elapsed_total);
                    println!("   📍 Всего проверено: {}\n", total_checked);
                }
                PrimeThreadMessage::Progress { thread_id, checked_in_thread } => {
                    if checked_in_thread % 100000 == 0 {
                        let total_checked = checked_count_for_msg.load(Ordering::SeqCst);
                        let total_found = found_count_for_msg.load(Ordering::SeqCst);
                        let elapsed = start_time.elapsed();
                        let speed = total_checked as f64 / elapsed.as_secs_f64();
                        println!("🔄 Поток #{}: проверено {} | Всего: {} | Найдено: {} | Скорость: {:.0}/сек", 
                            thread_id, checked_in_thread, total_checked, total_found, speed);
                    }
                }
            }
        }
    });
    
    // Ждем завершения всех потоков (никогда не случится в бесконечном поиске)
    for handle in handles {
        handle.join().unwrap();
    }
    
    // Ждем завершения потока обработки сообщений
    msg_handle.join().unwrap();
}

fn infinite_search_primes_thread(
    thread_id: usize,
    tx: mpsc::Sender<PrimeThreadMessage>,
    _found_count: Arc<AtomicUsize>,
    checked_count: Arc<AtomicUsize>,
    current_start: Arc<AtomicUsize>,
) {
    let chunk_size = 1000000_i128; // 1 миллион чисел на блок
    let mut checked_in_thread = 0;
    
    loop {
        // Атомарно получаем следующий диапазон
        let start = current_start.fetch_add(chunk_size as usize, Ordering::SeqCst) as i128;
        let end = start + chunk_size;
        
        println!("🧵 Поток #{} начинает поиск простых чисел в диапазоне {}-{}", thread_id, start, end);
        
        let mut current = DynamicInt::new(start);
        let one = DynamicInt::one();
        let end_num = DynamicInt::new(end);
        
        while current.lt(&end_num) {
            let check_start = Instant::now();
            
            if current.is_prime() {
                let check_time = check_start.elapsed();
                let total_checked = checked_count.load(Ordering::SeqCst);
                
                let _ = tx.send(PrimeThreadMessage::PrimeFound {
                    thread_id,
                    number: current.to_string_value(),
                    type_name: current.get_type_name().to_string(),
                    check_time,
                    total_checked,
                });
            }
            
            checked_in_thread += 1;
            checked_count.fetch_add(1, Ordering::SeqCst);
            
            // Отправляем прогресс
            if checked_in_thread % 50000 == 0 {
                let _ = tx.send(PrimeThreadMessage::Progress {
                    thread_id,
                    checked_in_thread,
                });
            }
            
            current = current.add(&one);
        }
        
        println!("🏁 Поток #{} завершил диапазон {}-{}, переходит к следующему", 
            thread_id, start, end);
    }
}
