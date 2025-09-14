struct Book {
    title: String,
    author: String,
    is_borrowed: bool,
}

fn add_book(books: &mut Vec<Book>) {
    let book = Book {
        title: "Oui".to_string(),
        author: "oui".to_string(),
        is_borrowed: false,
    };
    books.push(book);
}

fn show_books(books: &Vec<Book>) {
    for book in books {
        println!(
            "Titre: {}, Auteur: {}, Emprunt: {}",
            book.title, book.author, book.is_borrowed
        );
    }
}

fn borrow_book(title: &str, books: &mut Vec<Book>) {
    let book_index = books.iter().position(|book| book.title == title);
    // si book_index n'est pas nul
    if let Some(book_index) = book_index {
        books[book_index].is_borrowed = true;
    }
}

fn main() {
    let mut list_of_books: Vec<Book> = Vec::new();

    add_book(&mut list_of_books);

    show_books(&list_of_books);

    borrow_book("Oui", &mut list_of_books);

    show_books(&list_of_books);
}
