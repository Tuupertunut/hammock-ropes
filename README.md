Have you ever wondered where you should tie permanent [alpine butterfly loops](https://www.animatedknots.com/alpine-butterfly-loop-knot) (knots) in your hammock ropes to maximize the range where you can adjust their length? Ideally you want to be able to adjust the hammock length by small intervals by attaching the ropes in a different configuration, but also maximize the total length range where you can place it. At first this may sound like a simple problem until you consider that you can make different amounts of knots, with different intervals, turn either rope either end to the tree, use any knot pair in a P shaped (one end to hammock and rope through the other near the tree) or U shaped (both ends attached to the hammock) configuration and do all of this to either rope separately to reach the desired length. Mathematically this problem is interesting because you want to minimize the symmetry of your knots to maximize adjustment capabilities.

This tool will run a parallelized brute force search to give you the answer. Inputs are:

- ROPE_LENGTH: How long your ropes are without any extra knots. The ends are assumed to have loops anyway.
- KNOT_LENGTH: How much one alpine butterfly loop knot takes off the length of the rope.

- TREE_CIRCUMFERENCE: What size of trees you want to tie the hammock to. (Both trees are assumed equal size.)
- MAX_KNOTS: What is the maximum number of knots you are willing to tie to one rope. The ends are assumed to have loops anyway. (The calculation will become extremely slow for any value above 3.)
- MAX_INTERVAL: The maximum length difference between two configurations of ropes in the solution.

- KNOT_STEPS: How many possible knot positions along the ropes the algorithm will check. Higher values are slower but give more accurate results.