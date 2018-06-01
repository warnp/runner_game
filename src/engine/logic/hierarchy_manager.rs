use std::cell::RefCell;
use std::rc::Rc;
use cgmath::Matrix4;
use cgmath::prelude::*;
use super::entity::Entity;

#[derive(Debug, Clone)]
pub struct HierarchyManager {
    pub entities: RefCell<Vec<Rc<RefCell<Entity>>>>,
}

impl HierarchyManager {
    pub fn new() -> HierarchyManager {
        let entities = vec![Rc::new(RefCell::new(Entity { matrix: RefCell::new(Matrix4::identity()), children: RefCell::new(vec![]), name: "world".to_string(), parent: RefCell::new(None) }))];
        HierarchyManager {
            entities: RefCell::new(entities)
        }
    }

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
            let mut entities_mut = self.entities.borrow_mut();
            entities_mut.push(Rc::new(RefCell::new(entity)));
        }
    }

    pub fn update_matrix(&self) {
        for entity in self.entities.borrow().iter() {
            HierarchyManager::recur_update_matrix(entity.clone());
        }
    }

    fn recur_update_matrix(parent_entity: Rc<RefCell<Entity>>) {

        if let Ok(parent_entity_borrowed) = parent_entity.try_borrow() {
            let parent_matrix = parent_entity_borrowed.clone().matrix.into_inner();

            for entity in parent_entity_borrowed.children.borrow().iter() {
                let rc = entity.clone();
                let x = rc.borrow();

                {
                    let mut mut_actual = x.matrix.borrow_mut();
                    let plop = *mut_actual;
                    *mut_actual = parent_matrix * plop;
                }
                HierarchyManager::recur_update_matrix(entity.clone());
            }
        }
    }
}