
fn add_edge(mut _adj: Vec<Vec<u32>>, _v: u32, _w: u32) -> Vec<Vec<u32>> {
    _adj[_v as usize].push(_w);
    _adj
}

fn breadth_first_search(mut _adj: Vec<Vec<u32>>, mut _s: u32)  -> Vec<Vec<u32>> {
    
    // Mark all the vertices as not visited(By default 
    // set as false) 
    let mut visited: [bool; 4] = [false, false, false, false];
    
    // Create queue for BFS
    let mut _queue: Vec<u32> = Vec::new();

    // Mark the current node as visited and enqueue it
    visited[_s as usize] = true;
    _queue.push(_s);

    while _queue.len() != 0 {
        // Dequeue a vertex from queue and print it        
        _s = _queue.pop().unwrap();
        println!("{} test {}", _s, _s);

        // Get all adjacent vertices of the dequeued vertex s
        // If a adjacent has not been visited, then mark it
        // visited and enqueue it
        let iterator = _adj[_s as usize].iter();
        for n in iterator {
            if !visited[*n as usize] {
                visited[*n as usize] = true; 
                _queue.push(*n); 
            }
        }
    }

    _adj
}

fn main() {    
    let mut _adj: Vec<Vec<u32>> = Vec::new();
    
    for _x in 0..4 {
        _adj.push(Vec::new());
    }

    _adj = add_edge(_adj, 0, 1);
    _adj = add_edge(_adj, 0, 2);
    _adj = add_edge(_adj, 1, 2);
    _adj = add_edge(_adj, 2, 0);
    _adj = add_edge(_adj, 2, 3);
    _adj = add_edge(_adj, 3, 3);

    breadth_first_search(_adj, 2);
}