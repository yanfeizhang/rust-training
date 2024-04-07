


关于pin的总结
- 被固定后不能再移动的类型有一个标记 trait !Unpin。
- 固定保证了实现了 !Unpin trait 的对象不会被移动
- 获取已经被固定的 T 类型示例的 &mut T需要 unsafe
- 固定 !Unpin 对象到栈上需要 unsafe
- 固定 !Unpin 对象到堆上不需要 unsafe。Box::pin可以快速完成这种固定