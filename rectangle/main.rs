#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

fn volume(cuboid: &Cuboid) -> u32 {
    cuboid.width * cuboid.height * cuboid.depth
}

struct Cuboid {
    width: u32,
    height: u32,
    depth: u32,
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "rect1 ({:?}) is {} square pixels.",
        rect1,
        area(&rect1)
    );

    let c1 = Cuboid {
        width: 10,
        height: 20,
        depth: 15,
    };

    println!("The volume of the cuboid is {} voxels.", volume(&c1));
}

