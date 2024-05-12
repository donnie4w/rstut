// 从chrono库中仅使用NaiveDate模块
use chrono::NaiveDate;
// // 由于示例中未直接使用RefCell，此行注释掉了
// use std::cell::RefCell;
// 引入Rc，用于实现引用计数的智能指针
use std::rc::Rc;

// 定义书籍结构体，包含书名和共享的作者实例
#[derive(Debug)] // 使得结构体可以使用 {:?} 格式化为调试信息
struct Book {
    title: String,      // 书名，类型为String
    author: Rc<Author>, // 使用Rc智能指针共享作者实例
}

// 定义作者结构体，包含姓名和出生年份
#[derive(Debug)] // 同样使得结构体支持调试格式化
struct Author {
    name: String,    // 作者的名字
    birth_year: i32, // 作者的出生年份
}

// 定义借阅记录结构体，记录借阅的书籍和日期
#[derive(Debug)] // 支持调试输出
struct BorrowRecord {
    book: Rc<Book>,                 // 借阅的书籍，通过Rc共享
    borrowed_date: NaiveDate,       // 借书日期，使用chrono库的NaiveDate类型
    return_date: Option<NaiveDate>, // 还书日期，可选，使用Option来表示可能无值
}

// 定义图书馆结构体，包含书籍列表和借阅记录列表
struct Library {
    books: Vec<Rc<Book>>,              // 使用Rc共享的书籍列表
    borrow_records: Vec<BorrowRecord>, // 借阅记录列表
}

impl Library {
    // 构造方法，创建一个新的图书馆实例
    fn new() -> Self {
        Library {
            books: Vec::new(),          // 初始化空的书籍列表
            borrow_records: Vec::new(), // 初始化空的借阅记录列表
        }
    }

    // 添加书籍到图书馆，传入书名和作者实例
    fn add_book(&mut self, title: String, author: Rc<Author>) {
        let book = Rc::new(Book { title, author }); // 创建书籍实例，并用Rc包装以便共享
        self.books.push(book); // 将书籍添加到书籍列表
    }

    // 借阅书籍，根据书名和借书日期字符串进行操作
    fn borrow_book(&mut self, book_title: &str, borrow_date_str: &str) {
        if let Some(book) = self.books.iter().find(|book| book.title == book_title) {
            // 查找指定标题的书籍
            let borrowed_date = NaiveDate::parse_from_str(borrow_date_str, "%Y-%m-%d")
                .expect("Failed to parse date"); // 解析借书日期字符串为NaiveDate
            let record = BorrowRecord {
                book: Rc::clone(book), // 克隆书籍的Rc引用，不增加原始引用的计数
                borrowed_date,         // 使用解析得到的借书日期
                return_date: None,     // 初始设置还书日期为None
            };
            self.borrow_records.push(record); // 将借阅记录添加到记录列表
            println!("Book '{}' borrowed on {}", book_title, borrow_date_str); // 打印借书信息
        } else {
            println!("Book not found."); // 如果没有找到书，则打印提示信息
        }
    }

    // 归还书籍，根据书名和还书日期字符串更新记录
    fn return_book(&mut self, book_title: &str, return_date_str: &str) {
        if let Some(record) = self
            .borrow_records
            .iter_mut()
            .find(|record| record.book.title == book_title && record.return_date.is_none())
        {
            // 查找对应书名且尚未归还的记录
            let return_date = NaiveDate::parse_from_str(return_date_str, "%Y-%m-%d")
                .expect("Failed to parse date"); // 解析还书日期字符串为NaiveDate
            record.return_date = Some(return_date); // 更新记录的还书日期
            println!("Book '{}' returned on {}", book_title, return_date_str); // 打印还书信息
        } else {
            println!("Book not found or already returned."); // 如果书未找到或已归还，则打印提示信息
        }
    }

    // 打印图书馆的详细信息，包括书籍列表和借阅记录
    fn print_library_info(&self) {
        println!("Books:");
        for book in &self.books {
            println!(
                "Title: {}, Author: {} (Born: {})",
                book.title, book.author.name, book.author.birth_year
            ); // 打印书籍信息，包括作者的出生年份
        }
        println!("Borrow Records:");
        for record in &self.borrow_records {
            let return_date_str = match record.return_date {
                Some(date) => date.format("%Y-%m-%d").to_string(), // 格式化还书日期
                None => "Not returned yet".to_string(),            // 若未归还，则显示此信息
            };
            println!(
                "Title: {}, Borrowed on: {}, Returned on: {}",
                record.book.title,
                record.borrowed_date.format("%Y-%m-%d"), // 格式化借书日期
                return_date_str
            ); // 打印借阅记录详情
        }
    }
}

#[test]
fn testmain() {
    // 创建作者实例，并使用Rc包装以便共享
    let author = Rc::new(Author {
        name: "Steve Klabnik".into(), // 作者名字
        birth_year: 1980,             // 作者出生年份
    });

    // 创建图书馆实例
    let mut library = Library::new();
    // 添加书籍到图书馆
    library.add_book("Rust Programming".into(), Rc::clone(&author));

    // 模拟借阅书籍
    library.borrow_book("Rust Programming", "2023-04-01");
    // 模拟归还书籍
    library.return_book("Rust Programming", "2023-04-30");
    // 打印图书馆的全部信息
    library.print_library_info();
}