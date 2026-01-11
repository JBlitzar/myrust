#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn can_hold(&self, other: &Rectangle) -> bool {
        let w_pass = other.width < self.width;
        let h_pass = other.height < self.height;
        return w_pass && h_pass
    }
    fn double_dimensions(&mut self) {
        self.width *= 2;
        self.height *= 2;
    }
}
impl Rectangle {
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

struct Cuboid {
    width: u32,
    height: u32,
    depth: u32,
}

impl Cuboid {
    fn volume(&self) -> u32 {
        return self.width * self.height * self.depth
    }
}

fn main() {

 let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    println!(
        "rect1 ({:?}) is {} square pixels.",
        rect1,
        rect1.area()
    );

    let c1 = Cuboid {
        width: 10,
        height: 20,
        depth: 15,
    };

    println!("The volume of the cuboid is {} voxels.", c1.volume());

   
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    let s1 = Rectangle::square(10);

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
    println!(
        "s1 ({:?}) is {} square pixels.",
        s1,
        s1.area()
    );

    let mut rect = Rectangle {
        width: 2,
        height: 2
    };
    rect.double_dimensions();
    println!("Doubled rectangle: {:?}", rect);
}

