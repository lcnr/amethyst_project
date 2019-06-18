use amethyst::{
    core::transform::TransformBundle,
    ecs::Resources,
    prelude::*,
    renderer::{
        types::DefaultBackend, Factory, GraphBuilder, GraphCreator,
        RenderingSystem
    },
    window::{WindowBundle, DisplayConfig},
};

pub struct Start;

impl SimpleState for Start {
    fn on_start(&mut self, _data: StateData<'_, GameData<'_, '_>>) {}

    fn update(&mut self, _data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        panic!();
    }
}

fn main() -> amethyst::Result<()> {
    let game_data = GameDataBuilder::default()
        // The WindowBundle provides all the scaffolding for opening a window
        .with_bundle(WindowBundle::from_config(DisplayConfig::default()))?
        // Add the transform bundle which handles tracking entity positions
        .with_bundle(TransformBundle::new())?
        // The renderer must be executed on the same thread consecutively, so we initialize it as thread_local
        // which will always execute on the main thread.
        .with_thread_local(RenderingSystem::<DefaultBackend, _>::new(
            ExampleGraph,
        ));

    let mut game = Application::build(".Application/", Start)?
        .build(game_data)?;

    game.run();
    Ok(())
}

struct ExampleGraph;

impl GraphCreator<DefaultBackend> for ExampleGraph {
    // This trait method reports to the renderer if the graph must be rebuilt, usually because
    // the window has been resized. This implementation checks the screen size and returns true
    // if it has changed.
    fn rebuild(&mut self, _res: &Resources) -> bool {
        false
    }

    // This is the core of a RenderGraph, which is building the actual graph with subpasses and target
    // images.
    fn builder(
        &mut self,
        _factory: &mut Factory<DefaultBackend>,
        _res: &Resources,
    ) -> GraphBuilder<DefaultBackend, Resources> {
        GraphBuilder::new()
    }
}
