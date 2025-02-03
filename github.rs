use octocrab::Octocrab;

pub async fn monitor_github() {
    let octocrab = Octocrab::builder().personal_token("YOUR_GITHUB_TOKEN".to_string()).build().unwrap();
    
    let repos = octocrab.repos("ethereum", "eth2.0-specs").get().await.unwrap();
    
    println!("GitHub Repo: {:?}", repos.full_name);
}
