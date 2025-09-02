use bevy::{
    prelude::*,
    render::{
        RenderApp,
        WgpuWrapper,
        renderer::{
            RenderAdapter, RenderAdapterInfo, RenderDevice, RenderInstance, RenderQueue,
        },
    },
};
use burn::{
    backend::wgpu::{
        RuntimeOptions as BurnRuntimeOptions, WgpuDevice as BurnWgpuDevice,
        WgpuSetup as BurnWgpuSetup, init_device as init_burn_device,
    },
    prelude::Backend,
    tensor::{Shape, Tensor},
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(BurnPlugin)
        .add_systems(Startup, test_burn)
        .run();
}

struct BurnPlugin;

#[derive(Resource, Deref, DerefMut, Clone, Debug, Hash, PartialEq, Eq)]
struct BurnDevice(BurnWgpuDevice);

type BurnBackend = burn::backend::Wgpu<f32, i32>;

impl Plugin for BurnPlugin {
    fn build(&self, _app: &mut App) {}
    fn finish(&self, app: &mut App) {
        let render_app = app
            .get_sub_app_mut(RenderApp)
            .expect("Failed to setup Burn plugin: RenderApp not found");

        let bevy_adapter = render_app.world().resource::<RenderAdapter>();
        let wgpu_adapter = unwrap_wgpu_wrapper(&bevy_adapter.0);

        let bevy_device = render_app.world().resource::<RenderDevice>();
        let wgpu_device = bevy_device.wgpu_device().clone();

        let bevy_instance = render_app.world().resource::<RenderInstance>();
        let wgpu_instance = unwrap_wgpu_wrapper(&bevy_instance.0);

        let bevy_queue = render_app.world().resource::<RenderQueue>();
        let wgpu_queue = unwrap_wgpu_wrapper(&bevy_queue.0);

        let render_adapter_info = render_app.world().resource::<RenderAdapterInfo>();
        let wgpu_backend = render_adapter_info.backend;

        let wgpu_setup = BurnWgpuSetup {
            adapter: wgpu_adapter,
            device: wgpu_device,
            instance: wgpu_instance,
            queue: wgpu_queue,
            backend: wgpu_backend,
        };

        let runtime_options = BurnRuntimeOptions::default();
        let burn_device = init_burn_device(wgpu_setup, runtime_options);

        app.insert_resource(BurnDevice(burn_device));
    }
}

fn unwrap_wgpu_wrapper<T: Clone>(wrapper: &WgpuWrapper<T>) -> T {
    <WgpuWrapper<T> as Clone>::clone(wrapper).into_inner()
}

fn test_burn(burn_device: Res<BurnDevice>) {
    some_burn_function::<BurnBackend>(burn_device.0.clone());
}

fn some_burn_function<B: Backend>(device: B::Device) {
    let tensor = Tensor::<B, 2>::ones(Shape::new([2, 3]), &device);
    info!("{tensor}");
}
