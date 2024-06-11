use itertools::Itertools;

pub fn get_truth_table(variables: usize) -> Vec<Vec<bool>>
{
	let max = 1 << variables;

	(0..max)
		.rev()
		.map(|n| bits(n, variables))
		.collect_vec() 
}

fn bits(mut n: usize, nbits: usize) -> Vec<bool>
{
	let mut bits = (0..nbits).map(|_| false).collect_vec();

	for i in 0..nbits
	{
		if (n&1) == 1 {
			bits[nbits - i - 1] = true;
		}
		n >>= 1;
	}

	return bits
}