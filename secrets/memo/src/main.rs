use std::io::Result;

use memo::{Memo, Memos, Status};

use clap::Parser;

#[derive(Parser)]
/// Stores and manages simple reminders.
struct Args {
    /// Marks all matching memos as done
    #[arg(short, long)]
    done: bool,
    /// Removes all done memos, leaving only pending reminders
    #[arg(short, long)]
    purge: bool,
    /// Text of the memo to store or mark as done
    text: Vec<String>,
}

fn main() -> Result<()> {
    // let args: Vec<_> = env::args().skip(1).collect();
    // let mut memos = Memos::open("memos.json")?;
    // if args.is_empty() {
    //     for memo in memos.inner {
    //         println!("{memo}");
    //     }
    // } else {
    //     memos.inner.push(Memo {
    //         text: args.join(" "),
    //         status: Status::Pending,
    //     });
    //     memos.sync()?;
    // }

    let args = Args::parse();
    let mut memos = Memos::open("memos.json")?;
    let text = args.text.join(" ");
    if args.done {
        for m in memos.find_all(&text) {
            m.status = Status::Done;
            println!("Marked \"{}\" as done.", m.text);
        }
        memos.sync()?;
    } else if args.purge {
        memos.purge_done();
        memos.sync()?;
    } else if args.text.is_empty() {
        for memo in &memos.inner {
            println!("{memo}");
        }
    } else {
        memos.inner.push(Memo {
            text: text.clone(),
            status: Status::Pending,
        });
        memos.sync()?;
    }
    Ok(())
}
