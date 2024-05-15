use std::fmt;

struct User {
    active :bool,
    username: String,
    email: String,
    sign_in_count: u64
}

impl fmt::Display for User {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        let mut active_mod = String::new();
        if !self.active {
            active_mod = String::from("not ");
        }
        write!(f, "The email from the user {} is {}. They are {}active and they logged it {} times.", self.username, self.email, active_mod, self.sign_in_count)
    }
}

fn build_user(username: String, email:String) -> User {
     User {
        active: true,
        username,
        email,
        sign_in_count : 1u64
    }
}


fn main() {
    let mut new_user: User = build_user(String::from("firstUser"), String::from("cool_mail_name@mail.com"));

    println!("{}", new_user);
    
    new_user.email = String::from("another_cool_mail@coolerdomain.com");
    new_user.sign_in_count =  new_user.sign_in_count + 1;

    println!("{}", new_user);
    
    let copy_new_user = User{email : String::from("boring_name@boringdomain.com"), ..new_user};
    
    println!("{}", copy_new_user);
    

}
