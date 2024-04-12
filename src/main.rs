use std::io;
fn bubble_Sort(v1 : &mut Vec<i32>)
{
  let  n = v1.len();
  for i in 0..n {
    for j in 0..n - i - 1 {
        if v1[j] > v1[j+1]
        {
          let temp =  v1[j+1];
          v1[j+1]  =  v1[j];
          v1[j]    =   temp; 
        }
      
    }
} 
}
fn main() {


 let mut arr:Vec<i32> =  Vec::new();
 loop{
  println!("Enter the number you want to insert");

  let mut  input = String::new();


  io::stdin().read_line(&mut input)
     .expect("Failed to re3ad line");


 if input.trim() =="done"{
  break;
 }


 //please input to  an integer
 match input.trim().parse::<i32>(){
  Ok(num)=>{
      arr.push(num);
  },
  Err(_)=>{
          println!("Invalid input , Please enter  a valid integer or done")
  }
  
 }


 }
 bubble_Sort(&mut arr);
 println!("database content: {:?}", arr);
}
