//! # Examples:
//! * Print list of people with highest number of fastest solutions
//! ```
//! //make a client
//! let client = Client::new(String::from("Gemmady"));
//! let task_list = client.get_task_list(0, 1024, "", None, None).unwrap();
//! let mut hm = std::collections::HashMap::<String, u32>::new();
//! for i in task_list.tasks {
//!     let best = client.get_stats(&i.name).unwrap().best;
//!     if best.len() > 0 {
//!         let t = hm.entry(best[0].username.clone()).or_insert(0);
//!         *t += 1;
//!     }
//! }
//! let mut v: Vec<(u32, String)> = hm.iter().map(|x| (*x.1, x.0.clone())).collect();
//! v.sort();
//! for i in v.iter().rev() {
//!     println!("{} {}", i.1, i.0);
//! }
//! ```
//! * Resubmit all fastest solutions
//! ```
//! let username = String::from("user");
//! let password = "password";
//! let mut client = Client::new(username.clone());
//! client.login(password).unwrap();
//! let user = client.get_user(&username).unwrap();
//! for sc in user.scores.unwrap() {
//!     if sc.score == 100.0 {
//!         println!("{} has score 100", sc.title);
//!         let sub_list = client.get_submission_list(&sc.name).unwrap();
//!         let best_sub = sub_list.get_fastest_high(&client).unwrap();
//!         let files = &best_sub.files;
//!         if files.len() == 1 { // if it is not an output-only
//!             let mut submitted: bool = false;
//!             print!("Resubmitting {}", sc.name.clone());
//!             while !submitted { // because cmsocial has a limit to submission rate
//!                 print!(".");
//!                 if client
//!                     .submit_normal(
//!                         &sc.name,
//!                         &client.get_file(&files[0]).unwrap(),
//!                         files[0].name.split(".").collect::<Vec<&str>>().last().unwrap(),
//!                     )
//!                     .is_ok()
//!                 {
//!                     submitted = true;
//!                 }
//!             }
//!             println!("");
//!         }
//!     }
//! }
//! ```
//! # Not implemented yet:
//! * submission for output-only problems [example](https://training.olinfo.it/#/task/preoii_flow/submissions)
//! * test submission request [example is `invia` button at the bottom](https://training.olinfo.it/#/test/scolastiche2012_c), also that button does not have english localization
//! * admin requests ([this](https://github.com/algorithm-ninja/cmsocial/blob/88bb6e8992455d2d780c33214e895d8d3f5e63ed/cmsocial-web/scripts/admin.js#L38))
//! * any request I do not know the existence of

// User management related

/// Response for email and username checking
#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct CheckResponse {
    pub success: u8,
    pub error: Option<String>,
}

/// Responses for password recovery requests
#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct RecoverResponse {
    pub message: Option<String>,
    #[serde(rename = "type")]
    pub thing_type: u8,
    pub success: u8,
    pub error: Option<String>,
}

// Users related

/// Institute of an user
#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct Institute {
    pub province: Option<String>,
    pub city: Option<String>,
    pub region: Option<String>,
    pub id: Option<u32>,
    pub name: Option<String>,
}

/// User info
#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct User {
    pub username: String,
    pub mail_hash: String,
    pub first_name: String,
    pub last_name: String,
    pub institute: Institute,
    pub tasks_solved: i32,
    pub access_level: u8,
    pub join_date: f64,
    pub score: u32,
    pub global_access_level: u8,
    pub scores: Option<Vec<Score>>,
}

/// List of users
#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct UserList {
    pub num: usize,
    pub users: Vec<User>,
    pub success: u8,
}

// Task related

/// digest of statement file? apparently not
#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct Statement {
    pub it: Option<String>,
}

/// tags, like technique or event tags
#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct Tag {
    pub name: String,
    pub can_delete: bool,
}

/// Detailed task description
#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct DetailedTask {
    pub time_limit: Option<f64>,
    pub help_available: bool,
    pub statements: Statement,
    pub name: String,
    pub success: u8,
    pub title: String,
    pub submission_format: Vec<String>,
    pub memory_limit: Option<u16>,
    pub task_type: String,
    pub score_multiplier: f64,
    pub id: usize,
    pub tags: Vec<Tag>,
    pub attachments: Vec<Vec<String>>,
}

/// Task in a TaskList
#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct Task {
    pub score: Option<f64>,
    pub title: String,
    pub score_multiplier: f64,
    pub id: usize,
    pub name: String,
}

/// List of tasks
#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct TaskList {
    pub tasks: Vec<Task>,
    pub num: usize,
    pub success: u8,
}

/// Best time by someone on some task
#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct Stat {
    pub username: String,
    pub time: f64,
}

/// Stats of a certain task
#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct Stats {
    pub nsubscorrect: usize,
    pub success: u8,
    pub nusers: usize,
    pub nsubs: usize,
    pub nuserscorrect: usize,
    pub best: Vec<Stat>,
}

// Submission related

/// Score achieved on a certain task by someone
#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct Score {
    pub score: f64,
    pub name: String,
    pub title: String,
}

/// Description of a file
#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct File {
    pub name: String,
    pub digest: String,
}

/// Submission, not detailed
#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct Submission {
    pub files: Vec<File>,
    pub compilation_outcome: Option<String>,
    pub task_id: usize,
    pub timestamp: f64,
    pub evaluation_outcome: Option<String>,
    pub score: Option<f64>,
    pub id: usize,
}

/// Outcome of a certain submission for a specific testcase
#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct Testcase {
    pub text: String,
    pub outcome: String,
    pub time: f64,
    pub idx: Option<String>,
    pub memory: u64,
}

/// Score details for a specific submission
#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct ScoreDetail {
    pub testcases: Vec<Testcase>,
    pub score: f64,
    pub max_score: f64,
    pub idx: Option<usize>,
}

/// Details of a specific submission
#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct DetailedSubmission {
    pub files: Vec<File>,
    pub compilation_outcome: Option<String>,
    pub task_id: usize,
    pub language: Option<String>,
    pub score_details: Option<Vec<ScoreDetail>>,
    pub timestamp: f64,
    pub compilation_stderr: Option<String>,
    pub compilation_time: Option<f64>,
    pub evaluation_outcome: Option<String>,
    pub score: Option<f64>,
    pub compilation_stdout: Option<String>,
    pub success: u8,
    pub id: usize,
    pub compilation_memory: Option<u64>,
}
impl DetailedSubmission {
    /// get the maximum execution time among all testcases
    pub fn get_time(&self) -> Option<f64> {
        match &self.score_details {
            Some(sc) => {
                let mut res: f64 = 0.0;
                for subtask in sc {
                    for testcase in &subtask.testcases {
                        res = res.max(testcase.time);
                    }
                }
                Some(res)
            }
            _ => None,
        }
    }
    /// get the maximum memory used among all testcases
    pub fn get_memory(&self) -> Option<u64> {
        match &self.score_details {
            Some(sc) => {
                let mut res: u64 = 0;
                for subtask in sc {
                    for testcase in &subtask.testcases {
                        res = res.max(testcase.memory);
                    }
                }
                Some(res)
            }
            _ => None,
        }
    }
}

/// List of submissions by a user for a task
#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct SubmissionList {
    pub submissions: Vec<Submission>,
    pub success: u8,
}
impl SubmissionList {
    /// get the best score achieved among all submissions
    pub fn best_score(&self) -> f64 {
        let mut res: f64 = 0.0;
        for sub in &self.submissions {
            if let Some(score) = sub.score {
                res = res.max(score);
            }
        }
        res
    }
    /// get the last submission among those with highest score
    pub fn get_last_high(&self) -> Option<&Submission> {
        let mut best_score: f64 = -1.0;
        let mut best_ind: usize = self.submissions.len();
        for i in 0..self.submissions.len() {
            if let Some(score) = self.submissions[i].score {
                if score > best_score {
                    best_score = score;
                    best_ind = i;
                    if score > 100.0 {
                        break;
                    }
                }
            }
        }
        if best_ind == self.submissions.len() {
            return None;
        }
        return Some(&self.submissions[best_ind]);
    }
    /// get the fastest submission among those with highest score
    ///
    /// in case of parity, the latest is returned
    pub fn get_fastest_high(&self, client: &Client) -> Option<&Submission> {
        let best_score: f64 = self.best_score();
        let mut best_time: f64 = 100.0;
        let mut best_ind: usize = self.submissions.len();
        for i in 0..self.submissions.len() {
            if let Some(score) = self.submissions[i].score {
                if score == best_score {
                    if let Ok(sub) = client.get_submission(self.submissions[i].id) {
                        if let Some(time) = sub.get_time() {
                            if time < best_time {
                                best_time = time;
                                best_ind = i;
                            }
                        }
                    }
                }
            }
        }
        if best_ind == self.submissions.len() {
            return None;
        }
        return Some(&self.submissions[best_ind]);
    }
}

// Test related

/// Basic informations of a test
#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct TestHead {
    pub max_score: u8,
    pub name: String,
    pub description: String,
}

/// Question inside of a test
#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct Question {
    pub max_score: u8,
    pub text: String,
    #[serde(rename = "type")]
    pub question_type: String,
    pub choices: Option<Vec<String>>,
    pub answers: Option<Vec<(String, u32)>>,
}

/// Test, also known as Quiz
#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct Test {
    pub success: u8,
    pub name: String,
    pub questions: Vec<Question>,
    pub description: String,
}

/// List of Tests
#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct TestList {
    pub tests: Vec<TestHead>,
    pub success: u8,
}

/// Region
#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct Region {
    pub id: usize,
    pub name: String,
}

// Misc

/// List of regions
#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct RegionList {
    pub regions: Vec<Region>,
    pub success: u8,
}

/// List of `technique` tags
#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct TechniqueList {
    pub success: u8,
    pub tags: Vec<String>,
}

// Client

/// **Client** you will do almost everything with
pub struct Client {
    /// The reqwest client
    pub client: reqwest::Client,
    /// username
    pub username: String,
    /// if client has token, this should be true and false otherwise
    pub logged: bool,
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
    pub fn login(&mut self, password: &str) -> Result<bool, u8> {
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
    pub fn recover(&self, email: &str, code: &str) -> Result<RecoverResponse, u8> {
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
    pub fn user_update(&self, email: &str, password: &str, old_password: &str) -> Result<(), u8> {
        match self.client.post("https://training.olinfo.it/api/user").json(&serde_json::json!({"action":"update","email":email,"old_password":old_password,"password":password})).send() {
            Ok(mut response) => {
                #[derive(serde::Serialize, serde::Deserialize, Debug)]
                struct Resp {
                    success: u8
                }
                match response.json::<Resp>() {
                    Ok(resp) => {
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
    pub fn check_username(&self, username: &str) -> Result<CheckResponse, u8> {
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
    pub fn check_email(&self, email: &str) -> Result<CheckResponse, u8> {
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
    pub fn check_password(&self, password: &str) -> bool {
        password.len() > 4
    }

    // Users related

    /// check if there is an user with username = username
    ///
    /// [example is `Username` input](https://training.olinfo.it/#/signup)
    pub fn user_exists(&self, username: &str) -> Result<bool, u8> {
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
    pub fn get_user(&self, username: &str) -> Result<User, u8> {
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
        order: &str,
        tag: Option<&str>,
        search: Option<&str>,
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
    pub fn get_task(&self, name: &str) -> Result<DetailedTask, u8> {
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
    pub fn get_stats(&self, name: &str) -> Result<Stats, u8> {
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
    pub fn get_submission_list(&self, task_name: &str) -> Result<SubmissionList, u8> {
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

    /// submit a not output-only task
    ///
    /// lang should be the extension of the file (c, cpp, pas)
    ///
    /// [example is clicking on `submit` button](https://training.olinfo.it/#/task/fpb/submissions)
    pub fn submit_normal(
        &self,
        task_name: &str,
        text: &str,
        lang: &str,
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
    pub fn get_test(&self, test_name: &str) -> Result<Test, u8> {
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
        let client = Client::new(String::from("Gemmady"));
        let task_list = client.get_task_list(0, 20, "", None, None).unwrap();
        let mut hm = std::collections::HashMap::<String, u32>::new();
        for i in task_list.tasks {
            let best = client.get_stats(&i.name).unwrap().best;
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
        println!("\n\nLOGIN\n{:?}", m.login("w"));
        println!("\n\nRECOVER\n{:?}", m.recover("abcd@gmail.com", ""));
        println!("\n\nUSER_UPDATE\n{:?}", m.user_update("", "", ""));
        println!("\n\nCHECK_USERNAME\n{:?}", m.check_username("a"));
        println!(
            "\n\nCHECK_EMAIL\n{:?}",
            m.check_email("michaelchelli00@gmail.com")
        );
        println!("\n\nCHECK_PASSWORD\n{:?}", m.check_password("hello"));
        println!("\n\nUSER_EXISTS\n{:?}", m.user_exists("filippos"));
        println!("\n\nGET_USER_LIST\n{:?}", m.get_user_list(0, 8));
        println!("\n\nGET_USER\n{:?}", m.get_user("pollo"));
        println!(
            "\n\nGET_TASK_LIST\n{:?}",
            m.get_task_list(0, 8, "hardest", Some("dp"), Some("sa"))
        );
        println!("\n\nGET_TASK\n{:?}", m.get_task("tai_mle"));
        println!("\n\nGET_STATS\n{:?}", m.get_stats("preoii_crew"));
        println!(
            "\n\nGET_SUBMISSION_LIST\n{:?}",
            m.get_submission_list("preoii_piccioni")
        );
        println!("\n\nGET_SUBMISSION\n{:?}", m.get_submission(666));
        println!(
            "\n\nSUBMIT_NORMAL\n{:?}",
            m.submit_normal("ois_cake", "int main(){}", "cpp")
        );
        println!("\n\nGET_TEST_LIST\n{:?}", m.get_test_list());
        println!("\n\nGET_TEST\n{:?}", m.get_test("scolastiche2012_pas"));
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
        let mut m = Client::new(String::from("MyK_00L"));
        println!("{:?}", m.login("sure"));
        let id = m
            .get_submission_list("tai_mle")
            .unwrap()
            .get_fastest_high(&m)
            .unwrap()
            .id;
        let sub = m.get_submission(id).unwrap();
        println!("{}: {}", id, sub.get_memory().unwrap());
    }
}
