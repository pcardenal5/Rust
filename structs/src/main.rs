struct User {
    active :bool,
    username: String,
    email: String,
    sign_in_count: u64
}

fn main() {
    let mut new_user: User = User {
        active          : true,
        username        : String::from("firstUser"),
        email           : String::from("cool_mail_name@mail.com"),
        sign_in_count   : 1u64
    };
    println!("The email from the user {0} is {1}", new_user.username, new_user.email);
    
    new_user.email = String::from("another_cool_mail@coolerdomain.com");

    println!("The email from the user {0} has been changed to {1}", new_user.username, new_user.email);

}
