# Overview of the old and new traits

This document aims to give a mapping from the old traits (in v0.8) and their methods to the new (core) traits introduced for v0.9 and beyond.
The envisaged uses of this documentation are (so far):
- To aid in porting code to the new traits
- To keep track of the functionality not (yet) covered by the new traits

# The mapping

## Data

| old                 | new                 |
|---                  |---                  |
| `GraphBase::NodeId`   | `Graph::NodeId`       |
| `GraphBase::EdgeId`   | `Graph::EdgeId`       |
| | |
| `Data::NodeWeight`    | `Graph::NodeData`     |
| `Data::EdgeWeight`    | `Graph::EdgeData`     |
|                     |                     |
| `EdgeRef` | `Edge`/`EdgeRef` |
| `NodeRef` | `Node`/`NodeRef` |
| | |
| `GraphRef`            | -                   |
|                     |                     |

## Nodes/Edges/Neighbours

| old | new |
|---  |---  |
| `IntoNeighbours::neighbours()`  | `{DirectedGraph/UndirectedGraph}::adjacencies()` |
| | |
| `IntoNeighboursDirected::neighbours_directed()` | `{DirectedGraph/UndirectedGraph}{::predecessors()/::successors()}` |
| | |
| `IntoNodeIdentifiers::node_identifiers()` | `{DirectedGraph/UndirectedGraph}::nodes()` (`.id`) |
| `IntoNodeReferences::node_references()` | `{DirectedGraph/UndirectedGraph}::nodes()` |
| `IntoEdgeReferences::edge_references()` | `{DirectedGraph/UndirectedGraph}::edges()` |
| | |
| `IntoEdges::edges()` | `{DirectedGraph/UndirectedGraph}::incident_edges()` |
| | |

## Directedness

| old | new |
|---  |---  |
| `GraphProp::is_directed()` | Directedness is now expressed by the traits themselves (`DirectedGraph`/`UndirectedGraph`) |

## Indexable/Storable

| old | new |
|---  |---  |
| `NodeIndexable::node_bound()` | Use case now covered by the use of `Storable` trait |
| `NodeIndexable::to_index()` | ? |
| `NodeIndexable::from_index()` | ? |
| `EdgeIndexable::edge_bound()` | Use case now covered by the use of `Storable` trait |
| `EdgeIndexable::to_index()` | ? |
| `EdgeIndexable::from_index()` | ? |

## Counting

| old | new |
|---  |---  |
| `NodeCount::node_count()` | `{DirectedGraph/UndirectedGraph}::node_count()` |
| `EdgeCount::edge_count()` | `{DirectedGraph/UndirectedGraph}::edge_count()` |

## Visiting

| old | new |
|---  |---  |
| `VisitMap::visit()` | `VisitContainer::mark_visited()` |
| `VisitMap::unvisit()` | `VisitContainer::mark_unvisited()` |
| `VisitMap::is_visited()` | `VisitContainer::is_visited()` |
| | |
| `Visitable::visit_map()` | `Storable::{node_visit_container()/edge_visit_container()}` |
| `Visitable::reset_map()` | `VisitContainer::clear()` |

## Matrix Representation

| old | new |
|---  |---  |
| `GetAdjacencyMatrix::adjacency_matrix()` | - |

## Miscellaneous

| old | new |
|---  |---  |
| `FilterNode` | - |
| `FilterEdge` | - |
| `Measure` | - |
| `FloatMeasure` | - |
| `BoundedMeasure` | - |
| `UnitMeasure` | - |
| `PositiveMeasure` | - |
