use std::io;
pub fn choice_num()-> Result<u8,String>{
    let mut input =String::new();
    io::stdin().read_line(&mut input).map_err(|e| {
        format!("Error:{}",e)
    })?;
    input.trim().parse().map_err(|e| format!("Error:{}",e))
}   
pub fn read_num()-> Result<f64,String>{
    let mut input =String::new();
    io::stdin().read_line(&mut input).map_err(|e| {
        format!("Error:{}",e)
    })?;
    input.trim().parse().map_err(|e| format!("Error:{}",e))
}   


pub struct Tempertur{
    c:f64,
    f:f64,
}
impl Tempertur {
    pub fn new() -> Self{
        Self { c: 0.0, f: 0.0 }
    }
    pub fn addc(&mut self,dig:f64){
        self.c = dig;
    }
    pub fn addf(&mut self,dig:f64){
        self.f = dig;
    }
    pub fn calc_c2f(&self)-> f64{
        self.c * 1.8 + 32.0 
    }
    pub fn calc_f2c(&self)->f64{
        (self.f - 32.0 ) / 1.8
    }
}


pub fn fahrentheit2celsius()->  Result<f64 , String>{
    println!("enter dig fahrenheit");
    match read_num() {
        Ok(fahrenhet)=>{
            let mut temp = Tempertur::new();
            temp.addf(fahrenhet);
            Ok(temp.calc_f2c())
        },
        Err(e)=> Err(format!("Error:{e}")),
    }
}

pub fn celcius2fahren()-> Result<f64,String>{
    println!("enter dig celsius");
    match read_num(){
        Ok(celsius)=>{
            let mut temp = Tempertur::new();
            temp.addc(celsius);
            Ok(temp.calc_c2f())
        },
        Err(e)=>Err(String::from(format!("Error:{e}"))),
    }

}
