use cms_tools::*;

fn main() {
    //make a client
    let client = Client::new(String::from("Gemmady"));
    let task_list = client.get_task_list(0, 1024, "", None, None).unwrap();
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
