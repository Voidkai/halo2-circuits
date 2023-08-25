use std::collections::{BTreeMap, HashMap};

#[derive(Debug)]
struct Node{
    node_id: usize,
    node_name: String,
    node_property: HashMap<String,String>,
}

impl Node{
    fn new(node_id: usize, node_name: String) -> Self{
        Self{
            node_id,
            node_name,
            node_property: Default::default(),
        }
    }
    fn add_property(&mut self, property_name: String, property_value: String){
        self.node_property.insert(property_name, property_value);
    }
    fn get_property(&self, property_name: &str) -> Option<&str>{
        self.node_property.get(property_name).map(|s| s.as_str())
    }
}

#[derive(Debug,Clone)]
struct Edge {
    edge_id: usize,
    edge_name: String,
    from_node_id: usize,
    to_node_id: usize,
    edge_property: HashMap<String, String>
}

impl Edge{
    fn new(edge_id: usize, edge_name: String, from_node_id: usize, to_node_id: usize) -> Self{
        Self{
            edge_id,
            edge_name,
            from_node_id,
            to_node_id,
            edge_property: Default::default(),
        }
    }
}
#[derive(Debug)]
struct Graph<T>{
    pub matrix: Vec<Vec<Option<usize>>>,
    pub node: BTreeMap<usize, Option<T>>
}

impl<T> Graph<T>{
    fn new() -> Self{
        Self{
            matrix: vec![],
            node: BTreeMap::new(),
        }
    }

    fn is_empty(&self) -> usize{
        self.node.len()
    }

}


#[test]
fn graph_test(){
    let g: Graph<String> = Graph::new();
    println!("{:?}", g);
}
