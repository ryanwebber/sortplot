use std::{
    ops::{Coroutine, CoroutineState},
    pin::Pin,
};

use animation::Animation;
use nannou::prelude::*;

use crate::sort::{self, Swap};

const SWAP_DURATION: std::time::Duration = std::time::Duration::from_millis(50);
const SHUFFLE_DURATION: std::time::Duration = std::time::Duration::from_millis(250);
const DISPLAY_STEP_DURATION: std::time::Duration = std::time::Duration::from_millis(80);
const INTERMISSION_DURATION: std::time::Duration = std::time::Duration::from_secs(2);

// Padding around the entire window
const MAIN_PADDING: f32 = 32.0;

// Spacing between each bar
const BAR_SPACING: f32 = 2.0;

pub struct Visualizer;

impl Visualizer {
    pub fn new() -> Self {
        Self
    }

    pub fn run(self) {
        nannou::app(model).update(update).view(view).run();
    }
}

enum DisplayStep {
    Wait(std::time::Duration),
    Reset {
        data: Vec<usize>,
        algorithm: &'static str,
    },
    Swap(Swap),
}

struct Controller {
    generator: Pin<Box<dyn Coroutine<(), Yield = DisplayStep, Return = !>>>,
}

impl Controller {
    fn new(data_count: usize) -> Self {
        Self {
            generator: Box::pin(move || loop {
                for algorithm in sort::ALGORITHMS {
                    // Create new shuffled data
                    let data = {
                        use rand::seq::SliceRandom;
                        let mut rng = rand::thread_rng();
                        let mut data = (0..data_count).collect::<Vec<_>>();
                        data.shuffle(&mut rng);
                        data
                    };

                    yield DisplayStep::Wait(INTERMISSION_DURATION);
                    yield DisplayStep::Reset {
                        data: data.to_vec(),
                        algorithm: algorithm.name,
                    };

                    yield DisplayStep::Wait(INTERMISSION_DURATION);
                    let mut sorter = (algorithm.algorithm)(sort::SortData::new(data));
                    while let Some(swap) = sorter.step() {
                        yield DisplayStep::Swap(swap);
                        yield DisplayStep::Wait(DISPLAY_STEP_DURATION);
                    }
                }
            }),
        }
    }

    fn next(&mut self) -> DisplayStep {
        let generator = std::pin::pin!(&mut self.generator);
        match Coroutine::resume(generator, ()) {
            CoroutineState::Yielded(step) => step,
            CoroutineState::Complete(_) => unreachable!(),
        }
    }
}

struct Model {
    bars: Vec<Bar>,
    controller: Controller,
    wait_until: std::time::Duration,
    current_algorithm: Option<&'static str>,
}

impl Model {
    fn update(&mut self, update: Update) {
        if update.since_start < self.wait_until {
            return;
        }

        loop {
            match self.controller.next() {
                DisplayStep::Wait(duration) => {
                    self.wait_until = update.since_start + duration;
                    break;
                }
                DisplayStep::Reset { data, algorithm } => {
                    self.current_algorithm = Some(algorithm);
                    self.bars.sort_by(|a, b| a.value.cmp(&b.value));
                    for (i, value) in data.iter().enumerate() {
                        self.bars[*value].animate(
                            update.since_start + std::time::Duration::from_millis(i as u64 * 2),
                            i,
                            SHUFFLE_DURATION,
                        );
                    }
                }
                DisplayStep::Swap(swap) => {
                    let (i, j) = (swap.0, swap.1);
                    let bar_idx_i = self.bars.iter().position(|bar| bar.index == i).unwrap();
                    let bar_idx_j = self.bars.iter().position(|bar| bar.index == j).unwrap();
                    self.bars[bar_idx_i].animate(update.since_start, j, SWAP_DURATION);
                    self.bars[bar_idx_j].animate(update.since_start, i, SWAP_DURATION);
                }
            }
        }
    }
}

fn model(app: &App) -> Model {
    _ = app
        .new_window()
        .view(view)
        .build()
        .expect("Failed to build application window");

    const NUM_BARS: usize = 100;

    let controller = Controller::new(NUM_BARS);
    let bars = (0..NUM_BARS)
        .enumerate()
        .map(|(i, v)| {
            Bar::new(
                i,
                v,
                hsva(0.8 * (i as f32) / (NUM_BARS as f32), 0.8, 0.8, 0.4),
            )
        })
        .collect();

    Model {
        bars,
        controller,
        wait_until: std::time::Duration::ZERO,
        current_algorithm: None,
    }
}

fn update(_: &App, model: &mut Model, update: Update) {
    model.update(update);
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(gray(0.05));

    let layout = Layout {
        width: app.window_rect().w(),
        height: app.window_rect().h(),
        num_bars: model.bars.len(),
    };

    let elapsed = app.duration.since_start;
    for bar in &model.bars {
        bar.render(elapsed, &layout, &draw);
    }

    if let Some(algorithm) = model.current_algorithm {
        draw.text(algorithm)
            .x_y(0.0, layout.height / 2.0 - 12.0)
            .w(layout.width - 24.0)
            .font_size(12)
            .left_justify()
            .color(WHITE);
    }

    draw.to_frame(app, &frame).unwrap();
}

struct Layout {
    width: f32,
    height: f32,
    num_bars: usize,
}

impl Layout {
    fn frame_bar_element(&self, interpolated_index: f32, height: usize) -> Rect {
        // Available height for the bar
        let available_height = self.height - 2.0 * MAIN_PADDING;

        // The actual height of the bar, given it's value
        let bar_height = (height + 1) as f32 / self.num_bars as f32 * available_height;

        // Available width for each bar, accounting for padding and spacing
        let bar_width =
            (self.width - 2.0 * MAIN_PADDING - (self.num_bars as f32 - 1.0) * BAR_SPACING)
                / self.num_bars as f32;

        // The x position of the bar
        let x = MAIN_PADDING + interpolated_index * (bar_width + BAR_SPACING);

        // The y position of the bar
        let y = MAIN_PADDING + available_height - bar_height;

        // The window origin is at the center, and rects are drawn from their center,
        // so we need to offset our x and y positions
        let x = x - self.width / 2.0 + bar_width / 2.0;
        let y = y - self.height / 2.0 + bar_height / 2.0;

        Rect::from_x_y_w_h(x, -y, bar_width, bar_height)
    }
}

struct Bar {
    index: usize,
    value: usize,
    color: Hsva,
    animation: Option<Animation<f32>>,
}

impl Bar {
    fn new(index: usize, value: usize, color: Hsva) -> Self {
        Self {
            index,
            value,
            color,
            animation: None,
        }
    }

    fn animate(&mut self, start: std::time::Duration, index: usize, duration: std::time::Duration) {
        let initial_value = self.index as f32;
        let target_value = index as f32;
        self.animation = Some(Animation::new(
            initial_value,
            target_value,
            start,
            duration,
            animation::Easing::EaseInOut,
        ));

        self.index = index;
    }

    fn render(&self, time: std::time::Duration, layout: &Layout, draw: &Draw) {
        let interpolated_index = 'index: {
            if let Some(animation) = &self.animation {
                if let animation::Step::Updated(value) = animation.evaluate(time) {
                    break 'index value;
                }
            }

            self.index as f32
        };

        let frame = layout.frame_bar_element(interpolated_index, self.value);
        draw.rect().xy(frame.xy()).wh(frame.wh()).color(self.color);
    }
}

mod animation {

    pub enum Step<T> {
        Complete,
        Updated(T),
    }

    pub enum Easing {
        EaseInOut,
    }

    impl Easing {
        fn apply(&self, t: f32) -> f32 {
            match self {
                Easing::EaseInOut => {
                    let sqr = t * t;
                    sqr / (2.0 * (sqr - t) + 1.0)
                }
            }
        }
    }

    pub trait Interpolatable {
        fn interpolate(&self, other: &Self, t: f32) -> Self;
    }

    pub struct Interpolation<T: Interpolatable> {
        initial_value: T,
        target_value: T,
    }

    impl<T> Interpolation<T>
    where
        T: Interpolatable,
    {
        fn new(initial_value: T, target_value: T) -> Self {
            Self {
                initial_value,
                target_value,
            }
        }

        fn interpolate(&self, t: f32) -> T {
            T::interpolate(&self.initial_value, &self.target_value, t)
        }
    }

    pub struct Animation<T: Interpolatable> {
        start: std::time::Duration,
        duration: std::time::Duration,
        easing: Easing,
        interpolation: Interpolation<T>,
    }

    impl<T> Animation<T>
    where
        T: Interpolatable,
    {
        pub fn new(
            initial_value: T,
            target_value: T,
            start: std::time::Duration,
            duration: std::time::Duration,
            easing: Easing,
        ) -> Self {
            Self {
                start,
                duration,
                easing,
                interpolation: Interpolation::new(initial_value, target_value),
            }
        }

        pub fn evaluate(&self, time: std::time::Duration) -> Step<T> {
            let elapsed = time.saturating_sub(self.start);
            if elapsed >= self.duration {
                Step::Complete
            } else {
                let t = elapsed.as_secs_f32() / self.duration.as_secs_f32();
                Step::Updated(self.interpolation.interpolate(self.easing.apply(t)))
            }
        }
    }

    impl<T> Interpolatable for T
    where
        T: Copy
            + std::ops::Add<Output = T>
            + std::ops::Sub<Output = T>
            + std::ops::Mul<f32, Output = T>,
    {
        fn interpolate(&self, other: &Self, t: f32) -> Self {
            self.clone() + (*other - *self) * t
        }
    }
}
