use aoc::input::*;
use petgraph::graph::{ UnGraph, NodeIndex };
use petgraph::dot::{ Dot, Config };
use petgraph::algo::{ kosaraju_scc };
use std::collections::HashMap;

fn parse_components(input: Vec< String >) -> (UnGraph< String, String >, HashMap< String, NodeIndex >) {
    let mut graph = UnGraph::< String, String >::new_undirected();
    let mut nodes = HashMap::< String, NodeIndex >::new();
    for line in input {
        let splits = line.split(": ").collect::< Vec< &str > >();
        let src = splits[0].to_string();
        let trgs = splits[1].split(" ").map(|x| x.to_string()).collect::< Vec< String > >();
        // I wanted a lambda that inserts the nodes but mutable borrows hate me :/
        let src_ix = if let Some(ix) = nodes.get(&src) {
            *ix
        } else {
            let ix = graph.add_node(src.to_string());
            nodes.insert(src.to_string(), ix);
            ix
        };
        for trg in trgs {
            let trg_ix = if let Some(ix) = nodes.get(&trg) {
                *ix
            } else {
                let ix = graph.add_node(trg.clone());
                nodes.insert(trg, ix);
                ix
            };
            graph.add_edge(src_ix, trg_ix, "".to_string());
        }
    }
    return (graph, nodes);
}

fn main() {
    let (mut graph, nodes) = parse_components(get_lines("inputs/day25.txt"));
    //println!("{:?}", Dot::with_config(&graph, &[Config::EdgeNoLabel]));
    // dot -Kneato -Tsvg
    graph.remove_edge(graph.find_edge(*nodes.get("fht").unwrap(), *nodes.get("vtt").unwrap()).unwrap());
    graph.remove_edge(graph.find_edge(*nodes.get("bbg").unwrap(), *nodes.get("kbr").unwrap()).unwrap());
    graph.remove_edge(graph.find_edge(*nodes.get("czs").unwrap(), *nodes.get("tdk").unwrap()).unwrap());
    println!("{}", kosaraju_scc(&graph).iter().map(|scc| scc.len()).product::< usize >());
}
