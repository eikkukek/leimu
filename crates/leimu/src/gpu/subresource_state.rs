use leimu_mem::int::UInteger;

#[derive(Clone, Copy)]
pub(super) enum StateOverwrite<Range: StateRange> {
    NoOverlap,
    Combine(Range),
    Consume(Range::MemoryBarrier),
    Cut(Range, Range, Range::MemoryBarrier),
    Shrink(Range, Range::MemoryBarrier),
}

pub(super) trait StateRange: Clone + Copy {

    type MemoryBarrier;
    type State: PartialEq + Eq + Clone + Copy;
    type SizeType: UInteger;

    fn new(
        state: Self::State,
        offset: Self::SizeType,
        size: Self::SizeType,
    ) -> Self;

    fn memory_barrier(
        src: Self::State,
        dst: Self::State,
        offset: Self::SizeType,
        size: Self::SizeType,
    ) -> Self::MemoryBarrier;

    fn offset(&self) -> Self::SizeType;

    fn size(&self) -> Self::SizeType;

    fn state(&self) -> Self::State;

    fn overwrite(&self, dst: &Self) -> StateOverwrite<Self> {
        let src_offset = self.offset();
        let src_end = src_offset + self.size();
        let dst_offset = dst.offset();
        let dst_end = dst_offset + dst.size();
        if src_offset < dst_end && dst_offset < src_end {
            if self.state() == dst.state() {
                if src_offset >= dst_offset && src_end <= dst_end {
                    StateOverwrite::Combine(*dst)
                } else if
                    dst_offset >= src_offset &&
                    dst_end <= src_end
                {
                    StateOverwrite::Combine(*self)
                } else if src_offset < dst_offset {
                    let d = dst_offset - src_offset;
                    let end = (src_offset + d).max(dst_end);
                    let size = end - src_offset;
                    StateOverwrite::Combine(Self::new(
                        self.state(),
                        src_offset,
                        size,
                    ))
                } else {
                    let d = src_offset - dst_offset;
                    let end = (dst_offset + d).max(src_end);
                    let size = end - dst_offset;
                    StateOverwrite::Combine(Self::new(
                        self.state(),
                        dst_offset,
                        size,
                    ))
                }
            } else if src_offset >= dst_offset && src_end <= dst_end {
                let src_state = self.state();
                let dst_state = dst.state();
                let memory_barrier = Self::memory_barrier(
                    src_state, dst_state,
                    src_offset,
                    self.size()
                );
                StateOverwrite::Consume(memory_barrier)
            } else if dst_offset >= src_offset && dst_end <= src_end {
                let src_state = self.state();
                let dst_state = dst.state();
                let memory_barrier = Self::memory_barrier(
                    src_state, dst_state,
                    dst_offset,
                    dst.size()
                );
                let range_left = Self::new(
                    src_state,
                    src_offset,
                    dst_offset - src_offset
                );
                let range_right = Self::new(
                    src_state,
                    dst_end,
                    src_end - dst_end
                );
                StateOverwrite::Cut(
                    range_left, range_right,
                    memory_barrier,
                )
            } else if src_offset < dst_offset {
                let src_state = self.state();
                let dst_state = dst.state();
                let memory_barrier = Self::memory_barrier(
                    src_state,
                    dst_state,
                    dst_offset,
                    src_end - dst_offset
                );
                StateOverwrite::Shrink(
                    Self::new(
                        src_state,
                        src_offset,
                        dst_offset - src_offset,
                    ),
                    memory_barrier,
                )
            } else {
                let src_state = self.state();
                let dst_state = dst.state();
                let d = dst_end - src_offset;
                let memory_barrier = Self::memory_barrier(
                    src_state, dst_state,
                    src_offset,
                    d
                );
                StateOverwrite::Shrink(
                    Self::new(
                        src_state,
                        src_offset + d,
                        self.size() - d
                    ),
                    memory_barrier,
                )
            }
        } else {
            StateOverwrite::NoOverlap
        }
    }
}
