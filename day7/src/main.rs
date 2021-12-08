use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt")
        .expect("Something went wrong reading the input file");
    let mut horizontal_positions: Vec<u32> = contents.split(',').map(|pos| pos.parse().unwrap()).collect();

    let fuel_required = part1(&mut horizontal_positions);
    println!("Fuel required to reach optimal alignment position is {}", fuel_required);

    let fuel_required_with_adjustments = part2(&horizontal_positions);
    println!("Taking into account new findings, fuel required to reach optimal alignment position is {}", fuel_required_with_adjustments);
}

/*

Trying to minimise f(k) = Σ|xⁱ-k| where xⁱ is every horizontal position and k is the alignment position

To minimise, we differentiate:      y                        The result d(|xⁱ-k|)/dk = sgn(xⁱ-k)
df(k)/dk = Σ(sgn(xⁱ-k)) = 0         |  \      / y = |xⁱ-k|   makes intuitive sense from the diagram,
where sgn(x) = { -1 if x < 0        |   \    /               in the part where k > xⁱ, the gradient
               |  0 if x = 0        |    \  /                is 1, and in the part where k < xⁱ,
               {  1 if x > 0      __|_____\/_______          it is -1.
                                    |     xⁱ     k

The solution to df(k)/dk = 0 occurs when k takes a value that has equal amounts of xⁱ<k as xⁱ>k
(since each xⁱ<k contributes a -1 to the sum, and each xⁱ>k contributes a +1 to the sum).
This value of k is the median of the xⁱs.

*/

fn part1(horizontal_positions: &mut Vec<u32>) -> u32 {
    horizontal_positions.sort_unstable();
    let median = horizontal_positions[horizontal_positions.len() / 2];
    return horizontal_positions.iter().fold(0, |accum, pos| accum + if *pos >= median  { *pos - median } else { median - *pos });
}

/*

Here, the fuel consumption of each crab (with initial pos xⁱ) is the sum of 1..|xⁱ-k|
where k is the alignment position.

Using the fact that the sum from 1..n is n(1+n) / 2
(https://en.wikipedia.org/wiki/1_%2B_2_%2B_3_%2B_4_%2B_%E2%8B%AF)

We must try to minimise f(k) = Σ|xⁱ-k|(1+|xⁱ-k|)/2
                             = 1/2 * Σ((xⁱ-k)² + |xⁱ-k|)

df(k)/dk = 1/2 * Σ(2(xⁱ-k) + sgn(xⁱ-k)) = 0
Expanding,
Σ(2xⁱ) - Σ(2k) + Σ(sgn(xⁱ-k)) = 0
2Σxⁱ - 2nk + Σ(sgn(xⁱ-k)) = 0
nk = Σxⁱ + 1/2 * Σ(sgn(xⁱ-k))
k = Σ(xⁱ)/n + 1/2 * Σ(sgn(xⁱ-k))/n

Here, Σ(xⁱ)/n is the mean of the xⁱs, x̄, and although we can't find 1/2 * Σ(sgn(xⁱ-k))/n,
we can see that Σ(sgn(xⁱ-k)) is bounded by -n <= Σ(sgn(xⁱ-k)) <= n (because sgn(xⁱ-k) is either 1 or -1),
so [1/2 * Σ(sgn(xⁱ-k))/n] is bounded by -1/2 <= [1/2 * Σ(sgn(xⁱ-k))/n] <= 1/2, and therefore
k = x̄ ± 1/2

Therefore, all we need to do to find the alignment position, is find the mean of the xⁱs

*/

fn part2(horizontal_positions: &[u32]) -> u32 {
    let mean = horizontal_positions.iter().sum::<u32>() / horizontal_positions.len() as u32;
    return horizontal_positions.iter().fold(0,
                                            |accum, pos| {
                                                let diff = if *pos >= mean { *pos - mean } else { mean - *pos };
                                                accum + (diff + diff * diff) / 2
                                            });
}