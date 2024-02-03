use rust_hdl::prelude::*;

#[derive(LogicBlock)]
pub struct Counter<const N: usize> {
    pub clock: Signal<In, Clock>,
    pub reset: Signal<In, Bit>,
    pub out: Signal<Out, Bits<N>>,
    value: DFF<Bits<N>>,
}

impl<const N: usize> Logic for Counter<N> {
    #[hdl_gen]
    fn update(&mut self) {
        dff_setup!(self, clock, value);

        if self.reset.val() {
            self.value.d.next = 0.into();
        } else {
            self.value.d.next = self.value.q.val() + 1;
        }
        self.out.next = self.value.q.val();
    }
}
impl<const N: usize> Default for Counter<N> {
    fn default() -> Self {
        Self {
            clock: Signal::default(),
            reset: Default::default(),
            out: Default::default(),
            value: Default::default(),
        }
    }
}
