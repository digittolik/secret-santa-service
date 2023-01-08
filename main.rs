use tide::{Request, Error};
use futures::executor::block_on;


#[derive(Debug, serde::Deserialize, serde::Serialize)]
struct Reg {
    nickname: String,
    password: String,
}


async fn get_user(mut req: Request<()>) -> Result<User, Error> {
    let data: Reg = req.body_json().await?;
    Ok(User::new_with_fields(data.nickname, data.password).await)
}


#[derive(Debug, serde::Deserialize, serde::Serialize)]
struct CloseGroup {
    nickname: String,
    password: String,
    group_id: u64,
}


async fn get_clg_user(mut req: Request<()>) -> Result<(User, u64), Error> {
    let data: CloseGroup = req.body_json().await?;
    let user = User::new_with_fields(data.nickname, data.password).await;
    Ok((user, data.group_id))
}


#[derive(Debug, serde::Deserialize, serde::Serialize)]
struct MkAdmin {
    nickname: String,
    password: String,
    group_id: u64,
    other_user_id: u64,
}


async fn get_mka_user(mut req: Request<()>) -> Result<(User, u64, u64), Error> {
    let data: MkAdmin = req.body_json().await?;
    let user = User::new_with_fields(data.nickname, data.password).await;
    Ok((user, data.group_id, data.other_user_id))
}


async fn async_main() -> tide::Result<()> {

    let mut app = tide::new();

    app.at("/").get(|_: Request<()>| async move {
        Ok(format!("
        \navailable data:
        nickname(String),
        password(String),
        ids(u64) - group_id, other_user_id
        \npost request's data
        /registration       (nickname, password)
        /createGroup       (nickname, password)
        /checkGroups          (nickname, password)
        /all-groups         (nickname, password)
        /joinGroup         (nickname, password, group_id)
        /leaveGroup        (nickname, password, group_id)**
        /makeAdmin         (nickname, password, group_id, other_user_id)*
        /groupMembers      (nickname, password, group_id)*
        /dropGroup         (nickname, password, group_id)*
        /getPair           (nickname, password)
        *  - for adminis only
        ** - for members only
        "))
    });

    app.at("/registration").post(|req: Request<()>| async move {
        let user = get_user(req).await?;
        }
        Ok("\n\nAlready registered\n")
    });

    app.at("/createGroup").post(|req: Request<()>| async move {
        let mut user = get_user(req).await?;
        let user_id = is_registered(&user).await?;
        if user_id != 0 {
            user.set_id(user_id as u64).await;
            return Ok(format!("\n\ngroup {} created\n", group.get_id().await))
        }
        Ok(format!("\n\ngo register(/createGroup)\n"))
    });

    app.at("/checkGroups").post(|req: Request<()>| async move {
        let mut user = get_user(req).await?;
        let user_id = is_registered(&user).await?;
        if user_id != 0 {
            user.set_id(user_id as u64).await;
            let mut groups = String::new();
 
            Ok(format!("\n\n{}\n", groups))
        } else {
            Ok(format!("\n\ngo register(/checkGroups)\n"))
        }
    });

    app.at("/all-groups").post(|req: Request<()>| async move {
        let user = get_user(req).await?;
        if is_registered(&user).await? != 0 {
            let mut groups = String::new();
 
            Ok(format!("\n\n{}\n", groups))
        } else {
            Ok(format!("\n\ngo register(/all-groups)\n"))
        }
    });

    app.at("/joinGroup").post(|req: Request<()>| async move {
        let clg_user = get_clg_user(req).await?;
        let group_id = clg_user.1;
        let mut user = clg_user.0;
        let user_id = is_registered(&user).await?;
        if user_id != 0 {
            user.set_id(user_id as u64).await;
            return Ok(format!("\n\nfail\n"))
        }
        Ok(format!("\n\ngo-register(/joinGroup)\n"))
    });

    app.at("/leaveGroup").post(|req: Request<()>| async move {
        let clg_user = get_clg_user(req).await?;
        let group_id = clg_user.1;
        let mut user = clg_user.0;
        let user_id = is_registered(&user).await?;
        if user_id != 0 {
            user.set_id(user_id as u64).await;
            return Ok(format!("\n\nfail\n"))
        }
        Ok(format!("\n\ngo-register(/leaveGroup)\n"))
    });

    

    app.at("/makeAdmin").post(|req: Request<()>| async move {
        let mka_user = get_mka_user(req).await?;
        let group_id = mka_user.1;
        let other_user_id = mka_user.2;
        let mut user = mka_user.0;
        let user_id = is_registered(&user).await?;
        if user_id != 0 {
            user.set_id(user_id as u64).await;
            return Ok(format!("\n\nfail\n"))
        }
        Ok(format!("\n\ngo register(/makeAdmin)\n"))
    });

    

    app.at("/groupMembers").post(|req: Request<()>| async move {
        let clg_user = get_clg_user(req).await?;
        let group_id = clg_user.1;
        let mut user = clg_user.0;
        let user_id = is_registered(&user).await?;
        if user_id != 0 {
            user.set_id(user_id as u64).await;
 
                return Ok(format!("\n\n{res}\n"))
            }
            return Ok(format!("\n\nfail\n"))
        }
        Ok(format!("\n\ngo register(/groupMembers)\n"))
    });

    app.at("/dropGroup").post(|req: Request<()>| async move {
        let clg_user = get_clg_user(req).await?;
        let group_id = clg_user.1;
        let mut user = clg_user.0;
        let user_id = is_registered(&user).await?;
        if user_id != 0 {
            user.set_id(user_id as u64).await;

            return Ok(format!("\n\nfail\n"))
        }
        Ok(format!("\n\ngo register(/dropGroup)\n"))
    });

    app.at("/getPair").post(|req: Request<()>| async move {
        let clg_user = get_clg_user(req).await?;
        let group_id = clg_user.1;
        let user = clg_user.0;
        let user_id = is_registered(&user).await?;
        if user_id != 0 {

            return Ok(format!("\n\nfail\n"))
        }
        Ok(format!("\n\ngo register(/getPair)\n"))
    });

    app.listen("127.0.0.1:8080").await?;

    Ok(())
}

fn main() {
    block_on(async_main()).unwrap();
