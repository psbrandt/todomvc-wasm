use js_sys::JSON;
use wasm_bindgen::prelude::*;
//use crate::DefinePropertyAttrs;
// TODO remove
fn dbg(mssg: &str) {
    let v = wasm_bindgen::JsValue::from_str(&format!("{}", mssg));
    web_sys::console::log_1(&v);
}

pub struct Store {
    local_storage: web_sys::Storage,
    data: Option<ItemList>,
    name: String,
}

impl Store {
    /**
     * @param {!string} name Database name
     * @param {function()} [callback] Called when the Store is ready
     */
    pub fn new(name: &str) -> Option<Store> {
        if let Some(window) = web_sys::window() {
            if let Ok(Some(local_storage)) = window.local_storage() {
                Some(Store {
                    local_storage,
                    data: None,
                    name: String::from(name),
                })
            } else {
                None
            }
        } else {
            None
        }
    }

    /// Read the local ItemList from localStorage.
    /// @returns {ItemList} Current array of todos
    /// Uses mut here as the return is something we might want to manipulate
    fn get_local_storage(&mut self) -> &Option<ItemList> {
        let mut item_list = ItemList::new();
        if let Some(ref data) = self.data {
            return &self.data;
        }
        if let Ok(Some(value)) = self.local_storage.get_item(&self.name) {
            if let Ok(data) = JSON::parse(&value) {
                if let Ok(Some(iter)) = js_sys::try_iter(&data) {
                    for item in iter {
                        if let Ok(item) = item {
                            let item: Option<&js_sys::Array> = wasm_bindgen::JsCast::dyn_ref(&item);
                            if let Some(item) = item {
                                if let Some(title) = item.shift().as_string() {
                                    if let Some(completed) = item.shift().as_bool() {
                                        if let Some(id) = item.shift().as_f64() {
                                            let mut temp_item = Item {
                                                title,
                                                completed,
                                                id: id as usize,
                                            };
                                            item_list.push(temp_item);
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        self.data = Some(item_list);
        &self.data
    }

    /// Write the local ItemList to localStorage.
    /// @param {ItemList} todos Array of todos to write
    fn set_local_storage(&mut self, todos: ItemList) {
        /*
        fn define_property(object: &js_sys::Object, key: &str, val: JsValue) {
    let value = DefinePropertyAttrs::from(JsValue::from(js_sys::Object::new()));
    value.set_value(&val);
    let descriptor = js_sys::Object::from(JsValue::from(value));
/*
    let foo = foo_42();
    let foo = Object::define_property(&foo, &"bar".into(), &descriptor);
    assert!(foo.has_own_property(&"bar".into()));
*/

 //           let prototype: js_sys::Object = wasm_bindgen::JsValue::null().into();
  //          let descriptor = js_sys::Object::create(&prototype);
dbg("got here ---*q1");
web_sys::console::log_2(&val, &JsValue::from(key));
//let val: js_sys::Object = val.into();
// TODO below doesn't work val isn't a object it seems
dbg("got here ---*d2");
            //if let Some(val) = wasm_bindgen::JsCast::dyn_ref(&val): Option<&js_sys::Object> {
   //      let val = wasm_bindgen::JsCast::unchecked_ref(&val): &js_sys::Object;
          //let val = js_sys::Object::from(JsValue::from(val));
dbg("got here ---n3");
     //           js_sys::Object::define_property(&descriptor, &JsValue::from("value"), &val);
dbg("got here ---*a4");
                js_sys::Object::define_property(object, &key.into(), &descriptor);
         //   }
        }
*/
        let array = js_sys::Array::new();
        for item in todos.iter() {
            dbg("got here 99 {");
            let mut child = js_sys::Array::new();
            //let mut object = js_sys::Object::new();
            let s = String::from(item.title.clone());
            child.push(&JsValue::from(&s));
            child.push(&JsValue::from(item.completed));
            child.push(&JsValue::from(item.id as f64));
            /*
            define_property(&mut object, "title", JsValue::from(&s));
            define_property(&mut object, "completed", JsValue::from(item.completed));
            define_property(&mut object, "id", JsValue::from(item.id as f64));
            array.push(&JsValue::from(object));
*/
            array.push(&JsValue::from(child));

            dbg("got here 99 }");
        }
        //web_sys::console::log_1(&JsValue::from(array));
        if let Ok(storage_string) = JSON::stringify(&JsValue::from(array)) {
            let storage_string: String = storage_string.to_string().into();
            dbg(&storage_string);
            self.local_storage
                .set_item(&self.name, storage_string.as_str());
            self.data = Some(todos);
        }
    }

    /// Find items with properties matching those on query.
    /*
     * @param {ItemQuery} query Query to match
     * @param {function(ItemList)} callback Called when the query is done
     *
     * @example
     * db.find({completed: true}, data => {
     *	 // data shall contain items whose completed properties are true
     * })
     */
    pub fn find(&mut self, query: ItemQuery) -> Option<ItemListSlice> {
        self.get_local_storage();
        if let Some(ref todos) = self.data {
            Some(todos.iter().filter(|todo| query.matches(*todo)).collect())
        } else {
            None
        }
    }

    /// Update an item in the Store.
    /**
     * @param {ItemUpdate} update Record with an id and a property to update
     * @param {function()} [callback] Called when partialRecord is applied
     */
    pub fn update(&mut self, update: ItemUpdate) {
        //, callback) {
        let id = update.id();
        self.get_local_storage();
        self.data.take().map(|todos| {
            let todos = todos.into_iter();

            let todos = todos
                .map(|mut todo| {
                    if id == todo.id {
                        todo.update(&update);
                    }
                    todo
                }).collect();
            self.set_local_storage(todos);
        });
    }

    /// Insert an item into the Store.
    /**
     * @param {Item} item Item to insert
     * @param {function()} [callback] Called when item is inserted
     */
    pub fn insert(&mut self, item: Item) {
        self.get_local_storage();
        self.data.take().map(|mut todos| {
            todos.push(item);
            self.set_local_storage(todos);
        });
    }

    /**
     * Remove items from the Store based on a query.
     *
     * @param {ItemQuery} query Query matching the items to remove
     * @param {function(ItemList)|function()} [callback] Called when records matching query are removed
     */
    pub fn remove(&mut self, query: ItemQuery) {
        self.get_local_storage();
        if let Some(todos) = self.data.take() {
            let todos = todos
                .into_iter()
                .filter(|todo| query.matches(todo))
                .collect();
            self.set_local_storage(todos);
        }
    }

    /// Count total, active, and completed todos.
    pub fn count(&mut self) -> Option<(usize, usize, usize)> {
        self.find(ItemQuery::EmptyItemQuery).map(|data| {
            let total = data.length();

            let mut completed = 0;
            for item in data.iter() {
                if item.completed {
                    completed += 1;
                }
            }
            (total, total - completed, completed)
        })
    }
}

pub struct Item {
    pub id: usize,
    pub title: String,
    pub completed: bool,
}

impl Item {
    pub fn update(&mut self, update: &ItemUpdate) {
        match update {
            ItemUpdate::Title { id, title } => {
                self.title = title.to_string();
            }
            ItemUpdate::Completed { id, completed } => {
                self.completed = *completed;
            }
        }
    }
}

pub trait ItemListTrait<T> {
    fn new() -> Self;
    fn get(&self, i: usize) -> Option<&T>;
    fn length(&self) -> usize;
    fn push(&mut self, item: T);
    fn iter(&self) -> std::slice::Iter<T>;
}

pub struct ItemList {
    list: Vec<Item>,
}
impl ItemList {
    fn into_iter(self) -> std::vec::IntoIter<Item> {
        self.list.into_iter()
    }
}
impl ItemListTrait<Item> for ItemList {
    fn new() -> ItemList {
        ItemList { list: Vec::new() }
    }
    fn get(&self, i: usize) -> Option<&Item> {
        self.list.get(i)
    }
    fn length(&self) -> usize {
        self.list.len()
    }
    fn push(&mut self, item: Item) {
        self.list.push(item)
    }
    fn iter(&self) -> std::slice::Iter<Item> {
        self.list.iter()
    }
}
use std::iter::FromIterator;
impl<'a> FromIterator<Item> for ItemList {
    fn from_iter<I: IntoIterator<Item = Item>>(iter: I) -> Self {
        let mut c = ItemList::new();
        for i in iter {
            c.push(i);
        }

        c
    }
}

pub struct ItemListSlice<'a> {
    list: Vec<&'a Item>,
}
impl<'a> ItemListSlice<'a> {
    /*
TODO implement?
    fn new(boxed_slice: Box<[&'a Item]>) -> ItemListSlice<'a> {
        ItemListSlice { list: boxed_slice }
    }
*/
}
impl<'a> ItemListTrait<&'a Item> for ItemListSlice<'a> {
    fn new() -> ItemListSlice<'a> {
        ItemListSlice { list: Vec::new() }
    }
    fn get(&self, i: usize) -> Option<&&'a Item> {
        self.list.get(i)
    }
    fn length(&self) -> usize {
        self.list.len()
    }
    fn push(&mut self, item: &'a Item) {
        self.list.push(item)
    }
    fn iter(&self) -> std::slice::Iter<&'a Item> {
        self.list.iter()
    }
}
impl<'a> FromIterator<&'a Item> for ItemListSlice<'a> {
    fn from_iter<I: IntoIterator<Item = &'a Item>>(iter: I) -> Self {
        let mut c = ItemListSlice::new();
        for i in iter {
            c.push(i);
        }
        c
    }
}
impl<'a> Into<ItemList> for ItemListSlice<'a> {
    fn into(self) -> ItemList {
        let mut i = ItemList::new();
        let items = self.list.into_iter();
        for j in items {
            // TODO neaten this cloning?
            let item = Item {
                id: j.id,
                completed: j.completed,
                title: j.title.clone(),
            };
            i.push(item);
        }
        i
    }
}

pub enum ItemQuery {
    Id { id: usize },
    Completed { completed: bool },
    EmptyItemQuery,
}
impl ItemQuery {
    fn matches(&self, item: &Item) -> bool {
        match *self {
            ItemQuery::EmptyItemQuery => true,
            ItemQuery::Id { id } => item.id == id,
            ItemQuery::Completed { completed } => item.completed == completed,
        }
    }
}

pub enum ItemUpdate {
    Title { id: usize, title: String },
    Completed { id: usize, completed: bool },
}
impl ItemUpdate {
    fn id(&self) -> usize {
        match self {
            ItemUpdate::Title { id, title } => *id,
            ItemUpdate::Completed { id, completed } => *id,
        }
    }
}
