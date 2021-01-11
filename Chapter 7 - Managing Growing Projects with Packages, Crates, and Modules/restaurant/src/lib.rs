// #[cfg(test)]
// mod tests {
//     #[test]
//     fn it_works() {
//         assert_eq!(2 + 2, 4);
//     }
// }

mod front_of_house;

mod back_of_house;
fn serve_order() {}
// Using "use" to bring the modules into the scope
// use crate::front_of_house::hosting;

// Optionally, you can also use relative paths
// use self::front_of_house::hosting;

// To bring the names into scope as public, we can combine "pub" and "use"
pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    // Absolute path
    //crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    // front_of_house::hosting::add_to_waitlist();

    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();

    let mut meal = back_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // The next line won't compile if we uncomment it; we're not allowed
    // to see or modify the seasonal fruit that comes with the meal
    // meal.seasonal_fruit = String::from("blueberries");
    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}
