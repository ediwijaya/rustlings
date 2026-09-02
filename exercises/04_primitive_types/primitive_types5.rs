fn main() {
    let cat: (&str, f32) = ("Furry McFurson", 3.5);

    // TODO: Destructure the `cat` tuple in one statement so that the println works.
    // let /* your pattern here */ = cat;
    // let name: &str = &cat.0;
    // let age: &f32 = &cat.1;
    let (name, age) = cat;

    println!("{name} is {age} years old");
}
