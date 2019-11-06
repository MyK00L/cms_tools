
#[derive(serde::Serialize, serde::Deserialize, Debug)]
struct Statement {
    it: Option<String>
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
struct Tag {
    name: String,
    can_delete: bool
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
struct Task {
    time_limit: Option<f32>,
    help_available: bool,
    statements: Statement,
    name: String,
    success: u8,
    title: String,
    submission_format: Vec<String>,
    memory_limit: Option<u16>,
    task_type: String,
    score_multiplier: f64,
    id: usize,
    tags: Vec<Tag>,
    attachments: Vec<Vec<String> >
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
struct ListTask {
    score: Option<f32>,
    title: String,
    score_multiplier: f64,
    id: usize,
    name: String
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
struct TaskList {
    tasks: Vec<ListTask>,
    num: usize,
    success: u8
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
struct Stat {
    username: String,
    time: f32
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
struct Stats {
    nsubscorrect: usize,
    success: u8,
    nusers: usize,
    nsubs: usize,
    nuserscorrect: usize,
    best: Vec<Stat>
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
struct Score {
    score: f32,
    name: String,
    title: String
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
struct Institute{
    province: Option<String>,
    city: Option<String>,
    region: Option<String>,
    id: Option<u32>,
    name: Option<String>
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
struct User{
    username: String,
    mail_hash: String,
    first_name: String,
    last_name: String,
    institute: Institute,
    tasks_solved: i32,
    access_level: u8,
    join_date: f64,
    score: u32,
    global_access_level: u8,
    scores: Option<Vec<Score> >
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
struct List{
    num: usize,
    users: Vec<User>,
    success: u8
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
struct TestHead{
    max_score: u8,
    name: String,
    description: String
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
struct Question{
    max_score: u8,
    text: String,
    #[serde(rename = "type")]
    question_type: String,
    choices: Option<Vec<String> >,
    answers: Option<Vec<(String,u32) > >
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
struct Test{
    success: u8,
    name: String,
    questions: Vec<Question>,
    description: String
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
struct Tests{
    tests: Vec<TestHead>,
    success: u8
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
struct Region{
    id: usize,
    name: String
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
struct RegionList{
    regions: Vec<Region>,
    success: u8
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
struct CheckResponse{
    success: u8,
    error: Option<String>
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
struct Techniques{
    success: u8,
    tags: Vec<String>
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
struct RecoverResponse{
    message: Option<String>,
    #[serde(rename = "type")]
    thing_type: u8,
    success: u8,
    error: Option<String>
}

struct Client{
    client : reqwest::Client,
    username: String,
    logged: bool
}

impl Client{
    //create a new client with given username.
    //always use this to create a client
    fn new(username: String) -> Self {
        Client{client:reqwest::Client::builder().referer(false).cookie_store(true).build().unwrap(),username:username,logged:false}
    }
    //login
    fn login(&mut self, password: String) -> Result<bool,u8> {
        if self.logged {
            //already logged
            return Ok(true);
        }
        match self.client.post("https://training.olinfo.it/api/user").json(&serde_json::json!({"action":"login","keep_signed":"false","username":self.username,"password":password})).send() {
            Ok(mut response) => {
                #[derive(serde::Serialize, serde::Deserialize)]
                struct Resp{
                    success: u8
                }
                match response.json::<Resp>() {
                    Ok(resp) => {
                        match resp.success {
                            1 => {
                                self.logged = true;
                                //successful login
                                Ok(false)
                            }
                            //wrong username/password
                            _ => Err(3)
                        }
                    },
                    _ => {
                        //should be unreachable code
                        Err(2)
                    }
                }
            },
            _ => {
                //no connection
                Err(1)
            }
        }
    }
    //get list of users in reverse order of score in [first,last)
    fn get_ranking(&self, first: usize, last: usize) -> Result<List,u8> {
        match self.client.post("https://training.olinfo.it/api/user").json(&serde_json::json!({"action":"list","first":first,"last":last})).send() {
            Ok(mut response) => {
                match response.json::<List>() {
                    Ok(resp) => {
                        match resp.success {
                            1 => Ok(resp),
                            _ => Err(3)
                        }
                    },
                    _ => Err(2)
                }
            },
            _ => {
                Err(1)
            }
        }
    }
    //get the details of a specific user
    fn get_user(&self, username: String) -> Result<User,u8> {
        match self.client.post("https://training.olinfo.it/api/user").json(&serde_json::json!({"action":"get","username":username})).send() {
            Ok(mut response) => {
                match response.json::<User>() {
                    Ok(resp) => {
                        Ok(resp)
                    },
                    _ => Err(2)
                }
            },
            _ => {
                Err(1)
            }
        }
    }
    //get the details of a specific task
    fn get_task(&self, name: String) -> Result<Task,u8> {
        match self.client.post("https://training.olinfo.it/api/task").json(&serde_json::json!({"action":"get","name":name})).send() {
            Ok(mut response) => {
                match response.json::<Task>() {
                    Ok(resp) => {
                        match resp.success {
                            1 => Ok(resp),
                            _ => Err(3)
                        }
                    },
                    _ => Err(2)
                }
            },
            _ => {
                Err(1)
            }
        }
    }
    //get the statistics for a specific task
    fn get_stats(&self, name: String) -> Result<Stats,u8> {
        match self.client.post("https://training.olinfo.it/api/task").json(&serde_json::json!({"action":"stats","name":name})).send() {
            Ok(mut response) => {
                match response.json::<Stats>() {
                    Ok(resp) => {
                        match resp.success {
                            1 => Ok(resp),
                            _ => Err(3)
                        }
                    },
                    _ => Err(2)
                }
            },
            _ => {
                Err(1)
            }
        }
    }
    //get the list of available tests
    fn get_tests(&self) -> Result<Tests,u8> {
        match self.client.post("https://training.olinfo.it/api/test").json(&serde_json::json!({"action":"list"})).send() {
            Ok(mut response) => {
                match response.json::<Tests>() {
                    Ok(resp) => {
                        match resp.success {
                            1 => Ok(resp),
                            _ => Err(3)
                        }
                    },
                    _ => Err(2)
                }
            },
            _ => {
                Err(1)
            }
        }
    }
    //get the details and text of a specific test
    fn get_test(&self, test_name: String) -> Result<Test,u8> {
        match self.client.post("https://training.olinfo.it/api/test").json(&serde_json::json!({"action":"get","test_name":test_name})).send() {
            Ok(mut response) => {
                match response.json::<Test>() {
                    Ok(resp) => {
                        match resp.success {
                            1 => Ok(resp),
                            _ => Err(3)
                        }
                    },
                    _ => Err(2)
                }
            },
            _ => {
                Err(1)
            }
        }
    }
    //get list of tasks in [first,last) in the given order with the given tag that matches search
    //possible orders are: newest, easiest, hardest
    //if an invalid order is given, it is assumed to be newest
    fn get_task_list(&self, first: usize, last: usize, order: String, tag: Option<String>, search: Option<String> ) -> Result<TaskList,u8> {
        match self.client.post("https://training.olinfo.it/api/task").json(&serde_json::json!({"action":"list","first":first,"last":last,"order":order,"tag":tag,"search":search})).send() {
            Ok(mut response) => {
                match response.json::<TaskList>() {
                    Ok(resp) => {
                        match resp.success {
                            1 => Ok(resp),
                            _ => Err(3)
                        }
                    },
                    //probabilly invalid parameters, eg last>first
                    _ => Err(2)
                }
            },
            _ => {
                //no connection
                Err(1)
            }
        }
    }
    //get a list of the regions
    fn get_regions(&self) -> Result<RegionList,u8> {
        match self.client.post("https://training.olinfo.it/api/location").json(&serde_json::json!({"action":"listregions"})).send() {
            Ok(mut response) => {
                match response.json::<RegionList>() {
                    Ok(resp) => {
                        match resp.success {
                            1 => Ok(resp),
                            _ => Err(3)
                        }
                    },
                    _ => Err(2)
                }
            },
            _ => {
                Err(1)
            }
        }
    }
    //check if there is an user with username = username
    fn user_exists(&self, username: String) -> Result<bool,u8> {
        match self.client.post("https://training.olinfo.it/api/check").json(&serde_json::json!({"type":"username","value":username})).send() {
            Ok(mut response) => {
                match response.json::<CheckResponse>() {
                    Ok(resp) => {
                        match resp.success {
                            1 => Ok(false),
                            _ => {
                                match resp.error {
                                    Some(x) => {
                                        if x==String::from("This username is not available"){
                                            Ok(true)
                                        } else {
                                            Ok(false)
                                        }
                                    }
                                    _ => Err(3)
                                }
                            }
                        }
                    },
                    _ => Err(2)
                }
            },
            _ => {
                Err(1)
            }
        }
    }
    //check if username is valid, note: Ok does not mean username is valid
    fn check_username(&self, username: String) -> Result<CheckResponse,u8> {
        match self.client.post("https://training.olinfo.it/api/check").json(&serde_json::json!({"type":"username","value":username})).send() {
            Ok(mut response) => {
                match response.json::<CheckResponse>() {
                    Ok(resp) => Ok(resp),
                    _ => Err(2)
                }
            },
            _ => {
                Err(1)
            }
        }
    }
    //check if email is valid, note: Ok does not mean email is valid
    fn check_email(&self, email: String) -> Result<CheckResponse,u8> {
        match self.client.post("https://training.olinfo.it/api/check").json(&serde_json::json!({"type":"email","value":email})).send() {
            Ok(mut response) => {
                match response.json::<CheckResponse>() {
                    Ok(resp) => Ok(resp),
                    _ => Err(2)
                }
            },
            _ => {
                Err(1)
            }
        }
    }
    //get list of technique tags
    fn get_techniques(&self) -> Result<Techniques,u8> {
        match self.client.post("https://training.olinfo.it/api/tag").json(&serde_json::json!({"action":"list","filter":"techniques"})).send() {
            Ok(mut response) => {
                match response.json::<Techniques>() {
                    Ok(resp) => {
                        match resp.success {
                            1 => Ok(resp),
                            _ => Err(3)
                        }
                    },
                    _ => Err(2)
                }
            },
            _ => {
                Err(1)
            }
        }
    }
    //recover lost password, use empty code for getting email
    fn recover(&self, email: String, code: String) -> Result<RecoverResponse,u8> {
        match self.client.post("https://training.olinfo.it/api/user").json(&serde_json::json!({"action":"recover","code":code,"email":email})).send() {
            Ok(mut response) => {
                match response.json::<RecoverResponse>() {
                    Ok(resp) => {
                        match resp.success {
                            1 => Ok(resp),
                            _ => Err(3)
                        }
                    },
                    _ => Err(2)
                }
            },
            _ => {
                Err(1)
            }
        }
    }
    //update password/email, empty string for fields you dont want to update
    fn user_update(&self, email: String, password: String, old_password: String) -> Result<(),u8> {
        match self.client.post("https://training.olinfo.it/api/user").json(&serde_json::json!({"action":"update","email":email,"old_password":old_password,"password":password})).send() {
            Ok(mut response) => {
                #[derive(serde::Serialize, serde::Deserialize, Debug)]
                struct Resp {
                    success: u8
                }
                match response.json::<Resp>() {
                    Ok(resp) => {
                        println!("{:?}",resp);
                        match resp.success {
                            1 => Ok(()),
                            _ => Err(3)
                        }
                    },
                    _ => Err(2)
                }
            },
            _ => {
                Err(1)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    //preoii_flow : oo , no solves
    //preoii_crew : oo
    use super::*;
    #[test]
    fn best_times() {
        //make a client
        let m = Client::new(String::from("Gemmady"));
        let task_list = m.get_task_list(0,500,String::from("newest"),None,None).unwrap();
        let mut hm = std::collections::HashMap::<String,u32>::new();
        for i in task_list.tasks {
            let best = m.get_stats(i.name).unwrap().best;
            if best.len()>0 {
                let t = hm.entry(best[0].username.clone()).or_insert(0);
                *t+=1;
            }
        }
        let mut v : Vec<(u32,String)> = hm.iter().map(|x| (*x.1,x.0.clone())).collect();
        v.sort();
        for i in v.iter().rev() {
            println!("{} {}", i.1, i.0);
        }
    }
    #[test]
    fn it_works() {
        let mut m = Client::new(String::from("Gemmady"));
    }
}
