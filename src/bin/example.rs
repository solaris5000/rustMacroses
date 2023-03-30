use proc_macro_example::Counte;
use proc_macro_example::Identy;
use proc_macro_example::HelloMacro;

//#[derive(Counte, Identy)]
enum Example {
    Variant1,
    Variant2,
    Variant3,
}

#[derive(HelloMacro)]
struct Pancakes;



fn main() {
    let a = Example::Variant1;
    Pancakes::;
   // println!("Our enum has {} variants", Example::count());

   // println!("Out enum a has id {:?}", Example::identy());
}
