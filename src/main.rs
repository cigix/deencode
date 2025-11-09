use deencode::*;

fn main()
{
    let engines: Vec<&dyn Engine> = vec![
        &UTF8, &LATIN1, &LATIN2, &CP1253, &MIXED816BE, &MIXED816LE, &UTF7];

    for arg in std::env::args().skip(1) {
        let mut tree = deencode(&arg, &engines, 1);
        let _ = tree.deduplicate();
        println!("{}", tree);
    }
}
