es (59 sloc) 1.58 KB
#[path = "group.rs"] mod group;
pub use group::Group;


#[derive(Debug)]
pub struct User {
    nickname: String,
    password: String,
    groups: Vec<u64>,
}


impl User {
    pub async fn new() -> User {
        User {
            nickname: String::new(),
            password: String::new(),
            groups: Vec::new(),
        }
    }
    pub async fn new_with_fields(nickname: String, password: String) -> User {
        User {
            nickname,
            password,
            groups: Vec::new(),
        }
    pub async fn set_nickname(&mut self, name: String) {
        self.nickname = name
    }
    pub async fn set_password(&mut self, password: String) {
        self.password = password
    }
    
    pub async fn get_nickname(&self) -> &String {
        &self.nickname
    }
    pub async fn get_password(&self) -> &String {
        &self.password
    }
    pub async fn get_groups(&self) -> &Vec<u64> {
        &self.groups
    }
    pub async fn print_info(&self) {
        println!("\nUSER INFO:\n{}\n{}\n{}\n{:?}\n", self.nickname, self.password, self.groups)
    }
    
}