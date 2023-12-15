// fn main() {
//     use std::collections::HashMap;

//     let teams_list = vec![
//         ("中国队".to_string(), 100),
//         ("美国队".to_string(), 10),
//         ("日本队".to_string(), 50),
//     ];

//     let mut teams_map = HashMap::new();
//     for team in &teams_list {
//         teams_map.insert(&team.0, team.1);
//     }

//     println!("{:?}", teams_map)
// }

// 先将 Vec 转为迭代器，接着通过 collect 方法，将迭代器中的元素收集后，转成 HashMap：

// fn main() {
//     use std::collections::HashMap;

//     let teams_list = vec![
//         ("中国队".to_string(), 100),
//         ("美国队".to_string(), 10),
//         ("日本队".to_string(), 50),
//     ];

//     let teams_map: HashMap<_, _> = teams_list.into_iter().collect();

//     println!("{:?}", teams_map)
// }

fn main() {
    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert("Blue", 10);

    // 覆盖已有的值
    let old = scores.insert("Blue", 20);
    println!("{:?}", old); // Some(10)

    // 查询新插入的值
    let new = scores.get("Blue");
    println!("{:?}", new); // Some(20)

    // 查询Yellow对应的值，若不存在则插入新值
    let v = scores.entry("Yellow").or_insert(5);
    // 不存在，插入5
    println!("{:?}", v); // 5

    // 查询Yellow对应的值，若不存在则插入新值
    let v = scores.entry("Yellow").or_insert(50);
    // 已经存在，因此50没有插入
    println!("{:?}", v); // 5

    let text = "hello world wonderful world";

    let mut map = HashMap::new();
    // 根据空格来切分字符串(英文单词都是通过空格切分)
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map); // {"hello": 1, "world": 2, "wonderful": 1}
}
