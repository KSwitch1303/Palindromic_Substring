fn main() {
    let word: &str = "asasdleveldasd";
    let word_split: Vec<char> = word.chars().collect();
    let mut pointer1: usize = 0;
    let mut pointer2: usize;
    let mut foward: String;
    let mut backward: String;
    for x in 0..word.len()-1{
        foward= String::new();
        backward= String::new();
        for i in 0..word.len(){
            // println!("{}",word_split[i]);
             if i != x && word_split[pointer1] == word_split[i]{
                // println!("{}",i);
                 pointer2 = i;
                 for j in pointer1..pointer2+1{
                     foward = [foward,  word_split[j].to_string()].join("");
                 }
                 
                 for j in (pointer1..pointer2+1).rev(){
                     backward = [backward,  word_split[j].to_string()].join("");
                 }
                 
                 if foward==String::new(){
                    break;
                 }
                 if foward == backward{
                    println!("{}",foward);
                    println!("{}",backward);
                     println!("");
                 }else{
                     //println!("mismatch")
                 }
             }        
         }
         pointer1 += 1;
    }
    
}
