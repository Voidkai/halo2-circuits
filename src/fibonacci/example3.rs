use halo2_proofs::{ circuit::*, plonk::*, poly::Rotation};
use std::marker::PhantomData;
use halo2_proofs::arithmetic::Field;

#[derive(Debug, Clone)]
struct ACell<F: Field>(AssignedCell<F, F>);

// Fibonacci variant: x_1*x_2+x_2*x_3=x_4
#[derive(Debug, Clone)]
struct FibonacciConfig {
    advice: Column<Advice>,
    selector: Selector,
    instance: Column<Instance>,
}

#[derive(Debug, Clone)]
struct FibonacciChip<F: Field> {
    config: FibonacciConfig,
    _marker: PhantomData<F>,
}

impl<F: Field> FibonacciChip<F> {
    pub fn construct(config: FibonacciConfig) -> Self {
        Self {
            config,
            _marker: PhantomData,
        }
    }

    pub fn configure(
        meta: &mut ConstraintSystem<F>,
        // advice: Column<Advice>,
        // instance: Column<Instance>,
    ) -> FibonacciConfig {
        let selector = meta.selector();
        let advice = meta.advice_column();
        let instance = meta.instance_column();
        meta.enable_equality(advice);
        meta.enable_equality(instance);

        meta.create_gate("add", |meta| {
            //
            // advice | selector
            //   a    |   s
            //   b    |
            //   c    |
            //
            let s = meta.query_selector(selector);
            let a = meta.query_advice(advice, Rotation::cur());
            let b = meta.query_advice(advice, Rotation::next());
            let c = meta.query_advice(advice, Rotation(2));
            let d = meta.query_advice(advice, Rotation(3));
            vec![s * (a *b.clone() + b * c - d)]
        });

        FibonacciConfig {
            advice,
            selector,
            instance,
        }
    }

    pub fn assign(
        &self,
        mut layouter: impl Layouter<F>,
        nrows: usize,
    ) -> Result<AssignedCell<F, F>, Error> {
        layouter.assign_region(
            || "entire fibonacci table",
            |mut region| {
                self.config.selector.enable(&mut region, 0)?;

                let mut a_cell = region.assign_advice_from_instance(
                    || "1",
                    self.config.instance,
                    0,
                    self.config.advice,
                    0,
                )?;
                let mut b_cell = region.assign_advice_from_instance(
                    || "2",
                    self.config.instance,
                    1,
                    self.config.advice,
                    1,
                )?;

                let mut c_cell = region.assign_advice_from_instance(
                    || "3",
                    self.config.instance,
                    2,
                    self.config.advice,
                    2,
                )?;
                for row in 1..nrows{
                    if row < nrows - 3 {
                        self.config.selector.enable(&mut region, row)?;
                    }
                }

                for row in 3..nrows {
                    let d_cell = region.assign_advice(
                        || "advice",
                        self.config.advice,
                        row,
                        || a_cell.value().copied() * b_cell.value().copied() + b_cell.value().copied()*c_cell.value().copied(),
                    )?;

                    a_cell = b_cell;
                    b_cell = c_cell;
                    c_cell = d_cell;
                }

                Ok(c_cell)
            },
        )
    }

    pub fn expose_public(
        &self,
        mut layouter: impl Layouter<F>,
        cell: AssignedCell<F, F>,
        row: usize,
    ) -> Result<(), Error> {
        layouter.constrain_instance(cell.cell(), self.config.instance, row)
    }
}

#[derive(Default)]
struct MyCircuit<F>(PhantomData<F>);

impl<F: Field> Circuit<F> for MyCircuit<F> {
    type Config = FibonacciConfig;
    type FloorPlanner = SimpleFloorPlanner;

    fn without_witnesses(&self) -> Self {
        Self::default()
    }

    fn configure(meta: &mut ConstraintSystem<F>) -> Self::Config {
        FibonacciChip::configure(meta)
    }

    fn synthesize(
        &self,
        config: Self::Config,
        mut layouter: impl Layouter<F>,
    ) -> Result<(), Error> {
        let chip = FibonacciChip::construct(config);

        let out_cell = chip.assign(layouter.namespace(|| "entire table"), 6)?;
        chip.expose_public(layouter.namespace(|| "out"), out_cell, 3)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::MyCircuit;
    use std::marker::PhantomData;
    use halo2_proofs::{dev::MockProver, halo2curves::pasta::Fp};

    #[test]
    fn fibonacci_example3() {
        let k = 4;

        let a = Fp::from(1); // F[0]
        let b = Fp::from(2); // F[1]
        let c = Fp::from(3);
        let out = Fp::from(264); // F[5]

        let circuit = MyCircuit(PhantomData);

        let mut public_input = vec![a, b, c, out];

        let prover = MockProver::run(k, &circuit, vec![public_input.clone()]).unwrap();
        prover.assert_satisfied();

        // public_input[2] += Fp::one();
        // let _prover = MockProver::run(k, &circuit, vec![public_input]).unwrap();
        // //uncomment the following line and the assert will fail
        // _prover.assert_satisfied();
    }

    #[cfg(feature = "dev-graph")]
    #[test]
    fn plot_fibo2() {
        use plotters::prelude::*;
        use halo2_proofs::dev::CircuitLayout;

        let root = BitMapBackend::new("fib-2-layout.png", (1024, 3096)).into_drawing_area();
        root.fill(&WHITE).unwrap();
        let root = root.titled("Fib 2 Layout", ("sans-serif", 60)).unwrap();

        let circuit = MyCircuit::<Fp>(PhantomData);
        halo2_proofs::dev::CircuitLayout::default()
            .render(4, &circuit, &root)
            .unwrap();
    }
}
