use std::collections::BTreeMap;

pub enum NodeType{
    Empty = 0,
    PayRule = 10,
    PayValue = 20,
    PayVariable = 30,
    PayAbsence = 40,
    GaData = 50,
}

pub enum Dirty{
    Clean = 0,
    Touched = 1,
}

pub struct Node{
    kind:NodeType,
    name:Box<String>,
    family:Box<String>,
    subfamily:Box<String>,
    regulation:u64,
    
    //Grey is used to temporary flag a node during graph browsing.
    grey:Dirty,
}

pub struct Vertex{
    source:String, 
    target:String, 
    grey:Dirty,
}

pub struct Graph{
    nodes: BTreeMap::<String, Node>,
    vertices: Vec::<Vertex>,
}


impl Node{
    pub fn new(p_kind:NodeType, p_name:&str, p_family:&str, p_subfamily:&str, p_regulation:u64) -> Node{
        Node{
            kind: p_kind,
            name: Box::new(p_name.to_string()),
            family: Box::new(p_family.to_string()),
            subfamily: Box::new(p_subfamily.to_string()),
            grey: Dirty::Clean,
            regulation: 0,
        }
    }
}


impl Vertex{
    pub fn new(source_node:&str, target_node:&str) -> Vertex{
        Vertex{
            source: source_node.to_string(),
            target: target_node.to_string(),
            grey: Dirty::Clean,
        }
    }
}

impl Graph{
    pub fn new() -> Graph{
        Graph{
            nodes: BTreeMap::new(),
            vertices: Vec::new()
        }
    }
    pub fn create_node(&mut self, name:&str){
        self.nodes.insert("paie1".to_string(), Node::new(NodeType::PayRule, "rule1", "fam1", "subfam1", 0));
    }

    pub fn get_node(&mut self, name:&str) -> &mut Node{
        if self.nodes.contains_key(name) {
            let n = self.nodes.get_mut(name);
            match n{
                Some(x) => x,
                None => panic!("Node not found")
            }
        }else{
            panic!("Node not found")
        }
    }

    pub fn create_vertex(&mut self, source_node_name:&str, target_node_name:&str){
        self.vertices.push(Vertex::new(source_node_name, target_node_name));
    }
}