在使用多线程的 Future 执行器时，一个 Future 可能在线程间移动，所以任何在 async 体中使用的变量必须能够穿过线程，
因为任何 .await 都有可能导致线程切换。