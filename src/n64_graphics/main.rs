mod textures;

fn main() {
    println!("Hello, world!");
    println!(
        "Codec: {}",
        textures::codec_name(textures::N64Codec::RGBA16)
    );
}
