/*Implement basic function to split some generic
computational work between threads.
Split should occur only on some threshold -
if computational work (input length) is shorter
than this threshold, no splitting should occur and no threads should be created.

You get as input:

1. Vec<T>
2. Function f(t: T) -> R


Threshold can be just constant.

You should return:
   1. Up to you, but probably some Vec of the same length as input(1)*/

extern crate num_cpus;

use std::thread;
use std::cmp;

// фнкция для работы
fn thread_work(input: &[i32]) {
    let sum = input.iter().sum::<i32>();
    println!("Thread {:?} calculated sum: {}", thread::current().id(), sum);
}

// функция, которая разделяет общую работу между потоками
fn parallel_compute(input: Vec<i32>, threshold: usize) {
    // сравниваем длинну вектора c минимальным порогом
    if input.len() <= threshold {
        println!("Input vector is too short, parallel compute is not needed");
        return;
    }

    // разбиваем входной вектор на несколько частей, чтобы передать каждую часть в отдельный поток

    let num_threads = num_cpus::get(); // получаем количество доступных процессорных ядер
    let chunk_size = cmp::max(1, input.len() / num_threads); // определяем размер части входного вектора
    // проблема вот в чём
    // не дай бог `input.len() / num_threads` будет равен нулю, всё сломается
    // через `cmp::max` мы в любом случае будем создавать `num_threads`
    // каждый поток будет обрабатывать пустую часть входного вектора
    // тупое решение, но на другое не додумался


    let chunks = input.chunks(chunk_size); // создаем итератор по частям входного вектора

    // создаем потоки и запускаем на каждом из них функцию thread_work
    let threads = chunks.map(|chunk| {
        let chunk = chunk.to_vec(); // копируем часть входного вектора для передачи в поток
        thread::spawn(move || thread_work(&chunk))
    }).collect::<Vec<_>>(); // собираем всё

    // ждем, пока все потоки завершатся и отлавливаем ошибки
    for thread in threads {
        thread.join().unwrap();
    }

    // конец
}

fn main() {
    let input = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let threshold = 3;

    parallel_compute(input, threshold);
}