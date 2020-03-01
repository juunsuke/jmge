

use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::rc::Rc;
use std::cell::{RefCell, Ref, RefMut};


pub trait Component
{
	fn as_any(&self) -> &dyn Any;
	fn as_any_mut(&mut self) -> &mut dyn Any;
}





struct CompStorage
{
	vec: Vec<Option<Box<dyn Component>>>,
}

struct WorldData
{
	comps: HashMap<TypeId, CompStorage>,
	free_entities: Vec<usize>,
	max_entity: usize,
}

pub struct World
{
	data: Rc<RefCell<WorldData>>,
}

impl World
{
	pub fn new() -> World
	{
		let data = Rc::new(RefCell::new(WorldData
			{
				comps: HashMap::new(),
				free_entities: Vec::new(),
				max_entity: 0,
			}));

		World
		{
			data,
		}
	}

	pub fn register_component<T: 'static + Component>(&self)
	{
		let mut data = self.data.borrow_mut();

		// Get the type ID
		let id = TypeId::of::<T>();

		// Make sure it's not already registered
		if let Some(_) = data.comps.get(&id)
			{ panic!("World.register_component(): duplicate component"); }

		// Create the storage
		let store = CompStorage
			{
				vec: Vec::new(),
			};

		// Add it
		data.comps.insert(id, store);
	}

	pub fn new_entity(&self) -> Entity
	{
		let mut data = self.data.borrow_mut();

		// Reuse a free ID or increase the max ID
		let id = match data.free_entities.pop()
			{
				Some (id) => id,
				None =>
					{
						data.max_entity += 1;
						data.max_entity-1
					},
			};

		// Create a new empty entity
		Entity
		{
			id,
			world: Rc::clone(&self.data),
		}
	}

	pub fn iter_with<T: 'static, F>(&self, mut f: F)
	where F: FnMut(&T)
	{
		let data = self.data.borrow();

		// Get the component vector
		let vec = &data.comps.get(&TypeId::of::<T>()).expect("unregistered component type").vec;

		// Iterate through the values
		for v in vec.iter()
		{
			if let Some(v) = v
			{
				// Call the callback
				f(&v.as_any().downcast_ref::<T>().unwrap());
			}
		}
	}

}


//------------------------------------------------------------------------------------------------------------------------
//------------------------------------------------------------------------------------------------------------------------
//------------------------------------------------------------------------------------------------------------------------


pub struct Entity
{
	id: usize,
	world: Rc<RefCell<WorldData>>,
}

impl Entity
{

	pub fn try_get_component<T: 'static + Component>(&self) -> Option<Ref<'_, T>>
	{
		let data = self.world.borrow();

		// Get the component vector
		let vec = Ref::map(data, |data| &data.comps.get(&TypeId::of::<T>()).expect("unregistered component type").vec);

		if self.id>=vec.len() || vec[self.id].is_none()
			{ return None; }

		Some(Ref::map(vec, |vec| vec[self.id].as_ref().unwrap().as_any().downcast_ref::<T>().unwrap()))
	}

	pub fn get_component<T: 'static + Component>(&self) -> Ref<'_, T>
	{
		self.try_get_component::<T>().expect("no such component for this entity")
	}

	pub fn try_get_component_mut<T: 'static + Component>(&self) -> Option<RefMut<'_, T>>
	{
		let data = self.world.borrow_mut();

		// Get the component vector
		let vec = RefMut::map(data, |data| &mut data.comps.get_mut(&TypeId::of::<T>()).expect("unregistered component type").vec);

		if self.id>=vec.len() || vec[self.id].is_none()
			{ return None; }

		Some(RefMut::map(vec, |vec| vec[self.id].as_mut().unwrap().as_any_mut().downcast_mut::<T>().unwrap()))
	}

	pub fn get_component_mut<T: 'static + Component>(&self) -> RefMut<'_, T>
	{
		self.try_get_component_mut::<T>().expect("no such component for this entity")
	}

	pub fn set_component<T: 'static + Component>(&self, val: T)
	{
		let mut data = self.world.borrow_mut();

		// Get the component vector
		let vec = &mut data.comps.get_mut(&TypeId::of::<T>()).expect("unregistered component type").vec;

		// Enlarge it if needed
		if self.id>=vec.len()
		{
			vec.resize_with(self.id+1, || None);
		}

		// Set it
		vec[self.id] = Some(Box::new(val));
	}
}


