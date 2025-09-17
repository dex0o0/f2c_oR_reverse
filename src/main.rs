mod llib;
use clearscreen;

fn main() {
    loop {
        println!("\n\nwhat's your plane \n1:calc celciuse to fahrenheit\nOR\n2:calc fahrenheit to celcius\t\t(please enter number option)\n3:for exit");        
        let choice = llib::choice_num();
        match choice {
            Ok(num)=>{
                match num {
                    1=>{
                        match llib::celcius2fahren() {
                            Ok(result)=> {clearscreen::clear().unwrap();println!("=======Result: {:.2}F=======",result)},
                            Err(e)=> println!("{e}"),
                        }
                    },
                    2=>{
                        match llib::fahrentheit2celsius() {
                            Ok(result)=> {clearscreen::clear().unwrap() ;println!("=======Result: {:.2}C=======",result)},
                            Err(e)=> println!("{e}"),
                        }
                    },
                    3=>break,
                    _=>{},
                }
            },
            Err(e)=>println!("{e}"),
            
        }
    }
}
