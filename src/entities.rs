use bevy::prelude::*;
use uuid::Uuid;
use std::marker::PhantomData;

#[derive(Component)]
pub struct GameEntity<T> {
    pub id: Uuid,
    _marker: PhantomData<T>,
}

impl<T> GameEntity<T> {
    pub fn new(id: Uuid) -> Self {
        Self {
            id,
            _marker: PhantomData,
        }
    }
}
