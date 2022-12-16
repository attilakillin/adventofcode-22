use std::{vec, collections::{HashMap, HashSet}};

#[derive(Debug)]
pub struct Node {
    name: String,
    flow_rate: usize,
    tunnels: Vec<String>
}

#[derive(Debug)]
pub struct Graph {
    nodes: HashMap<String, Node>
}

impl Graph {
    pub fn new(lines: &[String]) -> Self {
        let mut nodes = HashMap::new();
        
        for line in lines {
            let head: Vec<String> = line.replace("Valve ", "").split(" has flow rate=").map(|s| s.to_string()).collect();
            let tail: Vec<String> = if head[1].contains("tunnels") {
                head[1].split("; tunnels lead to valves ").map(|s| s.to_string()).collect()
            } else {
                head[1].split("; tunnel leads to valve ").map(|s| s.to_string()).collect()
            };

            let mut tunnels = vec![];
            for tunnel in tail[1].split(", ") {
                tunnels.push(tunnel.to_string());
            }

            nodes.insert(head[0].to_string(), Node {
                name: head[0].to_string(),
                flow_rate: tail[0].parse().unwrap(),
                tunnels
            });
        }

        return Graph { nodes };
    }

    pub fn solve_task(&self, time: usize) -> usize {
        // 2D table, each value contains our largest pressure release where
        // the Y coordinate (outer index) is the elapsed time in minutes - 1,
        // the X coordinate (inner index, or hash key) is the name of the node.
        // Each entry contains a bool (whether this cell can be reached, starting from "AA")
        // a HashSet, containing the list of opened valves and an usize (which is the pressure release value).
        let mut table: Vec<HashMap<String, (bool, HashSet<String>, usize)>> = vec![HashMap::new(); time + 1];

        println!("Simulating minute 1...");

        // In zero minutes, every value is 0, and only AA can be reached.
        for (name, _) in &self.nodes {
            table[0].insert(name.to_string(), (name == "AA", HashSet::new(), 0));
        }

        // In the remaining minutes.
        for min in 1..=time {
            println!("Simulating minute {}...", min);

            for (name, node) in &self.nodes {
                // One way to raise our release value is to open the valve in the node we're in.
                // If it was already opened, we must have moved in the previous step, so route_one
                // is not applicable.
                let prev = table[min - 1].get(name).unwrap();
                let route_one = if prev.0 && !prev.1.contains(&node.name) {
                    let mut set = prev.1.clone();
                    set.insert(name.clone());
                    (true, set, prev.2 + node.flow_rate * (time - min))
                } else {
                    (false, HashSet::new(), 0)
                };

                // The other way is to check each cell in the previous row, and check if by moving here
                // instead of opening this valve, we reach a higher pressure release value.
                let mut route_two = (false, HashSet::new(), 0);
                for (src_name, src_node) in &self.nodes {
                    if src_node.tunnels.contains(name) {
                        let value = table[min - 1].get(src_name).unwrap();
                        if value.0 && value.2 >= route_two.2 {
                            route_two = value.clone();
                        }
                    }
                }

                let best = [route_one, route_two].iter()
                    .filter(|r| r.0)
                    .max_by(|r1, r2| r1.2.cmp(&r2.2))
                    .unwrap_or(&(false, HashSet::new(), 0)).clone();
                table[min].insert(name.to_string(), best);
            }
        }

        /*for row in table.iter().rev() {
            for (k, v) in row {
                print!("| {} {} {} ", k, v.0, v.2);
            }
            println!();
        }

        println!("{:?}", table[time - 1].iter().max_by(|a, b| a.1.2.cmp(&b.1.2)).unwrap());*/
        return table[time - 1].iter().max_by(|a, b| a.1.2.cmp(&b.1.2)).unwrap().1.2;
    }
}
