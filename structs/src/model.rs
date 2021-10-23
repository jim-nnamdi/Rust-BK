pub struct User {
   name: String,
   email: String 
}

trait summary {
   fn summarize(value: &string)
}

// implementing a trait for the user struct
impl summary for User {
   pub fn add(&mut self, value: i32){
      self.list.push(&value);
      self.update_average()
   }

   pub fn remove(&mut self, value: i32) -> i32 {
      self.list.pop(&value);
   }
}