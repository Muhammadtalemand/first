mod pakistan1{
    pub mod lahore{
        pub fn model_town(){
            println!("Very bad");
        }
    }
}
use pakistan;
use pakistan1::lahore;  //edimative path
//mod print;
fn main() {
   
    crate::pakistan1::lahore::model_town();//absolute path

   // print::Kpk::peshawar();
   lahore::model_town();//relative path
   pakistan::pakist::lahore::model_town();
}
