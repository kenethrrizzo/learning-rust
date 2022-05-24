struct User {
    name: String,
    email: String,
    birth_year: Option<u32>,
    active: bool,
    role: UserRole,
    website: Website
}

impl User {
    fn get_age(&self) -> u32 {
        let current_year: u32 = 2022;
        match self.birth_year {
            Some(year) => current_year - year,
            None => 0
        }
    }
}

// ENUM = ENUMERATION
enum UserRole {
    BASIC,
    ADMIN
}

enum Website {
    URL(String),
    INSTAGRAM(String),
    LINKEDIN(String)
}

fn main() {
    let mut user: User = new_user(
        String::from("Keneth"),
        String::from("kenethrierar@gmail.com"),
        Some(2000),
        UserRole::ADMIN,
        Website::LINKEDIN(String::from("kenethrrizzo"))
    );

    let website: String = match user.website {
        Website::INSTAGRAM(_) => String::from("Instagram"),
        Website::URL(_) => String::from("URL"),
        Website::LINKEDIN(_) => String::from("Linkedin"),
    };

    println!("Name {}, email: {}, birth_year: {}, active: {}, Age: {}, Website: {}",
        user.name, user.email, user.birth_year.unwrap_or_default(), user.active, user.get_age(), website);

    user.website = Website::URL(String::from("https://quantumm.org"));

    println!("Keneth's website changed");

    println!("Keneth has access? {}", has_access(&user.role));

    let user2: User = User {
        name: "Camila".to_string(),
        email: "camilall@gmail.com".to_string(),
        role: UserRole::BASIC,
        website: Website::INSTAGRAM(String::from("camiwi")),
        ..user
    };

    println!("Name {}, email: {}, birth_year: {}, active: {}, Age: {}",
        user2.name, user2.email, user2.birth_year.unwrap_or_default(), user2.active, user2.get_age());

    println!("Camila has access? {}", has_access(&user2.role));

    //tuple structs
    struct Point(i32, i32, i32);
    let point: Point = Point(10, 20, 30);
    println!("Point -> {} - {} - {}", point.0, point.1, point.2);

    let mut name: Option<String> = Some("Keneth".to_string());
    match name {
        None => println!("No name"),
        Some(n) => println!("{}", n)
    }
    name = None;
    match name {
        None => println!("No name"),
        Some(n) => println!("{}", n)
    }
}

fn new_user(name: String, email: String, birth_year: Option<u32>, role: UserRole, website: Website) -> User {
    User { name, email, birth_year, active: true, role, website }
}

fn has_access(user_role: &UserRole) -> bool {
    match user_role {
        UserRole::ADMIN => true,
        UserRole::BASIC => false
    }
}