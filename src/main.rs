// exterior scope
fn main() {
    // sets x to 4
    let x = 4;
    
    // prints x is 4
    println!("x is: {}", x);

    // 'interior scope'
    // inside the interior scope we can overwrite an immutable variable
    {
        // sets x to 2
        let x = 2;

        // prints x is 2
        println!("x is: {}", x);
    }

    // another interior scope
    // in here we can access x as already set
    {
        // sets x to 3
        let x = x - 1;

        // prints x is 3
        println!("x is: {}", x);
    }

    // back into the exterior scope so x is accessible and already set to 4
    // sets x to 5
    let x = x + 1;
    println!("x is: {}", x);
}
