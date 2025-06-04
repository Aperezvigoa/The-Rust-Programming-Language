use std::io;

struct Book {
    title: String,
    author: String,
    isbn: i32,
    available: bool,
}

impl Book {
    fn change_availability(self: &mut Self) {
        self.available = !self.available;
    }
    fn is_available(&self) -> bool {
        self.available
    }
}

struct User {
    name: String,
    id: i32,
    borrowed_books: Vec<i32>,
}

impl User {
    fn borrow_book(self: &mut Self, isbn: i32) {
        self.borrowed_books.push(isbn);
    }
    fn return_book(self: &mut Self, index: usize) {
        self.borrowed_books.remove(index);
    }
}

struct Library {
    books: Vec<Book>,
    users: Vec<User>,
}

impl Library {
    fn add_book(self: &mut Self, book: Book) {
        self.books.push(book);
    }

    fn add_user(self: &mut Self, user: User) {
        self.users.push(user);
    }
    fn show_books(self: &Self) {
        for b in &self.books {
            println!(
                "ISBN: {} - {} from {} - Available: {}",
                b.isbn, b.title, b.author, b.available
            );
        }
    }
    fn show_users(self: &Self) {
        for u in &self.users {
            println!("ID: {} - {}", u.id, u.name);
        }
    }
    fn filter_by_author(self: &Self, author: &String) {
        for b in &self.books {
            if b.author == *author {
                println!(
                    "ISBN: {} - {} from {} - Available: {}",
                    b.isbn, b.title, b.author, b.available
                );
            }
        }
    }
}

fn main() {
    let mut isbn_number: i32 = 0;
    let mut id_number: i32 = 0;

    let mut main_library = Library {
        books: Vec::new(),
        users: Vec::new(),
    };

    'program: loop {
        print_menu();
        let mut user_choice: String = String::new();
        overwrite_string(&mut user_choice);

        match user_choice.trim() {
            "1" => {
                let name: String = request_name();
                let id: i32 = generate_user_id(&mut id_number);
                let user = create_user(name, id);
                Library::add_user(&mut main_library, user);
            }
            "2" => {
                let title: String = request_title();
                let author: String = request_author();
                let isbn: i32 = generate_isbn(&mut isbn_number);
                let book = create_book(title, author, isbn);
                Library::add_book(&mut main_library, book);
            }
            "3" => {
                if main_library.users.len() > 0 && main_library.books.len() > 0 {
                    let (input_isbn, input_id) = request_user_and_book(&main_library);
                    let (book_index, book_exists) = get_book_index(&main_library, input_isbn);
                    let (user_index, user_exists) = get_user_index(&main_library, input_id);
                    if book_exists && user_exists && main_library.books[book_index].is_available() {
                        main_library.users[user_index].borrow_book(input_isbn);
                        main_library.books[book_index].change_availability();
                    } else {
                        println!("Cant finish the operation.");
                    }
                } else {
                    println!("No users os books.");
                }
            }
            "4" => {
                if main_library.users.len() > 0 && main_library.books.len() > 0 {
                    let (input_isbn, input_id) = request_user_and_book(&main_library);
                    let (book_index, book_exists) = get_book_index(&main_library, input_isbn);
                    let (user_index, user_exists) = get_user_index(&main_library, input_id);
                    if book_exists && user_exists && !main_library.books[book_index].is_available()
                    {
                        let borrowed_books_copy =
                            main_library.users[user_index].borrowed_books.clone();
                        let mut index_to_delete: usize = 0;

                        for i in borrowed_books_copy {
                            if i == input_isbn {
                                User::return_book(
                                    &mut main_library.users[user_index],
                                    index_to_delete,
                                );
                                Book::change_availability(&mut main_library.books[book_index]);
                                break;
                            }
                            index_to_delete += 1;
                        }
                    }
                } else {
                    println!("No users or books.");
                }
            }
            "5" => {
                let author_to_search = request_author();
                Library::filter_by_author(&main_library, &author_to_search);
            }
            "8" => {
                println!("Good bye!");
                break 'program;
            }
            _ => {
                println!("Invalid choice");
                continue 'program;
            }
        }
    }
}

fn print_menu() {
    println!();
    println!("1. Add user");
    println!("2. Add book");
    println!("3. Borrow book");
    println!("4. Return book");
    println!("5. Find by author");
    println!("6. Available books");
    println!("7. See user");
    println!("8. Exit");
    println!();
}

fn get_book_index(lib: &Library, isbn: i32) -> (usize, bool) {
    let mut index: usize = 0;
    for b in &lib.books {
        if isbn == b.isbn {
            return (index, true);
        }
        index += 1;
    }
    return (0, false);
}

fn get_user_index(lib: &Library, id: i32) -> (usize, bool) {
    let mut index: usize = 0;
    for u in &lib.users {
        if id == u.id {
            return (index, true);
        }
        index += 1;
    }
    return (0, false);
}

fn request_user_and_book(lib: &Library) -> (i32, i32) {
    println!("Select the ISBN:");
    Library::show_books(&lib);
    let mut isbn_input: String = String::new();
    overwrite_string(&mut isbn_input);

    println!("\nSelect the user ID:");
    Library::show_users(&lib);
    let mut id_input: String = String::new();
    overwrite_string(&mut id_input);

    let isbn: i32 = isbn_input.trim().parse().expect("Something went wrong");
    let id: i32 = id_input.trim().parse().expect("Something went wrong");
    (isbn, id)
}

fn create_user(name: String, id: i32) -> User {
    User {
        name,
        id,
        borrowed_books: Vec::new(),
    }
}

fn create_book(title: String, author: String, isbn: i32) -> Book {
    Book {
        title,
        author,
        isbn,
        available: true,
    }
}

fn generate_user_id(id: &mut i32) -> i32 {
    *id += 1;
    *id
}

fn request_name() -> String {
    println!("Please enter the user name:");
    let mut input: String = String::new();
    overwrite_string(&mut input);
    input.trim().to_string()
}

fn request_author() -> String {
    println!("Please introduce the author of the book");
    let mut input: String = String::new();
    overwrite_string(&mut input);
    input.trim().to_string()
}

fn request_title() -> String {
    println!("Please introduce the title of the book:");
    let mut input: String = String::new();
    overwrite_string(&mut input);
    input.trim().to_string()
}

fn generate_isbn(number: &mut i32) -> i32 {
    *number += 1;
    *number
}

fn overwrite_string(input: &mut String) {
    io::stdin()
        .read_line(input)
        .expect("Something went wrong...");
}
