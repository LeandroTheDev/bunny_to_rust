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
pub struct FootCollider {
    // Add fields here.
}
impl FootCollider {
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
pub static mut IS_ON_AIR: bool = false;
impl_component_provider!(FootCollider);

impl TypeUuidProvider for FootCollider {
    fn type_uuid() -> Uuid {
        uuid!("1a296833-770e-411f-9205-cc5d29f2d8af")
    }
}

impl ScriptTrait for FootCollider {
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
        //Check if is not in air
        unsafe {
            IS_ON_AIR = FootCollider::has_ground_contact(context.handle, &context.scene.graph)
        };
    }

    fn restore_resources(&mut self, _resource_manager: ResourceManager) {
        // Restore resource handles here.
    }

    fn id(&self) -> Uuid {
        Self::type_uuid()
    }
}
