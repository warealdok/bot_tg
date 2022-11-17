// mahahuha: короче
// - делаешь телеграмм бота
// - делаешь ему ответ на сообщение фа диез
// - в качестве ответа будет инфа, вычисленная по следующей схеме
// запрашиваешь даннные с https://jsonplaceholder.typicode.com/posts - там будет жсон на 100 итемов
// 1 - нужно найти id поста с самым длинным боди по количеству букв
// 2 - найти id поста с самым маленьким количеством слов в тайтле
// 3 - посчитать суммарное количество букв во всех боди всех постов

use std::vec;

use teloxide::prelude::*;
use reqwest;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Posts {
    user_id: i32,
    id: i32,
    title: String,
    body: String,
}

#[tokio::main]
async fn main() {    
    bot_tg().await;

}
// Функция обращения к json'у
async fn json_call(some_var: &mut (usize, (Vec<usize>, Vec<usize>)))  -> Result<(), reqwest::Error> {

    let mut lowest_title_vec = Vec::new();
    let mut temp_vec = Vec::new();

    let result: Vec<Posts> = reqwest::Client::new()
        .get("https://jsonplaceholder.typicode.com/posts")
        .send()
        .await?
        .json()
        .await?;
    
        for post in result.iter() {
            // Считаем количество букв правильным методом
            temp_vec.push(letter_counter(&post.body));
            lowest_title_vec.push(word_counter(&post.title));
        }
        
*some_var = json_calc(&temp_vec, &lowest_title_vec);

    Ok(())
}
// Функция подсчета данных json'a
fn json_calc(temp_vec: &Vec<usize>, lowest_title_vec: &Vec<usize>) -> (usize, (Vec<usize>, Vec<usize>)) {

    let lowest_value;
    let highest_value;
    let index_of_high_vec: Vec<usize> = Vec::new();
    let index_of_low_vec: Vec<usize> = Vec::new();

    let sum_of_all_body: usize = temp_vec.iter().sum();
    
    lowest_value = lowest_title_vec.iter().min().unwrap_or(&0);
    highest_value = temp_vec.iter().max().unwrap_or(&0);

    let output = compare_vec(&temp_vec, highest_value, index_of_high_vec, &lowest_title_vec, lowest_value, index_of_low_vec);

    // print!("Id поста с самым длинным боди по количеству букв: ");
    // for i in 0..output.0.len() {
    //     print!("{}", output.0[i]);
    //     if output.0.len() != 1 {
    //     print!(", ");
    //     }
        
    // }
    // print!("\nId поста с самым маленьким количеством слов в тайтле: ");
    // for i in 0..output.1.len() {
    //     print!("{}", output.1[i]);
    //     if i < output.1.len() - 1 {
    //     print!(", ");
    //     }
    // }
    // println!("\nСумма всех боди всех постов: {}", sum_of_all_body);
    return (sum_of_all_body, output)


}
// Функция подсчета букв
fn letter_counter(string: &str) -> usize {
    let mut letter_counter: usize = 0;
    let mut symbol;
    for i in 0..string.chars().count() {
        symbol = string.chars().nth(i).unwrap();
        if symbol != ' ' {
            letter_counter += 1
        }
    }
    return letter_counter;
}
// Функция подсчета слов
fn word_counter(string: &str) -> usize {
    let mut word_counter: usize = 0;
    let mut symbol;
    for i in 0..string.chars().count() {
        symbol = string.chars().nth(i).unwrap();
         if symbol == ' ' {
             word_counter += 1;
         }
}
        return word_counter;
}
// Функция вычисления id
fn compare_vec(temp_vec: &Vec<usize>, highest_value: &usize, mut index_of_high_vec: Vec<usize>, lowest_title_vec: &Vec<usize>, lowest_value: &usize, mut index_of_low_vec: Vec<usize>) -> (Vec<usize>, Vec<usize>) {

    for i in 0..temp_vec.len() {
        if *highest_value == temp_vec[i] {
            index_of_high_vec.push(i);
        }
    }

    for i in 0..temp_vec.len() {
        if *lowest_value == lowest_title_vec[i] {
            index_of_low_vec.push(i);
        }
    }

    for i in 0..index_of_high_vec.len() {
            index_of_high_vec[i] += 1;
    }

    for i in 0..index_of_low_vec.len() {
            index_of_low_vec[i] += 1;
        }
    
    return (index_of_high_vec, index_of_low_vec)

}
// fn some_numbers(output: (usize, (Vec<usize>, Vec<usize>))) {
// println!("{:#?}", output.0);
// println!("{:#?}", output.1.0);
// println!("{:#?}", output.1.1);
// }
// Функционал бота
async fn bot_tg() {
    pretty_env_logger::init();
    log::info!("Starting throw dice bot...");

    let bot = Bot::from_env();


    teloxide::repl(bot, |bot: Bot, msg: Message| async move {
        let test_var = &mut (11 as usize, (vec![] as Vec<usize>, vec![] as Vec<usize>));
        json_call(test_var).await.unwrap();
        let long_string = test_var.0.to_string();
        let borrowed_string: &str = "world";
        let new_owned_string = long_string + borrowed_string;

        bot.send_message(msg.chat.id, new_owned_string).await?;
        Ok(())
    })
    .await;
}