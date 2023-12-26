# https://github.com/hyper-neutrino/advent-of-code/blob/main/2023/day25.py
import networkx as nx

g = nx.Graph()

for line in open('input'):
    left, right = line.split(":")
    for node in right.strip().split():
        g.add_edge(left, node)
        g.add_edge(node, left)

g.remove_edges_from(nx.minimum_edge_cut(g))
a, b = nx.connected_components(g)

print(len(a) * len(b))
