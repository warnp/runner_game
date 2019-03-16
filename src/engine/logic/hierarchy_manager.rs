use std::cell::RefCell;
use std::rc::Rc;
use cgmath::Matrix4;
use cgmath::prelude::*;
use super::super::graphic::generic_object::GenericObject;
use super::super::graphic::generic_object_type::GenericObjectType;
use super::entity::Entity;

/// Hierarchy manager
///
/// Handle all logical entities in engine
#[derive(Debug, Clone)]
pub struct HierarchyManager {
    pub entities: RefCell<Vec<Rc<RefCell<Entity>>>>,
}

impl HierarchyManager {
    /// Construct a new Hierarchy manager with world as root.
    /// world have an identity matrix.
    ///
    /// #Example
    /// ```
    /// use engine::logic::hierarchy_manager::HierarchyManager;
    ///
    /// let manager = HierarchyManager::new();
    /// ```
    pub fn new() -> HierarchyManager {
        let entities = vec![Rc::new(RefCell::new(Entity {
            mesh_name: "".to_string(),
            matrix: RefCell::new(Matrix4::identity()),
            local_matrix: Matrix4::identity(),
            children: RefCell::new(vec![]),
            name: "world".to_string(),
            parent: RefCell::new(None),
        }))];
        HierarchyManager {
            entities: RefCell::new(entities)
        }
    }

    /// Add en entity to Hierarchy manager.
    /// If it doesn't find a parent to add this entity, it appends it to tree root, ie same level as world.
    ///
    /// #Example
    /// ```
    /// use engine::logic::hierarchy_manager::HierarchyManager;
    ///
    /// let manager = HierarchyManager::new();
    /// ```
    pub fn push_new_entity(&self, parent_name: &str, entity: Entity) {
        let mut entity = entity.clone();
        let vec = self.entities.borrow().clone();

        let x = vec.iter().filter(|x| x.borrow().name == parent_name.to_string()).collect::<Vec<&Rc<RefCell<Entity>>>>();
        let parent = x.get(0);


        if let Some(parent) = parent {
            {
                entity.parent = RefCell::new(Some(Rc::downgrade(&parent.clone())));
            }
            {
                let parent_mut = parent.clone().borrow();
                let mut vec = parent_mut.children.borrow_mut();
                vec.push(Rc::new(RefCell::new(entity)));
            }
        } else {
            let x = {
                let mut result = false;
                for entity_to_append_child in &vec {
                    let entity = entity.clone();
                    if HierarchyManager::recur_insert(parent_name, entity_to_append_child.clone(), entity) {
                        result = true;
                        break;
                    }
                }
                result
            };

            if !x {
                let mut entities_mut = self.entities.borrow_mut();
                entities_mut.push(Rc::new(RefCell::new(entity)));
            }
        }
    }

    fn recur_insert(parent_name: &str, parent_entity: Rc<RefCell<Entity>>, child_to_append: Entity) -> bool {
        if let Ok(parent_borrowed) = parent_entity.try_borrow() {
            let children_borrowed = parent_borrowed.children.borrow();

            for child in children_borrowed.iter() {
                let rc = child.clone();
                let borrowed_cloned_child = rc.borrow();
                if borrowed_cloned_child.name == parent_name {
                    let mut child_to_append_to_modify = child_to_append.clone();
                    child_to_append_to_modify.parent = RefCell::new(Some(Rc::downgrade(&rc.clone())));
                    let mut children_mut = borrowed_cloned_child.children.borrow_mut();
                    children_mut.push(Rc::new(RefCell::new(child_to_append_to_modify)));
                    return true;
                }
            }
            return false;
        }
        return false;
    }

    /// Get a flatten tree of GenericObject that graphic engine can handle
    pub fn get_flat_generic_objects<T: GenericObject>(&self) -> Vec<GeneratedObj> {
        let mut tmp_result = vec![];
        for entity in self.entities.borrow().iter() {
            tmp_result.push(entity.clone());
            for e in HierarchyManager::recur_flatten_tree(entity.clone()) {
                tmp_result.push(e.clone());
            }
        }

        let mut final_result = vec![];
        for res in &tmp_result {
            let local = res.borrow();
            let name = local.name.clone();
            let mesh = local.mesh_name.clone();
            let generated = GeneratedObj {
                matrix: *local.matrix.borrow(),
                name: name,
                description: String::new(),
                mesh: mesh,
                order: 0u8,
                obj_type: GenericObjectType::STATIC_MESH,
                size: (1.0, 1.0, 1.0),
                texture_id: 0,
                texture_coordinate: ((0.0, 0.0), (0.0, 0.0), (0.0, 0.0), (0.0, 0.0)),
            };

            final_result.push(generated);
        }

        final_result
    }

    fn recur_flatten_tree(parent_entity: Rc<RefCell<Entity>>) -> Vec<Rc<RefCell<Entity>>> {
        let mut result = vec![];
        if let Ok(parent_entity_borrowed) = parent_entity.try_borrow() {
            for entity in parent_entity_borrowed.children.borrow().iter() {
                result.push(entity.clone());
                for e in HierarchyManager::recur_flatten_tree(entity.clone()) {
                    result.push(e.clone());
                }
            }
        }
        result
    }

    /// Update all entities matrix in tree.
    pub fn update_matrix(&self) {
        for entity in self.entities.borrow().iter() {
            HierarchyManager::recur_update_matrix(entity.clone());
        }
    }

    /// Get entity by name
    pub fn get_entity(&self, entity_name: String) -> Option<Rc<RefCell<Entity>>> {
        for entity in self.entities.borrow().iter() {
            let option = HierarchyManager::recur_find_entity(entity_name.clone(), entity.clone());
            if let Some(ent) = option {
                return Some(ent);
            }
        }
        None
    }

    pub fn update_children(&self, entity_name: String) {
        if let Some(entity) = self.get_entity(entity_name) {
            let cloned_entity = entity.clone();
            let borrowed_entity = cloned_entity.borrow();
            for e in borrowed_entity.children.borrow().iter() {
                HierarchyManager::recur_update_matrix(entity.clone());
            }
        }
    }

    fn recur_find_entity(entity_name_searched: String, entity: Rc<RefCell<Entity>>) -> Option<Rc<RefCell<Entity>>> {
        let rc = entity.clone();
        let local_entity = rc.borrow();
        if local_entity.name == entity_name_searched {
            return Some(entity.clone());
        }
        for child in local_entity.children.borrow().iter() {
            if let Some(ent) = HierarchyManager::recur_find_entity(entity_name_searched.clone(), child.clone()) {
                return Some(ent);
            }
        }
        None
    }

    fn recur_update_matrix(parent_entity: Rc<RefCell<Entity>>) {
        if let Ok(parent_entity_borrowed) = parent_entity.try_borrow() {
            let parent_matrix = parent_entity_borrowed.clone().matrix.into_inner();

            for entity in parent_entity_borrowed.children.borrow().iter() {
                let entity_cloned = entity.clone();
                let entity_borrowed = entity_cloned.borrow();

                {
                    let mut mut_actual = entity_borrowed.matrix.borrow_mut();
                    let local_matrix_clone = entity_borrowed.local_matrix.clone();

                    *mut_actual = parent_matrix * local_matrix_clone;
                }
                HierarchyManager::recur_update_matrix(entity.clone());
            }
        }
    }
}

#[derive(Clone)]
pub struct GeneratedObj {
    mesh: String,
    obj_type: GenericObjectType,
    name: String,
    description: String,
    texture_id: i32,
    size: (f32, f32, f32),
    texture_coordinate: ((f32, f32), (f32, f32), (f32, f32), (f32, f32)),
    order: u8,
    matrix: Matrix4<f32>,
}

impl GenericObject for GeneratedObj {
    fn get_mesh(&self) -> String {
        self.mesh.clone()
    }

    fn get_type(&self) -> GenericObjectType {
        self.obj_type.clone()
    }

    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn get_description(&self) -> String {
        self.description.clone()
    }

    fn get_texture_id(&self) -> i32 {
        self.texture_id
    }

    fn get_size(&self) -> (f32, f32, f32) {
        self.size
    }

    fn get_texture_coordinates(&self) -> ((f32, f32), (f32, f32), (f32, f32), (f32, f32)) {
        self.texture_coordinate
    }

    fn get_order(&self) -> u8 {
        self.order
    }

    fn get_matrix(&self) -> Matrix4<f32> {
        self.matrix
    }
}