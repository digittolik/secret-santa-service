
#[derive(Debug)]
enum Access {
    Public,
    Private,
}


pub struct Group {
    access: Access,
    members: Vec<(u64, bool)>,
}


impl Group {
    pub async fn new(admin: u64) -> Group {
        Group {
            access: Access::Public,
            members: vec![(admin, true)],
        }
    }
   
    pub async fn print_info(&self) {
        println!("\nGROUP INFO\n{}\n{:?}\n{:?}\n",self.access, self.members);
    }