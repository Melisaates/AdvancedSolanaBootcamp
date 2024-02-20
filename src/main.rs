 #[derive(Debug)]

    struct BookInfo{
        title : String,
        author :  String,
        page_count : String,
    }

#[derive(Debug)]

    struct MagazineInfo{
        title : String,
        issue : String,
        topic : String,
    }


    enum Publication{
        Book(BookInfo),
        Magazine(MagazineInfo),
    }


fn printer(publications: &Vec<Publication>){

      for item in publications{
        match item
        {
            Publication::Book(book_info) => {
                println!("Book is {:#?}",book_info);
               /* println!("Kitap : {}", book_info.title);
                println!("Yazar : {}", book_info.author);
                println!("Sayfa : {}", book_info.page_count);*/
            }
            Publication::Magazine(magazine_info) => {
                println!("Magazine is {:#?}",magazine_info);
              /*println!("Dergi : {}", magazine_info.title);
                println!("Sayı : {}", magazine_info.issue);
                println!("Konu : {}", magazine_info.topic);*/
            }

        }
    }
   }


fn main(){

    let my_magazine =Publication::Magazine( MagazineInfo {
        title: String::from("Rust Programming Monthly"),
        issue: String::from("Issue 42"),
        topic: String::from("Programming Languages"),
    });



    let my_book = Publication::Book(BookInfo {
        title: String::from("The Rust Programming Language"),
        author: String::from("Steve Klabnik, Carol Nichols"),
        page_count: String::from("486"),
    });
    

    let mut examples : Vec<Publication> = Vec::new();

    examples.push(my_book);
    examples.push(my_magazine);


   printer(&examples);
   
}
