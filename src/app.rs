use std::collections::HashMap;

fn main() {
    let mut reviews: HashMap<String, String> = HashMap::new();

    reviews.insert(
        String::from("Ancient Roman History"),
        String::from("Very accurate."),
    );
    reviews.insert(
        String::from("Cooking with Rhubard"),
        String::from("Sweet recipes."),
    );
    reviews.insert(
        String::from("Cosmos"),
        String::from("All about deep space."),
    );
    reviews.insert(
        String::from("Programming in Rust"),
        String::from("Great Examples."),
    );
    reviews.insert(
        String::from("Black Holes & Time Warps: Einstein's Outrageous Legacy"),
        String::from("Mistery Black Hole."),
    );

    // Se busca una revisión específica...
    let book: &str = "Programming in Rust";
    println!("\nReview for \'{}\': {:?}", book, reviews.get(book));

    //////////////////////////////////////////////////////////

    // Se elimina una 'review'...
    let obsolete: &str = "Ancient Roman History";
    println!("\n'{}\' removed.", obsolete);
    reviews.remove(obsolete);

    // Confirma que ha sido eliminada la 'review'...
    println!("\nReview for \'{}\': {:?}", obsolete, reviews.get(obsolete));
}
