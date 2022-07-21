trait Hello{
     fn func()->void;
}

impl   dyn Hello {
     fn func()->void{
          println!("Hello1");
     }
}

pub use Hello1;