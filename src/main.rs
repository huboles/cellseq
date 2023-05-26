use cellseq::*;

fn main() {
    let mut map = Map::new(32, 64);
    map.randomize(0.5);

    loop {
        map.update();

        std::process::Command::new("clear").status().unwrap();

        println!("{map}");

        std::thread::sleep(std::time::Duration::from_millis(1000));
    }
}
