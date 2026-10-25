fn main() {

    let mut text = String::from("Ei_nlang7esw+ort");
    println!("\n--- Vorgabetext: {} ---", text);
    text.remove(2);
    text.remove(7);
    text.remove(10);
    println!("\n\tBearbeiteter Text nach Schritt 1: {}", text);

    text.insert(3, ' ');
    text.insert(10, ' ');
    println!("\n\tBearbeiteter Text nach Schritt 2: {}", text);

    text = text.replace("wort", "Wort");


     // Es gibt einen einfacheren Weg, den Text zu bearbeiten, indem man die Methode replace() verwendet.
    // text = text.replace("Ei_nlang7esw+ort", "Ein langes Wort");
    println!("\n\tBearbeiteter Text: {}\n", text);
}
