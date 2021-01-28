use super::*;
use assert2::assert;

#[test]
fn empty_input_data_returns_count_of_0() {
    // Given
    let expected_res = 0;
    let input_data = "";

    // When
    let res = word_count(input_data.to_string());

    // Then
    assert!(res == expected_res);
}

#[test]
fn input_data_with_1_word_returns_count_of_1() {
    // Given
    let expected_res = 1;
    let input_data = "hello";

    // When
    let res = word_count(input_data.to_string());

    // Then
    assert!(res == expected_res);
}

#[test]
fn input_data_with_lots_of_data_count_of_221() {
    // Given
    let expected_res = 221;
    let input_data = r#"
#
# A fatal error has been detected by the Java Runtime Environment:
#
#  SIGSEGV (0xb) at pc=0x00007feb77c5ea26, pid=38505, tid=39022
#
# JRE version: OpenJDK Runtime Environment JBR-11.0.9.1.11-1145.63-jcef (11.0.9.1+11) (build 11.0.9.1+11-b1145.63)
# Java VM: OpenJDK 64-Bit Server VM JBR-11.0.9.1.11-1145.63-jcef (11.0.9.1+11-b1145.63, mixed mode, tiered, compressed oops, concurrent mark sweep gc, linux-amd64)
# Problematic frame:
# C  [libc.so.6+0x44a26]
#
# No core dump will be written. Core dumps have been disabled. To enable core dumping, try "ulimit -c unlimited" before starting Java again
#
# If you would like to submit a bug report, please visit:
#   https://bugreport.java.com/bugreport/crash.jsp
# The crash happened outside the Java Virtual Machine in native code.
# See problematic frame for where to report the bug.
#

---------------  S U M M A R Y ------------

Command Line: -Xss2m -Xms256m -Xmx2000m -XX:NewSize=128m -XX:MaxNewSize=128m -XX:+UseConcMarkSweepGC -XX:SoftRefLRUPolicyMSPerMB=50 -XX:CICompilerCount=2 -XX:+HeapDumpOnOutOfMemoryError -XX:-OmitStackTraceInFastThrow -ea -Djdk.http.auth.tunneling.disabledSchemes="" -Djdk.attach.allowAttachSelf=true -Djdk.module.illegalAccess.silent=true -Dkotlinx.coroutines.debug=off -Dsun.tools.attach.tmp.only=true -Dide.no.platform.update=true -Dsun.io.useCanonCaches=false -XX:ReservedCodeCacheSize=512m -Didea.plugins.path=/home/brad/.local/share/JetBrains/Toolbox/apps/CLion/ch-0/203.6682.181.plugins -XX:ErrorFile=/home/brad/java_error_in_clion_%p.log -XX:HeapDumpPath=/home/brad/java_error_in_clion_.hprof -Didea.vendor.name=JetBrains -Didea.paths.selector=CLion2020.3 -Djb.vmOptionsFile=/home/brad/.local/share/JetBrains/Toolbox/apps/CLion/ch-0/203.6682.181.vmoptions -Didea.platform.prefix=CLion com.intellij.idea.Main

Host: AMD Ryzen Threadripper 1950X 16-Core Processor, 32 cores, 62G, Pop!_OS 20.10
Time: Thu Jan 21 04:46:24 2021 PST elapsed time: 60998.815475 seconds (0d 16h 56m 38s)

---------------  T H R E A D  ---------------

Current thread (0x00007feaf00c8000):  JavaThread "AWT-XAWT" daemon [_thread_in_native, id=39022, stack(0x00007feae63f5000,0x00007feae65f6000)]

Stack: [0x00007feae63f5000,0x00007feae65f6000],  sp=0x00007feae65f4380,  free space=2044k
Native frames: (J=compiled Java code, A=aot compiled Java code, j=interpreted, Vv=VM code, C=native code)
"#;

    // When
    let res = word_count(input_data.to_string());

    // Then
    assert!(res == expected_res);
}
