use std::io;
use futures::*;
use tokio::runtime::Runtime;
mod downloader;

fn main() -> Result<(), io::Error> {
    // let stdout = io::stdout().into_raw_mode()?;
    // let backend = TermionBackend::new(stdout);
    // let mut terminal = Terminal::new(backend)?;

    // let _res = terminal.draw(|f| {
    //     let size = f.size();
    //     let block = Block::default()
    //         .title("Block")
    //         .borders(Borders::ALL);
    //     f.render_widget(block, size);
    // });
    let runtime = Runtime::new().ok().unwrap();
    runtime.block_on(downloader::safebooru::test());
    Ok(())
}