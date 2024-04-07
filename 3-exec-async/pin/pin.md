关于pin的总结

- Pin 类型包装了指针类型, 保证没有实现 Unpin 指针指向的值不会被移动。被固定后不能再移动的类型有一个标记 trait !Unpin。
- 获取已经被固定的 T 类型示例的 &mut T需要 unsafe
- 固定 !Unpin 对象到栈上需要 unsafe
- 固定 !Unpin 对象到堆上不需要 unsafe。Box::pin可以快速完成这种固定