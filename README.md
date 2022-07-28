# rust

学习资料：  
Rust 程序设计语言：https://doc.rust-lang.org/book/  
中文版：https://kaisery.github.io/trpl-zh-cn/  

标准库文档：https://doc.rust-lang.org/std/prelude/index.html  

- [rust](#rust)
  - [1. Rust 的安装(Mac)](#1-rust-的安装mac)
  - [2. Cargo](#2-cargo)
    - [2.1. 使用 Cargo 创建项目](#21-使用-cargo-创建项目)
    - [2.2. 构建并运行 Cargo 项目](#22-构建并运行-cargo-项目)
    - [2.3. 发布（release）构建](#23-发布release构建)
  - [3. 基础语法](#3-基础语法)
    - [3.1. 变量](#31-变量)
    - [3.2. 常量](#32-常量)
    - [3.3. 隐藏](#33-隐藏)
  - [4. 数据类型](#4-数据类型)
    - [4.1. 标量类型](#41-标量类型)
      - [4.1.1. 整型](#411-整型)
      - [4.1.2. 浮点型](#412-浮点型)
    - [4.2. 数值运算](#42-数值运算)
    - [4.3. 布尔型](#43-布尔型)
    - [4.4. 字符类型](#44-字符类型)
    - [4.5. 复合类型](#45-复合类型)
      - [4.5.1. 元组类型](#451-元组类型)
      - [4.5.2. 数组类型](#452-数组类型)
  - [5. 函数](#5-函数)
    - [5.1. 参数](#51-参数)
    - [5.2. 语句和表达式](#52-语句和表达式)
    - [5.3. 函数的返回值](#53-函数的返回值)
  - [6. 控制流](#6-控制流)
    - [6.1. if 表达式](#61-if-表达式)
    - [6.2. 使用循环重复执行](#62-使用循环重复执行)
      - [6.2.1. 使用 loop 重复执行代码](#621-使用-loop-重复执行代码)
      - [6.2.2. while 条件循环](#622-while-条件循环)
      - [6.2.3. 使用 for 遍历集合](#623-使用-for-遍历集合)
  - [7. 所有权](#7-所有权)
    - [7.1. 引用与借用](#71-引用与借用)
    - [7.2. Slice 类型](#72-slice-类型)
  - [8. 结构体](#8-结构体)
    - [8.1. 方法](#81-方法)
  - [9. 枚举](#9-枚举)
  - [10. match 控制流](#10-match-控制流)

## 1. Rust 的安装(Mac)

```text
brew install rustup-init
```

然后执行

```text
rustup-init
```

## 2. Cargo

Cargo 是 Rust 的构建系统和包管理器。

### 2.1. 使用 Cargo 创建项目

```text
cargo new hello_cargo
```

进入 hello_cargo 目录并列出文件。将会看到 Cargo 生成了两个文件和一个目录：一个 Cargo.toml 文件，一个 src 目录，以及位于 src 目录中的 main.rs 文件。  

> 文件名: Cargo.toml: 使用 TOML (Tom's Obvious, Minimal Language) 格式，这是 Cargo 配置文件的格式。

```toml
[package]
name = "hello-rust"
version = "0.1.0"
edition = "2021"

[dependencies]
```

第一行，[package]，是一个片段（section）标题，表明下面的语句用来配置一个包。随着我们在这个文件增加更多的信息，还将增加其他片段（section）。  

最后一行，[dependencies]，是罗列项目依赖的片段的开始。在 Rust 中，代码包被称为 crates。  

### 2.2. 构建并运行 Cargo 项目

> cargo build: 会创建一个可执行文件 target/debug/hello_rust

首次运行 cargo build 时，也会使 Cargo 在项目根目录创建一个新文件：Cargo.lock。  

> cargo run 在一个命令中同时编译并运行生成的可执行文件，比起要记得运行 cargo build 之后再用可执行文件的完整路径来运行程序，使用 cargo run 可以实现完全相同的效果，而且要方便得多，所以大多数开发者会使用 cargo run。  

> cargo check 的命令。该命令快速检查代码确保其可以编译，但并不产生可执行文件.

### 2.3. 发布（release）构建

运行 cargo build --release 并使用 target/release 下的可执行文件进行测试。  

## 3. 基础语法

### 3.1. 变量

在变量名前添加 mut 来使其可变.

```rs
fn main() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
}
```

### 3.2. 常量

常量(constants) 是绑定到一个名称的不允许改变的值，不允许对常量使用 mut。

```rs
const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
```

### 3.3. 隐藏

可以用相同变量名称来隐藏一个变量，以及重复使用 let 关键字来多次隐藏.

```rs
fn main() {
  let x = 5;
  let x = x + 1;
  {
    let x = x * 2;
    println!("The value of x in the inner scope is: {x}");
  }
  println!("The value of x is: {x}");
}
```

mut 与隐藏的另一个区别是，当再次使用 let 时，实际上创建了一个新变量，可以改变值的类型，并且复用这个名字。  

## 4. 数据类型

### 4.1. 标量类型

标量（scalar）类型代表一个单独的值。Rust 有四种基本的标量类型：整型、浮点型、布尔类型和字符类型。

#### 4.1.1. 整型

没有小数部分的数字.  

| 长度	| 有符号 | 无符号 |
|  ----  | ----  |
| 8-bit |  i8 | u8 |
| 16-bit | i16 | u16 |
| 32-bit | i32 | u32 |
| 64-bit | i64 | u64 |
| 128-bit | i128 | u128 |
| arch | isize | usize |

有符号 和 无符号 代表数字能否为负值.  

每一个有符号的变体可以储存包含从 -2^(n-1)到 2^(n-1) - 1 在内的数字，这里 n 是变体使用的位数。所以 i8 可以储存从 -2^7 到 2^7 - 1 在内的数字，也就是从 -128 到 127。无符号的变体可以储存从 0 到 2^n - 1 的数字，所以 u8 可以储存从 0 到 2^8 - 1 的数字，也就是从 0 到 255。  

isize 和 usize 类型依赖运行程序的计算机架构：64 位架构上它们是 64 位的， 32 位架构上它们是 32 位的。  

| 数字字面值 | 例子 |
| Decimal (十进制) | 98_222 |
| Hex (十六进制) | 0xff |
| Octal (八进制) | 0o77 |
| Binary (二进制) | 0b1111_0000 |
| Byte (单字节字符)(仅限于u8) | b'A' |

如果拿不定主意，Rust 的默认类型通常是个不错的起点，数字类型默认是 i32。isize 或 usize 主要作为某些集合的索引。  

> 整型溢出

比方说有一个 u8 ，它可以存放从零到 255 的值。那么当你将其修改为 256 时会发生什么呢？这被称为 “整型溢出”（“integer overflow” ），这会导致以下两种行为之一的发生。当在 debug 模式编译时，Rust 检查这类问题并使程序 panic，这个术语被 Rust 用来表明程序因错误而退出。  

在 release 构建中，Rust 不检测溢出，相反会进行一种被称为二进制补码包装（two’s complement wrapping）的操作。简而言之，值 256 变成 0，值 257 变成 1，依此类推。依赖整型溢出被认为是一种错误，即便可能出现这种行为。如果你确实需要这种行为，标准库中有一个类型显式提供此功能，Wrapping。 为了显式地处理溢出的可能性，你可以使用标准库在原生数值类型上提供的以下方法:  

所有模式下都可以使用 wrapping_* 方法进行包装，如 wrapping_add  
如果 checked_* 方法出现溢出，则返回 None值  
用 overflowing_* 方法返回值和一个布尔值，表示是否出现溢出  
用 saturating_* 方法在值的最小值或最大值处进行饱和处理  

#### 4.1.2. 浮点型

rust 的浮点数类型是 f32 和 f64，分别占 32 位和 64 位。默认类型是 f64，因为在现代 CPU 中，它与 f32 速度几乎一样，不过精度更高。所有的浮点型都是有符号的。  

### 4.2. 数值运算

加法、减法、乘法、除法和取余。整数除法会向下舍入到最接近的整数。  

```rs
fn main() {
    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let floored = 2 / 3; // Results in 0

    // remainder
    let remainder = 43 % 5;
}
```

### 4.3. 布尔型

Rust 中的布尔类型有两个可能的值：true 和 false。Rust 中的布尔类型使用 bool 表示。  

```rs
fn main() {
    let t = true;
    let f: bool = false;
}
```

### 4.4. 字符类型

用单引号声明 char 字面量，使用双引号声明字符串字面量。

### 4.5. 复合类型

复合类型（Compound types）可以将多个值组合成一个类型。Rust 有两个原生的复合类型：元组（tuple）和数组（array）。

#### 4.5.1. 元组类型

元组是一个将多个其他类型的值组合进一个复合类型的主要方式。元组长度固定：一旦声明，其长度不会增大或缩小。  

```rs
fn main() {
  let tup: (i32, f64, u8) = (500, 6.4, 1);
}
```

可以使用模式匹配（pattern matching）来解构（destructure）元组值

```rs
fn main() {
  let tup = (500, 6.4, 1);
  let (x, y, z) = tup;
  println!("The value of y is: {y}");
}
```

可以使用点号（.）后跟值的索引来直接访问.

```rs
fn main() {
  let x: (i32, f64, u8) = (500, 6.4, 1);
  let five_hundred = x.0;
  let six_point_four = x.1;
  let one = x.2;
}
```

不带任何值的元组有个特殊的名称，叫做 单元（unit） 元组。这种值以及对应的类型都写作 ()，表示空值或空的返回类型。如果表达式不返回任何其他值，则会隐式返回单元值。

#### 4.5.2. 数组类型

与元组不同，数组中的每个元素的类型必须相同。Rust中的数组长度是固定的。  

数组并不如 vector 类型灵活。vector 类型是标准库提供的一个 允许 增长和缩小长度的类似数组的集合类型。当不确定是应该使用数组还是 vector 的时候，那么很可能应该使用 vector。  

```rs
let a: [i32; 5] = [1, 2, 3, 4, 5];
```

这里，i32 是每个元素的类型。分号之后，数字 5 表明该数组包含五个元素。

还可以通过在方括号中指定初始值加分号再加元素个数的方式来创建一个每个元素都为相同值的数组：

```rs
let a = [3; 5];
```

变量名为 a 的数组将包含 5 个元素，这些元素的值最初都将被设置为 3。这种写法与 let a = [3, 3, 3, 3, 3]; 效果相同，但更简洁。

> 访问数组元素

使用索引来访问数组的元素

```rs
fn main() {
  let a = [1, 2, 3, 4, 5];

  let first = a[0];
  let second = a[1];
}
```

## 5. 函数

Rust 代码中的函数和变量名使用 snake case 规范风格。在 snake case 中，所有字母都是小写并使用下划线分隔单词。

```rs
fn main() {
  println!("Hello, world!");
  another_function();
}

fn another_function() {
  println!("Another function.");
}
```

Rust 不关心函数定义所在的位置，只要函数被调用时出现在调用之处可见的作用域内就行。  

### 5.1. 参数

在函数签名中，必须 声明每个参数的类型。

```rs
fn main() {
  print_labeled_measurement(5, 'h');
}

fn print_labeled_measurement(value: i32, unit_label: char) {
  println!("The measurement is: {value}{unit_label}");
}
```

### 5.2. 语句和表达式

> 语句（Statements）是执行一些操作但不返回值的指令。  

let y = 6; 是一个语句。  

```rs
fn main() {
  let y = 6;
}
```

> 表达式（Expressions）计算并产生一个值。

函数调用是一个表达式。宏调用是一个表达式。用大括号创建的一个新的块作用域也是一个表达式.

```rs
fn main() {
  let y = {
    let x = 3;
    x + 1
  };

  println!("The value of y is: {y}");
}
```

表达式是一个代码块，它的值是 4。

```rs
{
  let x = 3;
  x + 1
}
```

### 5.3. 函数的返回值

要在箭头（->）后声明它的类型.  
函数的返回值等同于函数体最后一个表达式的值。使用 return 关键字和指定值，可从函数中提前返回；但大部分函数隐式的返回最后的表达式.

```rs
fn main() {
    let x = plus_one(5);

    println!("The value of x is: {x}");
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
```

运行代码会打印出 The value of x is: 6。但如果在包含 x + 1 的行尾加上一个分号，把它从表达式变成语句，将看到一个错误。

```rs
fn main() {
  let x = plus_one(5);

  println!("The value of x is: {x}");
}

fn plus_one(x: i32) -> i32 {
  x + 1;
}
```

```text
7 | fn plus_one(x: i32) -> i32 {
  |    --------            ^^^ expected `i32`, found `()`
  |    |
  |    implicitly returns `()` as its body has no tail or `return` expression
8 |     x + 1;
  |          - help: consider removing this semicolon

For more information about this error, try `rustc --explain E0308`.
error: could not compile `functions` due to previous error
```

“mismatched types”（类型不匹配），揭示了代码的核心问题。函数 plus_one 的定义说明它要返回一个 i32 类型的值，不过语句并不会返回值，使用单位类型 () 表示不返回值。因为不返回值与函数定义相矛盾，从而出现一个错误。在输出中，Rust 提供了一条信息，可能有助于纠正这个错误：它建议删除分号，这会修复这个错误。

## 6. 控制流

Rust 代码中最常见的用来控制执行流的结构是 if 表达式和循环。  

### 6.1. if 表达式

```rs
fn main() {
  let number = 3;

  if number < 5 {
    println!("condition was true");
  } else {
    println!("condition was false");
  }
}
```

> 在 let 语句中使用 if

```rs
fn main() {
    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {number}"); // 5
}
```

### 6.2. 使用循环重复执行

Rust 有三种循环：loop、while 和 for。

#### 6.2.1. 使用 loop 重复执行代码

可以使用 break 关键字来告诉程序何时停止循环。  
循环中的 continue 关键字告诉程序跳过这个循环迭代中的任何剩余代码，并转到下一个迭代。  

> 循环返回值

```rs
fn main() {
  let mut counter = 0;
  let result = loop {
      counter += 1;
      if counter == 10 {
          break counter * 2;
      }
  };
  println!("The result is {result}"); // 20
}
```

> 循环标签：在多个循环之间消除歧义

如果存在嵌套循环，break 和 continue 应用于此时最内层的循环。可以选择在一个循环上指定一个 循环标签（loop label），然后将标签与 break 或 continue 一起使用，使这些关键字应用于已标记的循环, 而不是最内层的循环。

```rs
fn main() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}"); // 2
}
```

#### 6.2.2. while 条件循环

```rs
fn main() {
  let mut number = 3;

  while number != 0 {
    println!("{number}!");

    number -= 1;
  }

  println!("LIFTOFF!!!");
}
```

#### 6.2.3. 使用 for 遍历集合

```rs
fn main() {
  let a = [10, 20, 30, 40, 50];

  for element in a {
      println!("the value is: {element}");
  }
}
```

## 7. 所有权

所有程序都必须管理其运行时使用计算机内存的方式。一些语言中具有垃圾回收机制，在程序运行时不断地寻找不再使用的内存；在另一些语言中，程序员必须亲自分配和释放内存。Rust 则选择了第三种方式：通过所有权系统管理内存，编译器在编译时会根据一系列的规则进行检查。如果违反了任何这些规则，程序都不能编译。在运行时，所有权系统的任何功能都不会减慢程序。

> 所有权规则

1. Rust 中的每一个值都有一个被称为其 所有者（owner）的变量。
2. 值在任一时刻有且只有一个所有者。
3. 当所有者（变量）离开作用域，这个值将被丢弃。

> 变量作用域

作用域（scope）是一个项（item）在程序中有效的范围。

> 内存与分配

内存在拥有它的变量离开作用域后就被自动释放。

1. 变量与数据交互的方式（一）：移动

```rs
fn main() {
    let s1 = String::from("hello");
    let s2 = s1;

    println!("{}, world!", s1);
}
```

为了确保内存安全，在 let s2 = s1 之后，Rust 认为 s1 不再有效，因此 Rust 不需要在 s1 离开作用域后清理任何东西。

```text
 --> src/main.rs:5:28
  |
2 |     let s1 = String::from("hello");
  |         -- move occurs because `s1` has type `String`, which does not implement the `Copy` trait
3 |     let s2 = s1;
  |              -- value moved here
4 | 
5 |     println!("{}, world!", s1);
  |                            ^^ value borrowed here after move

For more information about this error, try `rustc --explain E0382`.
error: could not compile `ownership` due to previous error
```

在其他语言中听说过术语 浅拷贝（shallow copy）和 深拷贝（deep copy），那么拷贝指针、长度和容量而不拷贝数据可能听起来像浅拷贝。不过因为 Rust 同时使第一个变量无效了，这个操作被称为 移动（move），而不是浅拷贝。  

Rust 永远也不会自动创建数据的 “深拷贝”。因此，任何 自动 的复制可以被认为对运行时性能影响较小。

2. 变量与数据交互的方式（二）：克隆

```rs
fn main() {
    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2);
}
```

3. 只在栈上的数据：拷贝

```rs
fn main() {
    let x = 5;
    let y = x;

    println!("x = {}, y = {}", x, y);
}
```

一个通用的规则，任何一组简单标量值的组合都可以实现 Copy，任何不需要分配内存或某种形式资源的类型都可以实现 Copy 。如下是一些 Copy 的类型：

- 所有整数类型，比如 u32。
- 布尔类型，bool，它的值是 true 和 false。
- 所有浮点数类型，比如 f64。
- 字符类型，char。
- 元组，当且仅当其包含的类型也都实现 Copy 的时候。比如，(i32, i32) 实现了 Copy，但 (i32, String) 就没有。

> 所有权与函数

```rs
fn main() {
    let s = String::from("hello");  // s 进入作用域

    takes_ownership(s);             // s 的值移动到函数里 ...
                                    // ... 所以到这里不再有效

    let x = 5;                      // x 进入作用域

    makes_copy(x);                  // x 应该移动函数里，
                                    // 但 i32 是 Copy 的，
                                    // 所以在后面可继续使用 x

} // 这里, x 先移出了作用域，然后是 s。但因为 s 的值已被移走，
  // 没有特殊之处

fn takes_ownership(some_string: String) { // some_string 进入作用域
    println!("{}", some_string);
} // 这里，some_string 移出作用域并调用 `drop` 方法。
  // 占用的内存被释放

fn makes_copy(some_integer: i32) { // some_integer 进入作用域
    println!("{}", some_integer);
} // 这里，some_integer 移出作用域。没有特殊之处
```

> 返回值与作用域

返回值也可以转移所有权。

```rs
fn main() {
    let s1 = gives_ownership();         // gives_ownership 将返回值
                                        // 转移给 s1

    let s2 = String::from("hello");     // s2 进入作用域

    let s3 = takes_and_gives_back(s2);  // s2 被移动到
                                        // takes_and_gives_back 中,
                                        // 它也将返回值移给 s3
} // 这里, s3 移出作用域并被丢弃。s2 也移出作用域，但已被移走，
  // 所以什么也不会发生。s1 离开作用域并被丢弃

fn gives_ownership() -> String {             // gives_ownership 会将
                                             // 返回值移动给
                                             // 调用它的函数

    let some_string = String::from("yours"); // some_string 进入作用域.

    some_string                              // 返回 some_string 
                                             // 并移出给调用的函数
                                             // 
}

// takes_and_gives_back 将传入字符串并返回该值
fn takes_and_gives_back(a_string: String) -> String { // a_string 进入作用域

    a_string  // 返回 a_string 并移出给调用的函数
}
```

### 7.1. 引用与借用

> 引用

引用（reference）像一个指针，因为它是一个地址，可以由此访问储存于该地址的属于其他变量的数据。与指针不同，引用确保指向某个特定类型的有效值。

```rs
fn main() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}
```

& 符号就是 引用，它们允许你使用值但不获取其所有权。

![引用](/images/1.svg)

&s1 语法让我们创建一个 指向 值 s1 的引用，但是并不拥有它。因为并不拥有这个值，所以当引用停止使用时，它所指向的值也不会被丢弃。

```rs
let s1 = String::from("hello");
let len = calculate_length(&s1);
```

> 借用

将创建一个引用的行为称为 借用（borrowing）。

```rs
fn main() {
    let s = String::from("hello");

    change(&s);
}

fn change(some_string: &String) {
    some_string.push_str(", world");
}
```

不允许修改引用的值。

```text
 --> src/main.rs:8:5
  |
7 | fn change(some_string: &String) {
  |                        ------- help: consider changing this to be a mutable reference: `&mut String`
8 |     some_string.push_str(", world");
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `some_string` is a `&` reference, so the data it refers to cannot be borrowed as mutable

For more information about this error, try `rustc --explain E0596`.
error: could not compile `ownership` due to previous error
```

> 可变引用

允许我们修改一个借用的值，这就是 可变引用（mutable reference）：

修复上述代码的错误: 

```rs
fn main() {
  let mut s = String::from("hello");
  change(&mut s);
}

fn change(some_string: &mut String) {
  some_string.push_str(", world");
}
```

将 s 改为 mut。然后在调用 change 函数的地方创建一个可变引用 &mut s，并更新函数签名以接受一个可变引用 some_string: &mut String。这就非常清楚地表明，change 函数将改变它所借用的值。

可变引用有一个很大的限制：在同一时间只能有一个对某一特定数据的可变引用。这些尝试创建两个 s 的可变引用的代码会失败：

```rs
fn main() {
    let mut s = String::from("hello");

    let r1 = &mut s;
    let r2 = &mut s;

    println!("{}, {}", r1, r2);
}
```

报错：

```text
4 |     let r1 = &mut s;
  |              ------ first mutable borrow occurs here
5 |     let r2 = &mut s;
  |              ^^^^^^ second mutable borrow occurs here
6 | 
7 |     println!("{}, {}", r1, r2);
  |                        -- first borrow later used here

For more information about this error, try `rustc --explain E0499`.
error: could not compile `ownership` due to previous error
```

可以使用大括号来创建一个新的作用域，以允许拥有多个可变引用，只是不能 同时 拥有：

```rs
fn main() {
    let mut s = String::from("hello");
    {
        let r1 = &mut s;
    } // r1 在这里离开了作用域，所以我们完全可以创建一个新的引用
    let r2 = &mut s;
}
```

不能在拥有不可变引用的同时拥有可变引用。不可变引用的用户可不希望在他们的眼皮底下值就被意外的改变了！然而，多个不可变引用是可以的，因为没有哪个只能读取数据的人有能力影响其他人读取到的数据。

```rs
fn main() {
  let mut s = String::from("hello");

  let r1 = &s; // 没问题
  let r2 = &s; // 没问题
  let r3 = &mut s; // 大问题

  println!("{}, {}, and {}", r1, r2, r3);
}
```

一个引用的作用域从声明的地方开始一直持续到最后一次使用为止。例如，因为最后一次使用不可变引用（println!)，发生在声明可变引用之前，所以如下代码是可以编译的：

```rs
fn main() {
  let mut s = String::from("hello");

  let r1 = &s; // 没问题
  let r2 = &s; // 没问题
  println!("{} and {}", r1, r2);
  // 此位置之后 r1 和 r2 不再使用

  let r3 = &mut s; // 没问题
  println!("{}", r3);
}
```

> 悬垂引用（Dangling References）

悬垂指针是其指向的内存可能已经被分配给其它持有者。相比之下，在 Rust 中编译器确保引用永远也不会变成悬垂状态：当你拥有一些数据的引用，编译器确保数据不会在其引用之前离开作用域。  

```rs
fn main() {
    let reference_to_nothing = dangle();
}

fn dangle() -> &String {
    let s = String::from("hello");

    &s
}
```

错误：

```text
 --> src/main.rs:5:16
  |
5 | fn dangle() -> &String {
  |                ^ expected named lifetime parameter
  |
  = help: this function's return type contains a borrowed value, but there is no value for it to be borrowed from
help: consider using the `'static` lifetime
  |
5 | fn dangle() -> &'static String {
  |                ~~~~~~~~

For more information about this error, try `rustc --explain E0106`.
error: could not compile `ownership` due to previous error
```

解决方法是直接返回 String：

```rs
fn no_dangle() -> String {
    let s = String::from("hello");

    s
}
```

### 7.2. Slice 类型

字符串 slice（string slice）是 String 中一部分值的引用

```rs
let s = String::from("hello world");
let hello = &s[0..5];
let world = &s[6..11];
```

使用一个由中括号中的 [starting_index..ending_index] 指定的 range 创建一个 slice，其中 starting_index 是 slice 的第一个位置，ending_index 则是 slice 最后一个位置的后一个值。在其内部，slice 的数据结构存储了 slice 的开始位置和长度，长度对应于 ending_index 减去 starting_index 的值。所以对于 let world = &s[6..11]; 的情况，world 将是一个包含指向 s 索引 6 的指针和长度值 5 的 slice。

对于 Rust 的 .. range 语法，如果想要从索引 0 开始，可以不写两个点号之前的值。如下两个语句是相同的：

```rs
let s = String::from("hello");

let slice = &s[0..2];
let slice = &s[..2];
```

如果 slice 包含 String 的最后一个字节，也可以舍弃尾部的数字。

```rs
let s = String::from("hello");

let len = s.len();

let slice = &s[3..len];
let slice = &s[3..];
```

同时舍弃这两个值来获取整个字符串的 slice。

```rs
let slice = &s[0..len];
let slice = &s[..];
```

定义一个获取字符串 slice 而不是 String 引用的函数使得我们的 API 更加通用并且不会丢失任何功能：

```rs
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn main() {
  let my_string = String::from("hello world");

  // `first_word` 适用于 `String`（的 slice），整体或全部
  let word = first_word(&my_string[0..6]);
  let word = first_word(&my_string[..]);
  // `first_word` 也适用于 `String` 的引用，
  // 这等价于整个 `String` 的 slice
  let word = first_word(&my_string);

  let my_string_literal = "hello world";

  // `first_word` 适用于字符串字面值，整体或全部
  let word = first_word(&my_string_literal[0..6]);
  let word = first_word(&my_string_literal[..]);

  // 因为字符串字面值已经 **是** 字符串 slice 了，
  // 这也是适用的，无需 slice 语法！
  let word = first_word(my_string_literal);
}
```

## 8. 结构体

结构体需要命名各部分数据以便能清楚的表明其值的意义。由于有了这些名字，结构体比元组更灵活：不需要依赖顺序来指定或访问实例中的值。

存储用户账号信息的结构体：

```rs
struct User {
  active: bool,
  username: String,
  email: String,
  sign_in_count: u64,
}
```

使用: 通过为每个字段指定具体值来创建这个结构体的实例。  

创建一个实例需要以结构体的名字开头，接着在大括号中使用 key: value 键-值对的形式提供字段.  

声明一个特定的用户：

```rs
struct User {
  active: bool,
  username: String,
  email: String,
  sign_in_count: u64,
}

fn main() {
  let user1 = User {
    email: String::from("someone@example.com"),
    username: String::from("someusername123"),
    active: true,
    sign_in_count: 1,
  };
}
```

为了从结构体中获取某个特定的值，可以使用点号。想要用户的邮箱地址，可以用 user1.email。  
要更改结构体中的值，如果结构体的实例是可变的，可以使用点号并为对应的字段赋值。  

```rs
fn main() {
  let mut user1 = User {
    email: String::from("someone@example.com"),
    username: String::from("someusername123"),
    active: true,
    sign_in_count: 1,
  };

  user1.email = String::from("anotheremail@example.com");
}
```

build_user 函数，它返回一个带有给定的 email 和用户名的 User 结构体实例。

```rs
fn build_user(email: String, username: String) -> User {
  User {
    email: email,
    username: username,
    active: true,
    sign_in_count: 1,
  }
}

// 简洁写法
fn build_user(email: String, username: String) -> User {
  User {
    email,
    username,
    active: true,
    sign_in_count: 1,
  }
}
```

.. 语法指定了剩余未显式设置值的字段应有与给定实例对应字段相同的值。

```rs
let user1 = User {
  email: String::from("someone@example.com"),
  username: String::from("someusername123"),
  active: true,
  sign_in_count: 1,
};

let user2 = User {
  active: user1.active,
  username: user1.username,
  email: String::from("another@example.com"),
  sign_in_count: user1.sign_in_count,
};

// ..语法
let user2 = User {
  email: String::from("another@example.com"),
  ..user1
};
```

> 定义元组结构体

以 struct 关键字和结构体名开头并后跟元组中的类型.  

```rs
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
  let black = Color(0, 0, 0);
  let origin = Point(0, 0, 0);
}
```

> 没有任何字段的类单元结构体

被称为 类单元结构体（unit-like structs）因为它们类似于 ()，即“元组类型”一节中提到的 unit 类型。

```rs
struct AlwaysEqual;

fn main() {
  let subject = AlwaysEqual;
}
```

### 8.1. 方法

方法（method）与函数类似：使用 fn 关键字和名称声明，可以拥有参数和返回值，同时包含在某处调用该方法时会执行的代码。不过方法与函数是不同的，因为它们在结构体的上下文中被定义（或者是枚举或 trait 对象的上下文），并且第一个参数总是 self，它代表调用该方法的结构体实例。

在 Rectangle 结构体上定义 area 方法

```rs
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}
```

在 area 的签名中，使用 &self 来替代 rectangle: &Rectangle，&self 实际上是 self: &Self 的缩写。  

self 前面使用 & 来表示这个方法借用了 Self 实例.  

## 9. 枚举

```rs
fn main() {
  enum IpAddrKind {
    V4,
    V6,
  }

  struct IpAddr {
    kind: IpAddrKind,
    address: String,
  }

  let home = IpAddr {
    kind: IpAddrKind::V4,
    address: String::from("127.0.0.1"),
  };

  let loopback = IpAddr {
    kind: IpAddrKind::V6,
    address: String::from("::1"),
  };
}
```

一个 Message 枚举，其每个成员都存储了不同数量和类型的值

```rs
enum Message {
  Quit,
  Move { x: i32, y: i32 },
  Write(String),
  ChangeColor(i32, i32, i32),
}
```

- Quit 没有关联任何数据。
- Move 类似结构体包含命名字段。
- Write 包含单独一个 String。
- ChangeColor 包含三个 i32。

## 10. match 控制流

将一个值与一系列的模式相比较，并根据相匹配的模式执行相应代码。模式可由字面值、变量、通配符和许多其他内容构成.  

```rs
enum Coin {
  Penny,
  Nickel,
  Dime,
  Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
  match coin {
    Coin::Penny => {
      println!("Lucky penny!");
      1
    }
    Coin::Nickel => 5,
    Coin::Dime => 10,
    Coin::Quarter => 25,
  }
}
```

## 11. if let 控制流

if let 语法获取通过等号分隔的一个模式和一个表达式。它的工作方式与 match 相同，这里的表达式对应 match 而模式则对应第一个分支

```rs
let config_max = Some(3u8);
match config_max {
  Some(max) => println!("The maximum is configured to be {}", max),
  _ => (),
}

// 功能同上
let config_max = Some(3u8);
if let Some(max) = config_max {
   println!("The maximum is configured to be {}", max);
}
```

## 12. 模块管理

Rust 有许多功能可以让你管理代码的组织，包括哪些内容可以被公开，哪些内容作为私有部分，以及程序每个作用域中的名字。这些功能被称为 “模块系统（the module system）”，包括：

- 包（Packages）： Cargo 的一个功能，它允许你构建、测试和分享 crate。
- Crates ：一个模块的树形结构，它形成了库或二进制项目。
- 模块（Modules）和 use： 允许你控制作用域和路径的私有性。
- 路径（path）：一个命名例如结构体、函数或模块等项的方式

### 12.1. 包和 Crate

crate 是一个二进制项或者库。crate root 是一个源文件，Rust 编译器以它为起始点，并构成你的 crate 的根模块.  
包（package） 是提供一系列功能的一个或者多个 crate。一个包会包含有一个 Cargo.toml 文件，阐述如何去构建这些 crate。  
包中可以包含至多一个库 crate(library crate)。可以包含任意多个二进制 crate(binary crate)，但是必须至少包含一个 crate（无论是库的还是二进制的）。  

cargo new命令

```text
$ cargo new my-project
     Created binary (application) `my-project` package
$ ls my-project
Cargo.toml
src
$ ls my-project/src
main.rs
```

### 12.2 定义模块来控制作用域与私有性

一个包含了其他内置了函数的模块的 front_of_house 模块

```rs
mod front_of_house {
    mod hosting {
        fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}
```

模块树的结构。

```text
crate
 └── front_of_house
     ├── hosting
     │   ├── add_to_waitlist
     │   └── seat_at_table
     └── serving
         ├── take_order
         ├── serve_order
         └── take_payment
```

路径有两种形式：

- 绝对路径（absolute path）从 crate 根开始，以 crate 名或者字面值 crate 开头。
- 相对路径（relative path）从当前模块开始，以 self、super 或当前模块的标识符开头。

以下代码会编译失败，暂时忽略

```rs
mod front_of_house {
    mod hosting {
        fn add_to_waitlist() {}
    }
}

pub fn eat_at_restaurant() {
    // 绝对路径
    crate::front_of_house::hosting::add_to_waitlist();

    // 相对路径
    front_of_house::hosting::add_to_waitlist();
}
```

> 使用 pub 关键字暴露路径

```rs
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

pub fn eat_at_restaurant() {
    // 绝对路径
    crate::front_of_house::hosting::add_to_waitlist();

    // 相对路径
    front_of_house::hosting::add_to_waitlist();
}
```

> 使用 super 起始的相对路径

使用 super 开头来构建从父模块开始的相对路径。这么做类似于文件系统中以 .. 开头的语法。

```rs
fn serve_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::serve_order();
    }

    fn cook_order() {}
}
```

> 使用 use 关键字将路径引入作用域

在作用域中增加 use 和路径类似于在文件系统中创建软连接（符号连接，symbolic link）。通过在 crate 根增加 use crate::front_of_house::hosting，现在 hosting 在作用域中就是有效的名称了，如同 hosting 模块被定义于 crate 根一样。通过 use 引入作用域的路径也会检查私有性，同其它路径一样。

```rs
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}
```

> 使用 as 关键字提供新的名称

用 as 指定一个新的本地名称或者别名

```rs
use std::fmt::Result;
use std::io::Result as IoResult;

fn function1() -> Result {
    // --snip--
}

fn function2() -> IoResult<()> {
    // --snip--
}
```






































