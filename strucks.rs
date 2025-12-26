struct User{
    username : String,
    email : String,
    sign_in_count : u64,
    active : bool
}

impl User{//Methods for the wrapper
    fn create(&self){
        //self.username
        //self email
    }
    fn compare(&self,other:&User){//To pass another instance of User

    }
    fn assoc_create(size:u32)->User{
        return User { email : String::from("sam.paul@test.com"),
        username : String::from("sam.paul"),
        sign_in_count : 1,
        active : true};
    }
}

fn main() {
    println!("Hello, world!");

    let mut user1 = User{
        email : String::from("sam.paul@test.com"),
        username : String::from("sam.paul"),
        sign_in_count : 1,
        active : true
    };

    println!("Before:{}",user1.username);
    user1.username = String::from("sam.paul.invalid");
    println!("After:{}",user1.username);
    let email = user1.email;
    let sign_in_count = user1.sign_in_count;
    let active = user1.active;

    let User3 = User::assoc_create(5);//To call the associated functions
}
