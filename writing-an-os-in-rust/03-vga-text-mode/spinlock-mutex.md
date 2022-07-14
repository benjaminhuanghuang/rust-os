# Spinlocks

## spinlock 的适用情况

自旋锁比较适用于锁使用者保持锁时间比较短的情况。正是由于自旋锁使用者一般保持锁时间非常短，因此选择自旋而不是睡眠是非常必要的，自旋锁的效率远高于互斥锁

信号量和读写信号量适合于保持时间较长的情况，它们会导致调用者睡眠，因此只能在进程上下文使用，而自旋锁适合于保持时间非常短的情况，它可以在任何上下文使用。如果被保护的共享资源只在进程上下文访问，使用信号量保护该共享资源非常合适，如果对共享资源的访问时间非常短，自旋锁也可以。但是如果被保护的共享资源需要在中断上下文访问（包括底半部即中断处理句柄和顶半部即软中断），就必须使用自旋锁。自旋锁保持期间是抢占失效的，而信号量和读写信号量保持期间是可以被抢占的。自旋锁只有在内核可抢占或 SMP（多处理器）的情况下才真正需要，在单 CPU 且不可抢占的内核下，自旋锁的所有操作都是空操作。另外格外注意一点：自旋锁不能递归使用。

## spinlock 与 mutex 对比

Mutex provides mutual exclusion by blocking threads when the resource is already locked.

Spinlock: instead of blocking, the threads simply try to lock it again and again in a tight loop and thus burn CPU time until the mutex is free again.

spinlock 不会使线程状态发生切换，mutex 在获取不到锁的时候会选择 sleep

mutex 获取锁分为两阶段，第一阶段在用户态采用 spinlock 锁总线的方式获取一次锁，如果成功立即返回；否则进入第二阶段，调用系统的 futex 锁去 sleep，当锁可用后被唤醒，继续竞争锁。

Spinlock 优点：没有昂贵的系统调用，一直处于用户态，执行速度快
Spinlock 缺点：一直占用 cpu，而且在执行过程中还会锁 bus 总线，锁总线时其他处理器不能使用总线

Mutex 优点：不会忙等，得不到锁会 sleep
Mutex 缺点：sleep 时会陷入到内核态，需要昂贵的系统调用
