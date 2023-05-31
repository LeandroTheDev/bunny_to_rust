use fyrox::{
    core::{
        pool::Handle,
        reflect::{FieldInfo, Reflect},
        uuid::{uuid, Uuid},
        visitor::prelude::*,
    },
    engine::resource_manager::ResourceManager,
    event::Event,
    impl_component_provider,
    scene::{
        collider::Collider,
        graph::Graph,
        node::{Node, TypeUuidProvider},
    },
    script::{ScriptContext, ScriptDeinitContext, ScriptTrait},
};

#[derive(Visit, Reflect, Default, Debug, Clone)]
pub struct FrontalCollider {
    // Add fields here.
}

impl FrontalCollider {
    pub fn has_ground_contact(handle: Handle<Node>, graph: &Graph) -> bool {
        if let Some(collider) = graph.try_get(handle).and_then(|n| n.cast::<Collider>()) {
            for contact in collider.contacts(&graph.physics) {
                for manifold in contact.manifolds.iter() {
                    if manifold.local_n1.y.abs() > 0.7 || manifold.local_n2.y.abs() > 0.7 {
                        return true;
                    }
                }
            }
        }
        false
    }
}
pub static mut IS_FRONTAL_COLLIDE: bool = false;
impl_component_provider!(FrontalCollider);

impl TypeUuidProvider for FrontalCollider {
    fn type_uuid() -> Uuid {
        uuid!("e354a7a8-99df-411c-8efc-6c97566517e0")
    }
}

impl ScriptTrait for FrontalCollider {
    fn on_init(&mut self, _context: &mut ScriptContext) {
        // Put initialization logic here.
    }

    fn on_start(&mut self, _context: &mut ScriptContext) {
        // There should be a logic that depends on other scripts in scene.
        // It is called right after **all** scripts were initialized.
    }

    fn on_deinit(&mut self, _context: &mut ScriptDeinitContext) {
        // Put de-initialization logic here.
    }

    fn on_os_event(&mut self, _event: &Event<()>, _context: &mut ScriptContext) {
        // Respond to OS events here.
    }

    fn on_update(&mut self, context: &mut ScriptContext) {
        // Put object logic here.
        unsafe {
            IS_FRONTAL_COLLIDE =
                FrontalCollider::has_ground_contact(context.handle, &context.scene.graph)
        };
    }

    fn restore_resources(&mut self, _resource_manager: ResourceManager) {
        // Restore resource handles here.
    }

    fn id(&self) -> Uuid {
        Self::type_uuid()
    }
}
