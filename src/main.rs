use deencode::*;

fn main()
{
    let engines: Vec<&dyn Engine> = vec![
        // Most standard encoding
        &UTF8,
        // Single byte encodings
        &LATIN1, &LATIN2, &CP1253, &CP1254, &CP1255,
        // My weird encodings that cause problems on purpose
        &MIXED816BE, &MIXED816LE];

    for arg in std::env::args().skip(1) {
        let mut tree = deencode(&arg, &engines, 1);
        let _ = tree.deduplicate();
        println!("{}", tree);
    }
}
