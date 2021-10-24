pub struct Games {
    title: String,
    developers: String,
    release_year: u32,
    sale_price: u32 
}

trait GameInfos {

   fn description(&self) -> String; 

   fn game_developers(&self) -> String; 

   fn average_sales(&self) -> u32;
}

impl GameInfos for Games {

    fn description(&self) -> String {
        return format!("{} was released in {}, by {} and sells for {} €", self.title, self.release_year, self.developers, self.sale_price);
    }

    fn average_sales(&self) -> u32 {
        return self.sale_price / 3;
    }

    fn game_developers(&self) -> String {
        return format!("{}", self.developers);
    }
}


fn main(){

    let games = Games{
        title : "counter strike go".to_string(),
        developers : "valve".to_string(),
        release_year: 1994,
        sale_price: 45
    };

    println!("{}", games.description());
    println!("{}", games.game_developers());
    println!("{}", games.average_sales());
}