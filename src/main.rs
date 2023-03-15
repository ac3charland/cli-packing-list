use chrono::Local;
use dialoguer::{Input, Select};
use std::cmp::min;

fn main() {
    let yes_no = ["Yes", "No"];
    let name: String = Input::new()
        .with_prompt("Enter your destination")
        .interact()
        .unwrap();
    let duration: u32 = Input::new()
        .with_prompt("How many days will you be gone?")
        .interact()
        .unwrap();
    let short_socks = min(duration / 2 + duration % 2 + 1, 7);
    let working = Select::new()
        .with_prompt("Will you be working on this trip?")
        .items(&yes_no)
        .default(1)
        .interact()
        .unwrap();
    let work_laptop = if working == 0 {
        "\n[ ] Work Laptop"
    } else {
        ""
    };
    let output = format!(
        "

{} Packing List - {}

[ ] {} pairs underwear
[ ] __ pairs short socks
[ ] __ pairs long socks
[ ] __ shirts
[ ] Swimsuit
[ ] Toiletries
[ ] Laptop/sleeve{}
[ ] Cell phone
[ ] Wallet & ID
[ ] Walking shoes
[ ] Wedding ring
[ ] Backpack
[ ] Suitcase
[ ] Water plants
[ ] Take out trash
Option chosen: {}

        ",
        name,
        Local::now().format("%b %e %Y"),
        duration,
        work_laptop,
        yes_no[working]
    );
    println!("{}", output);
}
