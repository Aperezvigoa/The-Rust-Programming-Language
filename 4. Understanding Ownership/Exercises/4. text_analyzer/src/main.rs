use std::{fs, io};

struct Document {
    content: String,
    total_words: i32,
}

fn main() {
    let doc = create_document("./file_to_read.txt");
    println!("Total words: {}", doc.total_words);

    let mut lines_data: Vec<&str> = Vec::new();
    split_content_by_lines(&doc.content, &mut lines_data);

    loop {
        print_menu();
        let mut choice: String = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Something went wrong.");

        match choice.trim() {
            "1" => {
                println!("{}", doc.content);
            }
            "2" => {
                let max = lines_data.len();
                println!("Select a paragraph (0..{})", max - 1);
                let mut option = String::new();
                io::stdin()
                    .read_line(&mut option)
                    .expect("Something went wrong...");
                let option: usize = option.trim().parse().expect("Something failed.");
                if option < max {
                    extract_paragraph(&lines_data, option);
                } else {
                    println!("Invalid choice!");
                }
            }
            "3" => {
                document_summary(&doc, &lines_data);
            }
            "4" => {
                break;
            }
            _ => {
                println!("Invalid choice...");
                continue;
            }
        }
    }
}

fn print_menu() {
    println!();
    println!("1. See Document");
    println!("2. Select Paragraph");
    println!("3. Document Summary");
    println!("4. Exit");
    println!();
}

fn document_summary(doc: &Document, lines_data: &Vec<&str>) {
    println!("Document Summary");
    println!("Total Words: {}", doc.total_words);
    let max_len = get_max_len_word(&lines_data);
    println!("Most length word: {max_len}");
    println!("First paragraph: {}", lines_data[0]);
    println!("Total paragraphs: {}", lines_data.len());
}

fn extract_paragraph(data: &Vec<&str>, number: usize) {
    println!("{}", data[number]);
}

fn get_max_len_word(lines: &Vec<&str>) -> String {
    let mut max_len_word = String::new();

    for line in lines {
        let bytes = line.as_bytes();
        let mut first_char = 0;
        for (index, &value) in bytes.iter().enumerate() {
            if value == b' ' {
                let temp_word = line[first_char..index].to_string();
                if temp_word.len() > max_len_word.len() {
                    max_len_word = temp_word;
                }
                first_char = index + 1;
            }
        }
    }
    max_len_word
}

fn split_content_by_lines<'a>(content: &'a String, data: &mut Vec<&'a str>) {
    data.clear();
    let bytes = content.as_bytes();

    let mut first_char = 0;

    for (index, &value) in bytes.iter().enumerate() {
        if value == b'\n' {
            let line: &str = &content[first_char..index];
            first_char = index + 1;
            if !line.is_empty() {
                data.push(line);
            }
        }
    }
}

fn create_document(path: &str) -> Document {
    let readed_content = read_content(path);
    let count = word_count(&readed_content);
    let new_document = Document {
        content: readed_content,
        total_words: count,
    };
    new_document
}

fn word_count(content: &String) -> i32 {
    let bytes = content.as_bytes();
    let mut word_count: i32 = 0;
    for byte in bytes {
        if *byte == b' ' {
            word_count += 1;
        }
    }
    return word_count;
}

fn read_content(path: &str) -> String {
    let content: String = fs::read_to_string(path).expect("Something went wrong");
    content
}
