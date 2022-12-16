#[derive(Clone, Copy, PartialEq, Eq)]
struct State {
    cost: usize,
    position: usize,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.cost.cmp(&other.cost)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn reconstruct_path(prev: &[usize], from: usize, to: usize) -> Vec<usize> {
    let mut current = to;
    let mut path = vec![];

    while current != from {
        path.push(current);
        current = prev[current];
    }

    path
}

// Dijkstra
fn find_path(system: &ValveSystem, from: usize, to: usize) -> Option<Vec<usize>> {
    if from == to {
        return Some(vec![]);
    }

    let vertices = system.valves.len();

    let mut prev: Vec<_> = (0..vertices).map(|_| vertices).collect();

    let mut dist: Vec<_> = (0..vertices).map(|_| usize::MAX).collect();
    let mut queue = BinaryHeap::new();

    dist[from] = 0;
    queue.push(State {
        cost: 0,
        position: from,
    });

    while let Some(State { cost, position }) = queue.pop() {
        if position == to {
            return Some(reconstruct_path(&prev, from, to));
        }

        if cost > dist[position] {
            continue;
        }

        for edge in &system.tunnels[position] {
            let next = State {
                cost: cost + 1,
                position: *edge,
            };

            if next.cost < dist[next.position] {
                queue.push(next);
                dist[next.position] = next.cost;
                prev[next.position] = position;
            }
        }
    }

    None
}
