mod resources;

use resources::RESOURCES;

fn main() {
	for i in RESOURCES {
		println!("new asset:"); 
		println!("> hash: {}\n> path: {}\n> url: {}", i.hash, i.path, i.url);
	}
}
