use deencode::{deencode, Engine, LATIN1, MIXED816LE, UTF7, UTF8};

fn main()
{
    let engines: Vec<&dyn Engine> = vec![&UTF8, &LATIN1, &MIXED816LE, &UTF7];

    let mut tree = deencode("Clément", &engines, 1);

    let _ = tree.deduplicate();

    //println!("{}", serde_json::to_string(&tree).unwrap());
    println!("{}", tree);
}
