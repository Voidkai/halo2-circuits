// use group::ff::Field;
// use halo2_proofs::circuit::AssignedCell;
//
// pub enum PaddedWord<F:Field>{
//     Message(AssignedCell<F,F>),
//     Padding(F),
// }
//
// #[cfg(test)]
// mod tests{
//     use group::ff::Field;
//     use halo2_proofs::circuit::AssignedCell;
//     use crate::hash::poseidon::PaddedWord;
//
//     fn get_padded_word() -> PaddedWord<dyn Field<Output=()>> {
//         let message = AssignedCell::new_input();
//         let padding = AssignedCell::new_input();
//         PaddedWord::Message(message)
//     }
// }