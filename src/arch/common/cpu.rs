pub trait CpuOps {
    fn enter_critical_section();
    fn exit_critical_section();
    fn switch_context(from: &mut TaskContext, to: &TaskContext);
    // ... 其他 CPU 操作接口
}