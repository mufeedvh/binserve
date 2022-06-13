mod cli;
mod core;

fn main() -> anyhow::Result<()> {
    // print a cool banner!
    cli::interface::banner();

    // engine takes off!
    core::engine::init()
}
