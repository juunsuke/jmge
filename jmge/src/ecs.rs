

use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::rc::{Rc, Weak};
use std::cell::{RefCell, Ref, RefMut};


pub trait Component
{
	fn as_any(&self) -> &dyn Any;
	fn as_any_mut(&mut self) -> &mut dyn Any;
}

pub trait System
{
	fn run(&mut self, world: &World);
}


//------------------------------------------------------------------------------------------------------------------------
//------------------------------------------------------------------------------------------------------------------------
//------------------------------------------------------------------------------------------------------------------------

pub struct CompIter<'a, T>
{
	cv: &'a CompVec,
	ents: &'a Vec<Option<Weak<usize>>>,
	pos: usize,
	phantom: std::marker::PhantomData<T>,
}

impl<'a, T> CompIter<'a, T>
{
	fn new(cv: &'a CompVec, ents: &'a Vec<Option<Weak<usize>>>) -> CompIter<'a, T>
	{
		CompIter
		{
			cv,
			ents,
			pos: 0,
			phantom: std::marker::PhantomData,
		}
	}
}

impl<'a, T:'static> Iterator for CompIter<'a, T>
{
	type Item = (Entity, Ref<'a, T>);

	fn next(&mut self) -> Option<Self::Item>
	{
		// Find the next set component
		while self.pos<self.cv.vec.len()
		{
			self.pos += 1;

			if let Some(ref ent) = self.ents[self.pos-1]
			{
				if let Some(id) = ent.upgrade()
				{
					if self.cv.contains(self.pos-1)
					{
						return Some((Entity { id }, self.cv.get_as(self.pos-1)));
					}
				}
			}
		}

		None
	}
}

//------------------------------------------------------------------------------------------------------------------------
//------------------------------------------------------------------------------------------------------------------------
//------------------------------------------------------------------------------------------------------------------------


pub struct CompIterMut<'a, T>
{
	cv: &'a CompVec,
	ents: &'a Vec<Option<Weak<usize>>>,
	pos: usize,
	phantom: std::marker::PhantomData<T>,
}

impl<'a, T> CompIterMut<'a, T>
{
	fn new(cv: &'a CompVec, ents: &'a Vec<Option<Weak<usize>>>) -> CompIterMut<'a, T>
	{
		CompIterMut
		{
			cv,
			ents,
			pos: 0,
			phantom: std::marker::PhantomData,
		}
	}
}

impl<'a, T:'static> Iterator for CompIterMut<'a, T>
{
	type Item = (Entity, RefMut<'a, T>);

	fn next(&mut self) -> Option<Self::Item>
	{
		// Find the next set component
		while self.pos<self.cv.vec.len()
		{
			self.pos += 1;

			if let Some(ref ent) = self.ents[self.pos-1]
			{
				if let Some(id) = ent.upgrade()
				{
					if self.cv.contains(self.pos-1)
					{
						return Some((Entity { id }, self.cv.get_mut_as(self.pos-1)));
					}
				}
			}
		}

		None
	}
}

//------------------------------------------------------------------------------------------------------------------------
//------------------------------------------------------------------------------------------------------------------------
//------------------------------------------------------------------------------------------------------------------------


pub struct CompVec
{
	vec: Vec<Option<Box<RefCell<dyn Component>>>>,
}

impl CompVec
{
	fn new() -> CompVec
	{
		CompVec
		{
			vec: Vec::new(),
		}
	}

	fn contains(&self, i: usize) -> bool
	{
		// Check if there is an entry
		if i>=self.vec.len()
			{ return false; }

		self.vec[i].is_some()
	}

	fn try_get(&self, i: usize) -> Option<Ref<dyn Component>>
	{
		if i>=self.vec.len()
			{ return None; }

		match self.vec[i].as_ref()
		{
			Some(v) => Some(v.borrow()),
			None => None,
		}
	}

	fn try_get_mut(&self, i: usize) -> Option<RefMut<dyn Component>>
	{
		if i>=self.vec.len()
			{ return None; }

		match self.vec[i].as_ref()
		{
			Some(v) => Some(v.borrow_mut()),
			None => None,
		}
	}

	fn try_get_as<T:'static>(&self, i: usize) -> Option<Ref<T>>
	{
		match self.try_get(i)
		{
			Some(v) => Some(Ref::map(v, |v| v.as_any().downcast_ref::<T>().unwrap())),
			None => None,
		}
	}

	fn try_get_mut_as<T:'static>(&self, i: usize) -> Option<RefMut<T>>
	{
		match self.try_get_mut(i)
		{
			Some(v) => Some(RefMut::map(v, |v| v.as_any_mut().downcast_mut::<T>().unwrap())),
			None => None,
		}
	}


/*
	fn get(&self, i: usize) -> Ref<dyn Component>
		{ self.try_get(i).expect("no such component for this entity") }

	fn get_mut(&self, i: usize) -> RefMut<dyn Component>
		{ self.try_get_mut(i).expect("no such component for this entity") }
*/

	fn get_as<T:'static>(&self, i: usize) -> Ref<T>
		{ self.try_get_as(i).expect("no such component for this entity") }

	fn get_mut_as<T:'static>(&self, i: usize) -> RefMut<T>
		{ self.try_get_mut_as(i).expect("no such component for this entity") }



	fn set(&mut self, i: usize, val: Box<RefCell<dyn Component>>)
	{
		// Enlarge the vector if needed
		if i>=self.vec.len()
		{
			self.vec.resize_with(i+1, || None);
		}

		// Set the component
		self.vec[i] = Some(val);
	}

	fn unset(&mut self, i: usize)
	{
		// Unset a component
		if i<self.vec.len()
		{
			self.vec[i] = None;
		}
	}
}

//------------------------------------------------------------------------------------------------------------------------
//------------------------------------------------------------------------------------------------------------------------
//------------------------------------------------------------------------------------------------------------------------

pub struct Entity
{
	id: Rc<usize>,
}

impl Entity
{
	pub fn id(&self) -> usize
	{
		*self.id
	}
}

impl Clone for Entity
{
	fn clone(&self) -> Entity
	{
		Entity
		{
			id: Rc::clone(&self.id),
		}
	}
}

//------------------------------------------------------------------------------------------------------------------------
//------------------------------------------------------------------------------------------------------------------------
//------------------------------------------------------------------------------------------------------------------------

struct WorldSystem
{
	name: String,
	active: bool,
	sys: Box<RefCell<dyn System>>,
}

pub struct World
{
	comps: HashMap<TypeId, CompVec>,
	ents: Vec<Option<Weak<usize>>>,
	free_ent: Vec<usize>,
	systems: Vec<WorldSystem>,
}

impl World
{
	pub fn new() -> World
	{
		World
		{
			comps: HashMap::new(),
			ents: Vec::new(),
			free_ent: Vec::new(),
			systems: Vec::new(),
		}
	}

	pub fn register<T>(&mut self)
	where
		T: 'static + Component
	{
		// Register a new component
		let id = TypeId::of::<T>();

		if self.comps.contains_key(&id)
		{
			panic!("component type already registered");
		}

		// Add it
		self.comps.insert(id, CompVec::new());
	}

	fn recycle_entity(&mut self) -> Option<Entity>
	{
		// Try to recycle an entity
		if let Some(id) = self.free_ent.pop()
		{
			let ent = Entity
				{
					id: Rc::new(id),
				};

			// Add it to the main vector
			self.ents[id] = Some(Rc::downgrade(&ent.id));

			Some(ent)
		}
		else
		{
			None
		}
	}

	pub fn new_entity(&mut self) -> Entity
	{
		// Try to recycle an old ID first
		if let Some(ent) = self.recycle_entity()
		{
			return ent;
		}

		// Create a new entity
		let id = self.ents.len();

		let ent = Entity
			{
				id: Rc::new(id),
			};

		self.ents.push(Some(Rc::downgrade(&ent.id)));

		ent
	}

	pub fn clean(&mut self)
	{
		// Cleanup all the unused components from dropped entities
		// And recycle those entities
		let mut ents = Vec::new();

		for (id, ent) in self.ents.iter_mut().enumerate()
		{
			// Expired ?
			if let Some(ent) = ent
			{
				if ent.strong_count()==0
				{
					// Yes
					// Unset all the components
					for (_, cv) in self.comps.iter_mut()
					{
						cv.unset(id);
					}

					// Mark the entity for removal
					ents.push(id);
				}
			}
		}

		// Cleanup all the removed entities
		for e in ents
		{
			// Mark as unused
			self.ents[e] = None;

			// Add to the free list
			self.free_ent.push(e);
		}
	}

	pub fn set<T>(&mut self, ent: &Entity, val: T)
	where
		T: 'static + Component
	{
		// Get the vec
		let cv = self.comps.get_mut(&TypeId::of::<T>()).expect("unregistered component");

		// Set the value
		cv.set(*ent.id, Box::new(RefCell::new(val)));
	}

	pub fn try_get<T>(&self, ent: &Entity) -> Option<Ref<T>>
	where
		T: 'static + Component
	{
		// Get the vec
		let cv = self.comps.get(&TypeId::of::<T>()).expect("unregistered component");

		// Get the value
		cv.try_get_as(*ent.id)
	}

	pub fn get<T>(&self, ent: &Entity) -> Ref<T>
	where
		T: 'static + Component
	{
		// Get the vec
		let cv = self.comps.get(&TypeId::of::<T>()).expect("unregistered component");

		// Get the value
		cv.get_as(*ent.id)
	}

	pub fn try_get_mut<T>(&self, ent: &Entity) -> Option<RefMut<T>>
	where
		T: 'static + Component
	{
		// Get the vec
		let cv = self.comps.get(&TypeId::of::<T>()).expect("unregistered component");

		// Get the value
		cv.try_get_mut_as(*ent.id)
	}

	pub fn get_mut<T>(&self, ent: &Entity) -> RefMut<T>
	where
		T: 'static + Component
	{
		// Get the vec
		let cv = self.comps.get(&TypeId::of::<T>()).expect("unregistered component");

		// Get the value
		cv.get_mut_as(*ent.id)
	}

	pub fn iter<T>(&self) -> CompIter<T>
	where
		T: 'static + Component
	{
		// Get the vec
		let cv = self.comps.get(&TypeId::of::<T>()).expect("unregistered component");

		// Create the iterator
		CompIter::new(cv, &self.ents)
	}

	pub fn iter_mut<T>(&self) -> CompIterMut<T>
	where
		T: 'static + Component
	{
		// Get the vec
		let cv = self.comps.get(&TypeId::of::<T>()).expect("unregistered component");

		// Create the iterator
		CompIterMut::new(cv, &self.ents)
	}

	pub fn run_once(&mut self, sys: &mut impl System)
	{
		// Run the system
		sys.run(self);
	}

	fn find_system_index(&self, name: &str) -> Option<usize>
	{
		// Find the system with the given name
		for (i, ws) in self.systems.iter().enumerate()
		{
			if ws.name==name
				{ return Some(i); }
		}

		None
	}

	fn find_system(&self, name: &str) -> Option<&WorldSystem>
	{
		// Find the system with the given name
		for ws in self.systems.iter()
		{
			if ws.name==name
				{ return Some(ws); }
		}

		None
	}

	fn find_system_mut(&mut self, name: &str) -> Option<&mut WorldSystem>
	{
		// Find the system with the given name
		for ws in self.systems.iter_mut()
		{
			if ws.name==name
				{ return Some(ws); }
		}

		None
	}

	pub fn add_system(&mut self, name: &str, sys: impl System+'static)
	{
		// Add a new system

		// Look for duplicates
		if let Some(_) = self.find_system(name)
		{
			panic!("duplicate system name");
		}

		// Add it
		let ws = WorldSystem
			{
				name: String::from(name),
				active: true,
				sys: Box::new(RefCell::new(sys)),
			};

		self.systems.push(ws);
	}

	pub fn run(&self, name: &str)
	{
		// Find the system and run it
		let ws = self.find_system(name).expect("no such system");
		ws.sys.borrow_mut().run(self);
	}

	pub fn run_all(&self)
	{
		// Run all the systems
		for ws in self.systems.iter().filter(|ws| ws.active)
		{
			ws.sys.borrow_mut().run(self);
		}
	}

	pub fn is_active(&self, name: &str) -> bool
	{
		// Check wether a system is active
		self.find_system(name).expect("no such system").active
	}

	pub fn set_active(&mut self, name: &str, active: bool)
	{
		// Activate/deactivate a system
		self.find_system_mut(name).expect("no such system").active = active;
	}

	pub fn remove_system(&mut self, name: &str)
	{
		// Find and remove a system
		let i = self.find_system_index(name).expect("no such system");
		self.systems.remove(i);
	}
}

