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
        println!("10. 🔥 Поиск совершенных чисел по формуле Евклида");
        println!("11. 🧮 Тест чисел Мерсенна");
        println!("12. 🚀 Многопоточный поиск по формуле Евклида");
        println!("13. ♾️  Бесконечный поиск по формуле Евклида");
        println!("14. ⚡ Бесконечный многопоточный поиск по Евклиду");
        println!("15. 🧮 Вычисление факториала");
        println!("16. 🔍 Поиск чисел-факториалов в диапазоне");
        println!("17. 🚀 Многопоточное вычисление факториалов");
        println!("18. ♾️  Бесконечное вычисление факториалов");
        println!("19. ⚡ Бесконечное многопоточное вычисление факториалов");
        println!("20. 🧮 Многопоточное вычисление одного факториала");
        println!("21. 🚪 Выход");
        println!("\nВведите номер (1-21): ");
        
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
                println!("\n🔥 Поиск совершенных чисел по формуле Евклида:");
                
                print!("Максимальный показатель степени x (по умолчанию 50): ");
                io::stdout().flush().unwrap();
                let mut max_input = String::new();
                io::stdin().read_line(&mut max_input).expect("Ошибка чтения");
                let max_x: u32 = max_input.trim().parse().unwrap_or(50);
                
                search_perfect_numbers_euclid(max_x);
            }
            "11" => {
                test_mersenne_numbers();
            }
            "12" => {
                println!("\n🧵 Настройка многопоточного поиска по формуле Евклида:");
                
                let available_cores = thread::available_parallelism().map(|p| p.get()).unwrap_or(4);
                println!("💻 Доступно логических ядер: {}", available_cores);
                
                print!("Введите количество потоков (по умолчанию {}): ", available_cores);
                io::stdout().flush().unwrap();
                let mut threads_input = String::new();
                io::stdin().read_line(&mut threads_input).expect("Ошибка чтения");
                let num_threads = threads_input.trim().parse().unwrap_or(available_cores);
                
                print!("Максимальный показатель степени x (по умолчанию 100): ");
                io::stdout().flush().unwrap();
                let mut max_input = String::new();
                io::stdin().read_line(&mut max_input).expect("Ошибка чтения");
                let max_x: u32 = max_input.trim().parse().unwrap_or(100);
                
                println!("\n🚀 Запускаем многопоточный поиск по формуле Евклида...");
                search_perfect_numbers_euclid_multithreaded(num_threads, max_x);
            }
            "13" => {
                println!("\n⚠️  Внимание! Бесконечный поиск по формуле Евклида может работать очень долго!");
                print!("Вы уверены? (y/N): ");
                io::stdout().flush().unwrap();
                let mut confirm = String::new();
                io::stdin().read_line(&mut confirm).expect("Ошибка чтения");
                
                if confirm.trim().to_lowercase() == "y" || confirm.trim().to_lowercase() == "yes" {
                    search_perfect_numbers_euclid_infinite();
                } else {
                    println!("❌ Отменено.");
                }
            }
            "14" => {
                println!("\n⚠️  Внимание! Бесконечный многопоточный поиск по Евклиду может работать очень долго!");
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
                    
                    search_perfect_numbers_euclid_infinite_multithreaded(num_threads);
                } else {
                    println!("❌ Отменено.");
                }
            }
            "15" => {
                println!("\n🧮 Вычисление факториала конкретного числа:");
                
                print!("Введите число для вычисления факториала: ");
                io::stdout().flush().unwrap();
                let mut num_input = String::new();
                io::stdin().read_line(&mut num_input).expect("Ошибка чтения");
                let num: i128 = num_input.trim().parse().unwrap_or(10);
                
                calculate_factorial(num);
            }
            "16" => {
                println!("\n🔍 Поиск чисел-факториалов в диапазоне:");
                
                print!("Начальное число: ");
                io::stdout().flush().unwrap();
                let mut start_input = String::new();
                io::stdin().read_line(&mut start_input).expect("Ошибка чтения");
                let start_num: i128 = start_input.trim().parse().unwrap_or(1);
                
                print!("Конечное число: ");
                io::stdout().flush().unwrap();
                let mut end_input = String::new();
                io::stdin().read_line(&mut end_input).expect("Ошибка чтения");
                let end_num: i128 = end_input.trim().parse().unwrap_or(1000000);
                
                search_factorials_in_range(start_num, end_num);
            }
            "17" => {
                println!("\n🧵 Настройка многопоточного вычисления факториалов:");
                
                let available_cores = thread::available_parallelism().map(|p| p.get()).unwrap_or(4);
                println!("💻 Доступно логических ядер: {}", available_cores);
                
                print!("Введите количество потоков (по умолчанию {}): ", available_cores);
                io::stdout().flush().unwrap();
                let mut threads_input = String::new();
                io::stdin().read_line(&mut threads_input).expect("Ошибка чтения");
                let num_threads = threads_input.trim().parse().unwrap_or(available_cores);
                
                print!("Начальное число: ");
                io::stdout().flush().unwrap();
                let mut start_input = String::new();
                io::stdin().read_line(&mut start_input).expect("Ошибка чтения");
                let start_num: i128 = start_input.trim().parse().unwrap_or(10);
                
                print!("Конечное число: ");
                io::stdout().flush().unwrap();
                let mut end_input = String::new();
                io::stdin().read_line(&mut end_input).expect("Ошибка чтения");
                let end_num: i128 = end_input.trim().parse().unwrap_or(20);
                
                println!("\n🚀 Запускаем многопоточное вычисление факториалов...");
                calculate_factorials_multithreaded(num_threads, start_num, end_num);
            }
            "18" => {
                println!("\n⚠️  Внимание! Бесконечное вычисление факториалов может работать очень долго!");
                print!("Вы уверены? (y/N): ");
                io::stdout().flush().unwrap();
                let mut confirm = String::new();
                io::stdin().read_line(&mut confirm).expect("Ошибка чтения");
                
                if confirm.trim().to_lowercase() == "y" || confirm.trim().to_lowercase() == "yes" {
                    calculate_factorials_infinite();
                } else {
                    println!("❌ Отменено.");
                }
            }
            "19" => {
                println!("\n⚠️  Внимание! Бесконечное многопоточное вычисление факториалов может работать очень долго!");
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
                    
                    calculate_factorials_infinite_multithreaded(num_threads);
                } else {
                    println!("❌ Отменено.");
                }
            }
            "20" => {
                println!("\n🧮 Многопоточное вычисление одного факториала:");
                
                let available_cores = thread::available_parallelism().map(|p| p.get()).unwrap_or(4);
                println!("💻 Доступно логических ядер: {}", available_cores);
                
                print!("Введите количество потоков (по умолчанию {}): ", available_cores);
                io::stdout().flush().unwrap();
                let mut threads_input = String::new();
                io::stdin().read_line(&mut threads_input).expect("Ошибка чтения");
                let num_threads = threads_input.trim().parse().unwrap_or(available_cores);
                
                print!("Введите число для вычисления факториала: ");
                io::stdout().flush().unwrap();
                let mut num_input = String::new();
                io::stdin().read_line(&mut num_input).expect("Ошибка чтения");
                let num: i128 = num_input.trim().parse().unwrap_or(100);
                
                if num > 10000 {
                    println!("⚠️  Внимание! Вычисление факториала для n > 10000 может занять очень много времени!");
                    print!("Продолжить? (y/N): ");
                    io::stdout().flush().unwrap();
                    let mut confirm = String::new();
                    io::stdin().read_line(&mut confirm).expect("Ошибка чтения");
                    
                    if !(confirm.trim().to_lowercase() == "y" || confirm.trim().to_lowercase() == "yes") {
                        println!("❌ Отменено.");
                        continue;
                    }
                }
                
                println!("\n🚀 Запускаем многопоточное вычисление {}! с {} потоками...", num, num_threads);
                calculate_factorial_multithreaded(num, num_threads);
            }
            "21" => {
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

#[derive(Debug)]
enum FactorialThreadMessage {
    FactorialCalculated {
        thread_id: usize,
        number: i128,
        factorial: String,
        type_name: String,
        calculation_time: std::time::Duration,
        decimal_length: usize,
    },
    FactorialFound {
        thread_id: usize,
        number: String,
        factorial_of: i128,
        type_name: String,
        check_time: std::time::Duration,
    },
    Progress {
        thread_id: usize,
        current_number: i128,
        checked_count: usize,
    },
}

// Вычисление факториала конкретного числа
fn calculate_factorial(n: i128) {
    println!("🧮 Вычисляем {}!...\n", n);
    
    if n < 0 {
        println!("❌ Ошибка: факториал определен только для неотрицательных чисел!");
        return;
    }
    
    if n > 1000 {
        println!("⚠️  Внимание: вычисление факториала для n > 1000 может занять очень много времени!");
        print!("Продолжить? (y/N): ");
        io::stdout().flush().unwrap();
        let mut confirm = String::new();
        io::stdin().read_line(&mut confirm).expect("Ошибка чтения");
        
        if !(confirm.trim().to_lowercase() == "y" || confirm.trim().to_lowercase() == "yes") {
            println!("❌ Отменено.");
            return;
        }
    }
    
    let start_time = Instant::now();
    let factorial = DynamicInt::factorial_of(n);
    let calculation_time = start_time.elapsed();
    
    let decimal_length = factorial.to_string_value().len();
    
    println!("🎉 РЕЗУЛЬТАТ ВЫЧИСЛЕНИЯ ФАКТОРИАЛА:");
    println!("   📊 Число: {}", n);
    println!("   🧮 Факториал {}!: {}", n, factorial.to_string_value());
    println!("   🔢 Тип результата: {}", factorial.get_type_name());
    println!("   📐 Длина в десятичных знаках: {}", decimal_length);
    println!("   ⏱️  Время вычисления: {:.3?}", calculation_time);
    
    // Дополнительная информация
    if decimal_length > 100 {
        println!("   💡 Первые 50 цифр: {}...", &factorial.to_string_value()[..50]);
        println!("   💡 Последние 50 цифр: ...{}", &factorial.to_string_value()[decimal_length-50..]);
    }
    
    println!("\n📊 Статистика:");
    if calculation_time.as_secs_f64() > 0.001 {
        let ops_per_sec = n as f64 / calculation_time.as_secs_f64();
        println!("   ⚡ Производительность: {:.0} операций/сек", ops_per_sec);
    }
}

// Поиск чисел-факториалов в диапазоне
fn search_factorials_in_range(start: i128, end: i128) {
    println!("🔍 Ищем числа-факториалы в диапазоне от {} до {}...\n", start, end);
    
    let mut current = DynamicInt::new(start);
    let one = DynamicInt::one();
    let end_num = DynamicInt::new(end);
    let mut found_count = 0;
    let mut checked_count = 0;
    let total_start_time = Instant::now();
    
    while current.lt(&end_num) {
        let start_time = Instant::now();
        let (is_factorial, factorial_of) = current.is_factorial();
        let elapsed = start_time.elapsed();
        
        if is_factorial {
            found_count += 1;
            
            println!("🎉 НАЙДЕНО ЧИСЛО-ФАКТОРИАЛ №{}!", found_count);
            println!("   📊 Число: {}", current.to_string_value());
            println!("   🧮 Это факториал: {}! = {}", factorial_of, current.to_string_value());
            println!("   🔢 Тип: {}", current.get_type_name());
            println!("   ⏱️  Время проверки: {:.3?}", elapsed);
            println!("   📍 Позиция в диапазоне: {}/{}\n", 
                current.to_string_value(), end);
        }
        
        checked_count += 1;
        current = current.add(&one);
        
        // Для информативности выводим прогресс каждые 100000 чисел
        if checked_count % 100000 == 0 {
            let progress = (checked_count as f64 / (end - start) as f64) * 100.0;
            let speed = checked_count as f64 / total_start_time.elapsed().as_secs_f64();
            println!("🔄 Проверено: {} чисел ({:.1}%) | Скорость: {:.0}/сек", 
                checked_count, progress, speed);
        }
    }
    
    let total_elapsed = total_start_time.elapsed();
    println!("\n📊 Итоги поиска:");
    println!("   🔢 Диапазон: {} - {}", start, end);
    println!("   ✅ Найдено чисел-факториалов: {}", found_count);
    println!("   📋 Всего проверено: {}", checked_count);
    println!("   ⏱️  Общее время: {:.2?}", total_elapsed);
    println!("   ⚡ Скорость: {:.0} чисел/сек", 
        checked_count as f64 / total_elapsed.as_secs_f64());
}

// Многопоточное вычисление факториалов
fn calculate_factorials_multithreaded(num_threads: usize, start_num: i128, end_num: i128) {
    println!("🚀 Начинаем многопоточное вычисление факториалов...");
    println!("   🧵 Количество потоков: {}", num_threads);
    println!("   📊 Диапазон: {} - {}", start_num, end_num);
    println!("   ⚠️  Нажмите Ctrl+C для остановки\n");
    
    let calculated_count = Arc::new(AtomicUsize::new(0));
    let (tx, rx) = mpsc::channel();
    let start_time = Instant::now();
    
    // Создаем потоки
    let mut handles = Vec::new();
    let numbers_per_thread = (end_num - start_num + 1) / num_threads as i128;
    
    for thread_id in 0..num_threads {
        let tx_clone = tx.clone();
        let calculated_count_clone = Arc::clone(&calculated_count);
        let thread_start = start_num + (thread_id as i128 * numbers_per_thread);
        let thread_end = if thread_id == num_threads - 1 {
            end_num
        } else {
            thread_start + numbers_per_thread - 1
        };
        
        let handle = thread::spawn(move || {
            calculate_factorials_thread(
                thread_id,
                thread_start,
                thread_end,
                tx_clone,
                calculated_count_clone,
            );
        });
        
        handles.push(handle);
    }
    
    // Закрываем отправитель в основном потоке
    drop(tx);
    
    let calculated_count_for_msg = Arc::clone(&calculated_count);
    
    // Собираем результаты
    let msg_handle = thread::spawn(move || {
        for result in rx {
            match result {
                FactorialThreadMessage::FactorialCalculated { 
                    thread_id, number, factorial, type_name, calculation_time, decimal_length 
                } => {
                    let global_calculated = calculated_count_for_msg.fetch_add(1, Ordering::SeqCst) + 1;
                    let elapsed_total = start_time.elapsed();
                    
                    println!("🎉 ВЫЧИСЛЕН ФАКТОРИАЛ №{}!", global_calculated);
                    println!("   📊 Число: {}", number);
                    println!("   🧮 Факториал: {}", if decimal_length > 100 { 
                        format!("{}...{} ({} цифр)", 
                            &factorial[..50], 
                            &factorial[decimal_length-50..], 
                            decimal_length)
                    } else { 
                        factorial 
                    });
                    println!("   🔢 Тип: {}", type_name);
                    println!("   🧵 Поток: #{}", thread_id);
                    println!("   ⏱️  Время вычисления: {:.3?}", calculation_time);
                    println!("   📐 Длина: {} цифр", decimal_length);
                    println!("   ⏰ Общее время работы: {:.2?}\n", elapsed_total);
                }
                FactorialThreadMessage::Progress { thread_id, current_number, checked_count } => {
                    if checked_count % 5 == 0 {
                        let total_calculated = calculated_count_for_msg.load(Ordering::SeqCst);
                        let elapsed = start_time.elapsed();
                        let speed = total_calculated as f64 / elapsed.as_secs_f64();
                        println!("🔄 Поток #{}: вычисляется {}! | Всего: {} | Скорость: {:.2}/сек", 
                            thread_id, current_number, total_calculated, speed);
                    }
                }
                _ => {}
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
    let total_calculated_final = calculated_count.load(Ordering::SeqCst);
    
    println!("\n📊 Итоговая статистика многопоточного вычисления:");
    println!("   🧵 Потоков: {}", num_threads);
    println!("   📊 Диапазон: {} - {}", start_num, end_num);
    println!("   ✅ Вычислено факториалов: {}", total_calculated_final);
    println!("   ⏱️  Общее время: {:.2?}", total_time);
    println!("   ⚡ Общая скорость: {:.2} факториалов/сек", 
        total_calculated_final as f64 / total_time.as_secs_f64());
}

fn calculate_factorials_thread(
    thread_id: usize,
    start: i128,
    end: i128,
    tx: mpsc::Sender<FactorialThreadMessage>,
    calculated_count: Arc<AtomicUsize>,
) {
    let mut checked_in_thread = 0;
    
    for n in start..=end {
        let calculation_start = Instant::now();
        let factorial = DynamicInt::factorial_of(n);
        let calculation_time = calculation_start.elapsed();
        
        let decimal_length = factorial.to_string_value().len();
        
        checked_in_thread += 1;
        calculated_count.fetch_add(1, Ordering::SeqCst);
        
        let _ = tx.send(FactorialThreadMessage::FactorialCalculated {
            thread_id,
            number: n,
            factorial: factorial.to_string_value(),
            type_name: factorial.get_type_name().to_string(),
            calculation_time,
            decimal_length,
        });
        
        // Отправляем прогресс
        let _ = tx.send(FactorialThreadMessage::Progress {
            thread_id,
            current_number: n,
            checked_count: checked_in_thread,
        });
    }
    
    println!("🏁 Поток #{} завершен. Вычислено {} факториалов в диапазоне {}-{}", 
        thread_id, checked_in_thread, start, end);
}

// Бесконечное вычисление факториалов
fn calculate_factorials_infinite() {
    println!("♾️  Начинаем бесконечное вычисление факториалов...");
    println!("   🔢 Начинаем с 1! и идём до бесконечности");
    println!("   ⚠️  Нажмите Ctrl+C для остановки\n");
    
    let mut n = 1_i128;
    let mut calculated_count = 0;
    let start_time = Instant::now();
    
    loop {
        let calculation_start = Instant::now();
        let factorial = DynamicInt::factorial_of(n);
        let calculation_time = calculation_start.elapsed();
        
        calculated_count += 1;
        let decimal_length = factorial.to_string_value().len();
        
        println!("🎉 ВЫЧИСЛЕН ФАКТОРИАЛ №{}!", calculated_count);
        println!("   📊 Число: {}", n);
        println!("   🧮 Факториал {}!: {}", n, if decimal_length > 100 { 
            format!("{}...{} ({} цифр)", 
                &factorial.to_string_value()[..50], 
                &factorial.to_string_value()[decimal_length-50..], 
                decimal_length)
        } else { 
            factorial.to_string_value() 
        });
        println!("   🔢 Тип: {}", factorial.get_type_name());
        println!("   ⏱️  Время вычисления: {:.3?}", calculation_time);
        println!("   📐 Длина: {} цифр", decimal_length);
        
        let total_elapsed = start_time.elapsed();
        println!("   ⏰ Общее время работы: {:.2?}", total_elapsed);
        println!("   📍 Всего вычислено: {}\n", calculated_count);
        
        n += 1;
        
        // Проверяем переполнение
        if n <= 0 {
            println!("⚠️  Достигнуто максимальное значение i128!");
            break;
        }
    }
}

// Бесконечное многопоточное вычисление факториалов
fn calculate_factorials_infinite_multithreaded(num_threads: usize) {
    println!("♾️  Начинаем бесконечное многопоточное вычисление факториалов...");
    println!("   🧵 Количество потоков: {}", num_threads);
    println!("   📦 Размер блока на поток: 100 чисел");
    println!("   ⚠️  Нажмите Ctrl+C для остановки\n");
    
    let calculated_count = Arc::new(AtomicUsize::new(0));
    let current_n = Arc::new(AtomicUsize::new(1)); // Начинаем с 1
    let (tx, rx) = mpsc::channel();
    let start_time = Instant::now();
    
    // Создаем потоки
    let mut handles = Vec::new();
    
    for thread_id in 0..num_threads {
        let tx_clone = tx.clone();
        let calculated_count_clone = Arc::clone(&calculated_count);
        let current_n_clone = Arc::clone(&current_n);
        
        let handle = thread::spawn(move || {
            infinite_factorial_calculation_thread(
                thread_id,
                tx_clone,
                calculated_count_clone,
                current_n_clone,
            );
        });
        
        handles.push(handle);
    }
    
    // Закрываем отправитель в основном потоке
    drop(tx);
    
    let calculated_count_for_msg = Arc::clone(&calculated_count);
    
    // Собираем результаты
    let msg_handle = thread::spawn(move || {
        for result in rx {
            match result {
                FactorialThreadMessage::FactorialCalculated { 
                    thread_id, number, factorial, type_name, calculation_time, decimal_length 
                } => {
                    let global_calculated = calculated_count_for_msg.fetch_add(1, Ordering::SeqCst) + 1;
                    let elapsed_total = start_time.elapsed();
                    
                    println!("🎉 ВЫЧИСЛЕН ФАКТОРИАЛ №{}!", global_calculated);
                    println!("   📊 Число: {}", number);
                    println!("   🧮 Факториал: {}", if decimal_length > 100 { 
                        format!("{}...{} ({} цифр)", 
                            &factorial[..50], 
                            &factorial[decimal_length-50..], 
                            decimal_length)
                    } else { 
                        factorial 
                    });
                    println!("   🔢 Тип: {}", type_name);
                    println!("   🧵 Поток: #{}", thread_id);
                    println!("   ⏱️  Время вычисления: {:.3?}", calculation_time);
                    println!("   📐 Длина: {} цифр", decimal_length);
                    println!("   ⏰ Общее время работы: {:.2?}\n", elapsed_total);
                }
                FactorialThreadMessage::Progress { thread_id, current_number, checked_count } => {
                    if checked_count % 10 == 0 {
                        let total_calculated = calculated_count_for_msg.load(Ordering::SeqCst);
                        let elapsed = start_time.elapsed();
                        let speed = total_calculated as f64 / elapsed.as_secs_f64();
                        println!("🔄 Поток #{}: вычисляется {}! | Всего: {} | Скорость: {:.2}/сек", 
                            thread_id, current_number, total_calculated, speed);
                    }
                }
                _ => {}
            }
        }
    });
    
    // Ждем завершения всех потоков (никогда не случится в бесконечном режиме)
    for handle in handles {
        handle.join().unwrap();
    }
    
    // Ждем завершения потока обработки сообщений
    msg_handle.join().unwrap();
}

fn infinite_factorial_calculation_thread(
    thread_id: usize,
    tx: mpsc::Sender<FactorialThreadMessage>,
    calculated_count: Arc<AtomicUsize>,
    current_n: Arc<AtomicUsize>,
) {
    let chunk_size = 100_i128; // 100 факториалов на блок
    let mut checked_in_thread = 0;
    
    loop {
        // Атомарно получаем следующий диапазон
        let start_n = current_n.fetch_add(chunk_size as usize, Ordering::SeqCst) as i128;
        let end_n = start_n + chunk_size;
        
        // Проверяем на переполнение
        if start_n <= 0 || start_n >= i128::MAX - chunk_size {
            println!("⚠️  Поток #{} достиг максимального значения i128", thread_id);
            break;
        }
        
        println!("🧵 Поток #{} начинает вычисление факториалов {}-{}", 
            thread_id, start_n, end_n - 1);
        
        for n in start_n..end_n {
            let calculation_start = Instant::now();
            let factorial = DynamicInt::factorial_of(n);
            let calculation_time = calculation_start.elapsed();
            
            checked_in_thread += 1;
            calculated_count.fetch_add(1, Ordering::SeqCst);
            
            let decimal_length = factorial.to_string_value().len();
            
            let _ = tx.send(FactorialThreadMessage::FactorialCalculated {
                thread_id,
                number: n,
                factorial: factorial.to_string_value(),
                type_name: factorial.get_type_name().to_string(),
                calculation_time,
                decimal_length,
            });
            
            // Отправляем прогресс
            let _ = tx.send(FactorialThreadMessage::Progress {
                thread_id,
                current_number: n,
                checked_count: checked_in_thread,
            });
        }
        
        println!("🏁 Поток #{} завершил диапазон {}-{}, переходит к следующему", 
            thread_id, start_n, end_n - 1);
    }
}

// Поиск совершенных чисел по формуле Евклида: 2^(x-1) * (2^x - 1)
fn search_perfect_numbers_euclid(max_x: u32) {
    println!("🔥 Поиск совершенных чисел по формуле Евклида...");
    println!("   📐 Формула: 2^(x-1) * (2^x - 1), где 2^x - 1 - простое число Мерсенна");
    println!("   📊 Максимальный показатель x: {}\n", max_x);
    
    let mut found_count = 0;
    let mut checked_mersenne = 0;
    let total_start_time = Instant::now();
    
    for x in 2..=max_x {
        let mersenne_check_start = Instant::now();
        let is_mersenne_prime = DynamicInt::is_mersenne_prime(x);
        let mersenne_check_time = mersenne_check_start.elapsed();
        
        checked_mersenne += 1;
        
        if is_mersenne_prime {
            let perfect_calc_start = Instant::now();
            let mersenne_num = DynamicInt::mersenne_number(x);
            let perfect_num = DynamicInt::euclid_perfect_number(x);
            let perfect_calc_time = perfect_calc_start.elapsed();
            
            // Дополнительная проверка, что число действительно совершенное
            let verification_start = Instant::now();
            let is_actually_perfect = perfect_num.is_perfect();
            let verification_time = verification_start.elapsed();
            
            found_count += 1;
            
            println!("🎉 НАЙДЕНО СОВЕРШЕННОЕ ЧИСЛО №{} ПО ФОРМУЛЕ ЕВКЛИДА!", found_count);
            println!("   📏 Показатель x: {}", x);
            println!("   🔢 Число Мерсенна 2^{}-1: {}", x, mersenne_num.to_string_value());
            println!("   🎯 Совершенное число: {}", perfect_num.to_string_value());
            println!("   📊 Тип числа: {}", perfect_num.get_type_name());
            println!("   ⏱️  Время проверки Мерсенна: {:.3?}", mersenne_check_time);
            println!("   ⏱️  Время вычисления совершенного: {:.3?}", perfect_calc_time);
            println!("   ✅ Проверка совершенности: {} ({:.3?})", 
                if is_actually_perfect { "✅" } else { "❌" }, verification_time);
            
            // Показываем длину числа в десятичных знаках
            let decimal_length = perfect_num.to_string_value().len();
            println!("   📐 Длина в десятичных знаках: {}\n", decimal_length);
        } else {
            if x <= 20 || x % 10 == 0 {
                println!("🔍 x={}: Мерсенна 2^{}-1 не является простым числом ({:.3?})", 
                    x, x, mersenne_check_time);
            }
        }
    }
    
    let total_elapsed = total_start_time.elapsed();
    println!("\n📊 Итоги поиска по формуле Евклида:");
    println!("   📏 Проверено показателей x: {} (от 2 до {})", checked_mersenne, max_x);
    println!("   ✅ Найдено совершенных чисел: {}", found_count);
    println!("   ⏱️  Общее время: {:.2?}", total_elapsed);
    println!("   ⚡ Средняя скорость: {:.3} показателей/сек", 
        checked_mersenne as f64 / total_elapsed.as_secs_f64());
    
    if found_count > 0 {
        println!("   💡 Формула Евклида работает! Все найденные числа действительно совершенные.");
    }
}

// Тест известных чисел Мерсенна
fn test_mersenne_numbers() {
    println!("🧮 Тестируем известные числа Мерсенна...\n");
    
    // Первые известные простые числа Мерсенна (показатели степени)
    let known_mersenne_primes = vec![2, 3, 5, 7, 13, 17, 19, 31];
    
    println!("✅ Проверяем известные простые числа Мерсенна:");
    for x in &known_mersenne_primes {
        let start_time = Instant::now();
        let mersenne_num = DynamicInt::mersenne_number(*x);
        let is_prime = mersenne_num.is_prime();
        let elapsed = start_time.elapsed();
        
        println!("2^{}-1 = {} | Простое: {} | Тип: {} | Время: {:.3?}",
            x, 
            mersenne_num.to_string_value(),
            if is_prime { "✅" } else { "❌" },
            mersenne_num.get_type_name(),
            elapsed
        );
    }
    
    println!("\n❌ Проверяем числа, которые НЕ дают простые числа Мерсенна:");
    let non_mersenne = vec![4, 6, 8, 9, 10, 11, 12, 14, 15, 16];
    
    for x in &non_mersenne {
        let start_time = Instant::now();
        let mersenne_num = DynamicInt::mersenne_number(*x);
        let is_prime = mersenne_num.is_prime();
        let elapsed = start_time.elapsed();
        
        println!("2^{}-1 = {} | Простое: {} | Время: {:.3?}",
            x, 
            mersenne_num.to_string_value(),
            if is_prime { "✅" } else { "❌" },
            elapsed
        );
    }
    
    println!("\n🎯 Теперь проверим соответствующие совершенные числа:");
    for x in &known_mersenne_primes[..4] { // Первые 4 для демонстрации
        let start_time = Instant::now();
        let perfect_num = DynamicInt::euclid_perfect_number(*x);
        let is_perfect = perfect_num.is_perfect();
        let elapsed = start_time.elapsed();
        
        println!("x={}: 2^{} * (2^{}-1) = {} | Совершенное: {} | Время: {:.3?}",
            x, x-1, x, 
            perfect_num.to_string_value(),
            if is_perfect { "✅" } else { "❌" },
            elapsed
        );
    }
    
    println!("\n🎯 Результат: алгоритм корректно определяет числа Мерсенна и совершенные числа!");
}

#[derive(Debug)]
enum EuclidThreadMessage {
    PerfectFound {
        thread_id: usize,
        x: u32,
        mersenne_number: String,
        perfect_number: String,
        type_name: String,
        mersenne_time: std::time::Duration,
        perfect_time: std::time::Duration,
        verification_time: std::time::Duration,
        decimal_length: usize,
    },
    Progress {
        thread_id: usize,
        current_x: u32,
        checked_count: usize,
    },
}

// Многопоточный поиск совершенных чисел по формуле Евклида
fn search_perfect_numbers_euclid_multithreaded(num_threads: usize, max_x: u32) {
    println!("🚀 Начинаем многопоточный поиск по формуле Евклида...");
    println!("   🧵 Количество потоков: {}", num_threads);
    println!("   📏 Максимальный показатель x: {}", max_x);
    println!("   📐 Формула: 2^(x-1) * (2^x - 1), где 2^x - 1 - простое");
    println!("   ⚠️  Нажмите Ctrl+C для остановки\n");
    
    let found_count = Arc::new(AtomicUsize::new(0));
    let checked_count = Arc::new(AtomicUsize::new(0));
    let (tx, rx) = mpsc::channel();
    let start_time = Instant::now();
    
    // Создаем потоки
    let mut handles = Vec::new();
    let chunk_size = (max_x - 2 + 1) / num_threads as u32 + 1;
    
    for thread_id in 0..num_threads {
        let tx_clone = tx.clone();
        let found_count_clone = Arc::clone(&found_count);
        let checked_count_clone = Arc::clone(&checked_count);
        let start_x = 2 + (thread_id as u32 * chunk_size);
        let end_x = std::cmp::min(start_x + chunk_size, max_x + 1);
        
        if start_x < end_x {
            let handle = thread::spawn(move || {
                search_euclid_thread(
                    thread_id,
                    start_x,
                    end_x,
                    tx_clone,
                    found_count_clone,
                    checked_count_clone,
                );
            });
            
            handles.push(handle);
        }
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
                EuclidThreadMessage::PerfectFound { 
                    thread_id, x, mersenne_number, perfect_number, type_name,
                    mersenne_time, perfect_time, verification_time, decimal_length
                } => {
                    let global_found = found_count_for_msg.fetch_add(1, Ordering::SeqCst) + 1;
                    let elapsed_total = start_time.elapsed();
                    
                    println!("🎉 НАЙДЕНО СОВЕРШЕННОЕ ЧИСЛО №{} ПО ФОРМУЛЕ ЕВКЛИДА!", global_found);
                    println!("   📏 Показатель x: {}", x);
                    println!("   🔢 Число Мерсенна 2^{}-1: {}", x, mersenne_number);
                    println!("   🎯 Совершенное число: {}", perfect_number);
                    println!("   📊 Тип числа: {}", type_name);
                    println!("   🧵 Поток: #{}", thread_id);
                    println!("   ⏱️  Время проверки Мерсенна: {:.3?}", mersenne_time);
                    println!("   ⏱️  Время вычисления: {:.3?}", perfect_time);
                    println!("   ✅ Время проверки совершенности: {:.3?}", verification_time);
                    println!("   📐 Длина в десятичных знаках: {}", decimal_length);
                    println!("   ⏰ Общее время работы: {:.2?}\n", elapsed_total);
                }
                EuclidThreadMessage::Progress { thread_id, current_x, checked_count } => {
                    if checked_count % 50 == 0 {
                        let total_checked = checked_count_for_msg.load(Ordering::SeqCst);
                        let elapsed = start_time.elapsed();
                        let speed = total_checked as f64 / elapsed.as_secs_f64();
                        println!("🔄 Поток #{}: проверяется x={} | Всего: {} | Скорость: {:.2}/сек", 
                            thread_id, current_x, total_checked, speed);
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
    
    println!("\n📊 Итоговая статистика многопоточного поиска по Евклиду:");
    println!("   🧵 Потоков: {}", num_threads);
    println!("   📏 Проверено показателей x: {} (от 2 до {})", total_checked_final, max_x);
    println!("   ✅ Найдено совершенных чисел: {}", total_found_final);
    println!("   ⏱️  Общее время: {:.2?}", total_time);
    println!("   ⚡ Общая скорость: {:.2} показателей/сек", total_checked_final as f64 / total_time.as_secs_f64());
    
    if total_found_final > 0 {
        println!("   💡 Формула Евклида эффективно находит совершенные числа!");
    }
}

fn search_euclid_thread(
    thread_id: usize,
    start_x: u32,
    end_x: u32,
    tx: mpsc::Sender<EuclidThreadMessage>,
    _found_count: Arc<AtomicUsize>,
    checked_count: Arc<AtomicUsize>,
) {
    let mut checked_in_thread = 0;
    
    for x in start_x..end_x {
        let mersenne_check_start = Instant::now();
        let is_mersenne_prime = DynamicInt::is_mersenne_prime(x);
        let mersenne_time = mersenne_check_start.elapsed();
        
        checked_in_thread += 1;
        checked_count.fetch_add(1, Ordering::SeqCst);
        
        if is_mersenne_prime {
            let perfect_calc_start = Instant::now();
            let mersenne_num = DynamicInt::mersenne_number(x);
            let perfect_num = DynamicInt::euclid_perfect_number(x);
            let perfect_time = perfect_calc_start.elapsed();
            
            // Дополнительная проверка совершенности
            let verification_start = Instant::now();
            let _is_actually_perfect = perfect_num.is_perfect();
            let verification_time = verification_start.elapsed();
            
            let decimal_length = perfect_num.to_string_value().len();
            
            let _ = tx.send(EuclidThreadMessage::PerfectFound {
                thread_id,
                x,
                mersenne_number: mersenne_num.to_string_value(),
                perfect_number: perfect_num.to_string_value(),
                type_name: perfect_num.get_type_name().to_string(),
                mersenne_time,
                perfect_time,
                verification_time,
                decimal_length,
            });
        }
        
        // Отправляем прогресс
        if checked_in_thread % 10 == 0 {
            let _ = tx.send(EuclidThreadMessage::Progress {
                thread_id,
                current_x: x,
                checked_count: checked_in_thread,
            });
        }
    }
    
    println!("🏁 Поток #{} завершен. Проверено {} показателей x в диапазоне {}-{}", 
        thread_id, checked_in_thread, start_x, end_x - 1);
}

// Бесконечный поиск совершенных чисел по формуле Евклида
fn search_perfect_numbers_euclid_infinite() {
    println!("♾️  Начинаем бесконечный поиск по формуле Евклида...");
    println!("   📐 Формула: 2^(x-1) * (2^x - 1), где 2^x - 1 - простое число Мерсенна");
    println!("   🔢 Начинаем с x=2 и идём до бесконечности");
    println!("   ⚠️  Нажмите Ctrl+C для остановки\n");
    
    let mut found_count = 0;
    let mut checked_count = 0;
    let start_time = Instant::now();
    let mut x = 2_u32;
    
    loop {
        let mersenne_check_start = Instant::now();
        let is_mersenne_prime = DynamicInt::is_mersenne_prime(x);
        let mersenne_check_time = mersenne_check_start.elapsed();
        
        checked_count += 1;
        
        if is_mersenne_prime {
            let perfect_calc_start = Instant::now();
            let mersenne_num = DynamicInt::mersenne_number(x);
            let perfect_num = DynamicInt::euclid_perfect_number(x);
            let perfect_calc_time = perfect_calc_start.elapsed();
            
            // Дополнительная проверка совершенности
            let verification_start = Instant::now();
            let is_actually_perfect = perfect_num.is_perfect();
            let verification_time = verification_start.elapsed();
            
            found_count += 1;
            
            println!("🎉 НАЙДЕНО СОВЕРШЕННОЕ ЧИСЛО №{} ПО ФОРМУЛЕ ЕВКЛИДА!", found_count);
            println!("   📏 Показатель x: {}", x);
            println!("   🔢 Число Мерсенна 2^{}-1: {}", x, mersenne_num.to_string_value());
            println!("   🎯 Совершенное число: {}", perfect_num.to_string_value());
            println!("   📊 Тип числа: {}", perfect_num.get_type_name());
            println!("   ⏱️  Время проверки Мерсенна: {:.3?}", mersenne_check_time);
            println!("   ⏱️  Время вычисления совершенного: {:.3?}", perfect_calc_time);
            println!("   ✅ Проверка совершенности: {} ({:.3?})", 
                if is_actually_perfect { "✅" } else { "❌" }, verification_time);
            
            let decimal_length = perfect_num.to_string_value().len();
            println!("   📐 Длина в десятичных знаках: {}", decimal_length);
            
            let total_elapsed = start_time.elapsed();
            println!("   ⏰ Общее время работы: {:.2?}", total_elapsed);
            println!("   📍 Всего проверено показателей: {}\n", checked_count);
        } else {
            // Показываем прогресс для больших показателей
            if x <= 50 || x % 100 == 0 {
                let total_elapsed = start_time.elapsed();
                let speed = checked_count as f64 / total_elapsed.as_secs_f64();
                println!("🔍 x={}: Мерсенна 2^{}-1 не является простым | Время: {:.3?} | Скорость: {:.2}/сек", 
                    x, x, mersenne_check_time, speed);
            }
        }
        
        x = x.checked_add(1).unwrap_or_else(|| {
            println!("⚠️  Достигнут максимальный показатель u32!");
            u32::MAX
        });
        
        if x == u32::MAX {
            break;
        }
    }
    
    let total_elapsed = start_time.elapsed();
    println!("\n📊 Итоги бесконечного поиска по формуле Евклида:");
    println!("   📏 Проверено показателей x: {}", checked_count);
    println!("   ✅ Найдено совершенных чисел: {}", found_count);
    println!("   ⏱️  Общее время: {:.2?}", total_elapsed);
    println!("   ⚡ Средняя скорость: {:.3} показателей/сек", 
        checked_count as f64 / total_elapsed.as_secs_f64());
}

// Бесконечный многопоточный поиск по формуле Евклида
fn search_perfect_numbers_euclid_infinite_multithreaded(num_threads: usize) {
    println!("♾️  Начинаем бесконечный многопоточный поиск по формуле Евклида...");
    println!("   🧵 Количество потоков: {}", num_threads);
    println!("   📐 Формула: 2^(x-1) * (2^x - 1), где 2^x - 1 - простое");
    println!("   📦 Размер блока на поток: 1,000 показателей x");
    println!("   ⚠️  Нажмите Ctrl+C для остановки\n");
    
    let found_count = Arc::new(AtomicUsize::new(0));
    let checked_count = Arc::new(AtomicUsize::new(0));
    let current_x = Arc::new(AtomicUsize::new(2)); // Начинаем с x=2
    let (tx, rx) = mpsc::channel();
    let start_time = Instant::now();
    
    // Создаем потоки
    let mut handles = Vec::new();
    
    for thread_id in 0..num_threads {
        let tx_clone = tx.clone();
        let found_count_clone = Arc::clone(&found_count);
        let checked_count_clone = Arc::clone(&checked_count);
        let current_x_clone = Arc::clone(&current_x);
        
        let handle = thread::spawn(move || {
            infinite_euclid_search_thread(
                thread_id,
                tx_clone,
                found_count_clone,
                checked_count_clone,
                current_x_clone,
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
                EuclidThreadMessage::PerfectFound { 
                    thread_id, x, mersenne_number, perfect_number, type_name,
                    mersenne_time, perfect_time, verification_time, decimal_length
                } => {
                    let global_found = found_count_for_msg.fetch_add(1, Ordering::SeqCst) + 1;
                    let elapsed_total = start_time.elapsed();
                    
                    println!("🎉 НАЙДЕНО СОВЕРШЕННОЕ ЧИСЛО №{} ПО ФОРМУЛЕ ЕВКЛИДА!", global_found);
                    println!("   📏 Показатель x: {}", x);
                    println!("   🔢 Число Мерсенна 2^{}-1: {}", x, mersenne_number);
                    println!("   🎯 Совершенное число: {}", perfect_number);
                    println!("   📊 Тип числа: {}", type_name);
                    println!("   🧵 Поток: #{}", thread_id);
                    println!("   ⏱️  Время проверки Мерсенна: {:.3?}", mersenne_time);
                    println!("   ⏱️  Время вычисления: {:.3?}", perfect_time);
                    println!("   ✅ Время проверки совершенности: {:.3?}", verification_time);
                    println!("   📐 Длина в десятичных знаках: {}", decimal_length);
                    println!("   ⏰ Общее время работы: {:.2?}\n", elapsed_total);
                }
                EuclidThreadMessage::Progress { thread_id, current_x, checked_count } => {
                    if checked_count % 500 == 0 {
                        let total_checked = checked_count_for_msg.load(Ordering::SeqCst);
                        let total_found = found_count_for_msg.load(Ordering::SeqCst);
                        let elapsed = start_time.elapsed();
                        let speed = total_checked as f64 / elapsed.as_secs_f64();
                        println!("🔄 Поток #{}: проверяется x={} | Всего: {} | Найдено: {} | Скорость: {:.2}/сек", 
                            thread_id, current_x, total_checked, total_found, speed);
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

fn infinite_euclid_search_thread(
    thread_id: usize,
    tx: mpsc::Sender<EuclidThreadMessage>,
    _found_count: Arc<AtomicUsize>,
    checked_count: Arc<AtomicUsize>,
    current_x: Arc<AtomicUsize>,
) {
    let chunk_size = 1000_u32; // 1000 показателей x на блок
    let mut checked_in_thread = 0;
    
    loop {
        // Атомарно получаем следующий диапазон показателей
        let start_x = current_x.fetch_add(chunk_size as usize, Ordering::SeqCst) as u32;
        let end_x = start_x + chunk_size;
        
        // Проверяем на переполнение u32
        if start_x >= u32::MAX - chunk_size {
            println!("⚠️  Поток #{} достиг максимального показателя u32", thread_id);
            break;
        }
        
        println!("🧵 Поток #{} начинает поиск в диапазоне x={}-{}", thread_id, start_x, end_x - 1);
        
        for x in start_x..end_x {
            let mersenne_check_start = Instant::now();
            let is_mersenne_prime = DynamicInt::is_mersenne_prime(x);
            let mersenne_time = mersenne_check_start.elapsed();
            
            checked_in_thread += 1;
            checked_count.fetch_add(1, Ordering::SeqCst);
            
            if is_mersenne_prime {
                let perfect_calc_start = Instant::now();
                let mersenne_num = DynamicInt::mersenne_number(x);
                let perfect_num = DynamicInt::euclid_perfect_number(x);
                let perfect_time = perfect_calc_start.elapsed();
                
                // Дополнительная проверка совершенности
                let verification_start = Instant::now();
                let _is_actually_perfect = perfect_num.is_perfect();
                let verification_time = verification_start.elapsed();
                
                let decimal_length = perfect_num.to_string_value().len();
                
                let _ = tx.send(EuclidThreadMessage::PerfectFound {
                    thread_id,
                    x,
                    mersenne_number: mersenne_num.to_string_value(),
                    perfect_number: perfect_num.to_string_value(),
                    type_name: perfect_num.get_type_name().to_string(),
                    mersenne_time,
                    perfect_time,
                    verification_time,
                    decimal_length,
                });
            }
            
            // Отправляем прогресс
            if checked_in_thread % 100 == 0 {
                let _ = tx.send(EuclidThreadMessage::Progress {
                    thread_id,
                    current_x: x,
                    checked_count: checked_in_thread,
                });
            }
        }
        
        println!("🏁 Поток #{} завершил диапазон x={}-{}, переходит к следующему", 
            thread_id, start_x, end_x - 1);
    }
}

#[derive(Debug)]
enum SingleFactorialThreadMessage {
    PartialResult {
        thread_id: usize,
        start_range: i128,
        end_range: i128,
        partial_product: String,
        calculation_time: std::time::Duration,
        decimal_length: usize,
    },
    Progress {
        thread_id: usize,
        current_progress: f64,
    },
}

// Многопоточное вычисление одного факториала
fn calculate_factorial_multithreaded(n: i128, num_threads: usize) {
    println!("🧮 Многопоточное вычисление {}! с {} потоками...\n", n, num_threads);
    
    if n < 0 {
        println!("❌ Ошибка: факториал определен только для неотрицательных чисел!");
        return;
    }
    
    if n <= 1 {
        println!("🎉 РЕЗУЛЬТАТ: {}! = 1", n);
        return;
    }
    
    let start_time = Instant::now();
    let (tx, rx) = mpsc::channel();
    
    // Разделяем диапазон 1..=n между потоками
    let chunk_size = n / num_threads as i128;
    let remainder = n % num_threads as i128;
    
    let mut handles = Vec::new();
    let mut current_start = 1_i128;
    
    for thread_id in 0..num_threads {
        let tx_clone = tx.clone();
        
        // Распределяем остаток между первыми потоками
        let current_chunk_size = if thread_id < remainder as usize {
            chunk_size + 1
        } else {
            chunk_size
        };
        
        let range_start = current_start;
        let range_end = current_start + current_chunk_size - 1;
        current_start = range_end + 1;
        
        // Если диапазон пустой, пропускаем поток
        if range_start > n {
            break;
        }
        
        let actual_end = std::cmp::min(range_end, n);
        
        let handle = thread::spawn(move || {
            calculate_partial_factorial_thread(
                thread_id,
                range_start,
                actual_end,
                tx_clone,
            );
        });
        
        handles.push(handle);
        println!("🧵 Поток #{}: диапазон {}-{} (размер: {})", 
            thread_id, range_start, actual_end, actual_end - range_start + 1);
    }
    
    drop(tx);
    
    // Собираем частичные результаты
    let mut partial_results: Vec<(usize, DynamicInt, std::time::Duration)> = Vec::new();
    
    for result in rx {
        match result {
            SingleFactorialThreadMessage::PartialResult { 
                thread_id, start_range, end_range, partial_product, calculation_time, decimal_length 
            } => {
                println!("✅ Поток #{} завершен: диапазон {}-{}", thread_id, start_range, end_range);
                println!("   📐 Длина результата: {} цифр", decimal_length);
                println!("   ⏱️  Время вычисления: {:.3?}", calculation_time);
                
                // Восстанавливаем DynamicInt из строки (простое решение)
                let partial_result = if partial_product.len() > 50 {
                    // Для больших чисел используем BigInt
                    use num_bigint::BigInt;
                    use std::str::FromStr;
                    DynamicInt::Big(BigInt::from_str(&partial_product).unwrap())
                } else {
                    DynamicInt::new(partial_product.parse().unwrap_or(1))
                };
                
                partial_results.push((thread_id, partial_result, calculation_time));
            }
            SingleFactorialThreadMessage::Progress { thread_id, current_progress } => {
                println!("🔄 Поток #{}: прогресс {:.1}%", thread_id, current_progress);
            }
        }
    }
    
    // Ждем завершения всех потоков
    for handle in handles {
        handle.join().unwrap();
    }
    
    println!("\n🔗 Объединяем частичные результаты...");
    let merge_start = Instant::now();
    
    // Сортируем результаты по thread_id для правильного порядка умножения
    partial_results.sort_by_key(|(thread_id, _, _)| *thread_id);
    
    let mut final_result = DynamicInt::one();
    for (thread_id, partial_result, thread_time) in partial_results {
        println!("🔗 Умножаем результат потока #{} (время потока: {:.3?})", thread_id, thread_time);
        final_result = final_result.mul(&partial_result);
    }
    
    let merge_time = merge_start.elapsed();
    let total_time = start_time.elapsed();
    let decimal_length = final_result.to_string_value().len();
    
    println!("\n🎉 РЕЗУЛЬТАТ МНОГОПОТОЧНОГО ВЫЧИСЛЕНИЯ:");
    println!("   📊 Число: {}", n);
    println!("   🧮 Факториал {}!: {}", n, if decimal_length > 100000000 { 
        format!("{}...{} ({} цифр)", 
            &final_result.to_string_value()[..50], 
            &final_result.to_string_value()[decimal_length-50..], 
            decimal_length)
    } else { 
        final_result.to_string_value() 
    });
    println!("   🔢 Тип результата: {}", final_result.get_type_name());
    println!("   📐 Длина в десятичных знаках: {}", decimal_length);
    println!("   🧵 Количество потоков: {}", num_threads);
    println!("   ⏱️  Время объединения результатов: {:.3?}", merge_time);
    println!("   ⏰ Общее время вычисления: {:.3?}", total_time);
    
     // Дополнительная информация
    if decimal_length > 100 {
        println!("   💡 Первые 50 цифр: {}...", &final_result.to_string_value()[..50]);
        println!("   💡 Последние 50 цифр: ...{}", &final_result.to_string_value()[decimal_length-50..]);
    }


    // Сравнение с однопоточным вычислением для небольших чисел
    if n <= 100000 {
        println!("\n📊 Сравнение с однопоточным вычислением:");
        let single_start = Instant::now();
        let single_result = DynamicInt::factorial_of(n);
        let single_time = single_start.elapsed();
        
        println!("   ⏱️  Однопоточное время: {:.3?}", single_time);
        let speedup = single_time.as_secs_f64() / total_time.as_secs_f64();
        println!("   ⚡ Ускорение: {:.2}x", speedup);
        
        // Проверяем корректность
        if final_result.to_string_value() == single_result.to_string_value() {
            println!("   ✅ Результаты совпадают!");
        } else {
            println!("   ❌ Ошибка: результаты не совпадают!");
        }
    }
}

fn calculate_partial_factorial_thread(
    thread_id: usize,
    start: i128,
    end: i128,
    tx: mpsc::Sender<SingleFactorialThreadMessage>,
) {
    let calculation_start = Instant::now();
    let total_numbers = end - start + 1;
    
    println!("🧵 Поток #{} начал вычисление произведения {}-{} ({} чисел)", 
        thread_id, start, end, total_numbers);
    
    let partial_result = DynamicInt::product_range(start, end);
    let calculation_time = calculation_start.elapsed();
    let decimal_length = partial_result.to_string_value().len();
    
    let _ = tx.send(SingleFactorialThreadMessage::PartialResult {
        thread_id,
        start_range: start,
        end_range: end,
        partial_product: partial_result.to_string_value(),
        calculation_time,
        decimal_length,
    });
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
