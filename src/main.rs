mod storage;

use storage::store::storage;
use storage::action::create;

fn main() {
    let mut my_map = storage();
    create(&mut my_map);
    for (key, value) in &my_map {
        println!("{key}: {value}");
    }
}
