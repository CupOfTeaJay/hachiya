use hachiya::ModRepository;

fn main() {
    let repository: ModRepository = ModRepository::new(".");
    println!("{:#?}", repository);
}
