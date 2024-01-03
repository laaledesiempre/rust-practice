use std::fmt;
#[derive(Debug)]
struct Complex{
    real:f32,
    imaginarie:f32,
}

// To use the `{}` marker, the trait `fmt::Display` must be implemented
// manually for the type.
impl fmt::Display for Complex{
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        write!(f, "{} + {}i", self.real, self.imaginarie)
    }
}

fn main(){
    let num= Complex{
        real: 3.3,
        imaginarie: 7.2,
    };
        println!("{}", num);
        println!("{:?}", num);
}

