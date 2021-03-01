use std::env;
use news::*;
use exitfailure::ExitFailure;

#[tokio::main]
async fn main() -> Result<(), ExitFailure> {
    let args: Vec<String> = env::args().collect();
    let q: Query = parse(args);
    let mut num: usize = q.limit;
    if num == 0 {
        println!("
 ███╗   ██╗███████╗██╗    ██╗███████╗
 ████╗  ██║██╔════╝██║    ██║██╔════╝
 ██╔██╗ ██║█████╗  ██║ █╗ ██║███████╗
 ██║╚██╗██║██╔══╝  ██║███╗██║╚════██║
 ██║ ╚████║███████╗╚███╔███╔╝███████║
 ╚═╝  ╚═══╝╚══════╝ ╚══╝╚══╝ ╚══════╝
                                    ");
        println!("Usage: news category limit\neg: news science 5");
        println!("Use 'news list' to list categories");
        println!("Default category is world");
    } else if q.section != "none" {
        let res = Response::new(q.section).await?;
        if num > res.num_results {
          num = res.num_results;
        }

        for i in 0..num {
            res.results[i].print(i+1);
        }
    }
    Ok(())
}

