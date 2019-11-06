
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
    scores: Vec<Score>
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

struct Client{
    client : reqwest::Client,
    username: String,
    logged: bool
}

impl Client{
    fn new(username: String) -> Self {
        Client{client:reqwest::Client::builder().referer(false).cookie_store(true).build().unwrap(),username:username,logged:false}
    }
    fn login(&mut self, password: String) -> Result<bool,u8> {
        if self.logged {
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
                                Ok(false)
                            }
                            _ => Err(3)
                        }
                    },
                    _ => {
                        Err(2)
                    }
                }
            },
            _ => {
                Err(1)
            }
        }
    }
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
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        //preoii_flow : oo , no solves
        //preoii_crew : oo

        //make a client
        let mut m = Client::new(String::from("Gemmady"));

        //login
        //println!("{:?}",m.login(String::from("not the real password")));

        println!("{:?}",m.get_test(String::from("scolastiche2012_pas")));
    }
}

