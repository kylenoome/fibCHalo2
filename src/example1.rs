use std::marker::PhantomData;

use halo2_proofs::{
    arithmetic::FieldExt,
    circuit::*,
    plonk::*
};

#[derive(Debug, Clone)]
struct FiboConfig{
    pub adive: [Column<Adivice>; 3],
    pub selector: Selector,
}

struct FiboChip<F: FieldExt> {
    config: FiboConfig,
    _marker: PhantomData<F>,
}

// Add funtions for fiboChip
impl<F: FieldExt> FiboChip<F> {
    fn contruct(config: FiboConfig) -> Self {
        Self { config, _marker: PhantomData }
    }

    // Define custome gate here
    fn config(meta: &mut ConstraintSystem<F>) -> FiboConfig {
        let col_a = meta.advice_column();
        let col_b = meta.advice_column();
        let col_c = meta.advice_column();
        let selector = meta.selector();

        meta.create_gate("add", |meta| {
            //
            // col_a | col_b | col_c | selector
            //   a      b        c       s
            //
            let s = meta.query_selector(selector);
            let a = meta.query_advice(col_a, Rotation::cur());
            let b = meta.query_advice(col_b, Rotation::cur());
            let c = meta.query_advice(col_c, Rotation::cur());
            vec![s * (a + b - c)]
        });
    }
    
}


fn main() {
    println!("Hello, world!");
}
