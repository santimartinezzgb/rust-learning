fn main() {
    // MUT (MUTABILITY)
    {
        // VARIABLE INMUTABLE. It doesn´t change
        // let x = 5;
        // print!("The value of x is: {}", x);
        // x = 6;
        // print!("The value of x is: {}", x);

        // VARIABLE MUTABLE. It can change
        let mut x = 5;
        print!("The value of x is: {}\n", x);
        x = 6;
        print!("The new value of x is: {}\n", x);
    }

    // LET (SHADOWING)
    {
        let x = 5;
        let x = x + 1;
        {
            let x = x * 2;
            println!("The value of x in the inner scope is: {}", x);
        }
        println!("The value of x is: {}\n", x);
    }

    // DIFFERENCE let vs mut
    {
        // With shadowing (let) you can change type:
        let spaces = "   "; // String
        let spaces = spaces.len(); // Number ✓

        // With mut you CANNOT change type:
        //let mut spaces = "   ";
        //spaces = spaces.len(); // ✗ Error: type mismatch
    }
}
