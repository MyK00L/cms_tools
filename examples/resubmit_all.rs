use cms_tools::*;

fn main(){
    let username = String::from("user");
    let password = "password";
    let mut client = Client::new(username.clone());
    client.login(password).unwrap();
    let user = client.get_user(&username).unwrap();
    for sc in user.scores.unwrap() {
        if sc.score == 100.0 {
            println!("{} has score 100", sc.title);
            let sub_list = client.get_submission_list(&sc.name).unwrap();
            let best_sub = sub_list.get_fastest_high(&client).unwrap();
            let files = &best_sub.files;
            if files.len() == 1 { // if it is not an output-only
                let mut submitted: bool = false;
                print!("Resubmitting {}", sc.name.clone());
                while !submitted { // because cmsocial has a limit to submission rate
                    print!(".");
                    if client
                        .submit_normal(
                            &sc.name,
                            &client.get_file(&files[0]).unwrap(),
                            files[0].name.split(".").collect::<Vec<&str>>().last().unwrap(),
                        )
                        .is_ok()
                    {
                        submitted = true;
                    }
                }
                println!("");
            }
        }
    }
}
