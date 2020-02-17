
use std::cell::{RefCell, Ref};
use std::rc::{Rc, Weak};


struct DataStore<T>
{
	data: Vec<RefCell<T>>,
	handles: Vec<Weak<DataBinHandle<T>>>,
}


pub struct DataBin<T>
{
	store: Rc<RefCell<DataStore<T>>>,
}

impl<T> DataBin<T>
{

	pub fn new() -> Self
	{
		// Create a new empty bin
		let store = DataStore
			{
				data: Vec::new(),
				handles: Vec::new(),
			};

		DataBin
		{
			store: Rc::new(RefCell::new(store)),
		}
	}

	pub fn insert(&self, val: T) -> Rc<DataBinHandle<T>>
	{
		// Insert a new value into the bin
		let mut store = self.store.borrow_mut();
		store.data.push(RefCell::new(val));

		// Create a handle for it
		let h = DataBinHandle
			{
				store: Rc::clone(&self.store),
				index: store.data.len()-1,
			};

		// Create a Rc and keep a weak handle to it
		let h = Rc::new(h);
		store.handles.push(Rc::downgrade(&h));

		h
	}
}


//------------------------------------------------------------------------------------------------------------------------
//------------------------------------------------------------------------------------------------------------------------
//------------------------------------------------------------------------------------------------------------------------


pub struct DataBinHandle<T>
{
	store: Rc<RefCell<DataStore<T>>>,
	index: usize,
}

impl<T> DataBinHandle<T>
{

}

