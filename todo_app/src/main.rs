/*
- Bu odevde vector kullarak basit bir todo app yapacagiz.
- Girilen itemlar todo listesine eklenecek.
- Listenin icerigi yazdirilabilecek.
- Listeye yeni item eklenecebilecek.
- Listeden item silinebilecek.
- Listedeki bir Item in completed kismi true yapilinca o item otomatik olarak listeden cikarilacak.
- Ikinci bir completed_tasks vectoru olacak.
- Yapilan task lar oraya eklenecek.



Menu:
1- Create List
Add item
Delete item
Complete item
See todo list
See completed items
Exit
*/
pub mod Materials;
use crate::Materials::Funcs;
use crate::Materials::Structs::Item;
fn main() {
    let mut todo:Vec<Item> = Vec::new();
    let mut comp:Vec<Item> = Vec::new();
    println!("       _.: atahanyild's ToDo app :._");
    loop {
        println!(
        
        "         _________________________
        |You can choose from list:|
        |1- Add item              |
        |2- Delete item           |
        |3- Complete item         |
        |4- See todo list         | 
        |5- See completed items   |
        |6- Exit                  |
        |_________________________|"
        
        );
        let input =Funcs::input("");
        
        match input.as_str().trim() {
            "1" => {
                Funcs::add_item(&mut todo);
            }
            "2" => {
                Funcs::delete_item(&mut todo);
            },
            "3" =>Funcs::complete_item(&mut todo, &mut comp),
            "4" => Funcs::print_list(&mut todo),
            "5" =>Funcs::print_list(&mut comp),
            "6" => break,
            _ => println!("Please only choose from list"),
        }
    }
}


