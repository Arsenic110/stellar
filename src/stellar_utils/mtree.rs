use std::collections::HashMap;

use serde::Serialize;
type Id = u32;

#[derive(Debug)]
pub struct MTree<T> where T: std::fmt::Debug {
    node_map: HashMap<Id, (Node, T)>,
    first_id: Id,
    last_id: Id,
}

#[derive(Debug)]
struct Node {
    parent: Option<Id>,
    children: Vec<Id>,
}

///Wrapper struct representing a handle to a particular node inside the MTree<T>.
#[derive(Debug)]
pub struct NodeHandle<'a, T> where T: std::fmt::Debug {
    tree: &'a mut MTree<T>,
    id: Id,
}

///Wrapper struct for an iterator over the MTree<T>.
pub struct MTreeIter<'a, T> where T: std::fmt::Debug {
    tree: &'a MTree<T>,
    stack: Vec<Id>
}

pub struct MTreeIterMut<'a, T> where T: std::fmt::Debug {
    tree: &'a mut MTree<T>,
    stack: Vec<Id>
}

//a: using an enum here will let us add other error states later.
///MTree error enum containing data about the error.
#[derive(Debug)]
pub enum NodeError {
    NotFound(Id),
}

impl<T> MTree<T> where T: std::fmt::Debug {

    ///Creates a new MTree.
    pub fn new(root_value: T) -> Self {
        //instantiate the hashmap used to map IDs to Node+Value tuples
        let mut node_map: HashMap<Id, (Node, T)> = HashMap::new();

        //create root node
        let root_node = Node {
            parent: None,
            children: vec![]
        };

        //insert this node into the hashmap
        node_map.insert(0, (root_node, root_value));

        //return the new MTree
        MTree {
            node_map,
            first_id: 0,
            last_id: 0,
        }
    }

    ///Creates a new Node, appends it to the one at the provided ID, 
    /// and retuns ID of the new Node.
    pub fn append(&mut self, parent: Id, node_data: T) -> Option<Id> {
        //get next id
        let new_id = self.next_id();

        let (node_parent, _) = match self.node_map.get_mut(&parent) {
            Some(x) => x,
            None => return None
        };

        //create a node for this data
        let node_insert = Node {
            parent: Some(parent),
            children: vec![]
        };

        //add it as a child to the parent, and to the hashmap
        node_parent.children.push(new_id);
        self.node_map.insert(new_id, (node_insert, node_data));

        //return the id of the new node
        Some(new_id)
    }

    ///Returns the value of a node at this ID.
    pub fn get_value(&self, node_id: Id) -> Option<&T> {
        match self.node_map.get(&node_id) {
            Some((_, value)) => Some(value),
            None => None
        }
    }

    //a: this function is private because we do not need to expose Node
    ///Returns just the node in the tree at this ID.
    fn get_node(&self, node_id: Id) -> Option<&Node> {
        match self.node_map.get(&node_id) {
            Some((node, _)) => Some(node),
            None => None
        }
    }

    ///Returns a mutable reference to the Node at this Id.
    fn get_node_mut(&mut self, node_id: Id) -> Option<&mut Node> {
        match self.node_map.get_mut(&node_id) {
            Some((node, _)) => Some(node),
            None => None
        }
    }

    ///Returns both node and value as a tuple.
    fn get(&self, node_id: Id) -> Option<&(Node, T)> {
        self.node_map.get(&node_id)
    } 

    ///Helper function to print a node and its children.
    fn print_node(&self, node_id: Id, depth: usize) {
        let (node, value) = match self.get(node_id) {
            Some(x) => x,
            None => return //can just return here and move on to the next child
        };
        println!("{}{:?} ({})", " | ".repeat(depth), value, node_id);
        for &child in &node.children {
            self.print_node(child, depth + 1);
        }
    }
    
    ///Helper function to print an entire tree.
    pub fn print_hierarchy(&self) {
        self.print_node(self.first_id, 0);
    }

    ///Returns a handle to the root node.
    pub fn root_handle(&mut self) -> NodeHandle<'_, T> {
        //passing in 'self' here by value means only one handle can exist
        //to a tree at any one time. This is very much intended and acts as
        //a natural extension to the borrow checker.
        let id = self.first_id;
        NodeHandle { tree: self, id: id }
    }

    ///Returns an iterator wrapper on this MTree<T>.
    pub fn iter(&self) -> MTreeIter<'_, T> {
        MTreeIter::new(self)
    }

    pub fn iter_mut(&mut self) -> MTreeIterMut<'_, T> {
        MTreeIterMut::new(self)
    }

    ///Returns the next usable Id. Mutates the MTree<T>'s last_id with a side effect.
    pub fn next_id(&mut self) -> Id {
        self.last_id += 1;
        self.last_id
    }

}

impl<'a, T> IntoIterator for &'a MTree<T> where T: std::fmt::Debug {
    type Item = &'a T;
    type IntoIter = MTreeIter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a, T> IntoIterator for &'a mut MTree<T> where T: std::fmt::Debug {
    type Item = &'a mut T;
    type IntoIter = MTreeIterMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

impl<'a, T> NodeHandle<'a, T> where T: std::fmt::Debug {

    pub fn append(&mut self, value: T) -> NodeHandle<'_, T> {

        // if this unwrap ever fails, freak out
        let child_id = self.tree.append(self.id(), value).unwrap(); 

        NodeHandle {
            tree: self.tree,
            id: child_id,
        }
    }

    ///Merge the provided MTree as a subtree of this NodeHandle.
    pub fn merge(&mut self, other_tree: MTree<T>) -> NodeHandle<'_, T> {

        //get next available ID as basis for how far to offset the IDs in donor tree
        let id_difference = self.tree.next_id();

        //iteration order does not matter as all are offset equally
        for (id, (mut node, data)) in other_tree.node_map.into_iter() {
            //remember to also offset children
            for child in node.children.iter_mut() {
                *child += id_difference;
            }
            //reinsert into destination tree nodemap
            self.tree.node_map.insert(id + id_difference, (node, data));
        }

        //dont forget to add root of donor tree as child of this node handle
        self.node_mut().children.push(other_tree.first_id + id_difference);
        //make sure to correctly offset last_id too
        self.tree.last_id = other_tree.last_id + id_difference;

        NodeHandle { tree: self.tree, id: self.id }
    }

    pub fn id(&self) -> Id {
        self.id
    }

    pub fn value(&self) -> &T {
        // the unwrap used here should be safe for the same reasons node_mut should be safe:
        //getting self from own mtree map
        self.tree.get_value(self.id).unwrap()
    }

    fn node(&self) -> &Node {
        self.tree.get_node(self.id()).unwrap()
    }

    // this function might not need to return an option
    // self should always be inside the self's mtree, so getting self
    // from the tree should always succeed?
    //a: yea i think this is fine. NodeHandle has to have a valid Node reference.
    fn node_mut(&mut self) -> &mut Node {
        let id = self.id();
        let (node, _) = self.tree.node_map.get_mut(&id).unwrap();

        node
    }

    pub fn nth_child(&mut self, n: usize) -> Option<NodeHandle<'_, T>> {
        let id = match self.node().children.get(n) {
            Some(&id) => id,
            None => return None
        };

        Some(NodeHandle { tree: self.tree, id: id })
    }
}

impl<'a, T> MTreeIter<'a, T> where T:std::fmt::Debug {
    fn new(tree: &'a MTree<T>) -> Self {
        Self {
            stack: vec![tree.first_id],
            tree,
        }
    }
}

impl<'a, T> MTreeIterMut<'a, T> where T:std::fmt::Debug {
    fn new(tree: &'a mut MTree<T>) -> Self {
        Self {
            stack: vec![tree.first_id],
            tree,
        }
    }
}

impl<'a, T> Iterator for MTreeIter<'a, T> where T: std::fmt::Debug {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        let id = self.stack.pop()?;
        let (node, value) = self.tree.get(id)?;
        let children = &node.children;
        for &c in children.iter().rev() {
            self.stack.push(c);
        }
        Some(value)
    }
}

impl<'a, T> Iterator for MTreeIterMut<'a, T> where T: std::fmt::Debug {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {

        //N.B.: This probably can't be done without unsafe.
        //Maybe possible with getting id and then looking them up later
        todo!();

        let id = self.stack.pop()?;
        let (node, value) = self.tree.node_map.get_mut(&id)?;
        let children = node.children.clone(); //cheap clone
        for &c in children.iter().rev() {
            self.stack.push(c);
        }
        Some(value)
    }
}

///Serialize a tree
impl<T> Serialize for MTree<T> where T: std::fmt::Debug {
    fn serialize<S>(&self, _serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer {
        
        //so nice of rust to compile without a return type.
        todo!();
    }
}
