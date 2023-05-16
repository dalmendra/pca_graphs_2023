use std::cmp::Ordering;
use std::collections::BinaryHeap;

#[derive(Clone, Debug, Eq, PartialEq)]
struct Vertex {
    name: &'static str,
    id: u32,
    distance: u32,
}

impl Ord for Vertex {
    fn cmp(&self, other: &Self) -> Ordering {
        other.distance.cmp(&self.distance)
    }
}

impl PartialOrd for Vertex {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Clone, Debug)]
struct Edge {
    source: u32,
    destination: u32,
    weight: u32,
}

pub struct Graph {
    vertices: u32,
    edges: Vec<Vec<Edge>>,
    vertex_names: Vec<&'static str>,
}

impl Graph {
    fn new(vertex_names: Vec<&'static str>) -> Self {
        let vertices = vertex_names.len() as u32;
        Graph {
            vertices,
            edges: vec![Vec::new(); vertices as usize],
            vertex_names,
        }
    }

    fn add_edge(&mut self, source: u32, destination: u32, weight: u32) {
        self.edges[source as usize].push(Edge {
            source,
            destination,
            weight,
        });
    }

    fn dijkstra(graph: &Graph, start: u32) -> Vec<u32> {
        
        // Inicializa um vetor de distâncias com valor inicial máximo para todos os vértices, exceto o vértice de origem
        let mut distances = vec![u32::MAX; graph.vertices as usize];
        
        // Cria uma fila de prioridade para armazenar os vértices e respectivas distâncias estimadas
        let mut priority_queue = BinaryHeap::new();
    
        // Define a distância do vértice de origem como 0 e o inclui na fila de prioridade
        distances[start as usize] = 0;
        priority_queue.push(Vertex {
            name: graph.vertex_names[start as usize],
            id: start,
            distance: 0,
        });
        println!("Distancias iniciais: {:?}", distances);

        // Loop principal do algoritmo Dijkstra. Roda enquanto houver vértices a serem visitados.
        while let Some(Vertex {name, id, distance }) = priority_queue.pop() {
            
            // Verifica se a distância que consta no vértice é maior que a distância armazenada no vetor distances
            // Caso positivo, o vértice já foi alcançado com uma distância menor, então ignora essa iteração
            println!("Novo loop while - id: {:?}, distance: {:?}, distancia armaz. no vetor: {:?}", id, distance, distances[id as usize]);
            if distance > distances[id as usize] {
                println!("Distancia>Distances");
                continue;
            }
    
            // Percorre as arestas saindo do vértice atual
            for edge in &graph.edges[id as usize] {
                // Calcula a nova distância considerando a distância atual e o peso da aresta
                let new_distance = distance + edge.weight;

                println!("Aresta sendo analisada: {:?}", edge);
    
                // Se a nova distância for menor que a distância armazenada no vetor para o vértice de destino,
                // a distância é atualizada (relaxamento) e adiciona o vértice de destino à fila de prioridade para futura visita
                if new_distance < distances[edge.destination as usize] {
                    distances[edge.destination as usize] = new_distance;
                    priority_queue.push(Vertex {
                        name: graph.vertex_names[edge.destination as usize],
                        id: edge.destination,
                        distance: new_distance,
                    });
                }
            }
            // Mostra o vetor de distâncias após cada iteração do loop for
            println!("Vetor distances após o loop: {:?}", distances);
        }
    
        // Retorna o vetor de distâncias final
        distances
    }
}


#[cfg(test)]
mod test_dijkstra {
    use std::i32::MAX;

    use crate::{Graph};

    #[test]
    pub fn test_dijkstra_dir() {
        let vertex_names = vec!["s", "t", "x", "y", "z"];
        let mut graph = Graph::new(vertex_names);
        graph.add_edge(0, 1, 10);
        graph.add_edge(0, 3, 5);
        graph.add_edge(1, 2, 1);
        graph.add_edge(1, 3, 2);
        graph.add_edge(2, 4, 4);
        graph.add_edge(3, 1, 3);
        graph.add_edge(3, 2, 9);
        graph.add_edge(3, 4, 2);
        graph.add_edge(4, 0, 7);
        graph.add_edge(4, 2, 6);

        let start_vertex = 0;
        let distances = Graph::dijkstra(&graph, start_vertex);

        for (index, distance) in distances.iter().enumerate() {
            if distance == &u32::MAX {
                println!("Distance from {} to {}: unreachable", start_vertex, index)
            }
            else {
                println!("Distance from {} to {}: {}", graph.vertex_names[start_vertex as usize], graph.vertex_names[index], distance);
            }
        }
    }
    
    #[test]
    pub fn test_dijkstra_nao_dir() {
        let vertex_names = vec!["A", "B", "C", "D", "E", "F", "G"];
        let mut graph = Graph::new(vertex_names);
        graph.add_edge(0, 1, 7);
        graph.add_edge(0, 2, 5);
        graph.add_edge(0, 3, 2);
        graph.add_edge(1, 0, 7);
        graph.add_edge(1, 4, 3);
        graph.add_edge(1, 5, 8);
        graph.add_edge(2, 0, 5);
        graph.add_edge(2, 3, 10);
        graph.add_edge(2, 4, 4);
        graph.add_edge(3, 0, 2);
        graph.add_edge(3, 2, 10);
        graph.add_edge(3, 5, 2);
        graph.add_edge(4, 1, 3);
        graph.add_edge(4, 2, 4);
        graph.add_edge(4, 5, 6);
        graph.add_edge(5, 1, 8);
        graph.add_edge(5, 3, 2);
        graph.add_edge(5, 4, 6);

        let start_vertex = 4;
        let distances = Graph::dijkstra(&graph, start_vertex);

        for (index, distance) in distances.iter().enumerate() {
            if distance == &u32::MAX {
                println!("Distance from {} to {}: unreachable", start_vertex, index)
            }
            else {
                println!("Distance from {} to {}: {}", graph.vertex_names[start_vertex as usize], graph.vertex_names[index], distance);
            }
        }
    }
  
}
