#![feature(coroutines)]
#![feature(coroutine_trait)]
#![feature(stmt_expr_attributes)]
#![feature(never_type)]

mod sort;
mod ui;

fn main() {
    ui::Visualizer::new().run();
}
