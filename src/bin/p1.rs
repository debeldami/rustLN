// Project 1: Interactive bill manager
//
// Summary:
//   Create a command line bills/expenses manager that runs
//   interactively. This mini project brings together many of
//   the concepts learn thus far into a single application.
//
//   The user stories/requirements are split into stages.
//   Fully implement each stage as a complete working program
//   before making changes for the next stage. Leverage the
//   compiler by using `cargo check --bin p1` when changing
//   between stages to help identify adjustments that need
//   to be made.
//
// User stories:
// * Stage 1:
//   - I want to add bills, including the name and amount owed.
//   - I want to view existing bills.
// * Stage 2:
//   - I want to remove bills.
// * Stage 3:
//   - I want to edit existing bills.
//   - I want to go back if I change my mind.
//
// Tips:
// * Use the loop keyword to create an interactive menu.
// * Each menu choice should be it's own function, so you can work on the
//   the functionality for that menu in isolation.
// * A vector is the easiest way to store the bills at stage 1, but a
//   hashmap will be easier to work with at stages 2 and 3.

use std::collections::HashMap;
use std::io;

#[derive(Debug)]
struct Bill {
    name: String,
    amount: f64,
}

pub struct Bills {
    inner: HashMap<String, Bill>,
}

impl Bills {
    fn new() -> Self {
        Self {
            inner: HashMap::new(),
        }
    }

    fn add(&mut self, bill: Bill) {
        self.inner.insert(bill.name.to_string(), bill);
    }

    fn remove(&mut self, name: &str) -> bool {
        self.inner.remove(name).is_some()
    }

    fn get_all(&self) -> Vec<&Bill> {
        self.inner.values().collect()
    }

    fn update(&mut self, name: &str, amount: f64) -> bool {
        match self.inner.get_mut(name) {
            Some(bill) => {
                bill.amount = amount;
                true
            }
            None => false,
        }
    }
}

fn get_input() -> Option<String> {
    let mut buffer = String::new();

    while io::stdin().read_line(&mut buffer).is_err() {}

    let input = buffer.trim().to_owned();

    if &input == "" {
        None
    } else {
        Some(input)
    }
}

fn get_bill_amount() -> Option<f64> {
    println!("Amount: ");
    loop {
        let input = match get_input() {
            Some(input) => input,
            None => return None,
        };

        if &input == "" {
            return None;
        }

        let parsed_input: Result<f64, _> = input.parse();

        match parsed_input {
            Ok(amount) => return Some(amount),
            Err(_) => println!("please enter a number"),
        }
    }
}

mod menu {
    use crate::{get_bill_amount, get_input, Bill, Bills};

    pub fn add_bill(bills: &mut Bills) {
        println!("Bill name: ");
        let name = match get_input() {
            Some(input) => input,
            None => return,
        };

        let amount = match get_bill_amount() {
            Some(amoumt) => amoumt,
            None => return,
        };

        let bill = Bill { name, amount };

        bills.add(bill);
    }

    pub fn remove_bill(bills: &mut Bills) {
        view_all(bills);

        println!("Enter bill name to remove: ");

        let name = match get_input() {
            Some(name) => name,
            None => return,
        };

        if bills.remove(name.as_str()) {
            println!("Bill successfully removed");
        } else {
            println!("Bill not found");
        }
    }

    pub fn update_bill(bills: &mut Bills) {
        view_all(bills);
        println!("Update bill: ");

        let name = match get_input() {
            Some(input) => input,
            None => return,
        };

        let amount = match get_bill_amount() {
            Some(amoumt) => amoumt,
            None => return,
        };

        if bills.update(name.as_str(), amount) {
            println!("updated");
        } else {
            println!("bill not found");
        }
    }

    pub fn view_all(bills: &Bills) {
        for bill in bills.get_all() {
            println!("{:?}", bill);
        }
    }
}

enum MainMenu {
    AddMenu,
    ViewMenu,
    RemoveMenu,
    UpdateMenu,
}

impl MainMenu {
    fn add_str(input: &str) -> Option<MainMenu> {
        match input {
            "1" => Some(Self::AddMenu),
            "2" => Some(Self::ViewMenu),
            "3" => Some(Self::RemoveMenu),
            "4" => Some(Self::UpdateMenu),
            _ => None,
        }
    }

    fn show() {
        println!("");
        println!(" == BILL MANAGER == ");
        println!("");
        println!("1. Add Bill ");
        println!("2. View Bill");
        println!("3. Remove Bill");
        println!("4. Update Bill");
        println!("Enter Selection: ");
    }
}

fn run_program() -> Option<()> {
    let mut bills = Bills::new();

    loop {
        MainMenu::show();
        let input = get_input()?;

        match MainMenu::add_str(input.as_str()) {
            Some(MainMenu::AddMenu) => menu::add_bill(&mut bills),
            Some(MainMenu::ViewMenu) => menu::view_all(&bills),
            Some(MainMenu::RemoveMenu) => menu::remove_bill(&mut bills),
            Some(MainMenu::UpdateMenu) => menu::update_bill(&mut bills),
            None => break,
        }
    }
    None
}

fn main() {
    run_program();
}
