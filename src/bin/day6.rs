// NOTE from the description:
//   Except for the universal Center of Mass (COM), every object in
//   space is in orbit around exactly one other object
//
//   -> directed, acyclic graph
//   -> nodes are 1:N (fanin:fanout)

use std::collections::HashMap;

type NodeId = usize;

struct Node {
    depth: i32,
    child_ids: Vec<NodeId>,
    parent_id: Option<NodeId>,
}

impl Node {
    fn new() -> Node {
        Node {
            depth: 0,
            child_ids: Vec::new(),
            parent_id: None,
        }
    }
}

struct UniversalOrbitMap {
    com_id: Option<NodeId>,
    name_to_id: HashMap<String, NodeId>,
    nodes: Vec<Node>,
}

impl UniversalOrbitMap {
    fn new() -> UniversalOrbitMap {
        UniversalOrbitMap {
            com_id: None,
            name_to_id: HashMap::new(),
            nodes: Vec::new(),
        }
    }

    // A)B -> A: center, B: orbit
    fn add_orbit(&mut self, center: String, orbit: String) {
        let (c_id, o_id) = (self.insert_or_get_id(center), self.insert_or_get_id(orbit));
        self.nodes[c_id].child_ids.push(o_id);
        self.nodes[o_id].parent_id = Some(c_id);
    }

    fn insert_or_get_id(&mut self, node: String) -> NodeId {
        if let Some(&id) = self.name_to_id.get(&node) {
            id
        } else {
            // new node
            let id = self.nodes.len();
            if node == "COM" {
                self.com_id = Some(id);
            }
            self.name_to_id.insert(node, id);
            self.nodes.push(Node::new());
            id
        }
    }

    // Recursively annotate every node in the graph with its depth
    // relative to 'COM' (origin).
    // A nodes depth encodes its distance to 'COM'.
    fn annotate_depth(&mut self) {
        let id = self.com_id.expect("Expected 'COM' node not found!");
        self.annotate_depth_subgraph(id, -1);
    }

    fn annotate_depth_subgraph(&mut self, root: NodeId, parent_depth: i32) {
        let node = &mut self.nodes[root];
        node.depth = parent_depth + 1;

        let children = node.child_ids.to_owned(); // need to copy due to poor design
        for child in children {
            self.annotate_depth_subgraph(child, parent_depth + 1);
        }
    }

    // The checksum is calculated by summing up every direct +
    // indirect orbit from 'COM'.
    // The checksum can be computed by summing up the distances of each node:
    //     sum(distance(COM, node) for node in nodes
    fn get_checksum(&self) -> i32 {
        self.nodes.iter().map(|n| n.depth).sum()
    }

    // Return the parent chain for a given node.
    //
    //         G - H   J - K
    //        /       /
    // COM - B - C - D - E
    //                \
    //                 I
    // get_parent_chain('C') -> [B, COM]
    // get_parent_chain('J') -> [D, C, B, COM]
    fn get_parent_chain(&self, node_name: &str) -> Option<Vec<NodeId>> {
        let mut node = *self.name_to_id.get(node_name)?;

        let mut parent_chain = Vec::new();
        while let Some(id) = self.nodes[node].parent_id {
            parent_chain.push(id);
            node = id;
        }
        Some(parent_chain)
    }
}

fn create_map_from_input() -> std::io::Result<UniversalOrbitMap> {
    let orbits = std::fs::read_to_string("./input/day6")?
        .lines()
        .map(|line| {
            let line = line.split(')').collect::<Vec<&str>>();
            assert_eq!(line.len(), 2);
            (line[0].to_string(), line[1].to_string())
        })
        .collect::<Vec<(String, String)>>();

    let mut uom = UniversalOrbitMap::new();
    for (c, o) in orbits {
        uom.add_orbit(c, o);
    }
    Ok(uom)
}

fn part_one() -> std::io::Result<i32> {
    let mut uom = create_map_from_input()?;
    uom.annotate_depth();
    Ok(uom.get_checksum())
}

fn part_two() -> std::io::Result<i32> {
    let uom = create_map_from_input()?;

    let chain_you = uom.get_parent_chain("YOU").unwrap();
    let chain_san = uom.get_parent_chain("SAN").unwrap();

    // find first common parent of 'YOU' and 'SAN' to compute minimal number of orbit transfers
    let mut min_orbit_transfers = 0;
    for (orbit_transfer_you, id_you) in chain_you.iter().enumerate() {
        if let Some(orbit_transfer_san) = chain_san.iter().position(|id_san| id_you == id_san) {
            min_orbit_transfers = orbit_transfer_you + orbit_transfer_san;
            break;
        }
    }
    Ok(min_orbit_transfers as i32)
}

fn main() -> std::io::Result<()> {
    let checksum = part_one()?;
    println!("Part One: checksum {}", checksum);

    let transfers = part_two()?;
    println!("Part Two: minimum number of orbit transfers {}", transfers);

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let checksum = part_one().unwrap();
        assert_eq!(checksum, 314247);
    }

    #[test]
    fn test_part2() {
        let transfers = part_two().unwrap();
        assert_eq!(transfers, 514);
    }

    #[test]
    fn test_example() {
        let orbits = vec![
            ("COM", "B"),
            ("B", "C"),
            ("C", "D"),
            ("D", "E"),
            ("E", "F"),
            ("B", "G"),
            ("G", "H"),
            ("D", "I"),
            ("E", "J"),
            ("J", "K"),
            ("K", "L"),
        ];

        let mut uom = UniversalOrbitMap::new();
        for (center, orbit) in orbits {
            uom.add_orbit(center.to_string(), orbit.to_string());
        }
        uom.annotate_depth();

        assert_eq!(uom.get_checksum(), 42);
    }

    #[test]
    fn test_parent_chain() {
        //              E
        //            /
        // COM - A - B - C
        //        \
        //         D
        let orbits = vec![("COM", "A"), ("A", "B"), ("B", "C"), ("A", "D"), ("B", "E")];

        let mut uom = UniversalOrbitMap::new();
        for (center, orbit) in orbits {
            uom.add_orbit(center.to_string(), orbit.to_string());
        }

        let chain_b = uom.get_parent_chain("B");
        assert_eq!(chain_b.unwrap(), vec![1, 0]);

        let chain_c = uom.get_parent_chain("C");
        assert_eq!(chain_c.unwrap(), vec![2, 1, 0]);

        let chain_d = uom.get_parent_chain("D");
        assert_eq!(chain_d.unwrap(), vec![1, 0]);

        let chain_e = uom.get_parent_chain("E");
        assert_eq!(chain_e.unwrap(), vec![2, 1, 0]);
    }
}
