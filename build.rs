fn main() {
    unsafe { std::env::set_var("CMAKE_POLICY_VERSION_MINIMUM", "3.6"); }
    // Check if intel-mkl feature is enabled
    if cfg!(feature = "intel-mkl") {
        // Check if current architecture is x86 or x86_64
        let is_intel_arch = cfg!(any(target_arch = "x86", target_arch = "x86_64"));

        // If current architecture is not Intel but intel-mkl feature is enabled, emit a compile error
        if !is_intel_arch {
            panic!(
                "intel-mkl feature can only be used on x86 or x86_64 architectures"
            );
        }
    }
}
