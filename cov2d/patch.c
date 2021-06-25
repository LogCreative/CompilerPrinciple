// Patch for compiling TVM 0.6.1 is from
// https://discuss.tvm.apache.org/t/make-fails-during-installation-on-arm-error-amdgcn-s-barrier-is-not-a-member-of-llvm-intrinsic/6685/8

// Find file llvm_common.h
// Add the following:

#if TVM_LLVM_VERSION >= 100
#include <llvm/IR/IntrinsicsAMDGPU.h>
#include <llvm/IR/IntrinsicsARM.h>
#include <llvm/IR/IntrinsicsNVPTX.h>
#include <llvm/IR/IntrinsicsX86.h>
#endif

// This problem has been patched in 0.7.0
// However, since the source code provided is based on the legacy API
// The choice should be 0.6.1 to be consist with the guide.