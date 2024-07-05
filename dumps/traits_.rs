pub struct Cat {voice_inp:String}
pub struct Duck{voice_inp: String}

pub trait Animal {
	fn voice(&self) -> String;
}
impl Animal for Cat {
	fn voice(&self)  -> String{
		format!("{} x3 cat is hungry!", &self.voice_inp)
	}
}

impl Animal for Duck {
    fn voice<'a>(&self)  -> String{
    if self.voice_inp.starts_with("q") == true {
        format!("{} quek  quek", &self.voice_inp)
        }
    else {
        format!("Cannot quek!")
        }
    }
}
fn main() {
    let bengal = Cat{voice_inp : String::from("meow")};
    println!("{}", bengal.voice());
    
    let bangau = Duck{voice_inp: String::from("quek")};
    println!("{}",bangau.voice());
}
