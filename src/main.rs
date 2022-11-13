use open;
use git2::{Repository};
use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(about = "Additional git tools")]
enum GitPlus {
    Open {name: Option<String>},
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = GitPlus::from_args();
    match args {
        GitPlus::Open { name } => {
            let name = name.unwrap_or("origin".to_string());
            let repo = Repository::open(".")?;
            match repo.find_remote(name.as_str())?.url() {
                Some(url) => open::that(url).unwrap(),
                None => println!("No origin found"),
            };   
        }
    }
    Ok(())
}
