//! # Examples:
//! * Print list of people with highest number of fastest solutions
//! ```
//! let m = Client::new(String::from("Gemmady"));
//! let task_list = m.get_task_list(0,1024,String::from(""),None,None).unwrap();
//! let mut hm = std::collections::HashMap::<String,u32>::new();
//! for i in task_list.tasks {
//!     let best = m.get_stats(i.name).unwrap().best;
//!     if best.len()>0 {
//!         let t = hm.entry(best[0].username.clone()).or_insert(0);
//!         *t+=1;
//!     }
//! }
//! let mut v : Vec<(u32,String)> = hm.iter().map(|x| (*x.1,x.0.clone())).collect();
//! v.sort();
//! for i in v.iter().rev() {
//!     println!("{} {}", i.1, i.0);
//! }
//! ```

// User management related

/// Response for email and username checking
#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct CheckResponse {
    success: u8,
    error: Option<String>,
}

/// Responses for password recovery requests
#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct RecoverResponse {
    message: Option<String>,
    #[serde(rename = "type")]
    thing_type: u8,
    success: u8,
    error: Option<String>,
}

// Users related

/// Institute of an user
#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct Institute {
    province: Option<String>,
    city: Option<String>,
    region: Option<String>,
    id: Option<u32>,
    name: Option<String>,
}

/// User info
#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct User {
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
    scores: Option<Vec<Score>>,
}

/// List of users
#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct UserList {
    num: usize,
    users: Vec<User>,
    success: u8,
}

// Task related

/// digest of statement file? apparently not
#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct Statement {
    it: Option<String>,
}

/// tags, like technique or event tags
#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct Tag {
    name: String,
    can_delete: bool,
}

/// Detailed task description
#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct DetailedTask {
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
    attachments: Vec<Vec<String>>,
}

/// Task in a TaskList
#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct Task {
    score: Option<f32>,
    title: String,
    score_multiplier: f64,
    id: usize,
    name: String,
}

/// List of tasks
#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct TaskList {
    tasks: Vec<Task>,
    num: usize,
    success: u8,
}

/// Best time by someone on some task
#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct Stat {
    username: String,
    time: f32,
}

/// Stats of a certain task
#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct Stats {
    nsubscorrect: usize,
    success: u8,
    nusers: usize,
    nsubs: usize,
    nuserscorrect: usize,
    best: Vec<Stat>,
}

// Submission related

/// Score achieved on a certain task by someone
#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct Score {
    score: f32,
    name: String,
    title: String,
}

/// Description of a file
#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct File {
    name: String,
    digest: String,
}

/// Submission, not detailed
#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct Submission {
    files: Vec<File>,
    compilation_outcome: Option<String>,
    task_id: usize,
    timestamp: f64,
    evaluation_outcome: Option<String>,
    score: Option<f32>,
    id: usize,
}

/// Outcome of a certain submission for a specific testcase
#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct Testcase {
    text: String,
    outcome: String,
    time: f64,
    idx: String,
    memory: u64,
}

/// Score details for a specific submission
#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct ScoreDetail {
    testcases: Vec<Testcase>,
    score: f32,
    max_score: f32,
    idx: usize,
}

/// Details of a specific submission
#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct DetailedSubmission {
    files: Vec<File>,
    compilation_outcome: Option<String>,
    task_id: usize,
    language: Option<String>,
    score_details: Option<Vec<ScoreDetail>>,
    timestamp: f64,
    compilation_stderr: Option<String>,
    compilation_time: Option<f64>,
    evaluation_outcome: Option<String>,
    score: Option<f64>,
    compilation_stdout: Option<String>,
    success: u8,
    id: usize,
    compilation_memory: Option<u64>,
}

/// List of submissions by a user for a task
#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct SubmissionList {
    submissions: Vec<Submission>,
    success: u8,
}

// Test related

/// Basic informations of a test
#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct TestHead {
    max_score: u8,
    name: String,
    description: String,
}

/// Question inside of a test
#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct Question {
    max_score: u8,
    text: String,
    #[serde(rename = "type")]
    question_type: String,
    choices: Option<Vec<String>>,
    answers: Option<Vec<(String, u32)>>,
}

/// Test, also known as Quiz
#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct Test {
    success: u8,
    name: String,
    questions: Vec<Question>,
    description: String,
}

/// List of Tests
#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct TestList {
    tests: Vec<TestHead>,
    success: u8,
}

/// Region
#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct Region {
    id: usize,
    name: String,
}

// Misc

/// List of regions
#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct RegionList {
    regions: Vec<Region>,
    success: u8,
}

/// List of `technique` tags
#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct TechniqueList {
    success: u8,
    tags: Vec<String>,
}

// Client

/// **Client** you will do **everything** with
pub struct Client {
    /// The reqwest client
    client: reqwest::Client,
    /// username
    username: String,
    /// if client has token, this should be true and false otherwise
    logged: bool,
}

/// Client functions return:
/// * `Err(1)` if request is not Ok, usually when server is unreachable
/// * `Err(2)` when json cannot be parsed, usually unreachable or due to incorrect parameters
/// * `Err(3)` if the json contains success=0
impl Client {
    // Client related

    /// create a new client with given username.
    ///
    /// always use this to create a client
    ///
    /// for requests you don't need to be logged in to do, username will not be used
    pub fn new(username: String) -> Self {
        Client {
            client: reqwest::Client::builder()
                .referer(false)
                .cookie_store(true)
                .build()
                .unwrap(),
            username: username,
            logged: false,
        }
    }

    /// login with self.username and password
    ///
    /// Returns `Ok(true)` if the client was already logged and `Ok(false)` if it was not and succeeds in logging
    ///
    /// [example is drop-down menu on the top-right corner](https://training.olinfo.it/#/overview)
    pub fn login(&mut self, password: String) -> Result<bool, u8> {
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

    // User management related

    /// recover lost password, use empty code to get the email
    ///
    /// [example cms page](https://training.olinfo.it/#/forgot-account)
    pub fn recover(&self, email: String, code: String) -> Result<RecoverResponse, u8> {
        match self
            .client
            .post("https://training.olinfo.it/api/user")
            .json(&serde_json::json!({"action":"recover","code":code,"email":email}))
            .send()
        {
            Ok(mut response) => match response.json::<RecoverResponse>() {
                Ok(resp) => match resp.success {
                    1 => Ok(resp),
                    _ => Err(3),
                },
                _ => Err(2),
            },
            _ => Err(1),
        }
    }

    /// update password/email, empty string for fields you dont want to update
    ///
    /// [example cms page](https://training.olinfo.it/#/user/Gemmady/edit)
    pub fn user_update(
        &self,
        email: String,
        password: String,
        old_password: String,
    ) -> Result<(), u8> {
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

    /// check if username is valid, note: `Ok` does not mean username is valid
    ///
    /// [example is `Username` input](https://training.olinfo.it/#/signup)
    pub fn check_username(&self, username: String) -> Result<CheckResponse, u8> {
        match self
            .client
            .post("https://training.olinfo.it/api/check")
            .json(&serde_json::json!({"type":"username","value":username}))
            .send()
        {
            Ok(mut response) => match response.json::<CheckResponse>() {
                Ok(resp) => Ok(resp),
                _ => Err(2),
            },
            _ => Err(1),
        }
    }

    /// check if email is valid, note: `Ok` does not mean email is valid
    ///
    /// [example is `E-mail address` input](https://training.olinfo.it/#/signup)
    pub fn check_email(&self, email: String) -> Result<CheckResponse, u8> {
        match self
            .client
            .post("https://training.olinfo.it/api/check")
            .json(&serde_json::json!({"type":"email","value":email}))
            .send()
        {
            Ok(mut response) => match response.json::<CheckResponse>() {
                Ok(resp) => Ok(resp),
                _ => Err(2),
            },
            _ => Err(1),
        }
    }

    /// check if password is valid, note: this is done locally
    ///
    /// unlike other functions, this returs true if password is acceptable and false otherwise
    ///
    /// [example is `Password` input](https://training.olinfo.it/#/signup)
    pub fn check_password(&self, password: String) -> bool {
        password.len() > 4
    }

    // Users related

    /// check if there is an user with username = username
    ///
    /// [example is `Username` input](https://training.olinfo.it/#/signup)
    pub fn user_exists(&self, username: String) -> Result<bool, u8> {
        match self
            .client
            .post("https://training.olinfo.it/api/check")
            .json(&serde_json::json!({"type":"username","value":username}))
            .send()
        {
            Ok(mut response) => match response.json::<CheckResponse>() {
                Ok(resp) => match resp.success {
                    1 => Ok(false),
                    _ => match resp.error {
                        Some(x) => {
                            if x == String::from("This username is not available") {
                                Ok(true)
                            } else {
                                Ok(false)
                            }
                        }
                        _ => Err(3),
                    },
                },
                _ => Err(2),
            },
            _ => Err(1),
        }
    }

    /// get list of users in reverse order of score in [first,last)
    ///
    /// [example cms page](https://training.olinfo.it/#/ranking/1)
    pub fn get_user_list(&self, first: usize, last: usize) -> Result<UserList, u8> {
        match self
            .client
            .post("https://training.olinfo.it/api/user")
            .json(&serde_json::json!({"action":"list","first":first,"last":last}))
            .send()
        {
            Ok(mut response) => match response.json::<UserList>() {
                Ok(resp) => match resp.success {
                    1 => Ok(resp),
                    _ => Err(3),
                },
                _ => Err(2),
            },
            _ => Err(1),
        }
    }

    /// get the details of a specific user
    ///
    /// [example cms page](https://training.olinfo.it/#/user/MyK_00L/profile)
    pub fn get_user(&self, username: String) -> Result<User, u8> {
        match self
            .client
            .post("https://training.olinfo.it/api/user")
            .json(&serde_json::json!({"action":"get","username":username}))
            .send()
        {
            Ok(mut response) => match response.json::<User>() {
                Ok(resp) => Ok(resp),
                _ => Err(2),
            },
            _ => Err(1),
        }
    }

    // Task related

    /// get list of tasks in [first,last) in the given order with the given tag that matches search
    ///
    /// possible orders are: newest, easiest, hardest
    ///
    /// if an invalid order is given, it is assumed to be newest
    ///
    /// [example cms page](https://training.olinfo.it/#/tasks/1)
    pub fn get_task_list(
        &self,
        first: usize,
        last: usize,
        order: String,
        tag: Option<String>,
        search: Option<String>,
    ) -> Result<TaskList, u8> {
        match self.client.post("https://training.olinfo.it/api/task").json(&serde_json::json!({"action":"list","first":first,"last":last,"order":order,"tag":tag,"search":search})).send() {
            Ok(mut response) => {
                match response.json::<TaskList>() {
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

    /// get the details of a specific task
    ///
    /// [example cms page](https://training.olinfo.it/#/task/ois_luck/statement)
    pub fn get_task(&self, name: String) -> Result<DetailedTask, u8> {
        match self
            .client
            .post("https://training.olinfo.it/api/task")
            .json(&serde_json::json!({"action":"get","name":name}))
            .send()
        {
            Ok(mut response) => match response.json::<DetailedTask>() {
                Ok(resp) => match resp.success {
                    1 => Ok(resp),
                    _ => Err(3),
                },
                _ => Err(2),
            },
            _ => Err(1),
        }
    }

    /// get the statistics for a specific task
    ///
    /// [example cms page](https://training.olinfo.it/#/task/ois_luck/stats)
    pub fn get_stats(&self, name: String) -> Result<Stats, u8> {
        match self
            .client
            .post("https://training.olinfo.it/api/task")
            .json(&serde_json::json!({"action":"stats","name":name}))
            .send()
        {
            Ok(mut response) => match response.json::<Stats>() {
                Ok(resp) => match resp.success {
                    1 => Ok(resp),
                    _ => Err(3),
                },
                _ => Err(2),
            },
            _ => Err(1),
        }
    }

    // Submission related

    /// get your submissions for a task
    ///
    /// [example cms page](https://training.olinfo.it/#/task/preoii_piccioni/submissions)
    pub fn get_submission_list(&self, task_name: String) -> Result<SubmissionList, u8> {
        match self
            .client
            .post("https://training.olinfo.it/api/submission")
            .json(&serde_json::json!({"action":"list","task_name":task_name}))
            .send()
        {
            Ok(mut response) => match response.json::<SubmissionList>() {
                Ok(resp) => match resp.success {
                    1 => Ok(resp),
                    _ => Err(3),
                },
                _ => Err(2),
            },
            _ => Err(1),
        }
    }

    /// get details for specific submission
    ///
    /// [example is clicking on submission id](https://training.olinfo.it/#/task/preoii_piccioni/submissions)
    pub fn get_submission(&self, id: usize) -> Result<DetailedSubmission, u8> {
        match self
            .client
            .post("https://training.olinfo.it/api/submission")
            .json(&serde_json::json!({"action":"details","id":id}))
            .send()
        {
            Ok(mut response) => match response.json::<DetailedSubmission>() {
                Ok(resp) => match resp.success {
                    1 => Ok(resp),
                    _ => Err(3),
                },
                _ => Err(2),
            },
            _ => Err(1),
        }
    }

    /// get the worst execution time among all testcases for a certain submission
    ///
    /// [example is clicking on submission id and getting the highest value in the `Time` column](https://training.olinfo.it/#/task/preoii_piccioni/submissions)
    pub fn get_submission_time(&self, sub: &DetailedSubmission) -> Result<f64,()> {
        match &sub.score_details {
            Some(v) => {
                let mut res: f64 = 0.0;
                for sc in v {
                    for tc in &sc.testcases {
                        res = res.max(tc.time);
                    }
                }
                Ok(res)
            },
            _ => Err(())
        }
    }

    /// submit a not output-only task
    ///
    /// lang should be the extension of the file (c, cpp, pas)
    ///
    /// [example is clicking on `submit` button](https://training.olinfo.it/#/task/fpb/submissions)
    pub fn submit_normal(
        &self,
        task_name: String,
        text: String,
        lang: String,
    ) -> Result<DetailedSubmission, u8> {
        match self.get_task(task_name.clone()) {
            Ok(t) => {
                match self.client.post("https://training.olinfo.it/api/submission").json(&serde_json::json!({"action":"new","files":{t.submission_format[0].as_str():{"data":base64::encode(&text),"filename":format!("ace.{}",lang)}},"task_name":task_name})).send() {
                    Ok(mut response) => {
                        match response.json::<DetailedSubmission>() {
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
            Err(t) => Err(t)
        }
    }

    // Test related

    /// get the list of available tests
    ///
    /// [example cms page](https://training.olinfo.it/#/tests)
    pub fn get_test_list(&self) -> Result<TestList, u8> {
        match self
            .client
            .post("https://training.olinfo.it/api/test")
            .json(&serde_json::json!({"action":"list"}))
            .send()
        {
            Ok(mut response) => match response.json::<TestList>() {
                Ok(resp) => match resp.success {
                    1 => Ok(resp),
                    _ => Err(3),
                },
                _ => Err(2),
            },
            _ => Err(1),
        }
    }

    /// get the details and text of a specific test
    ///
    /// [example cms page](https://training.olinfo.it/#/test/scolastiche2012_c)
    pub fn get_test(&self, test_name: String) -> Result<Test, u8> {
        match self
            .client
            .post("https://training.olinfo.it/api/test")
            .json(&serde_json::json!({"action":"get","test_name":test_name}))
            .send()
        {
            Ok(mut response) => match response.json::<Test>() {
                Ok(resp) => match resp.success {
                    1 => Ok(resp),
                    _ => Err(3),
                },
                _ => Err(2),
            },
            _ => Err(1),
        }
    }

    /// Misc

    /// get a list of the regions
    ///
    /// [example cms page](https://training.olinfo.it/#/signup)
    pub fn get_region_list(&self) -> Result<RegionList, u8> {
        match self
            .client
            .post("https://training.olinfo.it/api/location")
            .json(&serde_json::json!({"action":"listregions"}))
            .send()
        {
            Ok(mut response) => match response.json::<RegionList>() {
                Ok(resp) => match resp.success {
                    1 => Ok(resp),
                    _ => Err(3),
                },
                _ => Err(2),
            },
            _ => Err(1),
        }
    }

    /// get list of technique tags
    ///
    /// [example cms page](https://training.olinfo.it/#/tags/techniques)
    pub fn get_technique_list(&self) -> Result<TechniqueList, u8> {
        match self
            .client
            .post("https://training.olinfo.it/api/tag")
            .json(&serde_json::json!({"action":"list","filter":"techniques"}))
            .send()
        {
            Ok(mut response) => match response.json::<TechniqueList>() {
                Ok(resp) => match resp.success {
                    1 => Ok(resp),
                    _ => Err(3),
                },
                _ => Err(2),
            },
            _ => Err(1),
        }
    }

    /// download file
    ///
    /// [example is an incorrect solution for missioni by Gemmady](https://training.olinfo.it/api/files/3ab02f1a746cc64fbc1fe846e46dd9e4dd2ca0e4/missioni.cpp)
    pub fn get_file(&self, file: &File) -> Result<String, u8> {
        match self
            .client
            .get(
                format!(
                    "https://training.olinfo.it/api/files/{}/{}",
                    file.digest, file.name
                )
                .as_str(),
            )
            .send()
        {
            Ok(mut response) => match response.text() {
                Ok(resp) => Ok(resp),
                _ => Err(2),
            },
            _ => Err(1),
        }
    }
}

#[cfg(test)]
mod tests {
    //preoii_flow : oo, no solves
    //preoii_crew : oo
    use super::*;
    #[test]
    fn best_times() {
        //make a client
        let m = Client::new(String::from("Gemmady"));
        let task_list = m
            .get_task_list(0, 20, String::from(""), None, None)
            .unwrap();
        let mut hm = std::collections::HashMap::<String, u32>::new();
        for i in task_list.tasks {
            let best = m.get_stats(i.name).unwrap().best;
            if best.len() > 0 {
                let t = hm.entry(best[0].username.clone()).or_insert(0);
                *t += 1;
            }
        }
        let mut v: Vec<(u32, String)> = hm.iter().map(|x| (*x.1, x.0.clone())).collect();
        v.sort();
        for i in v.iter().rev() {
            println!("{} {}", i.1, i.0);
        }
    }
    #[test]
    fn it_works() {
        let mut m = Client::new(String::from("MyK_00L"));
        println!("\n\nLOGIN\n{:?}", m.login(String::from("not")));
        println!(
            "\n\nRECOVER\n{:?}",
            m.recover(String::from("abcd@gmail.com"), String::from(""))
        );
        println!(
            "\n\nUSER_UPDATE\n{:?}",
            m.user_update(String::from(""), String::from(""), String::from(""))
        );
        println!(
            "\n\nCHECK_USERNAME\n{:?}",
            m.check_username(String::from("a"))
        );
        println!(
            "\n\nCHECK_EMAIL\n{:?}",
            m.check_email(String::from("michaelchelli00@gmail.com"))
        );
        println!(
            "\n\nCHECK_PASSWORD\n{:?}",
            m.check_password(String::from("hello"))
        );
        println!(
            "\n\nUSER_EXISTS\n{:?}",
            m.user_exists(String::from("filippos"))
        );
        println!("\n\nGET_USER_LIST\n{:?}", m.get_user_list(0, 8));
        println!("\n\nGET_USER\n{:?}", m.get_user(String::from("pollo")));
        println!(
            "\n\nGET_TASK_LIST\n{:?}",
            m.get_task_list(
                0,
                8,
                String::from("hardest"),
                Some(String::from("dp")),
                Some(String::from("sa"))
            )
        );
        println!("\n\nGET_TASK\n{:?}", m.get_task(String::from("tai_mle")));
        println!(
            "\n\nGET_STATS\n{:?}",
            m.get_stats(String::from("preoii_crew"))
        );
        println!(
            "\n\nGET_SUBMISSION_LIST\n{:?}",
            m.get_submission_list(String::from("preoii_piccioni"))
        );
        println!("\n\nGET_SUBMISSION\n{:?}", m.get_submission(666));
        println!("\n\nGET_SUBMISSION_TIME\n{:?}",m.get_submission_time(&m.get_submission(394592).unwrap()));
        println!(
            "\n\nSUBMIT_NORMAL\n{:?}",
            m.submit_normal(
                String::from("ois_cake"),
                String::from("int main(){}"),
                String::from("cpp")
            )
        );
        println!("\n\nGET_TEST_LIST\n{:?}", m.get_test_list());
        println!(
            "\n\nGET_TEST\n{:?}",
            m.get_test(String::from("scolastiche2012_pas"))
        );
        println!("\n\nGET_REGION_LIST\n{:?}", m.get_region_list());
        println!("\n\nGET_TECHNIQUE_LIST\n{:?}", m.get_technique_list());
        println!(
            "\n\nGET_FILE\n{:?}",
            m.get_file(&File {
                digest: String::from("3ab02f1a746cc64fbc1fe846e46dd9e4dd2ca0e4"),
                name: String::from("missioni.cpp")
            })
        );
    }

    #[test]
    fn my_test() {
        let mut m = Client::new(String::from("Gemmady"));
        m.login(String::from("falseisanicelanguage"));
        println!("{:?}",m.get_submission_time(&m.get_submission(394592).unwrap()));
    }
}
