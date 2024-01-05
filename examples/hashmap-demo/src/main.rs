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

// fn main() {
//     use std::collections::HashMap;

//     let mut scores = HashMap::new();

//     scores.insert("Blue", 10);

//     // 覆盖已有的值
//     let old = scores.insert("Blue", 20);
//     println!("{:?}", old); // Some(10)

//     // 查询新插入的值
//     let new = scores.get("Blue");
//     println!("{:?}", new); // Some(20)

//     // 查询Yellow对应的值，若不存在则插入新值
//     let v = scores.entry("Yellow").or_insert(5);
//     // 不存在，插入5
//     println!("{:?}", v); // 5

//     // 查询Yellow对应的值，若不存在则插入新值
//     let v = scores.entry("Yellow").or_insert(50);
//     // 已经存在，因此50没有插入
//     println!("{:?}", v); // 5

//     let text = "hello world wonderful world";

//     let mut map = HashMap::new();
//     // 根据空格来切分字符串(英文单词都是通过空格切分)
//     for word in text.split_whitespace() {
//         let count = map.entry(word).or_insert(0);
//         *count += 1;
//     }

//     println!("{:?}", map); // {"hello": 1, "world": 2, "wonderful": 1}
// }

// fn main() {
// use std::collections::HashMap;
//     // 创建一个HashMap，用于存储宝石种类和对应的数量
//     let mut my_gems = HashMap::new();

//     // 将宝石类型和对应的数量写入表中
//     my_gems.insert("红宝石", 1);
//     my_gems.insert("蓝宝石", 2);
//     my_gems.insert("河边捡的误以为是宝石的破石头", 18);

//     println!("{:?}", my_gems); // {"河边捡的误以为是宝石的破石头": 18, "蓝宝石": 2, "红宝石": 1}
// }

// fn main() {
//     use std::collections::HashMap;

//     let teams_list = vec![
//         ("中国队".to_string(), 100),
//         ("美国队".to_string(), 10),
//         ("日本队".to_string(), 50),
//     ];

//     let teams_map: HashMap<_,_> = teams_list.into_iter().collect();

//     println!("{:?}",teams_map) // {"日本队": 50, "中国队": 100, "美国队": 10}
// }

// fn main() {
//     use std::collections::HashMap;

//     let name = String::from("Sunface");
//     let age = 18;

//     let mut handsome_boys = HashMap::new();
//     handsome_boys.insert(name, age);

//     println!("因为过于无耻，{}已经被从帅气男孩名单中除名", name); // ^^^^ value borrowed here after move
//     //    handsome_boys.insert(name.clone(), age);
//     println!("还有，他的真实年龄远远不止{}岁", age);
// }

// fn main() {
//     use std::collections::HashMap;

//     let name = String::from("Sunface");
//     let age = 18;

//     let mut handsome_boys = HashMap::new();
//     handsome_boys.insert(&name, age);

//     std::mem::drop(name); //  ^^^^ move out of `name` occurs here
//     println!("因为过于无耻，{:?}已经被除名", handsome_boys); //  ------------- borrow later used here
//     println!("还有，他的真实年龄远远不止{}岁", age);
// }

// fn main() {
//     use std::collections::HashMap;

//     let mut scores = HashMap::new();

//     scores.insert(String::from("Blue"), 10);
//     scores.insert(String::from("Yellow"), 50);

//     let team_name = String::from("Blue");
//     let score: Option<&i32> = scores.get(&team_name);
//     println!("{:?}", score); // Some(10)

//     let score2: i32 = scores.get(&team_name).copied().unwrap_or(0);
//     println!("{:?}", score2);  // 10
// }

// fn main() {
//     use std::collections::HashMap;

//     let mut scores = HashMap::new();

//     scores.insert("Blue", 10);

//     // 覆盖已有的值
//     let old = scores.insert("Blue", 20);
//     assert_eq!(old, Some(10));

//     // 查询新插入的值
//     let new = scores.get("Blue");
//     assert_eq!(new, Some(&20));

//     // 查询Yellow对应的值，若不存在则插入新值
//     let v = scores.entry("Yellow").or_insert(5);
//     assert_eq!(*v, 5); // 不存在，插入5

//     // 查询Yellow对应的值，若不存在则插入新值
//     let v = scores.entry("Yellow").or_insert(50);
//     assert_eq!(*v, 5); // 已经存在，因此50没有插入
// }

fn main() {
    use std::collections::HashMap;

    let text = "hello world wonderful world";

    let mut map = HashMap::new();
    // 根据空格来切分字符串(英文单词都是通过空格切分)
    for word in text.split_whitespace() {
        // 若之前没有插入过，则使用该词语作 Key，插入次数 0 作为 Value，若之前插入过则取出之前统计的该词语出现的次数，对其加一。
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map); // {"hello": 1, "wonderful": 1, "world": 2}
}
