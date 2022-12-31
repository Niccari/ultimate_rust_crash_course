#[derive(Debug)]
struct Grapes {
    amount_left: i32
}

trait Bite {
    fn bite(self: &mut Self);
}

impl Bite for Grapes {
    fn bite(self: &mut Self) {
        self.amount_left -= 1;
    }
}

fn main() {
    // Once you finish #1 above, this part should work.
    let mut carrot = Carrot { percent_left: 100.0 };
    carrot.bite();
    println!("I take a bite: {:?}", carrot);

    // 4. Uncomment and adjust the code below to match how you defined your
    // Grapes struct.
    //
    let mut grapes = Grapes { amount_left: 100 };
    grapes.bite();
    println!("Eat a grape: {:?}", grapes);

    bunny_nibbles(&mut carrot);
    println!("Bunny nibbles for awhile: {:?}", carrot);
}

fn bunny_nibbles<T: Bite>(fruit: &mut T){
    fruit.bite();
    fruit.bite();
    fruit.bite();
    fruit.bite();
}

#[derive(Debug)] // This enables using the debugging format string "{:?}"
struct Carrot {
    percent_left: f32,
}

impl Bite for Carrot {
    fn bite(self: &mut Self) {
        // Eat 20% of the remaining carrot. It may take awhile to eat it all...
        self.percent_left *= 0.8;
    }
}