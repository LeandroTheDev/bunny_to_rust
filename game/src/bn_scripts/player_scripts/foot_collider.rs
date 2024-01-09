use fyrox::{
    core::{
        pool::Handle,
        reflect::prelude::*,
        uuid::{uuid, Uuid},
        visitor::prelude::*,
        TypeUuidProvider,
    },
    impl_component_provider,
    scene::{collider::Collider, graph::Graph, node::Node},
    script::{ScriptContext, ScriptTrait},
};

#[derive(Visit, Reflect, Default, Debug, Clone)]
pub struct FootCollider {
    pub is_on_air: bool,
}
impl FootCollider {
    pub fn has_ground_contact(&mut self, handle: Handle<Node>, graph: &Graph) -> bool {
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
    pub fn is_in_air(&mut self) -> bool {
        return self.is_on_air;
    }
}
impl_component_provider!(FootCollider);

impl TypeUuidProvider for FootCollider {
    fn type_uuid() -> Uuid {
        uuid!("1a296833-770e-411f-9205-cc5d29f2d8af")
    }
}

impl ScriptTrait for FootCollider {
    fn on_init(&mut self, _context: &mut ScriptContext) {
        // Declaring variables
        self.is_on_air = false;
    }

    fn on_update(&mut self, context: &mut ScriptContext) {
        // Updating the variable
        self.is_on_air = !self.has_ground_contact(context.handle, &context.scene.graph);
    }

    fn id(&self) -> Uuid {
        Self::type_uuid()
    }
}
