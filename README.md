# clifford

* https://doi.org/10.1007/s00006-019-0987-7
* https://arxiv.org/abs/1607.04767
* http://versor.mat.ucsb.edu/ArticulatingSpace.pdf

0->0, 1->1,2->2,3->4,4->3,5->6->5

fn phi(x: usize) -> usize;
fn omega(x: usize) -> usize;

forall x. assert_eq!(x, phi(omega(x)));

fn outer(a: MV, b: MV) -> MV {
	let mut c = MV::zero();
	for i in 0..a.len() {
		for j in 0..b.len() {
			c[phi(i ^ j)] = a[omega(i)] * a[omega(j)];
		}
	}
	c
}

fn outer(a: MV, b: MV) -> MV {
	let mut c = MV::zero();
	for i in 0..a.len() {
		for j in 0..b.len() {
			c[phi(i & j)] = a[omega(i)] * a[omega(j)];
		}
	}
	c
}
