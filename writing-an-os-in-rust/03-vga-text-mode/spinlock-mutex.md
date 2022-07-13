Spinlocks

## spinlock 与 mutex 对比

spinlock 不会使线程状态发生切换，mutex 在获取不到锁的时候会选择 sleep

mutex 获取锁分为两阶段，第一阶段在用户态采用 spinlock 锁总线的方式获取一次锁，如果成功立即返回；否则进入第二阶段，调用系统的 futex 锁去 sleep，当锁可用后被唤醒，继续竞争锁。

Spinlock 优点：没有昂贵的系统调用，一直处于用户态，执行速度快
Spinlock 缺点：一直占用 cpu，而且在执行过程中还会锁 bus 总线，锁总线时其他处理器不能使用总线

Mutex 优点：不会忙等，得不到锁会 sleep
Mutex 缺点：sleep 时会陷入到内核态，需要昂贵的系统调用
